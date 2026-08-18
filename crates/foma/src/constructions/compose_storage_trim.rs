//! External coaccessibility trim for a spilled composition product.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::sigma::sigma_create;
use crate::structures::{fsm_empty, fsm_sigma_destroy};

use super::super::{NO, Tern, YES, fsm_update_flags};
use super::output::{ComposeOutputMetadata, SpilledOutput, apply_metadata, sentinel};
use super::{FomaError, Fsm, FsmState, io_error, state_from_bytes, state_to_bytes};

const EDGE_BYTES: usize = 8;
const STATE_BYTES: usize = 14;
const PAGE_BYTES: usize = 64 * 1024;
const CACHE_PAGES: usize = 16;
const MERGE_FAN_IN: usize = 32;
const MERGE_BUFFER_BYTES: usize = 32 * 1024;
const MIN_SORT_BYTES: u64 = 1024 * 1024;
const MAX_SORT_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ReverseEdge {
    target: i32,
    source: i32,
}

pub(super) fn install_spilled(
    spilled: SpilledOutput,
    metadata: ComposeOutputMetadata,
    net: &mut Fsm,
) -> Result<(), FomaError> {
    let sorted = sort_reverse_edges(&spilled)?;
    let (offsets, predecessors) = build_predecessors(&spilled, &sorted)?;
    let mut live = mark_live_states(&spilled, &offsets, &predecessors)?;
    if spilled.state_count == 0 || !live.contains(0)? {
        install_empty(net);
        return Ok(());
    }
    let (ranks, live_count) = write_ranks(&spilled, &mut live)?;
    let rewritten = rewrite_live_rows(&spilled, &mut live, &ranks)?;
    let mut rows = load_rows(&rewritten.path)?;
    rows.push(sentinel());
    let pruned_metadata = ComposeOutputMetadata {
        arccount: rewritten.arc_count,
        statecount: i32::try_from(live_count)
            .map_err(|_| FomaError::CapacityExceeded("composition survivor count"))?,
        finalcount: rewritten.final_count,
        ..metadata
    };
    apply_metadata(net, pruned_metadata, rows.len())?;
    net.is_pruned = Tern::Yes;
    net.states = rows.into();
    Ok(())
}

fn install_empty(net: &mut Fsm) {
    net.states = fsm_empty().into();
    fsm_sigma_destroy(core::mem::take(&mut net.sigma));
    net.sigma = sigma_create();
    net.statecount = 1;
    net.finalcount = 0;
    net.arccount = 0;
    net.linecount = 2;
    net.pathcount = 0;
    fsm_update_flags(net, YES, YES, YES, YES, YES, NO);
    net.is_pruned = Tern::Yes;
}

fn sort_reverse_edges(spilled: &SpilledOutput) -> Result<PathBuf, FomaError> {
    let sort_bytes = spilled
        .output_cap_bytes
        .clamp(MIN_SORT_BYTES, MAX_SORT_BYTES);
    let chunk_len = usize::try_from(sort_bytes / EDGE_BYTES as u64)
        .unwrap_or(usize::MAX)
        .max(1);
    let input = File::open(&spilled.reverse_path).map_err(|error| {
        io_error(
            "opening composition reverse edges",
            &spilled.reverse_path,
            error,
        )
    })?;
    let mut reader = BufReader::with_capacity(MERGE_BUFFER_BYTES, input);
    let mut chunk = Vec::new();
    chunk
        .try_reserve_exact(chunk_len)
        .map_err(|_| FomaError::CapacityExceeded("composition reverse sort chunk"))?;
    let mut runs = Vec::new();
    loop {
        chunk.clear();
        while chunk.len() < chunk_len {
            match read_edge_optional(&mut reader, &spilled.reverse_path)? {
                Some(edge) => chunk.push(edge),
                None => break,
            }
        }
        if chunk.is_empty() {
            break;
        }
        chunk.sort_unstable();
        chunk.dedup();
        let path = spilled
            .scratch_path
            .join(format!("reverse-run-0-{}.bin", runs.len()));
        write_edge_run(&path, &chunk)?;
        runs.push(path);
    }
    if runs.is_empty() {
        let path = spilled.scratch_path.join("reverse-run-empty.bin");
        write_edge_run(&path, &[])?;
        return Ok(path);
    }

    let mut generation = 1usize;
    while runs.len() > 1 {
        let mut next = Vec::new();
        for (group_index, group) in runs.chunks(MERGE_FAN_IN).enumerate() {
            let path = spilled
                .scratch_path
                .join(format!("reverse-run-{generation}-{group_index}.bin"));
            merge_edge_runs(group, &path)?;
            for old in group {
                fs::remove_file(old)
                    .map_err(|error| io_error("removing composition reverse run", old, error))?;
            }
            next.push(path);
        }
        runs = next;
        generation += 1;
    }
    Ok(runs.pop().expect("nonempty run generation"))
}

fn write_edge_run(path: &Path, edges: &[ReverseEdge]) -> Result<(), FomaError> {
    let file = create_file(path, "composition reverse run")?;
    let mut writer = BufWriter::with_capacity(MERGE_BUFFER_BYTES, file);
    for edge in edges {
        writer
            .write_all(&edge_bytes(*edge))
            .map_err(|error| io_error("writing composition reverse run", path, error))?;
    }
    flush_and_sync(&mut writer, path, "composition reverse run")
}

fn merge_edge_runs(inputs: &[PathBuf], output: &Path) -> Result<(), FomaError> {
    let mut readers = Vec::with_capacity(inputs.len());
    let mut heap = BinaryHeap::new();
    for (index, path) in inputs.iter().enumerate() {
        let file = File::open(path)
            .map_err(|error| io_error("opening composition reverse run", path, error))?;
        let mut reader = BufReader::with_capacity(MERGE_BUFFER_BYTES, file);
        if let Some(edge) = read_edge_optional(&mut reader, path)? {
            heap.push(Reverse((edge, index)));
        }
        readers.push(reader);
    }
    let file = create_file(output, "merged composition reverse run")?;
    let mut writer = BufWriter::with_capacity(MERGE_BUFFER_BYTES, file);
    let mut previous = None;
    while let Some(Reverse((edge, input_index))) = heap.pop() {
        if previous != Some(edge) {
            writer.write_all(&edge_bytes(edge)).map_err(|error| {
                io_error("writing merged composition reverse run", output, error)
            })?;
            previous = Some(edge);
        }
        if let Some(next) = read_edge_optional(&mut readers[input_index], &inputs[input_index])? {
            heap.push(Reverse((next, input_index)));
        }
    }
    flush_and_sync(&mut writer, output, "merged composition reverse run")
}

fn build_predecessors(
    spilled: &SpilledOutput,
    sorted: &Path,
) -> Result<(PathBuf, PathBuf), FomaError> {
    let offsets_path = spilled.scratch_path.join("predecessor-offsets.bin");
    let predecessors_path = spilled.scratch_path.join("predecessors.bin");
    let offsets_file = create_file(&offsets_path, "composition predecessor offsets")?;
    let predecessors_file = create_file(&predecessors_path, "composition predecessors")?;
    let mut offsets = BufWriter::with_capacity(MERGE_BUFFER_BYTES, offsets_file);
    let mut predecessors = BufWriter::with_capacity(MERGE_BUFFER_BYTES, predecessors_file);
    let sorted_file = File::open(sorted)
        .map_err(|error| io_error("opening sorted composition reverse edges", sorted, error))?;
    let mut edges = BufReader::with_capacity(MERGE_BUFFER_BYTES, sorted_file);
    let mut current = read_edge_optional(&mut edges, sorted)?;
    let mut predecessor_bytes = 0u64;

    for state in 0..spilled.state_count {
        offsets
            .write_all(&predecessor_bytes.to_le_bytes())
            .map_err(|error| {
                io_error(
                    "writing composition predecessor offsets",
                    &offsets_path,
                    error,
                )
            })?;
        while current.is_some_and(|edge| edge.target == state as i32) {
            let edge = current.expect("edge checked above");
            if edge.source < 0 || edge.source as u32 >= spilled.state_count {
                return Err(FomaError::Format(format!(
                    "composition reverse edge has invalid source {}",
                    edge.source
                )));
            }
            predecessors
                .write_all(&edge.source.to_le_bytes())
                .map_err(|error| {
                    io_error(
                        "writing composition predecessors",
                        &predecessors_path,
                        error,
                    )
                })?;
            predecessor_bytes =
                predecessor_bytes
                    .checked_add(4)
                    .ok_or(FomaError::CapacityExceeded(
                        "composition predecessor stream",
                    ))?;
            current = read_edge_optional(&mut edges, sorted)?;
        }
        if let Some(edge) = current
            && (edge.target < 0 || edge.target as u32 >= spilled.state_count)
        {
            return Err(FomaError::Format(format!(
                "composition reverse edge has invalid target {}",
                edge.target
            )));
        }
    }
    offsets
        .write_all(&predecessor_bytes.to_le_bytes())
        .map_err(|error| {
            io_error(
                "writing composition predecessor offsets",
                &offsets_path,
                error,
            )
        })?;
    if current.is_some() {
        return Err(FomaError::Format(
            "composition reverse stream contains out-of-range targets".to_string(),
        ));
    }
    flush_and_sync(
        &mut offsets,
        &offsets_path,
        "composition predecessor offsets",
    )?;
    flush_and_sync(
        &mut predecessors,
        &predecessors_path,
        "composition predecessors",
    )?;
    Ok((offsets_path, predecessors_path))
}

fn mark_live_states(
    spilled: &SpilledOutput,
    offsets_path: &Path,
    predecessors_path: &Path,
) -> Result<DiskBitSet, FomaError> {
    let live_path = spilled.scratch_path.join("live-states.bin");
    let mut live = DiskBitSet::create(&live_path, spilled.state_count)?;
    let queue_path = spilled.scratch_path.join("live-queue.bin");
    let mut queue = DiskQueue::create(&queue_path)?;
    let finals_file = File::open(&spilled.finals_path).map_err(|error| {
        io_error(
            "opening composition final states",
            &spilled.finals_path,
            error,
        )
    })?;
    let mut finals = BufReader::with_capacity(MERGE_BUFFER_BYTES, finals_file);
    while let Some(state) = read_i32_optional(&mut finals, &spilled.finals_path)? {
        if state < 0 || state as u32 >= spilled.state_count {
            return Err(FomaError::Format(format!(
                "composition final state {state} is out of range"
            )));
        }
        if live.insert(state as u32)? {
            queue.push(state)?;
        }
    }

    let mut offsets = File::open(offsets_path).map_err(|error| {
        io_error(
            "opening composition predecessor offsets",
            offsets_path,
            error,
        )
    })?;
    let mut predecessors = File::open(predecessors_path)
        .map_err(|error| io_error("opening composition predecessors", predecessors_path, error))?;
    while let Some(state) = queue.pop()? {
        let state = u32::try_from(state)
            .map_err(|_| FomaError::Format("negative composition queue state".to_string()))?;
        let start = read_u64_at(&mut offsets, offsets_path, u64::from(state) * 8)?;
        let end = read_u64_at(&mut offsets, offsets_path, u64::from(state + 1) * 8)?;
        if end < start || (end - start) % 4 != 0 {
            return Err(FomaError::Format(format!(
                "invalid predecessor range {start}..{end} for state {state}"
            )));
        }
        predecessors.seek(SeekFrom::Start(start)).map_err(|error| {
            io_error("seeking composition predecessors", predecessors_path, error)
        })?;
        for _ in 0..(end - start) / 4 {
            let source = read_i32(&mut predecessors, predecessors_path, "predecessor")?;
            if source < 0 || source as u32 >= spilled.state_count {
                return Err(FomaError::Format(format!(
                    "composition predecessor {source} is out of range"
                )));
            }
            if live.insert(source as u32)? {
                queue.push(source)?;
            }
        }
    }
    live.flush()?;
    Ok(live)
}

fn write_ranks(
    spilled: &SpilledOutput,
    live: &mut DiskBitSet,
) -> Result<(PathBuf, u32), FomaError> {
    let path = spilled.scratch_path.join("live-ranks.bin");
    let file = create_file(&path, "composition survivor ranks")?;
    let mut writer = BufWriter::with_capacity(MERGE_BUFFER_BYTES, file);
    let mut next_rank = 0u32;
    for state in 0..spilled.state_count {
        let rank = if live.contains(state)? {
            let rank = next_rank;
            next_rank = next_rank
                .checked_add(1)
                .ok_or(FomaError::CapacityExceeded("composition survivor ranks"))?;
            rank
        } else {
            u32::MAX
        };
        writer
            .write_all(&rank.to_le_bytes())
            .map_err(|error| io_error("writing composition survivor ranks", &path, error))?;
    }
    flush_and_sync(&mut writer, &path, "composition survivor ranks")?;
    Ok((path, next_rank))
}

struct RewrittenRows {
    path: PathBuf,
    arc_count: i32,
    final_count: i32,
}

fn rewrite_live_rows(
    spilled: &SpilledOutput,
    live: &mut DiskBitSet,
    ranks_path: &Path,
) -> Result<RewrittenRows, FomaError> {
    let path = spilled.scratch_path.join("product-rows-trimmed.bin");
    let file = create_file(&path, "trimmed composition product rows")?;
    let mut writer = BufWriter::with_capacity(MERGE_BUFFER_BYTES, file);
    let rows_file = File::open(&spilled.rows_path).map_err(|error| {
        io_error(
            "opening composition product rows",
            &spilled.rows_path,
            error,
        )
    })?;
    let mut rows = BufReader::with_capacity(MERGE_BUFFER_BYTES, rows_file);
    let mut ranks = ReadPageCache::open(ranks_path)?;
    let mut group = RewriteGroup::default();
    let mut arc_count = 0i32;
    let mut final_count = 0i32;

    while let Some(row) = read_state_optional(&mut rows, &spilled.rows_path)? {
        if row.state_no < 0 || row.state_no as u32 >= spilled.state_count {
            return Err(FomaError::Format(format!(
                "composition row has invalid state {}",
                row.state_no
            )));
        }
        if group.source != Some(row.state_no) {
            group.finish(&mut writer, &path)?;
            let live_source = live.contains(row.state_no as u32)?;
            let source_rank = if live_source {
                Some(read_rank(&mut ranks, row.state_no as u32)?)
            } else {
                None
            };
            group = RewriteGroup {
                source: Some(row.state_no),
                source_rank,
                final_state: row.final_state,
                start_state: row.start_state,
                emitted: false,
            };
            if live_source && row.final_state != 0 {
                final_count = final_count
                    .checked_add(1)
                    .ok_or(FomaError::CapacityExceeded("composition final-state count"))?;
            }
        }
        let Some(source_rank) = group.source_rank else {
            continue;
        };
        let target = if row.target == -1 {
            Some(-1)
        } else if row.target < 0 || row.target as u32 >= spilled.state_count {
            return Err(FomaError::Format(format!(
                "composition row has invalid target {}",
                row.target
            )));
        } else if live.contains(row.target as u32)? {
            Some(read_rank(&mut ranks, row.target as u32)? as i32)
        } else {
            None
        };
        if let Some(target) = target {
            let rewritten = FsmState {
                state_no: source_rank as i32,
                target,
                ..row
            };
            writer
                .write_all(&state_to_bytes(rewritten))
                .map_err(|error| {
                    io_error("writing trimmed composition product rows", &path, error)
                })?;
            group.emitted = true;
            if target != -1 {
                arc_count = arc_count.checked_add(1).ok_or(FomaError::CapacityExceeded(
                    "composition survivor arc count",
                ))?;
            }
        }
    }
    group.finish(&mut writer, &path)?;
    flush_and_sync(&mut writer, &path, "trimmed composition product rows")?;
    Ok(RewrittenRows {
        path,
        arc_count,
        final_count,
    })
}

#[derive(Default)]
struct RewriteGroup {
    source: Option<i32>,
    source_rank: Option<u32>,
    final_state: i8,
    start_state: i8,
    emitted: bool,
}

impl RewriteGroup {
    fn finish(&mut self, writer: &mut BufWriter<File>, path: &Path) -> Result<(), FomaError> {
        if !self.emitted
            && self.final_state != 0
            && let Some(source_rank) = self.source_rank
        {
            writer
                .write_all(&state_to_bytes(FsmState {
                    state_no: source_rank as i32,
                    r#in: -1,
                    out: -1,
                    target: -1,
                    final_state: self.final_state,
                    start_state: self.start_state,
                }))
                .map_err(|error| {
                    io_error("writing trimmed composition final marker", path, error)
                })?;
            self.emitted = true;
        }
        Ok(())
    }
}

fn load_rows(path: &Path) -> Result<Vec<FsmState>, FomaError> {
    let length = fs::metadata(path)
        .map_err(|error| io_error("reading trimmed composition metadata", path, error))?
        .len();
    if length % STATE_BYTES as u64 != 0 {
        return Err(FomaError::Format(format!(
            "trimmed composition row file {} has a partial record",
            path.display()
        )));
    }
    let count = usize::try_from(length / STATE_BYTES as u64)
        .map_err(|_| FomaError::CapacityExceeded("final composition rows"))?;
    let mut result = Vec::new();
    result
        .try_reserve_exact(count + 1)
        .map_err(|_| FomaError::CapacityExceeded("final composition rows"))?;
    let file = File::open(path)
        .map_err(|error| io_error("opening trimmed composition rows", path, error))?;
    let mut reader = BufReader::with_capacity(MERGE_BUFFER_BYTES, file);
    while let Some(row) = read_state_optional(&mut reader, path)? {
        result.push(row);
    }
    Ok(result)
}

struct DiskQueue {
    writer: File,
    reader: File,
    path: PathBuf,
    written: u64,
    read: u64,
}

impl DiskQueue {
    fn create(path: &Path) -> Result<Self, FomaError> {
        let writer = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|error| io_error("creating composition live queue", path, error))?;
        let reader = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|error| io_error("opening composition live queue", path, error))?;
        Ok(Self {
            writer,
            reader,
            path: path.to_path_buf(),
            written: 0,
            read: 0,
        })
    }

    fn push(&mut self, state: i32) -> Result<(), FomaError> {
        self.writer
            .write_all(&state.to_le_bytes())
            .map_err(|error| io_error("writing composition live queue", &self.path, error))?;
        self.written += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<Option<i32>, FomaError> {
        if self.read == self.written {
            return Ok(None);
        }
        let state = read_i32(&mut self.reader, &self.path, "live queue state")?;
        self.read += 1;
        Ok(Some(state))
    }
}

struct DiskBitSet {
    cache: PageCache,
    bit_count: u32,
}

impl DiskBitSet {
    fn create(path: &Path, bit_count: u32) -> Result<Self, FomaError> {
        let byte_count = u64::from(bit_count).div_ceil(8);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|error| io_error("creating composition live bitmap", path, error))?;
        file.set_len(byte_count)
            .map_err(|error| io_error("sizing composition live bitmap", path, error))?;
        Ok(Self {
            cache: PageCache::new(file, path.to_path_buf(), byte_count, true),
            bit_count,
        })
    }

    fn contains(&mut self, bit: u32) -> Result<bool, FomaError> {
        self.validate(bit)?;
        let byte = self.cache.read(u64::from(bit / 8))?;
        Ok(byte & (1 << (bit % 8)) != 0)
    }

    fn insert(&mut self, bit: u32) -> Result<bool, FomaError> {
        self.validate(bit)?;
        let offset = u64::from(bit / 8);
        let mask = 1 << (bit % 8);
        let value = self.cache.read(offset)?;
        if value & mask != 0 {
            return Ok(false);
        }
        self.cache.write(offset, value | mask)?;
        Ok(true)
    }

    fn flush(&mut self) -> Result<(), FomaError> {
        self.cache.flush()
    }

    fn validate(&self, bit: u32) -> Result<(), FomaError> {
        if bit >= self.bit_count {
            return Err(FomaError::Format(format!(
                "composition live-bit index {bit} is out of range"
            )));
        }
        Ok(())
    }
}

struct ReadPageCache(PageCache);

impl ReadPageCache {
    fn open(path: &Path) -> Result<Self, FomaError> {
        let file = File::open(path)
            .map_err(|error| io_error("opening composition survivor ranks", path, error))?;
        let length = file
            .metadata()
            .map_err(|error| io_error("reading composition rank metadata", path, error))?
            .len();
        Ok(Self(PageCache::new(
            file,
            path.to_path_buf(),
            length,
            false,
        )))
    }

    fn read_u32(&mut self, offset: u64) -> Result<u32, FomaError> {
        let mut bytes = [0; 4];
        for (index, byte) in bytes.iter_mut().enumerate() {
            *byte = self.0.read(offset + index as u64)?;
        }
        Ok(u32::from_le_bytes(bytes))
    }
}

struct PageCache {
    file: File,
    path: PathBuf,
    length: u64,
    writable: bool,
    pages: Vec<CachedPage>,
    clock: u64,
}

struct CachedPage {
    index: u64,
    data: Vec<u8>,
    valid: usize,
    dirty: bool,
    used: u64,
}

impl PageCache {
    fn new(file: File, path: PathBuf, length: u64, writable: bool) -> Self {
        Self {
            file,
            path,
            length,
            writable,
            pages: Vec::new(),
            clock: 0,
        }
    }

    fn read(&mut self, offset: u64) -> Result<u8, FomaError> {
        let (page, within) = self.locate(offset)?;
        Ok(self.pages[page].data[within])
    }

    fn write(&mut self, offset: u64, value: u8) -> Result<(), FomaError> {
        if !self.writable {
            return Err(FomaError::Io(format!(
                "attempted to write read-only page cache {}",
                self.path.display()
            )));
        }
        let (page, within) = self.locate(offset)?;
        self.pages[page].data[within] = value;
        self.pages[page].dirty = true;
        Ok(())
    }

    fn locate(&mut self, offset: u64) -> Result<(usize, usize), FomaError> {
        if offset >= self.length {
            return Err(FomaError::Format(format!(
                "composition scratch offset {offset} exceeds {}",
                self.length
            )));
        }
        self.clock = self.clock.wrapping_add(1);
        let page_index = offset / PAGE_BYTES as u64;
        if let Some(index) = self.pages.iter().position(|page| page.index == page_index) {
            self.pages[index].used = self.clock;
            return Ok((index, (offset % PAGE_BYTES as u64) as usize));
        }
        let slot = if self.pages.len() < CACHE_PAGES {
            self.pages.push(CachedPage {
                index: 0,
                data: vec![0; PAGE_BYTES],
                valid: 0,
                dirty: false,
                used: 0,
            });
            self.pages.len() - 1
        } else {
            self.pages
                .iter()
                .enumerate()
                .min_by_key(|(_, page)| page.used)
                .map(|(index, _)| index)
                .expect("nonempty page cache")
        };
        self.flush_page(slot)?;
        let start = page_index * PAGE_BYTES as u64;
        let valid = usize::try_from((self.length - start).min(PAGE_BYTES as u64))
            .expect("page length fits usize");
        self.file
            .seek(SeekFrom::Start(start))
            .map_err(|error| io_error("seeking composition page cache", &self.path, error))?;
        self.file
            .read_exact(&mut self.pages[slot].data[..valid])
            .map_err(|error| io_error("reading composition page cache", &self.path, error))?;
        self.pages[slot].index = page_index;
        self.pages[slot].valid = valid;
        self.pages[slot].dirty = false;
        self.pages[slot].used = self.clock;
        Ok((slot, (offset % PAGE_BYTES as u64) as usize))
    }

    fn flush(&mut self) -> Result<(), FomaError> {
        for index in 0..self.pages.len() {
            self.flush_page(index)?;
        }
        if self.writable {
            self.file
                .sync_data()
                .map_err(|error| io_error("syncing composition page cache", &self.path, error))?;
        }
        Ok(())
    }

    fn flush_page(&mut self, index: usize) -> Result<(), FomaError> {
        if index >= self.pages.len() || !self.pages[index].dirty {
            return Ok(());
        }
        let start = self.pages[index].index * PAGE_BYTES as u64;
        self.file
            .seek(SeekFrom::Start(start))
            .map_err(|error| io_error("seeking composition page cache", &self.path, error))?;
        self.file
            .write_all(&self.pages[index].data[..self.pages[index].valid])
            .map_err(|error| io_error("writing composition page cache", &self.path, error))?;
        self.pages[index].dirty = false;
        Ok(())
    }
}

fn read_rank(cache: &mut ReadPageCache, state: u32) -> Result<u32, FomaError> {
    let rank = cache.read_u32(u64::from(state) * 4)?;
    if rank == u32::MAX {
        return Err(FomaError::Format(format!(
            "live composition state {state} has no survivor rank"
        )));
    }
    Ok(rank)
}

fn create_file(path: &Path, label: &str) -> Result<File, FomaError> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| io_error(&format!("creating {label}"), path, error))
}

fn flush_and_sync(writer: &mut BufWriter<File>, path: &Path, label: &str) -> Result<(), FomaError> {
    writer
        .flush()
        .map_err(|error| io_error(&format!("flushing {label}"), path, error))?;
    writer
        .get_ref()
        .sync_data()
        .map_err(|error| io_error(&format!("syncing {label}"), path, error))
}

fn edge_bytes(edge: ReverseEdge) -> [u8; EDGE_BYTES] {
    let mut bytes = [0; EDGE_BYTES];
    bytes[..4].copy_from_slice(&edge.target.to_le_bytes());
    bytes[4..].copy_from_slice(&edge.source.to_le_bytes());
    bytes
}

fn read_edge_optional(
    reader: &mut impl Read,
    path: &Path,
) -> Result<Option<ReverseEdge>, FomaError> {
    let Some(bytes) = read_record_optional::<EDGE_BYTES>(reader, path, "reverse edge")? else {
        return Ok(None);
    };
    Ok(Some(ReverseEdge {
        target: i32::from_le_bytes(bytes[..4].try_into().expect("four edge-target bytes")),
        source: i32::from_le_bytes(bytes[4..].try_into().expect("four edge-source bytes")),
    }))
}

fn read_state_optional(reader: &mut impl Read, path: &Path) -> Result<Option<FsmState>, FomaError> {
    Ok(read_record_optional::<STATE_BYTES>(reader, path, "product row")?.map(state_from_bytes))
}

fn read_i32_optional(reader: &mut impl Read, path: &Path) -> Result<Option<i32>, FomaError> {
    Ok(read_record_optional::<4>(reader, path, "state id")?.map(i32::from_le_bytes))
}

fn read_record_optional<const N: usize>(
    reader: &mut impl Read,
    path: &Path,
    label: &str,
) -> Result<Option<[u8; N]>, FomaError> {
    let mut bytes = [0; N];
    match reader.read(&mut bytes[..1]) {
        Ok(0) => return Ok(None),
        Ok(1) => {}
        Ok(_) => unreachable!("one-byte read returned more than one byte"),
        Err(error) => {
            return Err(io_error(
                &format!("reading composition {label}"),
                path,
                error,
            ));
        }
    }
    reader.read_exact(&mut bytes[1..]).map_err(|error| {
        io_error(
            &format!("reading complete composition {label} (scratch may be truncated)"),
            path,
            error,
        )
    })?;
    Ok(Some(bytes))
}

fn read_i32(file: &mut File, path: &Path, label: &str) -> Result<i32, FomaError> {
    let mut bytes = [0; 4];
    file.read_exact(&mut bytes)
        .map_err(|error| io_error(&format!("reading composition {label}"), path, error))?;
    Ok(i32::from_le_bytes(bytes))
}

fn read_u64_at(file: &mut File, path: &Path, offset: u64) -> Result<u64, FomaError> {
    file.seek(SeekFrom::Start(offset))
        .map_err(|error| io_error("seeking composition predecessor offsets", path, error))?;
    let mut bytes = [0; 8];
    file.read_exact(&mut bytes)
        .map_err(|error| io_error("reading composition predecessor offsets", path, error))?;
    Ok(u64::from_le_bytes(bytes))
}

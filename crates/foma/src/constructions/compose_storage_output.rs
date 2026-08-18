//! Budgeted composition output rows and builder-compatible metadata.

use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::mem::size_of;
use std::path::{Path, PathBuf};

use super::super::{EPSILON, PATHCOUNT_UNKNOWN, Tern};
use super::trim;
use super::{ComposeScratch, FomaError, Fsm, FsmState, io_error, state_to_bytes};

const IO_BUFFER_BYTES: usize = 64 * 1024;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ComposeOutputMetadata {
    pub(super) arity: i32,
    pub(super) arccount: i32,
    pub(super) statecount: i32,
    pub(super) finalcount: i32,
    pub(super) is_deterministic: bool,
    pub(super) is_epsilon_free: bool,
}

pub(crate) enum ComposeProduct {
    Memory {
        rows: Vec<FsmState>,
        metadata: ComposeOutputMetadata,
    },
    Scratch {
        spilled: SpilledOutput,
        metadata: ComposeOutputMetadata,
        scratch: ComposeScratch,
    },
}

impl ComposeProduct {
    /// Install the product rows into `net`. Returns true when the scratch path
    /// already applied coaccessibility pruning externally.
    pub(crate) fn install_into(self, net: &mut Fsm) -> Result<bool, FomaError> {
        match self {
            Self::Memory { mut rows, metadata } => {
                rows.push(sentinel());
                apply_metadata(net, metadata, rows.len())?;
                net.states = rows.into();
                Ok(false)
            }
            Self::Scratch {
                spilled,
                metadata,
                scratch,
            } => {
                let result = trim::install_spilled(spilled, metadata, net);
                drop(scratch);
                result.map(|()| true)
            }
        }
    }
}

pub(crate) struct SpilledOutput {
    pub(super) rows_path: PathBuf,
    pub(super) reverse_path: PathBuf,
    pub(super) finals_path: PathBuf,
    pub(super) scratch_path: PathBuf,
    pub(super) output_cap_bytes: u64,
    pub(super) state_count: u32,
}

pub(super) struct ComposeOutput {
    rows: RowStorage,
    current_state_no: i32,
    current_final: i32,
    current_start: i32,
    current_trans: bool,
    num_finals: i32,
    num_initials: i32,
    arity: i32,
    statecount: i32,
    is_deterministic: bool,
    is_epsilon_free: bool,
    arccount: i32,
    mainloop: u32,
    sigma_size: usize,
    dedup: Vec<DedupCell>,
}

impl ComposeOutput {
    pub(super) fn unbounded(sigma_size: i32) -> Self {
        Self::new(RowStorage::unbounded(), sigma_size)
    }

    pub(super) fn bounded(output_cap_bytes: u64, scratch_dir: PathBuf, sigma_size: i32) -> Self {
        Self::new(
            RowStorage::bounded(output_cap_bytes, scratch_dir),
            sigma_size,
        )
    }

    fn new(rows: RowStorage, sigma_size: i32) -> Self {
        let sigma_size = usize::try_from(sigma_size + 1).unwrap_or_default();
        Self {
            rows,
            current_state_no: 0,
            current_final: 0,
            current_start: 0,
            current_trans: false,
            num_finals: 0,
            num_initials: 0,
            arity: 1,
            statecount: 0,
            is_deterministic: true,
            is_epsilon_free: true,
            arccount: 0,
            mainloop: 1,
            sigma_size,
            // Sigma-square lookup is fixed by the alphabet, not product size,
            // and is intentionally outside the caller's graph-memory allowance.
            dedup: vec![DedupCell::default(); sigma_size.saturating_mul(sigma_size)],
        }
    }

    pub(super) fn begin_state(&mut self, state_no: i32, final_state: i32, start_state: i32) {
        self.current_state_no = state_no;
        self.current_final = final_state;
        self.current_start = start_state;
        self.current_trans = false;
        if final_state == 1 {
            self.num_finals += 1;
        }
        if start_state == 1 {
            self.num_initials += 1;
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn add_arc(
        &mut self,
        state_no: i32,
        input: i32,
        output: i32,
        target: i32,
        final_state: i32,
        start_state: i32,
    ) -> Result<(), FomaError> {
        if input != output {
            self.arity = 2;
        }
        if input == EPSILON && output == EPSILON {
            if state_no == target {
                return Ok(());
            }
            self.is_deterministic = false;
            self.is_epsilon_free = false;
        }

        if input != -1 && output != -1 {
            let input = usize::try_from(input).map_err(|_| {
                FomaError::MalformedInput("negative compose input label".to_string())
            })?;
            let output = usize::try_from(output).map_err(|_| {
                FomaError::MalformedInput("negative compose output label".to_string())
            })?;
            let index = self
                .sigma_size
                .checked_mul(input)
                .and_then(|base| base.checked_add(output))
                .ok_or(FomaError::CapacityExceeded("composition duplicate lookup"))?;
            let cell = self.dedup.get_mut(index).ok_or_else(|| {
                FomaError::MalformedInput(format!(
                    "composition label pair {input}:{output} exceeds merged sigma"
                ))
            })?;
            if cell.mainloop == self.mainloop {
                if cell.target == target {
                    return Ok(());
                }
                self.is_deterministic = false;
            }
            self.arccount = self
                .arccount
                .checked_add(1)
                .ok_or(FomaError::CapacityExceeded("composition arc count"))?;
            cell.mainloop = self.mainloop;
            cell.target = target;
        }

        self.current_trans = true;
        self.rows.push(FsmState {
            state_no,
            r#in: input as i16,
            out: output as i16,
            target,
            final_state: final_state as i8,
            start_state: start_state as i8,
        })
    }

    pub(super) fn end_state(&mut self) -> Result<(), FomaError> {
        if !self.current_trans {
            self.add_arc(
                self.current_state_no,
                -1,
                -1,
                -1,
                self.current_final,
                self.current_start,
            )?;
        }
        self.statecount = self
            .statecount
            .checked_add(1)
            .ok_or(FomaError::CapacityExceeded("composition state count"))?;
        self.mainloop = self.mainloop.wrapping_add(1);
        Ok(())
    }

    pub(super) fn finish(
        self,
        scratch: Option<ComposeScratch>,
    ) -> Result<ComposeProduct, FomaError> {
        let metadata = ComposeOutputMetadata {
            arity: self.arity,
            arccount: self.arccount,
            statecount: self.statecount,
            finalcount: self.num_finals,
            is_deterministic: self.is_deterministic && self.num_initials <= 1,
            is_epsilon_free: self.is_epsilon_free,
        };
        match self.rows.finish()? {
            FinishedRows::Memory(rows) => {
                drop(scratch);
                Ok(ComposeProduct::Memory { rows, metadata })
            }
            FinishedRows::Scratch(spilled) => Ok(ComposeProduct::Scratch {
                spilled,
                metadata,
                scratch: scratch.expect("spilled rows have operation scratch"),
            }),
        }
    }

    #[cfg(test)]
    pub(super) fn is_spilled(&self) -> bool {
        matches!(self.rows, RowStorage::Disk(_))
    }
}

#[derive(Clone, Copy, Default)]
struct DedupCell {
    target: i32,
    mainloop: u32,
}

enum RowStorage {
    Memory {
        rows: Vec<FsmState>,
        limit: Option<MemoryRowLimit>,
    },
    Disk(DiskRows),
}

#[derive(Clone)]
struct MemoryRowLimit {
    cap_bytes: u64,
    scratch_dir: PathBuf,
}

impl RowStorage {
    fn unbounded() -> Self {
        Self::Memory {
            rows: Vec::new(),
            limit: None,
        }
    }

    fn bounded(cap_bytes: u64, scratch_dir: PathBuf) -> Self {
        Self::Memory {
            rows: Vec::new(),
            limit: Some(MemoryRowLimit {
                cap_bytes,
                scratch_dir,
            }),
        }
    }

    fn push(&mut self, row: FsmState) -> Result<(), FomaError> {
        if let Self::Disk(disk) = self {
            return disk.write_row(row);
        }

        let should_spill = match self {
            Self::Memory {
                rows,
                limit: Some(limit),
            } if rows.len() == rows.capacity() => {
                let prospective = next_capacity(rows.capacity());
                if row_capacity_bytes(prospective)? > limit.cap_bytes {
                    true
                } else {
                    rows.try_reserve_exact(prospective - rows.capacity())
                        .map_err(|_| FomaError::CapacityExceeded("composition output rows"))?;
                    row_capacity_bytes(rows.capacity())? > limit.cap_bytes
                }
            }
            _ => false,
        };
        if should_spill {
            self.migrate()?;
            return match self {
                Self::Disk(disk) => disk.write_row(row),
                Self::Memory { .. } => unreachable!("row migration installed disk backing"),
            };
        }

        match self {
            Self::Memory { rows, .. } => {
                rows.push(row);
                Ok(())
            }
            Self::Disk(_) => unreachable!("disk rows returned above"),
        }
    }

    fn migrate(&mut self) -> Result<(), FomaError> {
        let (rows, limit) = match self {
            Self::Memory {
                rows,
                limit: Some(limit),
            } => (rows, limit.clone()),
            _ => unreachable!("only bounded memory rows migrate"),
        };
        let disk = DiskRows::from_memory(&limit.scratch_dir, limit.cap_bytes, rows)?;
        *self = Self::Disk(disk);
        Ok(())
    }

    fn finish(self) -> Result<FinishedRows, FomaError> {
        match self {
            Self::Memory { rows, .. } => Ok(FinishedRows::Memory(rows)),
            Self::Disk(disk) => Ok(FinishedRows::Scratch(disk.finish()?)),
        }
    }
}

enum FinishedRows {
    Memory(Vec<FsmState>),
    Scratch(SpilledOutput),
}

struct DiskRows {
    rows: BufWriter<File>,
    reverse: BufWriter<File>,
    finals: BufWriter<File>,
    rows_path: PathBuf,
    reverse_path: PathBuf,
    finals_path: PathBuf,
    scratch_path: PathBuf,
    output_cap_bytes: u64,
    max_state: i32,
}

impl DiskRows {
    fn from_memory(
        parent: &Path,
        output_cap_bytes: u64,
        rows: &[FsmState],
    ) -> Result<Self, FomaError> {
        let rows_path = parent.join("product-rows.bin");
        let reverse_path = parent.join("reverse-edges.bin");
        let finals_path = parent.join("final-states.bin");
        let mut disk = Self {
            rows: create_writer(&rows_path, "composition product rows")?,
            reverse: create_writer(&reverse_path, "composition reverse edges")?,
            finals: create_writer(&finals_path, "composition final states")?,
            rows_path,
            reverse_path,
            finals_path,
            scratch_path: parent.to_path_buf(),
            output_cap_bytes,
            max_state: -1,
        };
        for row in rows {
            disk.write_row(*row)?;
        }
        Ok(disk)
    }

    fn write_row(&mut self, row: FsmState) -> Result<(), FomaError> {
        self.rows.write_all(&state_to_bytes(row)).map_err(|error| {
            io_error("writing composition product rows", &self.rows_path, error)
        })?;
        self.max_state = self.max_state.max(row.state_no);
        if row.target != -1 && row.target != row.state_no {
            self.reverse
                .write_all(&edge_bytes(row.target, row.state_no))
                .map_err(|error| {
                    io_error(
                        "writing composition reverse edges",
                        &self.reverse_path,
                        error,
                    )
                })?;
        }
        if row.final_state != 0 {
            self.finals
                .write_all(&row.state_no.to_le_bytes())
                .map_err(|error| {
                    io_error("writing composition final states", &self.finals_path, error)
                })?;
        }
        Ok(())
    }

    fn finish(mut self) -> Result<SpilledOutput, FomaError> {
        flush_and_sync(&mut self.rows, &self.rows_path, "composition product rows")?;
        flush_and_sync(
            &mut self.reverse,
            &self.reverse_path,
            "composition reverse edges",
        )?;
        flush_and_sync(
            &mut self.finals,
            &self.finals_path,
            "composition final states",
        )?;
        let state_count = u32::try_from(self.max_state + 1)
            .map_err(|_| FomaError::CapacityExceeded("composition state count"))?;
        Ok(SpilledOutput {
            rows_path: self.rows_path,
            reverse_path: self.reverse_path,
            finals_path: self.finals_path,
            scratch_path: self.scratch_path,
            output_cap_bytes: self.output_cap_bytes,
            state_count,
        })
    }
}

fn create_writer(path: &Path, label: &str) -> Result<BufWriter<File>, FomaError> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| io_error(&format!("creating {label}"), path, error))?;
    Ok(BufWriter::with_capacity(IO_BUFFER_BYTES, file))
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

fn edge_bytes(target: i32, source: i32) -> [u8; 8] {
    let mut bytes = [0; 8];
    bytes[..4].copy_from_slice(&target.to_le_bytes());
    bytes[4..].copy_from_slice(&source.to_le_bytes());
    bytes
}

fn next_capacity(current: usize) -> usize {
    if current == 0 {
        4
    } else {
        current.saturating_mul(2)
    }
}

fn row_capacity_bytes(capacity: usize) -> Result<u64, FomaError> {
    u64::try_from(capacity)
        .ok()
        .and_then(|count| count.checked_mul(size_of::<FsmState>() as u64))
        .ok_or(FomaError::CapacityExceeded("composition output accounting"))
}

pub(super) fn apply_metadata(
    net: &mut Fsm,
    metadata: ComposeOutputMetadata,
    row_count_with_sentinel: usize,
) -> Result<(), FomaError> {
    net.arity = metadata.arity;
    net.arccount = metadata.arccount;
    net.statecount = metadata.statecount;
    net.linecount = i32::try_from(row_count_with_sentinel)
        .map_err(|_| FomaError::CapacityExceeded("composition line count"))?;
    net.finalcount = metadata.finalcount;
    net.pathcount = PATHCOUNT_UNKNOWN;
    net.is_deterministic = Tern::from_wire(metadata.is_deterministic as i32);
    net.is_pruned = Tern::Unk;
    net.is_minimized = Tern::Unk;
    net.is_epsilon_free = Tern::from_wire(metadata.is_epsilon_free as i32);
    net.is_loop_free = Tern::Unk;
    net.is_completed = Tern::Unk;
    net.arcs_sorted_in = false;
    net.arcs_sorted_out = false;
    Ok(())
}

pub(super) fn sentinel() -> FsmState {
    FsmState {
        state_no: -1,
        r#in: -1,
        out: -1,
        target: -1,
        final_state: -1,
        start_state: -1,
    }
}

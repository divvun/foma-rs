//! Memory-bounded LIFO frontier for composition.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::path::{Path, PathBuf};

use super::{FomaError, io_error};

const WORK_RECORD_BYTES: u64 = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ComposeWorkItem {
    pub(crate) state: i32,
    pub(crate) left: i32,
    pub(crate) right: i32,
    pub(crate) mode: i32,
}

pub(super) enum ComposeWorkStack {
    Unbounded(Vec<ComposeWorkItem>),
    Bounded(BoundedWorkStack),
}

impl ComposeWorkStack {
    pub(super) fn unbounded() -> Self {
        Self::Unbounded(Vec::new())
    }

    pub(super) fn bounded(memory_cap_bytes: u64, scratch_dir: PathBuf) -> Self {
        Self::Bounded(BoundedWorkStack {
            memory_cap_bytes,
            scratch_dir,
            memory: Some(Vec::new()),
            disk: None,
        })
    }

    pub(super) fn push(&mut self, item: ComposeWorkItem) -> Result<(), FomaError> {
        match self {
            Self::Unbounded(stack) => {
                stack.push(item);
                Ok(())
            }
            Self::Bounded(stack) => stack.push(item),
        }
    }

    pub(super) fn pop(&mut self) -> Result<Option<ComposeWorkItem>, FomaError> {
        match self {
            Self::Unbounded(stack) => Ok(stack.pop()),
            Self::Bounded(stack) => stack.pop(),
        }
    }

    #[cfg(test)]
    pub(super) fn is_spilled(&self) -> bool {
        matches!(self, Self::Bounded(stack) if stack.disk.is_some())
    }
}

pub(super) struct BoundedWorkStack {
    memory_cap_bytes: u64,
    scratch_dir: PathBuf,
    memory: Option<Vec<ComposeWorkItem>>,
    disk: Option<DiskWorkStack>,
}

impl BoundedWorkStack {
    fn push(&mut self, item: ComposeWorkItem) -> Result<(), FomaError> {
        if let Some(disk) = &mut self.disk {
            return disk.push(item);
        }
        let stack = self.memory.as_mut().expect("bounded stack has a backing");
        if stack.len() == stack.capacity() {
            let prospective = next_capacity(stack.capacity());
            if capacity_bytes(prospective)? > self.memory_cap_bytes {
                self.migrate_to_disk()?;
                return self
                    .disk
                    .as_mut()
                    .expect("migration installed disk work stack")
                    .push(item);
            }
            stack
                .try_reserve_exact(prospective - stack.capacity())
                .map_err(|_| FomaError::CapacityExceeded("composition work stack"))?;
            if capacity_bytes(stack.capacity())? > self.memory_cap_bytes {
                self.migrate_to_disk()?;
                return self
                    .disk
                    .as_mut()
                    .expect("migration installed disk work stack")
                    .push(item);
            }
        }
        self.memory
            .as_mut()
            .expect("bounded stack has memory backing")
            .push(item);
        Ok(())
    }

    fn pop(&mut self) -> Result<Option<ComposeWorkItem>, FomaError> {
        if let Some(disk) = &mut self.disk {
            disk.pop()
        } else {
            Ok(self
                .memory
                .as_mut()
                .expect("bounded stack has a backing")
                .pop())
        }
    }

    fn migrate_to_disk(&mut self) -> Result<(), FomaError> {
        let memory = self
            .memory
            .as_ref()
            .expect("work stack migration needs memory backing");
        let disk = DiskWorkStack::from_memory(&self.scratch_dir, memory)?;
        self.disk = Some(disk);
        self.memory = None;
        Ok(())
    }
}

fn next_capacity(current: usize) -> usize {
    if current == 0 {
        4
    } else {
        current.saturating_mul(2)
    }
}

fn capacity_bytes(capacity: usize) -> Result<u64, FomaError> {
    u64::try_from(capacity)
        .ok()
        .and_then(|count| count.checked_mul(size_of::<ComposeWorkItem>() as u64))
        .ok_or(FomaError::CapacityExceeded(
            "composition work-stack accounting",
        ))
}

struct DiskWorkStack {
    file: File,
    path: PathBuf,
    len: u64,
}

impl DiskWorkStack {
    fn from_memory(parent: &Path, items: &[ComposeWorkItem]) -> Result<Self, FomaError> {
        let path = parent.join("work.bin");
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| io_error("creating composition work stack", &path, error))?;
        for item in items {
            file.write_all(&work_item_bytes(*item))
                .map_err(|error| io_error("writing composition work stack", &path, error))?;
        }
        file.sync_data()
            .map_err(|error| io_error("syncing composition work stack", &path, error))?;
        Ok(Self {
            file,
            path,
            len: items.len() as u64,
        })
    }

    fn push(&mut self, item: ComposeWorkItem) -> Result<(), FomaError> {
        let offset = self
            .len
            .checked_mul(WORK_RECORD_BYTES)
            .ok_or(FomaError::CapacityExceeded("composition work-stack offset"))?;
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|error| io_error("seeking composition work stack", &self.path, error))?;
        self.file
            .write_all(&work_item_bytes(item))
            .map_err(|error| io_error("writing composition work stack", &self.path, error))?;
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<Option<ComposeWorkItem>, FomaError> {
        if self.len == 0 {
            return Ok(None);
        }
        self.len -= 1;
        let offset = self
            .len
            .checked_mul(WORK_RECORD_BYTES)
            .ok_or(FomaError::CapacityExceeded("composition work-stack offset"))?;
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|error| io_error("seeking composition work stack", &self.path, error))?;
        let mut bytes = [0; WORK_RECORD_BYTES as usize];
        self.file
            .read_exact(&mut bytes)
            .map_err(|error| io_error("reading composition work stack", &self.path, error))?;
        Ok(Some(work_item_from_bytes(bytes)))
    }
}

fn work_item_bytes(item: ComposeWorkItem) -> [u8; WORK_RECORD_BYTES as usize] {
    let mut bytes = [0; WORK_RECORD_BYTES as usize];
    bytes[..4].copy_from_slice(&item.state.to_le_bytes());
    bytes[4..8].copy_from_slice(&item.left.to_le_bytes());
    bytes[8..12].copy_from_slice(&item.right.to_le_bytes());
    bytes[12..].copy_from_slice(&item.mode.to_le_bytes());
    bytes
}

fn work_item_from_bytes(bytes: [u8; WORK_RECORD_BYTES as usize]) -> ComposeWorkItem {
    ComposeWorkItem {
        state: i32::from_le_bytes(bytes[..4].try_into().expect("four work-state bytes")),
        left: i32::from_le_bytes(bytes[4..8].try_into().expect("four work-left bytes")),
        right: i32::from_le_bytes(bytes[8..12].try_into().expect("four work-right bytes")),
        mode: i32::from_le_bytes(bytes[12..].try_into().expect("four work-mode bytes")),
    }
}

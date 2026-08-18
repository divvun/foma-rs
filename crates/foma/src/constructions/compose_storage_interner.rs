//! Spillable `(left, right, filter-mode) -> dense state id` interner.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::path::{Path, PathBuf};

use super::super::{
    Triplethash, TriplethashTriplets, triplet_hash_find, triplet_hash_init, triplet_hash_insert,
    triplethash_hashf,
};
use super::{FomaError, io_error};

const INITIAL_SLOTS: u32 = 128;
const SLOT_BYTES: u64 = 16;

pub(super) struct InternedState {
    pub(super) state: i32,
    pub(super) is_new: bool,
}

pub(super) enum ComposeInterner {
    Unbounded(Triplethash),
    Bounded(SpillableInterner),
}

impl ComposeInterner {
    pub(super) fn unbounded() -> Self {
        Self::Unbounded(triplet_hash_init())
    }

    pub(super) fn bounded(memory_cap_bytes: u64, scratch_dir: PathBuf) -> Self {
        Self::Bounded(SpillableInterner {
            memory_cap_bytes,
            scratch_dir,
            memory: None,
            disk: None,
        })
    }

    pub(super) fn intern(
        &mut self,
        left: i32,
        right: i32,
        mode: i32,
    ) -> Result<InternedState, FomaError> {
        match self {
            Self::Unbounded(table) => {
                if let Some(state) = triplet_hash_find(table, left, right, mode) {
                    return Ok(InternedState {
                        state,
                        is_new: false,
                    });
                }
                Ok(InternedState {
                    state: triplet_hash_insert(table, left, right, mode),
                    is_new: true,
                })
            }
            Self::Bounded(store) => store.intern(left, right, mode),
        }
    }

    #[cfg(test)]
    pub(super) fn is_spilled(&self) -> bool {
        matches!(self, Self::Bounded(store) if store.disk.is_some())
    }
}

pub(super) struct SpillableInterner {
    memory_cap_bytes: u64,
    scratch_dir: PathBuf,
    memory: Option<Triplethash>,
    disk: Option<DiskInterner>,
}

impl SpillableInterner {
    fn intern(&mut self, left: i32, right: i32, mode: i32) -> Result<InternedState, FomaError> {
        if let Some(disk) = &mut self.disk {
            return disk.intern(left, right, mode);
        }

        if let Some(table) = &self.memory
            && let Some(state) = triplet_hash_find(table, left, right, mode)
        {
            return Ok(InternedState {
                state,
                is_new: false,
            });
        }

        let prospective_slots = match &self.memory {
            None => INITIAL_SLOTS,
            Some(table) if table.occupancy as u32 + 1 > table.tablesize / 2 => table
                .tablesize
                .checked_mul(2)
                .ok_or(FomaError::CapacityExceeded("composition state ids"))?,
            Some(table) => table.tablesize,
        };
        if slot_capacity_bytes(prospective_slots)? > self.memory_cap_bytes {
            self.migrate_to_disk()?;
            return self
                .disk
                .as_mut()
                .expect("migration installed the disk interner")
                .intern(left, right, mode);
        }

        if self.memory.is_none() {
            self.memory = Some(empty_memory_table(INITIAL_SLOTS)?);
        }
        let table = self.memory.as_mut().expect("memory table was initialized");
        if prospective_slots != table.tablesize {
            grow_memory_table(table, prospective_slots)?;
        }
        let state = insert_memory(table, left, right, mode)?;
        Ok(InternedState {
            state,
            is_new: true,
        })
    }

    fn migrate_to_disk(&mut self) -> Result<(), FomaError> {
        let disk = DiskInterner::from_memory(&self.scratch_dir, self.memory.as_ref())?;
        self.disk = Some(disk);
        self.memory = None;
        Ok(())
    }
}

fn empty_slot() -> TriplethashTriplets {
    TriplethashTriplets {
        a: 0,
        b: 0,
        c: 0,
        key: -1,
    }
}

fn empty_memory_table(slot_count: u32) -> Result<Triplethash, FomaError> {
    let len = usize::try_from(slot_count)
        .map_err(|_| FomaError::CapacityExceeded("composition state hash slots"))?;
    let mut triplets = Vec::new();
    triplets
        .try_reserve_exact(len)
        .map_err(|_| FomaError::CapacityExceeded("composition state hash slots"))?;
    triplets.resize(len, empty_slot());
    Ok(Triplethash {
        triplets,
        tablesize: slot_count,
        occupancy: 0,
    })
}

fn grow_memory_table(table: &mut Triplethash, slot_count: u32) -> Result<(), FomaError> {
    let mut replacement = empty_memory_table(slot_count)?;
    for slot in &table.triplets {
        if slot.key != -1 {
            insert_memory_with_key(&mut replacement, slot.a, slot.b, slot.c, slot.key)?;
        }
    }
    replacement.occupancy = table.occupancy;
    *table = replacement;
    Ok(())
}

fn insert_memory(
    table: &mut Triplethash,
    left: i32,
    right: i32,
    mode: i32,
) -> Result<i32, FomaError> {
    let state = table.occupancy;
    if state == i32::MAX {
        return Err(FomaError::CapacityExceeded("composition state ids"));
    }
    insert_memory_with_key(table, left, right, mode, state)?;
    table.occupancy += 1;
    Ok(state)
}

fn insert_memory_with_key(
    table: &mut Triplethash,
    left: i32,
    right: i32,
    mode: i32,
    key: i32,
) -> Result<(), FomaError> {
    let mut slot = triplethash_hashf(left, right, mode) % table.tablesize;
    for _ in 0..table.tablesize {
        let entry = &mut table.triplets[slot as usize];
        if entry.key == -1 {
            *entry = TriplethashTriplets {
                a: left,
                b: right,
                c: mode,
                key,
            };
            return Ok(());
        }
        slot = (slot + 1) % table.tablesize;
    }
    Err(FomaError::CapacityExceeded("composition state hash table"))
}

fn slot_capacity_bytes(slot_count: u32) -> Result<u64, FomaError> {
    u64::from(slot_count)
        .checked_mul(size_of::<TriplethashTriplets>() as u64)
        .ok_or(FomaError::CapacityExceeded(
            "composition state hash accounting",
        ))
}

struct DiskInterner {
    slots: File,
    slots_path: PathBuf,
    slot_count: u32,
    generation: u32,
    len: u32,
}

impl DiskInterner {
    fn from_memory(parent: &Path, memory: Option<&Triplethash>) -> Result<Self, FomaError> {
        let slot_count = memory.map_or(INITIAL_SLOTS, |table| table.tablesize);
        let slots_path = parent.join("pairs-0.bin");
        let slots = create_slot_file(&slots_path, slot_count)?;
        let mut disk = Self {
            slots,
            slots_path,
            slot_count,
            generation: 0,
            len: 0,
        };
        if let Some(table) = memory {
            for entry in &table.triplets {
                if entry.key != -1 {
                    disk.place(entry.a, entry.b, entry.c, entry.key)?;
                }
            }
            disk.len = u32::try_from(table.occupancy)
                .map_err(|_| FomaError::CapacityExceeded("composition state ids"))?;
        }
        disk.slots
            .sync_data()
            .map_err(|error| io_error("syncing composition pair table", &disk.slots_path, error))?;
        Ok(disk)
    }

    fn intern(&mut self, left: i32, right: i32, mode: i32) -> Result<InternedState, FomaError> {
        if let Some(state) = self.find(left, right, mode)? {
            return Ok(InternedState {
                state,
                is_new: false,
            });
        }
        if self.len + 1 > self.slot_count / 2 {
            self.grow()?;
            if let Some(state) = self.find(left, right, mode)? {
                return Ok(InternedState {
                    state,
                    is_new: false,
                });
            }
        }
        let state = i32::try_from(self.len)
            .map_err(|_| FomaError::CapacityExceeded("composition state ids"))?;
        self.place(left, right, mode, state)?;
        self.len += 1;
        Ok(InternedState {
            state,
            is_new: true,
        })
    }

    fn find(&mut self, left: i32, right: i32, mode: i32) -> Result<Option<i32>, FomaError> {
        let mut slot = triplethash_hashf(left, right, mode) % self.slot_count;
        for _ in 0..self.slot_count {
            let entry = read_disk_slot(&mut self.slots, &self.slots_path, slot)?;
            if entry.id_plus_one == 0 {
                return Ok(None);
            }
            if entry.left == left && entry.right == right && entry.mode == mode {
                return Ok(Some((entry.id_plus_one - 1) as i32));
            }
            slot = (slot + 1) % self.slot_count;
        }
        Err(FomaError::CapacityExceeded("composition state hash table"))
    }

    fn place(&mut self, left: i32, right: i32, mode: i32, state: i32) -> Result<(), FomaError> {
        place_in_file(
            &mut self.slots,
            &self.slots_path,
            self.slot_count,
            DiskSlot {
                left,
                right,
                mode,
                id_plus_one: state as u32 + 1,
            },
        )
    }

    fn grow(&mut self) -> Result<(), FomaError> {
        let new_count = self
            .slot_count
            .checked_mul(2)
            .ok_or(FomaError::CapacityExceeded("composition state hash slots"))?;
        let generation = self
            .generation
            .checked_add(1)
            .ok_or(FomaError::CapacityExceeded(
                "composition state hash generations",
            ))?;
        let new_path = self
            .slots_path
            .with_file_name(format!("pairs-{generation}.bin"));
        let mut new_file = create_slot_file(&new_path, new_count)?;
        self.slots
            .seek(SeekFrom::Start(0))
            .map_err(|error| io_error("seeking composition pair table", &self.slots_path, error))?;
        for _ in 0..self.slot_count {
            let entry = read_disk_slot_next(&mut self.slots, &self.slots_path)?;
            if entry.id_plus_one != 0 {
                place_in_file(&mut new_file, &new_path, new_count, entry)?;
            }
        }
        new_file.sync_data().map_err(|error| {
            io_error("syncing resized composition pair table", &new_path, error)
        })?;

        let old_path = std::mem::replace(&mut self.slots_path, new_path);
        let old_file = std::mem::replace(&mut self.slots, new_file);
        self.slot_count = new_count;
        self.generation = generation;
        drop(old_file);
        fs::remove_file(&old_path)
            .map_err(|error| io_error("removing old composition pair table", &old_path, error))?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct DiskSlot {
    left: i32,
    right: i32,
    mode: i32,
    id_plus_one: u32,
}

fn create_slot_file(path: &Path, slot_count: u32) -> Result<File, FomaError> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| io_error("creating composition pair table", path, error))?;
    file.set_len(u64::from(slot_count) * SLOT_BYTES)
        .map_err(|error| io_error("sizing composition pair table", path, error))?;
    Ok(file)
}

fn place_in_file(
    file: &mut File,
    path: &Path,
    slot_count: u32,
    entry: DiskSlot,
) -> Result<(), FomaError> {
    let mut slot = triplethash_hashf(entry.left, entry.right, entry.mode) % slot_count;
    for _ in 0..slot_count {
        let existing = read_disk_slot(file, path, slot)?;
        if existing.id_plus_one == 0 {
            return write_disk_slot(file, path, slot, entry);
        }
        slot = (slot + 1) % slot_count;
    }
    Err(FomaError::CapacityExceeded("composition state hash table"))
}

fn read_disk_slot(file: &mut File, path: &Path, slot: u32) -> Result<DiskSlot, FomaError> {
    file.seek(SeekFrom::Start(u64::from(slot) * SLOT_BYTES))
        .map_err(|error| io_error("seeking composition pair table", path, error))?;
    read_disk_slot_next(file, path)
}

fn read_disk_slot_next(file: &mut File, path: &Path) -> Result<DiskSlot, FomaError> {
    let mut bytes = [0; SLOT_BYTES as usize];
    file.read_exact(&mut bytes)
        .map_err(|error| io_error("reading composition pair table", path, error))?;
    Ok(DiskSlot {
        left: i32::from_le_bytes(bytes[..4].try_into().expect("four left-state bytes")),
        right: i32::from_le_bytes(bytes[4..8].try_into().expect("four right-state bytes")),
        mode: i32::from_le_bytes(bytes[8..12].try_into().expect("four filter-mode bytes")),
        id_plus_one: u32::from_le_bytes(bytes[12..].try_into().expect("four state-id bytes")),
    })
}

fn write_disk_slot(
    file: &mut File,
    path: &Path,
    slot: u32,
    entry: DiskSlot,
) -> Result<(), FomaError> {
    let mut bytes = [0; SLOT_BYTES as usize];
    bytes[..4].copy_from_slice(&entry.left.to_le_bytes());
    bytes[4..8].copy_from_slice(&entry.right.to_le_bytes());
    bytes[8..12].copy_from_slice(&entry.mode.to_le_bytes());
    bytes[12..].copy_from_slice(&entry.id_plus_one.to_le_bytes());
    file.seek(SeekFrom::Start(u64::from(slot) * SLOT_BYTES))
        .map_err(|error| io_error("seeking composition pair table", path, error))?;
    file.write_all(&bytes)
        .map_err(|error| io_error("writing composition pair table", path, error))
}

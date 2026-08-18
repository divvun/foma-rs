//! Resource-controlled storage for composition's scalable working sets.
//!
//! A bounded composition reserves ten percent of the caller's allowance for
//! allocator metadata, page cache, and short migration overlap. The remaining
//! bytes are partitioned once between the product-state interner, DFS work
//! stack, and output rows. Loaded operands, sigma-sized lookup tables, the
//! final returned [`Fsm`], and a small fixed spill-I/O envelope are outside
//! this accounting, so the allowance is not a process RSS ceiling.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::FomaError;

use super::{Fsm, FsmState};

mod interner;
mod output;
mod trim;
mod work;

use interner::ComposeInterner;
use output::ComposeOutput;
pub(crate) use output::ComposeProduct;
use work::{ComposeWorkItem, ComposeWorkStack};

static SCRATCH_NONCE: AtomicU64 = AtomicU64::new(0);

/// Memory and scratch policy for one owned composition.
///
/// `None` retains the legacy unbounded in-memory implementation. A bounded
/// policy uses `scratch_dir` only after minimizing the operands has established
/// that the product is nonempty.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComposeResourceConfig {
    memory_limit_bytes: Option<u64>,
    scratch_dir: Option<PathBuf>,
}

impl ComposeResourceConfig {
    /// Preserve the legacy unbounded in-memory behavior without touching the
    /// filesystem.
    pub fn unbounded() -> Self {
        Self {
            memory_limit_bytes: None,
            scratch_dir: None,
        }
    }

    /// Bound budget-aware composition structures and place spill files beneath
    /// `scratch_dir`.
    pub fn bounded(memory_limit_bytes: u64, scratch_dir: impl Into<PathBuf>) -> Self {
        Self {
            memory_limit_bytes: Some(memory_limit_bytes),
            scratch_dir: Some(scratch_dir.into()),
        }
    }

    pub fn memory_limit_bytes(&self) -> Option<u64> {
        self.memory_limit_bytes
    }

    pub fn scratch_dir(&self) -> Option<&Path> {
        self.scratch_dir.as_deref()
    }
}

impl Default for ComposeResourceConfig {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// One-time partition of a caller's exact allowance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ComposeMemoryPlan {
    Unbounded,
    Bounded {
        allowance_bytes: u64,
        tracked_cap_bytes: u64,
        pair_interner_cap_bytes: u64,
        work_stack_cap_bytes: u64,
        output_cap_bytes: u64,
    },
}

impl ComposeMemoryPlan {
    pub(crate) fn from_allowance(allowance_bytes: Option<u64>) -> Self {
        let Some(allowance_bytes) = allowance_bytes else {
            return Self::Unbounded;
        };

        // floor(allowance * 9 / 10), without overflowing at u64::MAX.
        let quotient = allowance_bytes / 10;
        let remainder = allowance_bytes % 10;
        let tracked_cap_bytes = quotient * 9 + remainder * 9 / 10;
        // Pair lookup is random-access and receives 60%; output rows receive
        // 30%; the normally shallow DFS stack receives 10%. Integer remainder
        // stays with the pair interner.
        let work_stack_cap_bytes = tracked_cap_bytes / 10;
        let output_cap_bytes =
            tracked_cap_bytes / 10 * 3 + (tracked_cap_bytes % 10).saturating_mul(3) / 10;
        let pair_interner_cap_bytes = tracked_cap_bytes
            .saturating_sub(work_stack_cap_bytes)
            .saturating_sub(output_cap_bytes);
        Self::Bounded {
            allowance_bytes,
            tracked_cap_bytes,
            pair_interner_cap_bytes,
            work_stack_cap_bytes,
            output_cap_bytes,
        }
    }
}

/// Operation-local product state and output storage.
pub(crate) struct ComposeRuntime {
    scratch: Option<ComposeScratch>,
    interner: ComposeInterner,
    work: ComposeWorkStack,
    output: ComposeOutput,
}

impl ComposeRuntime {
    pub(crate) fn new(config: &ComposeResourceConfig, sigma_size: i32) -> Result<Self, FomaError> {
        match ComposeMemoryPlan::from_allowance(config.memory_limit_bytes) {
            ComposeMemoryPlan::Unbounded => Ok(Self {
                scratch: None,
                interner: ComposeInterner::unbounded(),
                work: ComposeWorkStack::unbounded(),
                output: ComposeOutput::unbounded(sigma_size),
            }),
            ComposeMemoryPlan::Bounded {
                pair_interner_cap_bytes,
                work_stack_cap_bytes,
                output_cap_bytes,
                ..
            } => {
                let parent = config.scratch_dir.as_deref().ok_or_else(|| {
                    FomaError::MalformedInput(
                        "bounded compose configuration has no scratch directory".to_string(),
                    )
                })?;
                let scratch = ComposeScratch::create(parent)?;
                let path = scratch.path().to_path_buf();
                Ok(Self {
                    scratch: Some(scratch),
                    interner: ComposeInterner::bounded(pair_interner_cap_bytes, path.clone()),
                    work: ComposeWorkStack::bounded(work_stack_cap_bytes, path.clone()),
                    output: ComposeOutput::bounded(output_cap_bytes, path, sigma_size),
                })
            }
        }
    }

    pub(crate) fn intern_state(
        &mut self,
        left: i32,
        right: i32,
        mode: i32,
    ) -> Result<i32, FomaError> {
        let interned = self.interner.intern(left, right, mode)?;
        if interned.is_new {
            self.work.push(ComposeWorkItem {
                state: interned.state,
                left,
                right,
                mode,
            })?;
        }
        Ok(interned.state)
    }

    pub(crate) fn pop_state(&mut self) -> Result<Option<ComposeWorkItem>, FomaError> {
        self.work.pop()
    }

    pub(crate) fn begin_state(&mut self, state: i32, final_state: i32, start_state: i32) {
        self.output.begin_state(state, final_state, start_state);
    }

    pub(crate) fn add_arc(
        &mut self,
        state: i32,
        input: i32,
        output: i32,
        target: i32,
        final_state: i32,
        start_state: i32,
    ) -> Result<(), FomaError> {
        self.output
            .add_arc(state, input, output, target, final_state, start_state)
    }

    pub(crate) fn end_state(&mut self) -> Result<(), FomaError> {
        self.output.end_state()
    }

    pub(crate) fn finish(self) -> Result<ComposeProduct, FomaError> {
        let Self {
            scratch,
            interner,
            work,
            output,
        } = self;
        drop(interner);
        drop(work);
        output.finish(scratch)
    }

    #[cfg(test)]
    fn spill_status(&self) -> (bool, bool, bool) {
        (
            self.interner.is_spilled(),
            self.work.is_spilled(),
            self.output.is_spilled(),
        )
    }
}

/// Owns a collision-resistant scratch directory and removes it on every normal
/// success/error/drop path.
#[derive(Debug)]
pub(crate) struct ComposeScratch {
    path: Option<PathBuf>,
}

impl ComposeScratch {
    fn create(parent: &Path) -> Result<Self, FomaError> {
        for _ in 0..4096 {
            let nonce = SCRATCH_NONCE.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!(
                ".foma-compose.{}.{}.scratch",
                std::process::id(),
                nonce
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path: Some(path) }),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => {
                    return Err(io_error(
                        "creating composition scratch directory",
                        &path,
                        error,
                    ));
                }
            }
        }
        Err(FomaError::Io(format!(
            "could not allocate a unique composition scratch directory beneath {}",
            parent.display()
        )))
    }

    pub(crate) fn path(&self) -> &Path {
        self.path
            .as_deref()
            .expect("live composition scratch has a path")
    }
}

impl Drop for ComposeScratch {
    fn drop(&mut self) {
        if let Some(path) = self.path.take() {
            let _ = fs::remove_dir_all(path);
        }
    }
}

pub(super) fn io_error(action: &str, path: &Path, error: std::io::Error) -> FomaError {
    FomaError::Io(format!("{action} {}: {error}", path.display()))
}

pub(super) fn state_to_bytes(state: FsmState) -> [u8; 14] {
    let mut bytes = [0; 14];
    bytes[..4].copy_from_slice(&state.state_no.to_le_bytes());
    bytes[4..6].copy_from_slice(&state.r#in.to_le_bytes());
    bytes[6..8].copy_from_slice(&state.out.to_le_bytes());
    bytes[8..12].copy_from_slice(&state.target.to_le_bytes());
    bytes[12] = state.final_state as u8;
    bytes[13] = state.start_state as u8;
    bytes
}

pub(super) fn state_from_bytes(bytes: [u8; 14]) -> FsmState {
    FsmState {
        state_no: i32::from_le_bytes(bytes[..4].try_into().expect("four state bytes")),
        r#in: i16::from_le_bytes(bytes[4..6].try_into().expect("two input bytes")),
        out: i16::from_le_bytes(bytes[6..8].try_into().expect("two output bytes")),
        target: i32::from_le_bytes(bytes[8..12].try_into().expect("four target bytes")),
        final_state: bytes[12] as i8,
        start_state: bytes[13] as i8,
    }
}

#[cfg(test)]
mod tests;

//! CLI session state — the re-entrant home for the interactive front-end state
//! that foma's C source kept in file-static globals.
//!
//! A `Session` owns the interactive command stack: the sentinel-terminated
//! doubly-linked arena that `stack.rs` manipulates through an `impl Session`
//! block. Threading `&mut Session` through the `iface` command layer replaces the
//! former `MAIN_STACK` / `ARENA` thread_locals, so independent sessions can
//! coexist on one thread (embeddable) with nothing hidden shared between them.
//!
//! The define registry follows in a later tier as a further `Session` field.

use crate::options::FomaOptions;
use crate::types::StackEntry;

/// The mutable state of one interactive foma session.
pub struct Session {
    /// C: `struct stack_entry *main_stack` (the network-stack list head) as an
    /// arena index. `Some` after `new()`; the `stack_*` methods keep it valid.
    /// See `stack.rs` for the arena/sentinel representation.
    pub(crate) stack_head: Option<usize>,
    /// Arena backing the malloc'd `stack_entry` nodes (see `stack.rs`).
    pub(crate) stack_arena: Vec<StackEntry>,
    /// The session's option set (C: the `g_*` globals; CLI `set variable`).
    pub opts: FomaOptions,
}

impl Session {
    /// Create a session with a freshly-initialised, empty command stack and
    /// default options.
    pub fn new() -> Session {
        let mut session = Session {
            stack_head: None,
            stack_arena: Vec::new(),
            opts: FomaOptions::default(),
        };
        session.stack_init();
        session
    }
}

impl Default for Session {
    fn default() -> Self {
        Session::new()
    }
}

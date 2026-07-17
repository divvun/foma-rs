//! The FSM line table — the arc storage of an [`Fsm`](crate::types::Fsm).
//!
//! Historically this was a bare, sentinel-terminated `Vec<FsmState>`: one row
//! per arc, rows grouped by `state_no`, the whole run closed by a `state_no ==
//! -1` terminator row. A state with no outgoing arcs still occupies one
//! "marker" row (`target == -1`) that records its `final_state`/`start_state`.
//! Every consumer walked that flat table by index, peeking `fsm[i+1].state_no`
//! to find state boundaries.
//!
//! `LineTable` is the seam that lets the backing store change without rewriting
//! all those walks at once. Today it is a transparent newtype over the flat
//! `Vec<FsmState>` and consumers still reach the rows through `Deref`. The
//! backing store will later become a per-state compressed form (each arc drops
//! its redundant `state_no`/`final_state`/`start_state`, which are properties of
//! the state, not the arc), roughly halving arc memory; consumers move onto the
//! accessor methods and the `Deref` view retires with that flip.

use core::ops::{Deref, DerefMut};

use crate::types::FsmState;

/// The sentinel-terminated arc table of an [`Fsm`](crate::types::Fsm).
///
/// An empty table (no rows at all) corresponds to the C `NULL` line table.
#[derive(Debug, Clone, Default)]
pub struct LineTable {
    rows: Vec<FsmState>,
}

impl LineTable {
    /// An empty table (C: a `NULL` `net->states`).
    pub fn new() -> LineTable {
        LineTable { rows: Vec::new() }
    }

    /// Wrap a flat, sentinel-terminated row sequence.
    pub fn from_rows(rows: Vec<FsmState>) -> LineTable {
        LineTable { rows }
    }

    /// Consume the table, yielding the flat row sequence.
    pub fn into_rows(self) -> Vec<FsmState> {
        self.rows
    }
}

impl From<Vec<FsmState>> for LineTable {
    fn from(rows: Vec<FsmState>) -> LineTable {
        LineTable { rows }
    }
}

impl Deref for LineTable {
    type Target = Vec<FsmState>;
    fn deref(&self) -> &Vec<FsmState> {
        &self.rows
    }
}

impl DerefMut for LineTable {
    fn deref_mut(&mut self) -> &mut Vec<FsmState> {
        &mut self.rows
    }
}

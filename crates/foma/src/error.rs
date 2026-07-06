//! Library-wide error type for the foma port (Wave 4 idiomatization).
//!
//! The Wave-2 library reproduced the C's fatal paths literally: `exit(1)`
//! on stack overflow, panics on malformed input, dead prototypes that were
//! link errors in C. Per docs/port/rust-conventions.md (Wave 4), library
//! code returns a `Result` instead so binaries can translate failures into
//! exit codes and messages. The enum is hand-rolled (no `thiserror`
//! dependency): `Display`, `Error` and `From` impls are written out below.

use std::fmt;

/// Errors surfaced by the foma library instead of `exit()`/panic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FomaError {
    /// A C prototype that was never defined (a link error in C), or an
    /// algorithm the port has not implemented yet. Carries a static label
    /// naming the entry point.
    Unimplemented(&'static str),
    /// Input bytes were malformed for the operation — e.g. an invalid
    /// UTF-8 lead byte where a well-formed sequence was expected.
    MalformedInput(String),
    /// A bounded resource was exhausted (e.g. a fixed-capacity stack in the
    /// C sources that called `exit(1)` on overflow).
    CapacityExceeded(&'static str),
}

impl fmt::Display for FomaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FomaError::Unimplemented(what) => write!(f, "unimplemented: {what}"),
            FomaError::MalformedInput(msg) => write!(f, "malformed input: {msg}"),
            FomaError::CapacityExceeded(what) => write!(f, "capacity exceeded: {what}"),
        }
    }
}

impl std::error::Error for FomaError {}

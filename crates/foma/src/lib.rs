//! foma: a finite-state toolkit and library — Rust port.
//!
//! Wave-2 literal (bug-for-bug) port of the C foma library. See
//! docs/port/rust-conventions.md for the binding conventions. Modules
//! mirror the C source files one-to-one and are added as each Wave-2
//! concern lands.

/// The one version line the foma, flookup and cgflookup binaries print:
/// "Divvun foma v<version> (<YYYY-MM-DD>, <rev>)". The version is the
/// package's; build.rs stamps the UTC build date and the short git revision,
/// which carries a `-dirty` suffix for a modified tree and reads `unknown`
/// outside a git checkout. The upstream C library's own version is
/// `structures::fsm_get_library_version_string`.
pub const VERSION_LINE: &str = concat!(
    "Divvun foma v",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("FOMA_BUILD_DATE"),
    ", ",
    env!("FOMA_BUILD_REV"),
    ")"
);

pub mod apply;
pub mod coaccessible;
pub mod constructions;
pub mod define;
pub mod determinize;
pub mod dynarray;
pub mod error;
pub mod extract;
pub mod flags;
pub mod iface;
pub mod int_stack;
pub mod io;
pub mod lexcread;
pub mod line_table;
pub mod mem;
pub mod minimize;
pub mod options;
pub mod regex;
pub mod reverse;
pub mod rewrite;
pub mod session;
pub mod sigma;
pub mod spelling;
pub mod stack;
pub mod stringhash;
pub mod structures;
pub mod topsort;
pub mod trie;
pub mod types;
pub mod utf8;

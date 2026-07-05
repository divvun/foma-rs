//! foma/iface.c — literal Wave-2 (bug-for-bug) port per
//! docs/port/rust-conventions.md. Sem rules: docs/spec/port/foma/iface.md
//! (per-file `iface.*` ids) plus the foma.h prototype ids (`foma.iface-*`)
//! carried at each single Rust site.
//!
//! Both slices are now present. Slice 1 covers the top of iface.c through
//! `iface_quit` (C-file order); slice 2 (below the SLICE 2 banner) covers the
//! second half, including the four callees slice 1 had stubbed
//! (iface_stack_check, print_stats, print_dot, print_net).
//!
//! Handle/entry access: the CLI stack keeps each entry's fsm / apply handle /
//! med handle inside stack.c's private thread_local arena, reachable only by
//! index (see crate::stack module notes). C dereferences the returned
//! `struct stack_entry *` directly (`stack_find_top()->fsm`,
//! `apply_down(stack_get_ah(), ...)`); the Rust twin routes those through the
//! stack_entry_fsm / stack_entry_ah / stack_entry_amedh closure accessors.

use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use flate2::Compression;
use flate2::write::GzEncoder;

use crate::apply::{
    apply_clear, apply_down, apply_init, apply_lower_words, apply_random_lower, apply_random_upper,
    apply_random_words, apply_reset_enumerator, apply_set_epsilon, apply_set_obey_flags,
    apply_set_print_pairs, apply_set_print_space, apply_set_separator, apply_set_show_flags,
    apply_set_space_symbol, apply_up, apply_upper_words, apply_words,
};
use crate::coaccessible::fsm_coaccessible;
use crate::constructions::{
    fsm_bimachine, fsm_close_sigma, fsm_compact, fsm_complement, fsm_complete, fsm_compose,
    fsm_concat, fsm_count, fsm_cross_product, fsm_equivalent, fsm_ignore, fsm_intersect, fsm_invert,
    fsm_kleene_plus, fsm_kleene_star, fsm_letter_machine, fsm_minus, fsm_sequentialize, fsm_shuffle,
    fsm_substitute_label, fsm_substitute_symbol, fsm_symbol, fsm_symbol_occurs, fsm_union,
};
use crate::define::{G_DEFINES, G_DEFINES_F, find_defined, remove_defined};
use crate::determinize::fsm_determinize;
use crate::extract::{fsm_lower, fsm_upper};
use crate::flags::{flag_eliminate, flag_twosided};
use crate::io::{
    foma_write_prolog, fsm_read_binary_file_multiple, fsm_read_binary_file_multiple_init,
    fsm_read_prolog, fsm_read_spaced_text_file, fsm_read_text_file, load_defined, net_print_att,
    read_att, save_defined,
};
use crate::mem::{
    G_ATT_EPSILON, G_COMPOSE_TRISTATE, G_FLAG_IS_EPSILON, G_LEXC_ALIGN, G_LIST_LIMIT,
    G_LIST_RANDOM_LIMIT, G_MED_CUTOFF, G_MED_LIMIT, G_MINIMAL, G_MINIMIZE_HOPCROFT, G_NAME_NETS,
    G_OBEY_FLAGS, G_PRINT_PAIRS, G_PRINT_SIGMA, G_PRINT_SPACE, G_QUIT_ON_FAIL, G_QUOTE_SPECIAL,
    G_RECURSIVE_DEFINE, G_SHOW_FLAGS, G_SORT_ARCS, G_VERBOSE,
};
use crate::minimize::fsm_minimize;
use crate::reverse::fsm_reverse;
use crate::sigma::sigma_sort;
use crate::spelling::{
    apply_med, apply_med_get_cost, apply_med_get_instring, apply_med_set_med_cutoff,
    apply_med_set_med_limit, apply_med_set_heap_max, cmatrix_print, cmatrix_print_att,
};
use crate::stack::{
    stack_add, stack_entry_ah, stack_entry_amedh, stack_entry_fsm, stack_entry_next,
    stack_find_bottom, stack_find_second, stack_find_top, stack_get_ah, stack_get_med_ah,
    stack_isempty, stack_pop, stack_rotate, stack_size,
};
use crate::structures::{
    fsm_copy, fsm_destroy, fsm_extract_ambiguous, fsm_extract_ambiguous_domain,
    fsm_extract_unambiguous, fsm_identity, fsm_isempty, fsm_isfunctional, fsm_isidentity,
    fsm_issequential, fsm_isunambiguous, fsm_sigma_net, fsm_sigma_pairs_net, fsm_sort_arcs,
};
use crate::topsort::fsm_topsort;
use crate::types::{
    AP_D, AP_U, ApplyHandle, EPSILON, Fsm, IDENTITY, M_LOWER, M_UPPER, OP_IGNORE_ALL,
    PATHCOUNT_CYCLIC, Sigma, UNKNOWN, YES,
};
use crate::utf8::{dequote_string, escape_string, utf8strlen, xstrrev};

// [spec:foma:def:iface.foma-net-print-fn]
// [spec:foma:sem:iface.foma-net-print-fn]
// C: `extern int foma_net_print(struct fsm *net, gzFile outfile);` — a forward
// declaration in iface.c of the function implemented in foma/io.c
// (`io.foma-net-print-fn`). The Rust twin re-exports io's implementation at this
// site so this file carries the iface.c extern-declaration annotation.
pub use crate::io::foma_net_print;

const FVAR_BOOL: i32 = 1;
const FVAR_INT: i32 = 2;
const FVAR_STRING: i32 = 3;

/// C: `#define LINE_LIMIT 8192` — fgets buffer size in iface_apply_file.
const LINE_LIMIT: usize = 8192;

/// DEVIATION from C: `struct g_v`'s `void *ptr` points at either an `int` global
/// (FVAR_BOOL / FVAR_INT) or a `char *` global (FVAR_STRING). Safe Rust cannot
/// hold a stable raw pointer into a thread_local, so the target is modelled as an
/// enum over the two real global kinds; the `type` field still distinguishes
/// FVAR_BOOL from FVAR_INT (both `int`), exactly as in C.
pub enum GvPtr {
    Int(&'static std::thread::LocalKey<Cell<i32>>),
    Str(&'static std::thread::LocalKey<RefCell<String>>),
}

// [spec:foma:def:iface.g-v]
// C: struct g_v { void *ptr; char *name; int type; } — element type of the
// global-variable dispatch table `global_vars[]`. The table and its consumers
// (iface_set_variable/iface_show_variable/iface_show_variables) are in the second
// half of iface.c; the table is built by `global_vars()` below.
pub struct Gv {
    pub ptr: GvPtr,
    pub name: &'static str,
    pub r#type: i32,
}

/// C: the file-static `struct g_v global_vars[]` table (NULL-terminated). Built
/// fresh here (read-only data, observably equivalent to the static array); the
/// trailing `{NULL, NULL, 0}` sentinel is represented by the end of the Vec.
pub(crate) fn global_vars() -> Vec<Gv> {
    vec![
        Gv { ptr: GvPtr::Int(&G_FLAG_IS_EPSILON), name: "flag-is-epsilon", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_MINIMAL), name: "minimal", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_NAME_NETS), name: "name-nets", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_OBEY_FLAGS), name: "obey-flags", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_PRINT_PAIRS), name: "print-pairs", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_PRINT_SIGMA), name: "print-sigma", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_PRINT_SPACE), name: "print-space", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_QUIT_ON_FAIL), name: "quit-on-fail", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_RECURSIVE_DEFINE), name: "recursive-define", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_QUOTE_SPECIAL), name: "quote-special", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_SHOW_FLAGS), name: "show-flags", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_SORT_ARCS), name: "sort-arcs", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_VERBOSE), name: "verbose", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_MINIMIZE_HOPCROFT), name: "hopcroft-min", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_COMPOSE_TRISTATE), name: "compose-tristate", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Int(&G_MED_LIMIT), name: "med-limit", r#type: FVAR_INT },
        Gv { ptr: GvPtr::Int(&G_MED_CUTOFF), name: "med-cutoff", r#type: FVAR_INT },
        Gv { ptr: GvPtr::Int(&G_LEXC_ALIGN), name: "lexc-align", r#type: FVAR_BOOL },
        Gv { ptr: GvPtr::Str(&G_ATT_EPSILON), name: "att-epsilon", r#type: FVAR_STRING },
    ]
}

/// C: `char warranty[]` — printed verbatim by iface_warranty.
static WARRANTY: &str = "\nLicensed under the Apache License, Version 2.0 (the \"License\")\nyou may not use this file except in compliance with the License.\nYou may obtain a copy of the License at\n\n    http://www.apache.org/licenses/LICENSE-2.0\n\n";

// [spec:foma:def:iface.global-help]
// C: struct global_help { char *name; char *help; char *longhelp; }.
pub struct GlobalHelp {
    pub name: &'static str,
    pub help: &'static str,
    pub longhelp: &'static str,
}

/// C: the file-static `struct global_help global_help[]` table (NULL-terminated).
/// The trailing `{NULL, NULL, NULL}` sentinel is represented by the slice end.
static GLOBAL_HELP: &[GlobalHelp] = &[
    GlobalHelp { name: "regex <regex>", help: "read a regular expression", longhelp: "Enter a regular expression and add result to top of stack.\nShort form: re\nSee `help operator' for operators, or `help precedence' for operator precedence." },
    GlobalHelp { name: "ambiguous upper", help: "returns the input words which have multiple paths in a transducer", longhelp: "Short form: ambiguous\n" },
    GlobalHelp { name: "apply up <string>", help: "apply <string> up to the top network on stack", longhelp: "Short form: up <string>\n" },
    GlobalHelp { name: "apply down <string>", help: "apply <string> down to the top network on stack", longhelp: "Short form: down <string>\n" },
    GlobalHelp { name: "apply med <string>", help: "find approximate matches to string in top network by minimum edit distance", longhelp: "Short form: med <string>\n" },
    GlobalHelp { name: "apply up", help: "enter apply up mode (Ctrl-D exits)", longhelp: "Short form: up\n" },
    GlobalHelp { name: "apply down", help: "enter apply down mode (Ctrl-D exits)", longhelp: "Short form: down\n" },
    GlobalHelp { name: "apply med", help: "enter apply med mode (Ctrl-D exits)", longhelp: "Short form: med\n" },
    GlobalHelp { name: "apropos <string>", help: "search help for <string>", longhelp: "" },
    GlobalHelp { name: "clear stack", help: "clears the stack", longhelp: "" },
    GlobalHelp { name: "close sigma", help: "removes unknown symbols from FSM", longhelp: "" },
    GlobalHelp { name: "compact sigma", help: "removes redundant symbols from FSM", longhelp: "" },
    GlobalHelp { name: "complete net", help: "completes the FSM", longhelp: "" },
    GlobalHelp { name: "compose net", help: "composes networks on stack", longhelp: "" },
    GlobalHelp { name: "concatenate", help: "concatenates networks on stack", longhelp: "" },
    GlobalHelp { name: "crossproduct net", help: "cross-product of top two FSMs on stack", longhelp: "See ×\n" },
    GlobalHelp { name: "define <name> <r.e.>", help: "define a network", longhelp: "Example: \ndefine A x -> y;\n  and\nA = x -> y;\n\nare equivalent\n" },
    GlobalHelp { name: "define <fname>(<v1,..,vn>) <r.e.>", help: "define function", longhelp: "Example: define Remove(X) [X -> 0].l;" },
    GlobalHelp { name: "determinize net", help: "determinizes top FSM on stack", longhelp: "" },
    GlobalHelp { name: "echo <string>", help: "echo a string", longhelp: "" },
    GlobalHelp { name: "eliminate flag <name>", help: "eliminate flag <name> diacritics from the top network", longhelp: "" },
    GlobalHelp { name: "eliminate flags", help: "eliminate all flag diacritics from the top network", longhelp: "" },
    GlobalHelp { name: "export cmatrix (filename)", help: "export the confusion matrix as an AT&T transducer", longhelp: "" },
    GlobalHelp { name: "extract ambiguous", help: "extracts the ambiguous paths of a transducer", longhelp: "Short form: examb" },
    GlobalHelp { name: "extract unambiguous", help: "extracts the unambiguous paths of a transducer", longhelp: "Short form: exunamb" },
    GlobalHelp { name: "help license", help: "prints license", longhelp: "" },
    GlobalHelp { name: "help warranty", help: "prints warranty information", longhelp: "" },
    GlobalHelp { name: "ignore net", help: "applies ignore to top two FSMs on stack", longhelp: "See /\n" },
    GlobalHelp { name: "intersect net", help: "intersects FSMs on stack", longhelp: "See ∩ (or &)\n" },
    GlobalHelp { name: "invert net", help: "inverts top FSM", longhelp: "See ⁻¹ (or .i)\n" },
    GlobalHelp { name: "label net", help: "extracts all attested symbol pairs from FSM", longhelp: "See also: sigma net" },
    GlobalHelp { name: "letter machine", help: "Converts top FSM to a letter machine", longhelp: "See also: _lm(L)" },
    GlobalHelp { name: "load stack <filename>", help: "Loads networks and pushes them on the stack", longhelp: "Short form: load" },
    GlobalHelp { name: "load defined <filename>", help: "Restores defined networks from file", longhelp: "Short form: loadd" },
    GlobalHelp { name: "lower-side net", help: "takes lower projection of top FSM", longhelp: "See ₂ (or .l)\n" },
    GlobalHelp { name: "minimize net", help: "minimizes top FSM", longhelp: "Minimization can be controlled through the variable minimal: when set to OFF FSMs are never minimized.\nAlso, hopcroft-min can be set to OFF in which case minimization is done by double reversal and determinization (aka Brzozowski's algorithm).  It is likely to be much slower.\n" },
    GlobalHelp { name: "name net <string>", help: "names top FSM", longhelp: "" },
    GlobalHelp { name: "negate net", help: "complements top FSM", longhelp: "See ¬\n" },
    GlobalHelp { name: "one-plus net", help: "Kleene plus on top FSM", longhelp: "See +\n" },
    GlobalHelp { name: "pop stack", help: "remove top FSM from stack", longhelp: "" },
    GlobalHelp { name: "print cmatrix", help: "prints the confusion matrix associated with the top network in tabular format", longhelp: "" },
    GlobalHelp { name: "print defined", help: "prints defined symbols and functions", longhelp: "" },
    GlobalHelp { name: "print dot (>filename)", help: "prints top FSM in Graphviz dot format", longhelp: "" },
    GlobalHelp { name: "print lower-words", help: "prints words on the lower side of top FSM", longhelp: "" },
    GlobalHelp { name: "print lower-words > filename", help: "prints words on the lower side of top FSM to file", longhelp: "" },
    GlobalHelp { name: "print name", help: "prints the name of the top FSM", longhelp: "" },
    GlobalHelp { name: "print net", help: "prints all information about top FSM", longhelp: "Short form: net\n" },
    GlobalHelp { name: "print pairs", help: "prints input-output pairs from top FSM", longhelp: "Short form: pairs\n" },
    GlobalHelp { name: "print pairs > filename", help: "prints input-output pairs from top FSM to file", longhelp: "Short form: pairs\n" },
    GlobalHelp { name: "print random-lower", help: "prints random words from lower side", longhelp: "Short form: random-lower\n" },
    GlobalHelp { name: "print random-upper", help: "prints random words from upper side", longhelp: "Short form: random-upper" },
    GlobalHelp { name: "print random-words", help: "prints random words from top FSM", longhelp: "Short form: random-words\n" },
    GlobalHelp { name: "print random-pairs", help: "prints random input-output pairs from top FSM", longhelp: "Short form: random-pairs\n" },
    GlobalHelp { name: "print sigma", help: "prints the alphabet of the top FSM", longhelp: "Short form: sigma\n" },
    GlobalHelp { name: "print size", help: "prints size information about top FSM", longhelp: "Short form: size\n" },
    GlobalHelp { name: "print shortest-string", help: "prints the shortest string of the top FSM", longhelp: "Short form: pss\n" },
    GlobalHelp { name: "print shortest-string-size", help: "prints length of shortest string", longhelp: "Short form: psz\n" },
    GlobalHelp { name: "print upper-words", help: "prints words on the upper side of top FSM", longhelp: "Short form: upper-words" },
    GlobalHelp { name: "print upper-words > filename", help: "prints words on the upper side of top FSM to file", longhelp: "Short form:upper-words" },
    GlobalHelp { name: "print words", help: "prints words of top FSM", longhelp: "Short form: words" },
    GlobalHelp { name: "print words > filename", help: "prints words of top FSM to file", longhelp: "Short form: words" },
    GlobalHelp { name: "prune net", help: "makes top network coaccessible", longhelp: "" },
    GlobalHelp { name: "push (defined) <name>", help: "adds a defined FSM to top of stack", longhelp: "" },
    GlobalHelp { name: "quit", help: "exit foma", longhelp: "" },
    GlobalHelp { name: "read att <filename>", help: "read a file in AT&T FSM format and add to top of stack", longhelp: "Short form: ratt" },
    GlobalHelp { name: "read cmatrix <filename>", help: "read a confusion matrix and associate it with the network on top of the stack", longhelp: "" },
    GlobalHelp { name: "read prolog <filename>", help: "reads prolog format file", longhelp: "" },
    GlobalHelp { name: "read lexc <filename>", help: "read and compile lexc format file", longhelp: "" },
    GlobalHelp { name: "read spaced-text <filename>", help: "compile space-separated words/word-pairs separated by newlines into a FST", longhelp: "" },
    GlobalHelp { name: "read text <filename>", help: "compile a list of words separated by newlines into an automaton", longhelp: "" },
    GlobalHelp { name: "reverse net", help: "reverses top FSM", longhelp: "Short form: rev\nSee .r\n" },
    GlobalHelp { name: "rotate stack", help: "rotates stack", longhelp: "" },
    GlobalHelp { name: "save defined <filename>", help: "save all defined networks to binary file", longhelp: "Short form: saved" },
    GlobalHelp { name: "save stack <filename>", help: "save stack to binary file", longhelp: "Short form: ss" },
    GlobalHelp { name: "set <variable> <ON|OFF>", help: "sets a global variable (see show variables)", longhelp: "" },
    GlobalHelp { name: "show variables", help: "prints all variable/value pairs", longhelp: "" },
    GlobalHelp { name: "shuffle net", help: "asynchronous product on top two FSMs on stack", longhelp: "See ∥ (or <>)\n" },
    GlobalHelp { name: "sigma net", help: "Extracts the alphabet and creates a FSM that accepts all single symbols in it", longhelp: "See also: label net" },
    GlobalHelp { name: "source <file>", help: "read and compile script file", longhelp: "" },
    GlobalHelp { name: "sort net", help: "sorts arcs topologically on top FSM", longhelp: "" },
    GlobalHelp { name: "sort in", help: "sorts input arcs by sigma numbers on top FSM", longhelp: "" },
    GlobalHelp { name: "sort out", help: "sorts output arcs by sigma number on top FSM", longhelp: "" },
    GlobalHelp { name: "substitute defined X for Y", help: "substitutes defined network X at all arcs containing Y ", longhelp: "" },
    GlobalHelp { name: "substitute symbol X for Y", help: "substitutes all occurrences of Y in an arc with X", longhelp: "" },
    GlobalHelp { name: "system <cmd>", help: "execute a system command", longhelp: "" },
    GlobalHelp { name: "test unambiguous", help: "test if top FST is unambiguous", longhelp: "Short form: tunam\n" },
    GlobalHelp { name: "test equivalent", help: "test if the top two FSMs are equivalent", longhelp: "Short form: equ\nNote: equivalence is undecidable for transducers in the general case.  The result is reliable only for recognizers.\n" },
    GlobalHelp { name: "test functional", help: "test if the top FST is functional (single-valued)", longhelp: "Short form: tfu\n" },
    GlobalHelp { name: "test identity", help: "test if top FST represents identity relations only", longhelp: "Short form: tid\n" },
    GlobalHelp { name: "test lower-universal", help: "test if lower side is Σ*", longhelp: "Short form: tlu\n" },
    GlobalHelp { name: "test upper-universal", help: "test if upper side is Σ*", longhelp: "Short form: tuu\n" },
    GlobalHelp { name: "test non-null", help: "test if top machine is not the empty language", longhelp: "Short form:tnn\n" },
    GlobalHelp { name: "test null", help: "test if top machine is the empty language (∅)", longhelp: "Short form: tnu\n" },
    GlobalHelp { name: "test sequential", help: "tests if top machine is sequential", longhelp: "Short form: tseq\n" },
    GlobalHelp { name: "turn stack", help: "turns stack upside down", longhelp: "" },
    GlobalHelp { name: "twosided flag-diacritics", help: "changes flags to always be identity pairs", longhelp: "Short form: tfd" },
    GlobalHelp { name: "undefine <name>", help: "remove <name> from defined networks", longhelp: "See define\n" },
    GlobalHelp { name: "union net", help: "union of top two FSMs", longhelp: "See ∪ (or |)\n" },
    GlobalHelp { name: "upper-side net", help: "upper projection of top FSM", longhelp: "See ₁ (or .u)\n" },
    GlobalHelp { name: "view net", help: "display top network (if supported)", longhelp: "" },
    GlobalHelp { name: "zero-plus net", help: "Kleene star on top fsm", longhelp: "See *\n" },
    GlobalHelp { name: "variable compose-tristate", help: "use the tristate composition algorithm", longhelp: "Default value: OFF\n" },
    GlobalHelp { name: "variable show-flags", help: "show flag diacritics in `apply'", longhelp: "Default value: ON\n" },
    GlobalHelp { name: "variable obey-flags", help: "obey flag diacritics in `apply'", longhelp: "Default value: ON\n" },
    GlobalHelp { name: "variable minimal", help: "minimize resulting FSMs", longhelp: "Default value: ON\n" },
    GlobalHelp { name: "variable print-pairs", help: "always print both sides when applying", longhelp: "Default value: OFF\n" },
    GlobalHelp { name: "variable print-space", help: "print spaces between symbols", longhelp: "Default value: OFF\n" },
    GlobalHelp { name: "variable print-sigma", help: "print the alphabet when printing network", longhelp: "Default value: ON\n" },
    GlobalHelp { name: "quit-on-fail", help: "Abort operations when encountering errors", longhelp: "Default value: ON\n" },
    GlobalHelp { name: "variable recursive-define", help: "Allow recursive definitions", longhelp: "Default value: OFF\n" },
    GlobalHelp { name: "variable verbose", help: "Verbosity of interface", longhelp: "Default value: ON\n" },
    GlobalHelp { name: "variable hopcroft-min", help: "ON = Hopcroft minimization, OFF = Brzozowski minimization", longhelp: "Default value: ON\n" },
    GlobalHelp { name: "variable med-limit", help: "the limit on number of matches in apply med", longhelp: "Default value: 3\n" },
    GlobalHelp { name: "variable med-cutoff", help: "the cost limit for terminating a search in apply med", longhelp: "Default value: 3\n" },
    GlobalHelp { name: "variable att-epsilon", help: "the EPSILON symbol when reading/writing AT&T files", longhelp: "Default value: @0@\n" },
    GlobalHelp { name: "variable lexc-align", help: "Forces X:0 X:X of 0:X alignment of lexicon entry symbols", longhelp: "Default value: OFF\n" },
    GlobalHelp { name: "write prolog (> filename)", help: "writes top network to prolog format file/stdout", longhelp: "Short form: wpl" },
    GlobalHelp { name: "write att (> <filename>)", help: "writes top network to AT&T format file/stdout", longhelp: "Short form: watt" },
    GlobalHelp { name: "re operator: (∀<var name>)(F)", help: "universal quantification", longhelp: "Example: $.A is equivalent to:\n(∃x)(x ∈ A ∧ (∀y)(¬(y ∈ A ∧ ¬(x = y))))" },
    GlobalHelp { name: "re operator: (∃<var name>)(F)", help: "existential quantification", longhelp: "Example: $.A is equivalent to:\n(∃x)(x ∈ A ∧ ¬(∃y)(y ∈ A ∧ ¬(x = y)))" },
    GlobalHelp { name: "logic re operator: ∈", help: "`in' predicate for logical formulae", longhelp: "" },
    GlobalHelp { name: "logic re operator: S(t1,t2)", help: "successor-of predicate for logical formulae", longhelp: "" },
    GlobalHelp { name: "logic re operator: ≤", help: "less-than or equal-to", longhelp: "Refers to position of quantified substring\n" },
    GlobalHelp { name: "logic re operator: ≥", help: "more-than or equal-to", longhelp: "Refers to position of quantified substring\n" },
    GlobalHelp { name: "logic re operator: ≺", help: "precedes", longhelp: "Refers to position of quantified substring\n" },
    GlobalHelp { name: "logic re operator: ≻", help: "follows", longhelp: "Refers to position of quantified substring\n" },
    GlobalHelp { name: "logic re operator: ∧", help: "conjunction", longhelp: "Operationally equivalent to ∩\n" },
    GlobalHelp { name: "logic re operator: ∨", help: "disjunction", longhelp: "Operationally equivalent to ∪\n" },
    GlobalHelp { name: "logic re operator: →", help: "implication", longhelp: "A → B is equivalent to ¬A ∨ B " },
    GlobalHelp { name: "logic re operator: ↔", help: "biconditional", longhelp: "A ↔ B is equivalent to (¬A ∨ B) ∧ (¬B ∨ A)" },
    GlobalHelp { name: "re operator: ∘ (or .o.) ", help: "compose", longhelp: "A .o. B is the composition of transducers/recognizers A and B\nThe composition algorithm can be controlled with the variable\ncompose-tristate.  The default algorithm is a `bistate' composition that eliminates redundant paths but may fail to find the shortest path.\n" },
    GlobalHelp { name: "re operator: × (or .x.) ", help: "cross-product", longhelp: "A × B (where A and B are recognizers, not transducers\nyields the cross-product of A and B.\n" },
    GlobalHelp { name: "re operator: .O. ", help: "`lenient' composition", longhelp: "Lenient composition as defined in Karttunen(1998)  A .O. B = [A ∘ B] .P. B\n" },
    GlobalHelp { name: "re operator: ∥ (or <>) ", help: "shuffle (asynchronous product)", longhelp: "A ∥ B yields the asynchronous (or shuffle) product of FSM A and B.\n" },
    GlobalHelp { name: "re operator: => ", help: "context restriction, e.g. A => B _ C, D _ E", longhelp: "A => B _ C yields the language where every instance of a substring drawn from A is surrounded by B and C.  Multiple contexts can be specified if separated by commas, e.g.: A => B _ C, D _ E" },
    GlobalHelp { name: "re operator: ->, <-, <->, etc.", help: "replacement operators", longhelp: "If LHS is a transducer, no RHS is needed in rule." },
    GlobalHelp { name: "re operator: @->, @>, etc.", help: "directed replacement operators", longhelp: "" },
    GlobalHelp { name: "re operator: (->), (@->), etc. ", help: "optional replacements", longhelp: "Optional replacement operators variants.  Note that the directional modes leftmost/rightmost/longest/shortest are not affected by optionality, i.e. only replacement is optional, not mode.  Hence A (@->) B is not in general equivalent to the parallel rule A @-> B, A -> ... " },
    GlobalHelp { name: "re operator: ||,\\/,\\\\,// ", help: "replacement direction specifiers", longhelp: "Rewrite rules direction specifier meaning is:\nA -> B || C _ D (replace if C and D match on upper side)\nA -> B // C _ D (replace if C matches of lower side and D matches on upper side)\nA -> B \\\\ C _ D (replace if C matches on upper side and D matches on lower side)\nA -> B \\/ C _ D (replace if C and D match on lower side)\n" },
    GlobalHelp { name: "re operator: _ ", help: "replacement or restriction context specifier", longhelp: "" },
    GlobalHelp { name: "re operator: ,,", help: "parallel context replacement operator", longhelp: "Separates parallel rules, e.g.:\nA -> B , C @-> D || E _ F ,, G -> H \\/ I _ J\n" },
    GlobalHelp { name: "re operator: ,", help: "parallel replacement operator", longhelp: "Separates rules and contexts. Example: A -> B, C <- D || E _ F" },
    GlobalHelp { name: "re operator: [.<r.e.>.]", help: "single-epsilon control in replacement LHS, e.g. [..] -> x", longhelp: "If the LHS contains the empty string, as does [.a*.] -> x, the rule yields a transducer where the empty string is assumed to occur exactly once between each symbol." },
    GlobalHelp { name: "re operator: ...", help: "markup replacement control (e.g. A -> B ... C || D _ E)", longhelp: "A -> B ... C yields a replacement transducer where the center A is left untouched and B and C inserted around A." },
    GlobalHelp { name: "re operator:  ", help: "concatenation", longhelp: "Binary operator: A B\nConcatenation is performed implicitly according to its precedence level without overt specification\n" },
    GlobalHelp { name: "re operator: ∪ (or |) ", help: "union", longhelp: "Binary operator: A|B" },
    GlobalHelp { name: "re operator: ∩ (or &) ", help: "intersection", longhelp: "Binary operator: A&B" },
    GlobalHelp { name: "re operator: - ", help: "set minus", longhelp: "Binary operator A-B" },
    GlobalHelp { name: "re operator: .P.", help: "priority union (upper)", longhelp: "Binary operator A .P. B\nEquivalent to: A .P. B = A ∪ [¬[A₁] ∘ B]\n" },
    GlobalHelp { name: "re operator: .p.", help: "priority union (lower)", longhelp: "Binary operator A .p. B\nEquivalent to: A .p. B = A ∪ [¬[A₂] ∘ B]" },
    GlobalHelp { name: "re operator: <", help: "precedes", longhelp: "Binary operator A < B\nYields the language where no instance of A follows an instance of B." },
    GlobalHelp { name: "re operator: >", help: "follows", longhelp: "Binary operator A > B\nYields the language where no instance of A precedes an instance of B." },
    GlobalHelp { name: "re operator: /", help: "ignore", longhelp: "Binary operator A/B\nYield the language/transducer where arbitrary sequences of strings/mappings from B are interspersed in A.  For single-symbol languages B, A/B = A ∥ B*" },
    GlobalHelp { name: "re operator: ./.", help: "ignore except at edges", longhelp: "Yields the language where arbitrary sequences from B are interspersed in A, except as the very first and very last symbol." },
    GlobalHelp { name: "re operator: \\\\\\", help: "left quotient", longhelp: "Binary operator: A\\\\\\B\nInformally:  the set of suffixes one can add to A to get strings in B\n" },
    GlobalHelp { name: "re operator: ///", help: "right quotient", longhelp: "Binary operator A///B\nInformally: the set of prefixes one can add to B to get a string in A\n" },
    GlobalHelp { name: "re operator: /\\/", help: "interleaving quotient", longhelp: "Binary operator A/\\/B\nInformally: the set of strings you can interdigitate (non-continuously) to B to get strings in A\n" },
    GlobalHelp { name: "re operator: ¬ (or ~) ", help: "complement", longhelp: "Unary operator ~A, equivalent to Σ* - A\n" },
    GlobalHelp { name: "re operator: $", help: "contains a factor of", longhelp: "Unary operator $A\nEquivalent to: Σ* A Σ*\n" },
    GlobalHelp { name: "re operator: $.", help: "contains exactly one factor of", longhelp: "Unary operator $.A\nYields the language that contains exactly one factor from A.\nExample: if A = [a b|b a], $.A contains strings ab, ba, abb, bba, but not abab, baba, aba, bab, etc.\n" },
    GlobalHelp { name: "re operator: $?", help: "contains maximally one factor of", longhelp: "Unary operator: $?A, yields the language that contains zero or one factors from A. See also $.A." },
    GlobalHelp { name: "re operator: +", help: "Kleene plus", longhelp: "Unary operator A+\n" },
    GlobalHelp { name: "re operator: *", help: "Kleene star", longhelp: "Unary operator A*\n" },
    GlobalHelp { name: "re operator: ^n ^<n ^>n ^{m,n}", help: "m, n-ary concatenations", longhelp: "A^n: A concatenated with itself exactly n times\nA^<n: A concatenated with itself less than n times\nA^>n: A concatenated with itself more than n times\nA^{m,n}: A concatenated with itself between m and n times\n" },
    GlobalHelp { name: "re operator: ₁ (or .1 or .u)", help: "upper projection", longhelp: "Unary operator A.u\n" },
    GlobalHelp { name: "re operator: ₂ (or .2 or .l)", help: "lower projection", longhelp: "Unary operator A.l\n" },
    GlobalHelp { name: "re operator: ⁻¹ (or .i)", help: "inverse of transducer", longhelp: "Unary operator A.i\n" },
    GlobalHelp { name: "re operator: .f", help: "eliminate all flags", longhelp: "Unary operator A.f: eliminates all flag diacritics in A" },
    GlobalHelp { name: "re operator: .r", help: "reverse of FSM", longhelp: "Unary operator A.r\n" },
    GlobalHelp { name: "re operator: :", help: "cross-product", longhelp: "Binary operator A:B, see also A × B\n" },
    GlobalHelp { name: "re operator: \\", help: "term complement (\\x = [Σ-x])", longhelp: "Unary operator \\A\nSingle symbols not in A.  Equivalent to [Σ-A]\n" },
    GlobalHelp { name: "re operator: `", help: "substitution/homomorphism", longhelp: "Ternary operator `[A,B,C] Replace instances of symbol B with symbol C in language A.  Also removes the substituted symbol from the alphabet.\n" },
    GlobalHelp { name: "re operator: { ... }", help: "concatenate symbols", longhelp: "Single-symbol-concatenation\nExample: {abcd} is equivalent to a b c d\n" },
    GlobalHelp { name: "re operator: (A)", help: "optionality", longhelp: "Equivalent to A | ε\nNote: parentheses inside logical formulas function as grouping, see ∀,∃\n" },
    GlobalHelp { name: "re operator: @\"filename\"", help: "read saved network from file", longhelp: "Note: loads networks stored with e.g. \"save stack\" but if file contains more than one network, only the first one is used in the regular expression.  See also \"load stack\" and \"load defined\"\n" },
    GlobalHelp { name: "special symbol: Σ (or ?)", help: "`any' symbol in r.e.", longhelp: "" },
    GlobalHelp { name: "special symbol: ε (or 0, [])", help: "epsilon symbol in r.e.", longhelp: "" },
    GlobalHelp { name: "special symbol: ∅", help: "the empty language symbol in r.e.", longhelp: "" },
    GlobalHelp { name: "special symbol: .#.", help: "word boundary symbol in replacements, restrictions", longhelp: "Signifies both end and beginning of word/string\nExample: A => B _ .#. (allow A only between B and end-of-string)\nExample: A -> B || .#. _ C (replace A with B if it occurs in the beginning of a word and is followed by C)\n" },
    GlobalHelp { name: "operator precedence: ", help: "see: `help precedence'", longhelp: "\\ `\n:\n+ * ^ ₁ ₂ ⁻¹ .f .r\n¬ $ $. $?\n(concatenation)\n> <\n∪ ∩ - .P. .p.\n=> -> (->) @-> etc.\n∥\n× ∘ .O.\nNote: compatibility variants (i.e. | = ∪ etc.) are not listed." },
];

// [spec:foma:def:iface.iface-help-fn]
// [spec:foma:sem:iface.iface-help-fn]
// [spec:foma:def:foma.iface-help-fn]
// [spec:foma:sem:foma.iface-help-fn]
pub fn iface_help() {
    let mut maxlen: i32 = 0;
    for gh in GLOBAL_HELP {
        let l = utf8strlen(gh.name.as_bytes());
        maxlen = if maxlen < l { l } else { maxlen };
    }
    for gh in GLOBAL_HELP {
        print!("{}", gh.name);
        // padding loop runs from maxlen-len down to 0 inclusive → always >= 1 space
        let mut i = maxlen - utf8strlen(gh.name.as_bytes());
        while i >= 0 {
            print!("{}", " ");
            i -= 1;
        }
        print!("{}\n", gh.help);
    }
}

// [spec:foma:def:iface.iface-ambiguous-upper-fn]
// [spec:foma:sem:iface.iface-ambiguous-upper-fn]
// [spec:foma:def:foma.iface-ambiguous-upper-fn]
// [spec:foma:sem:foma.iface-ambiguous-upper-fn]
pub fn iface_ambiguous_upper() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_extract_ambiguous_domain(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-apropos-fn]
// [spec:foma:sem:iface.iface-apropos-fn]
// [spec:foma:def:foma.iface-apropos-fn]
// [spec:foma:sem:foma.iface-apropos-fn]
pub fn iface_apropos(s: &str) {
    let mut maxlen: i32 = 0;
    for gh in GLOBAL_HELP {
        // strstr(x, s) != NULL ↔ x contains s as a byte substring
        if gh.name.contains(s) || gh.help.contains(s) {
            let l = utf8strlen(gh.name.as_bytes());
            maxlen = if maxlen < l { l } else { maxlen };
        }
    }
    for gh in GLOBAL_HELP {
        if gh.name.contains(s) || gh.help.contains(s) {
            print!("{}", gh.name);
            let mut i = maxlen - utf8strlen(gh.name.as_bytes());
            while i >= 0 {
                print!("{}", " ");
                i -= 1;
            }
            print!("{}\n", gh.help);
        }
    }
}

// [spec:foma:def:iface.iface-help-search-fn]
// [spec:foma:sem:iface.iface-help-search-fn]
// [spec:foma:def:foma.iface-help-search-fn]
// [spec:foma:sem:foma.iface-help-search-fn]
pub fn iface_help_search(s: &str) {
    for gh in GLOBAL_HELP {
        if gh.name.contains(s) || gh.help.contains(s) {
            print!("##\n");
            // printf("%-32.32s%s\n%s\n", name, help, longhelp): name is left-
            // justified and truncated/padded to exactly 32 BYTES (byte-based, not
            // UTF-8 aware), so the truncation is written as raw bytes.
            let nb = gh.name.as_bytes();
            let take = if nb.len() < 32 { nb.len() } else { 32 };
            let mut out = std::io::stdout();
            let _ = out.write_all(&nb[..take]);
            for _ in take..32 {
                print!("{}", " ");
            }
            print!("{}\n{}\n", gh.help, gh.longhelp);
        }
    }
}

// [spec:foma:def:iface.iface-print-bool-fn]
// [spec:foma:sem:iface.iface-print-bool-fn]
pub fn iface_print_bool(value: i32) {
    print!("{} (1 = TRUE, 0 = FALSE)\n", value);
}

// [spec:foma:def:iface.iface-warranty-fn]
// [spec:foma:sem:iface.iface-warranty-fn]
// [spec:foma:def:foma.iface-warranty-fn]
// [spec:foma:sem:foma.iface-warranty-fn]
pub fn iface_warranty() {
    print!("{}", WARRANTY);
}

// [spec:foma:def:iface.iface-apply-set-params-fn]
// [spec:foma:sem:iface.iface-apply-set-params-fn]
// [spec:foma:def:foma.iface-apply-set-params-fn]
// [spec:foma:sem:foma.iface-apply-set-params-fn]
pub fn iface_apply_set_params(h: &mut ApplyHandle) {
    apply_set_print_space(h, G_PRINT_SPACE.with(|v| v.get()));
    apply_set_print_pairs(h, G_PRINT_PAIRS.with(|v| v.get()));
    apply_set_show_flags(h, G_SHOW_FLAGS.with(|v| v.get()));
    apply_set_obey_flags(h, G_OBEY_FLAGS.with(|v| v.get()));
}

// DEVIATION from C: perror(s) prints "s: <strerror(errno)>\n" to stderr. Rust has
// no libc errno; `std::io::Error::last_os_error()` reads the current thread's
// errno (set by the preceding failed syscall) and its Display is close to
// strerror (adds "(os error N)"). Unannotated plumbing; shared with slice 2.
fn perror(s: &str) {
    eprint!("{}: {}\n", s, std::io::Error::last_os_error());
}

// [spec:foma:def:iface.iface-apply-med-fn]
// [spec:foma:sem:iface.iface-apply-med-fn]
// [spec:foma:def:foma.iface-apply-med-fn]
// [spec:foma:sem:foma.iface-apply-med-fn]
pub fn iface_apply_med(word: &str) {
    if iface_stack_check(1) == 0 {
        return;
    }
    // amedh = stack_get_med_ah() — arena index of the top entry (see module notes)
    let amedh = stack_get_med_ah().unwrap();

    stack_entry_amedh(amedh, |h| {
        apply_med_set_heap_max(h, 4194304 + 1);
        apply_med_set_med_limit(h, G_MED_LIMIT.with(|v| v.get()));
        apply_med_set_med_cutoff(h, G_MED_CUTOFF.with(|v| v.get()));
    });

    let result = stack_entry_amedh(amedh, |h| apply_med(h, Some(word)));
    match result {
        None => {
            print!("???\n");
            return;
        }
        Some(r) => {
            print!("{}\n", r);
            let (instr, cost) =
                stack_entry_amedh(amedh, |h| (apply_med_get_instring(h), apply_med_get_cost(h)));
            print!("{}\n", instr.unwrap_or_default());
            print!("Cost[f]: {}\n\n", cost);
        }
    }
    loop {
        let result = stack_entry_amedh(amedh, |h| apply_med(h, None));
        match result {
            None => break,
            Some(r) => {
                print!("{}\n", r);
                let (instr, cost) =
                    stack_entry_amedh(amedh, |h| (apply_med_get_instring(h), apply_med_get_cost(h)));
                print!("{}\n", instr.unwrap_or_default());
                print!("Cost[f]: {}\n\n", cost);
            }
        }
    }
}

// [spec:foma:def:iface.iface-apply-file-fn]
// [spec:foma:sem:iface.iface-apply-file-fn]
// [spec:foma:def:foma.iface-apply-file-fn]
// [spec:foma:sem:foma.iface-apply-file-fn]
pub fn iface_apply_file(infilename: &str, outfilename: Option<&str>, direction: i32) -> i32 {
    let _ = LINE_LIMIT; // fgets buffer size; read_line reads whole lines here.
    if direction != AP_D && direction != AP_U {
        perror("Invalid direction in iface_apply_file().\n");
        return 1;
    }
    if iface_stack_check(1) == 0 {
        return 0;
    }
    let infile = match File::open(infilename) {
        Ok(f) => f,
        Err(_) => {
            eprint!("{}: ", infilename);
            perror("Error opening file");
            return 1;
        }
    };

    // C: OUTFILE = fopen(outfilename, "w") happens BEFORE the "Writing output to
    // file" message, which is BEFORE the NULL check — so the message prints even
    // when the open fails.
    let mut outfile: Box<dyn Write> = match outfilename {
        None => Box::new(std::io::stdout()),
        Some(name) => {
            let res = File::create(name);
            print!("Writing output to file {}.\n", name);
            match res {
                Ok(f) => Box::new(f),
                Err(_) => {
                    eprint!("{}: ", name);
                    perror("Error opening output file.");
                    return 1;
                }
            }
        }
    };

    let ah = stack_get_ah().unwrap();
    stack_entry_ah(ah, |h| iface_apply_set_params(h));

    let mut reader = BufReader::new(infile);
    let mut inword = String::new();
    loop {
        inword.clear();
        // fgets: NULL at EOF. read_line returns Ok(0) at EOF.
        // DEVIATION from C: read_line requires UTF-8; a decode error is treated as
        // end-of-input here (C reads raw bytes).
        let n = reader.read_line(&mut inword).unwrap_or(0);
        if n == 0 {
            break;
        }
        // C: if (inword[strlen(inword)-1] == '\n') inword[strlen-1] = '\0';
        // DEVIATION from C (on a line whose first byte is NUL, strlen==0 and the C
        // indexes inword[-1] — OOB; guard non-empty and strip a trailing '\n').
        if !inword.is_empty() && inword.as_bytes()[inword.len() - 1] == b'\n' {
            inword.pop();
        }

        let _ = write!(outfile, "\n{}\n", inword);
        let result = if direction == AP_D {
            stack_entry_ah(ah, |h| apply_down(h, Some(&inword)))
        } else {
            stack_entry_ah(ah, |h| apply_up(h, Some(&inword)))
        };

        let result = match result {
            None => {
                let _ = write!(outfile, "???\n");
                continue;
            }
            Some(r) => r,
        };
        let _ = write!(outfile, "{}\n", result);
        loop {
            let result = if direction == AP_D {
                stack_entry_ah(ah, |h| apply_down(h, None))
            } else {
                stack_entry_ah(ah, |h| apply_up(h, None))
            };
            match result {
                None => break,
                Some(r) => {
                    let _ = write!(outfile, "{}\n", r);
                }
            }
        }
    }
    // C: fclose(OUTFILE) only when outfilename != NULL; the input file is never
    // fclose'd (latent leak). Rust drops (closes) both at scope end; stdout is not
    // closed. The observable difference (leak vs. drop) is unrepresentable safely.
    0
}

// [spec:foma:def:iface.iface-apply-down-fn]
// [spec:foma:sem:iface.iface-apply-down-fn]
// [spec:foma:def:foma.iface-apply-down-fn]
// [spec:foma:sem:foma.iface-apply-down-fn]
pub fn iface_apply_down(word: &str) {
    if iface_stack_check(1) == 0 {
        return;
    }
    let ah = stack_get_ah().unwrap();
    stack_entry_ah(ah, |h| iface_apply_set_params(h));
    let result = stack_entry_ah(ah, |h| apply_down(h, Some(word)));
    match result {
        None => {
            print!("???\n");
            return;
        }
        Some(r) => {
            print!("{}\n", r);
        }
    }
    let mut i = G_LIST_LIMIT.with(|v| v.get());
    while i > 0 {
        let result = stack_entry_ah(ah, |h| apply_down(h, None));
        match result {
            None => break,
            Some(r) => print!("{}\n", r),
        }
        i -= 1;
    }
}

// [spec:foma:def:iface.iface-apply-up-fn]
// [spec:foma:sem:iface.iface-apply-up-fn]
// [spec:foma:def:foma.iface-apply-up-fn]
// [spec:foma:sem:foma.iface-apply-up-fn]
pub fn iface_apply_up(word: &str) {
    if iface_stack_check(1) == 0 {
        return;
    }
    let ah = stack_get_ah().unwrap();
    stack_entry_ah(ah, |h| iface_apply_set_params(h));
    let result = stack_entry_ah(ah, |h| apply_up(h, Some(word)));
    match result {
        None => {
            print!("???\n");
            return;
        }
        Some(r) => {
            print!("{}\n", r);
        }
    }
    let mut i = G_LIST_LIMIT.with(|v| v.get());
    while i > 0 {
        let result = stack_entry_ah(ah, |h| apply_up(h, None));
        match result {
            None => break,
            Some(r) => print!("{}\n", r),
        }
        i -= 1;
    }
}

// [spec:foma:def:iface.iface-close-fn]
// [spec:foma:sem:iface.iface-close-fn]
// [spec:foma:def:foma.iface-close-fn]
// [spec:foma:sem:foma.iface-close-fn]
pub fn iface_close() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_topsort(fsm_minimize(fsm_close_sigma(stack_pop().unwrap(), 0))));
    }
}

// [spec:foma:def:iface.iface-compact-fn]
// [spec:foma:sem:iface.iface-compact-fn]
// [spec:foma:def:foma.iface-compact-fn]
// [spec:foma:sem:foma.iface-compact-fn]
pub fn iface_compact() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |f| fsm_compact(f));
        stack_entry_fsm(top, |f| sigma_sort(f));
        stack_add(fsm_topsort(fsm_minimize(stack_pop().unwrap())));
    }
}

// [spec:foma:def:iface.iface-complete-fn]
// [spec:foma:sem:iface.iface-complete-fn]
// [spec:foma:def:foma.iface-complete-fn]
// [spec:foma:sem:foma.iface-complete-fn]
pub fn iface_complete() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_complete(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-compose-fn]
// [spec:foma:sem:iface.iface-compose-fn]
// [spec:foma:def:foma.iface-compose-fn]
// [spec:foma:sem:foma.iface-compose-fn]
pub fn iface_compose() {
    if iface_stack_check(2) != 0 {
        while stack_size() > 1 {
            let one = stack_pop().unwrap();
            let two = stack_pop().unwrap();
            stack_add(fsm_topsort(fsm_minimize(fsm_compose(one, two))));
        }
    }
}

// [spec:foma:def:iface.iface-conc-fn]
// [spec:foma:sem:iface.iface-conc-fn]
// [spec:foma:def:foma.iface-conc-fn]
// [spec:foma:sem:foma.iface-conc-fn]
pub fn iface_conc() {
    if iface_stack_check(2) != 0 {
        while stack_size() > 1 {
            // leftover debug printf("dd") (no newline) — shipped behavior
            print!("dd");
            let one = stack_pop().unwrap();
            let two = stack_pop().unwrap();
            stack_add(fsm_topsort(fsm_minimize(fsm_concat(one, two))));
        }
    }
}

// [spec:foma:def:iface.iface-crossproduct-fn]
// [spec:foma:sem:iface.iface-crossproduct-fn]
// [spec:foma:def:foma.iface-crossproduct-fn]
// [spec:foma:sem:foma.iface-crossproduct-fn]
pub fn iface_crossproduct() {
    if iface_stack_check(2) != 0 {
        let one = stack_pop().unwrap();
        let two = stack_pop().unwrap();
        stack_add(fsm_topsort(fsm_minimize(fsm_cross_product(one, two))));
    }
}

// [spec:foma:def:iface.iface-determinize-fn]
// [spec:foma:sem:iface.iface-determinize-fn]
// [spec:foma:def:foma.iface-determinize-fn]
// [spec:foma:sem:foma.iface-determinize-fn]
pub fn iface_determinize() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_determinize(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-eliminate-flags-fn]
// [spec:foma:sem:iface.iface-eliminate-flags-fn]
// [spec:foma:def:foma.iface-eliminate-flags-fn]
// [spec:foma:sem:foma.iface-eliminate-flags-fn]
pub fn iface_eliminate_flags() {
    if iface_stack_check(1) != 0 {
        stack_add(flag_eliminate(stack_pop().unwrap(), None));
    }
}

// [spec:foma:def:iface.iface-extract-ambiguous-fn]
// [spec:foma:sem:iface.iface-extract-ambiguous-fn]
// [spec:foma:def:foma.iface-extract-ambiguous-fn]
// [spec:foma:sem:foma.iface-extract-ambiguous-fn]
pub fn iface_extract_ambiguous() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_extract_ambiguous(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-extract-unambiguous-fn]
// [spec:foma:sem:iface.iface-extract-unambiguous-fn]
// [spec:foma:def:foma.iface-extract-unambiguous-fn]
// [spec:foma:sem:foma.iface-extract-unambiguous-fn]
pub fn iface_extract_unambiguous() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_extract_unambiguous(stack_pop().unwrap()));
    }
}

// C atoi: skip leading whitespace, optional +/-, then decimal digits until a
// non-digit; empty/no-digit → 0. Overflow is UB in C; wrapping here. Unannotated
// plumbing used by iface_extract_number.
fn atoi(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let mut sign: i32 = 1;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        if bytes[i] == b'-' {
            sign = -1;
        }
        i += 1;
    }
    let mut n: i32 = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        n = n.wrapping_mul(10).wrapping_add((bytes[i] - b'0') as i32);
        i += 1;
    }
    sign.wrapping_mul(n)
}

// [spec:foma:def:iface.iface-extract-number-fn]
// [spec:foma:sem:iface.iface-extract-number-fn]
// [spec:foma:def:foma.iface-extract-number-fn]
// [spec:foma:sem:foma.iface-extract-number-fn]
pub fn iface_extract_number(s: &str) -> i32 {
    // Scan to the first ASCII digit (compared as unsigned char), then atoi.
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() && (bytes[i] < b'0' || bytes[i] > b'9') {
        i += 1;
    }
    atoi(&s[i..])
}

// [spec:foma:def:iface.iface-eliminate-flag-fn]
// [spec:foma:sem:iface.iface-eliminate-flag-fn]
// [spec:foma:def:foma.iface-eliminate-flag-fn]
// [spec:foma:sem:foma.iface-eliminate-flag-fn]
pub fn iface_eliminate_flag(name: &str) {
    if iface_stack_check(1) != 0 {
        stack_add(flag_eliminate(stack_pop().unwrap(), Some(name)));
    }
}

// [spec:foma:def:iface.iface-factorize-fn]
// [spec:foma:sem:iface.iface-factorize-fn]
// [spec:foma:def:foma.iface-factorize-fn]
// [spec:foma:sem:foma.iface-factorize-fn]
pub fn iface_factorize() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_bimachine(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-sequentialize-fn]
// [spec:foma:sem:iface.iface-sequentialize-fn]
// [spec:foma:def:foma.iface-sequentialize-fn]
// [spec:foma:sem:foma.iface-sequentialize-fn]
pub fn iface_sequentialize() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_sequentialize(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-ignore-fn]
// [spec:foma:sem:iface.iface-ignore-fn]
// [spec:foma:def:foma.iface-ignore-fn]
// [spec:foma:sem:foma.iface-ignore-fn]
pub fn iface_ignore() {
    if iface_stack_check(2) != 0 {
        let one = stack_pop().unwrap();
        let two = stack_pop().unwrap();
        stack_add(fsm_topsort(fsm_minimize(fsm_ignore(one, two, OP_IGNORE_ALL))));
    }
}

// [spec:foma:def:iface.iface-intersect-fn]
// [spec:foma:sem:iface.iface-intersect-fn]
// [spec:foma:def:foma.iface-intersect-fn]
// [spec:foma:sem:foma.iface-intersect-fn]
pub fn iface_intersect() {
    if iface_stack_check(2) != 0 {
        while stack_size() > 1 {
            // C: fsm_intersect(stack_pop(), stack_pop()) — the two pops are one
            // expression (C order unspecified); Rust evaluates arguments
            // left-to-right. Intersection is commutative, so the language matches.
            let a = stack_pop().unwrap();
            let b = stack_pop().unwrap();
            stack_add(fsm_topsort(fsm_minimize(fsm_intersect(a, b))));
        }
    }
}

// [spec:foma:def:iface.iface-invert-fn]
// [spec:foma:sem:iface.iface-invert-fn]
// [spec:foma:def:foma.iface-invert-fn]
// [spec:foma:sem:foma.iface-invert-fn]
pub fn iface_invert() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_invert(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-label-net-fn]
// [spec:foma:sem:iface.iface-label-net-fn]
// [spec:foma:def:foma.iface-label-net-fn]
// [spec:foma:sem:foma.iface-label-net-fn]
pub fn iface_label_net() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_sigma_pairs_net(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-letter-machine-fn]
// [spec:foma:sem:iface.iface-letter-machine-fn]
// [spec:foma:def:foma.iface-letter-machine-fn]
// [spec:foma:sem:foma.iface-letter-machine-fn]
pub fn iface_letter_machine() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_topsort(fsm_minimize(fsm_letter_machine(stack_pop().unwrap()))));
    }
}

// [spec:foma:def:iface.iface-load-defined-fn]
// [spec:foma:sem:iface.iface-load-defined-fn]
// [spec:foma:def:foma.iface-load-defined-fn]
// [spec:foma:sem:foma.iface-load-defined-fn]
pub fn iface_load_defined(filename: &str) {
    G_DEFINES.with(|g| {
        let mut g = g.borrow_mut();
        // C: load_defined(g_defines, filename); g_defines is the init'd dummy head.
        if let Some(d) = g.as_deref_mut() {
            load_defined(d, filename);
        }
    });
}

// [spec:foma:def:iface.iface-load-stack-fn]
// [spec:foma:sem:iface.iface-load-stack-fn]
// [spec:foma:def:foma.iface-load-stack-fn]
// [spec:foma:sem:foma.iface-load-stack-fn]
pub fn iface_load_stack(filename: &str) {
    let mut fsrh = fsm_read_binary_file_multiple_init(filename);
    if fsrh.is_none() {
        eprint!("{}: ", filename);
        perror("File error");
        return;
    }
    while let Some(net) = fsm_read_binary_file_multiple(&mut fsrh) {
        stack_add(net);
    }
}

// [spec:foma:def:iface.iface-lower-side-fn]
// [spec:foma:sem:iface.iface-lower-side-fn]
// [spec:foma:def:foma.iface-lower-side-fn]
// [spec:foma:sem:foma.iface-lower-side-fn]
pub fn iface_lower_side() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_topsort(fsm_minimize(fsm_lower(stack_pop().unwrap()))));
    }
}

// [spec:foma:def:iface.iface-minimize-fn]
// [spec:foma:sem:iface.iface-minimize-fn]
// [spec:foma:def:foma.iface-minimize-fn]
// [spec:foma:sem:foma.iface-minimize-fn]
pub fn iface_minimize() {
    if iface_stack_check(1) != 0 {
        let store_minimal_var = G_MINIMAL.with(|v| v.get());
        G_MINIMAL.with(|v| v.set(1));
        stack_add(fsm_topsort(fsm_minimize(stack_pop().unwrap())));
        G_MINIMAL.with(|v| v.set(store_minimal_var));
    }
}

// [spec:foma:def:iface.iface-one-plus-fn]
// [spec:foma:sem:iface.iface-one-plus-fn]
// [spec:foma:def:foma.iface-one-plus-fn]
// [spec:foma:sem:foma.iface-one-plus-fn]
pub fn iface_one_plus() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_topsort(fsm_minimize(fsm_kleene_plus(stack_pop().unwrap()))));
    }
}

// [spec:foma:def:iface.iface-pop-fn]
// [spec:foma:sem:iface.iface-pop-fn]
// [spec:foma:def:foma.iface-pop-fn]
// [spec:foma:sem:foma.iface-pop-fn]
pub fn iface_pop() {
    if stack_size() < 1 {
        print!("Stack is empty.\n");
    } else {
        let net = stack_pop().unwrap();
        fsm_destroy(net);
    }
}

// [spec:foma:def:iface.iface-lower-words-fn]
// [spec:foma:sem:iface.iface-lower-words-fn]
// [spec:foma:def:foma.iface-lower-words-fn]
// [spec:foma:sem:foma.iface-lower-words-fn]
pub fn iface_lower_words(limit: i32) {
    if iface_stack_check(1) == 0 {
        return;
    }
    let limit = if limit == -1 {
        G_LIST_LIMIT.with(|v| v.get())
    } else {
        limit
    };
    if iface_stack_check(1) != 0 {
        let ah = stack_get_ah().unwrap();
        stack_entry_ah(ah, |h| iface_apply_set_params(h));
        let mut i = limit;
        while i > 0 {
            let result = stack_entry_ah(ah, |h| apply_lower_words(h));
            match result {
                None => break,
                Some(r) => print!("{}\n", r),
            }
            i -= 1;
        }
        stack_entry_ah(ah, |h| apply_reset_enumerator(h));
    }
}

// [spec:foma:def:iface.iface-name-net-fn]
// [spec:foma:sem:iface.iface-name-net-fn]
// [spec:foma:def:foma.iface-name-net-fn]
// [spec:foma:sem:foma.iface-name-net-fn]
pub fn iface_name_net(name: &str) {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |f| {
            // strncpy(top->fsm->name, name, 40): copy <= 40 bytes; when
            // strlen(name) >= 40 the field is left WITHOUT a NUL terminator, i.e.
            // truncated to 40 bytes (latent bug — reproduced literally).
            let bytes = name.as_bytes();
            let n = if bytes.len() < 40 { bytes.len() } else { 40 };
            f.name = String::from_utf8_lossy(&bytes[..n]).into_owned();
        });
        iface_print_name();
    }
}

// [spec:foma:def:iface.iface-negate-fn]
// [spec:foma:sem:iface.iface-negate-fn]
// [spec:foma:def:foma.iface-negate-fn]
// [spec:foma:sem:foma.iface-negate-fn]
pub fn iface_negate() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_topsort(fsm_minimize(fsm_complement(stack_pop().unwrap()))));
    }
}

// [spec:foma:def:iface.iface-print-dot-fn]
// [spec:foma:sem:iface.iface-print-dot-fn]
// [spec:foma:def:foma.iface-print-dot-fn]
// [spec:foma:sem:foma.iface-print-dot-fn]
pub fn iface_print_dot(filename: Option<&str>) {
    if iface_stack_check(1) != 0 {
        if let Some(f) = filename {
            print!("Writing dot file to {}.\n", f);
        }
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |net| print_dot(net, filename));
    }
}

// [spec:foma:def:iface.iface-print-net-fn]
// [spec:foma:sem:iface.iface-print-net-fn]
// [spec:foma:def:foma.iface-print-net-fn]
// [spec:foma:sem:foma.iface-print-net-fn]
pub fn iface_print_net(netname: Option<&str>, filename: Option<&str>) {
    match netname {
        Some(netname) => {
            // net = find_defined(g_defines, netname); NULL g_defines ↔ not found.
            let found = G_DEFINES.with(|g| {
                let mut g = g.borrow_mut();
                match g.as_deref_mut() {
                    Some(defs) => match find_defined(defs, netname) {
                        Some(net) => {
                            print_net(net, filename);
                            true
                        }
                        None => false,
                    },
                    None => false,
                }
            });
            if !found {
                if G_VERBOSE.with(|v| v.get()) != 0 {
                    eprint!("No defined network {}.\n", netname);
                    // fflush(stderr) — stderr is unbuffered
                }
            }
        }
        None => {
            if iface_stack_check(1) != 0 {
                let top = stack_find_top().unwrap();
                stack_entry_fsm(top, |net| print_net(net, filename));
            }
        }
    }
}

// [spec:foma:def:iface.iface-print-cmatrix-att-fn]
// [spec:foma:sem:iface.iface-print-cmatrix-att-fn]
// [spec:foma:def:foma.iface-print-cmatrix-att-fn]
// [spec:foma:sem:foma.iface-print-cmatrix-att-fn]
pub fn iface_print_cmatrix_att(filename: Option<&str>) {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        // C: medlookup == NULL || medlookup->confusion_matrix == NULL. Empty Vec ↔ NULL.
        let has_cm = stack_entry_fsm(top, |f| {
            !(f.medlookup.is_none() || f.medlookup.as_ref().unwrap().confusion_matrix.is_empty())
        });
        if !has_cm {
            print!("No confusion matrix defined.\n");
        } else {
            match filename {
                None => {
                    stack_entry_fsm(top, |f| cmatrix_print_att(f, &mut std::io::stdout()));
                }
                Some(name) => {
                    // C: outfile = fopen(name,"w"); message; result NOT NULL-checked.
                    let res = File::create(name);
                    print!("Writing confusion matrix to file '{}'.\n", name);
                    // DEVIATION from C (unchecked fopen → NULL deref crash on
                    // failure; expect() panics at the nearest safe point instead).
                    let mut file = res.expect("Error opening output file");
                    stack_entry_fsm(top, |f| cmatrix_print_att(f, &mut file));
                    // C never fclose's the file (latent leak); Rust closes on drop.
                }
            }
        }
    }
}

// [spec:foma:def:iface.iface-print-cmatrix-fn]
// [spec:foma:sem:iface.iface-print-cmatrix-fn]
// [spec:foma:def:foma.iface-print-cmatrix-fn]
// [spec:foma:sem:foma.iface-print-cmatrix-fn]
pub fn iface_print_cmatrix() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let has_cm = stack_entry_fsm(top, |f| {
            !(f.medlookup.is_none() || f.medlookup.as_ref().unwrap().confusion_matrix.is_empty())
        });
        if !has_cm {
            print!("No confusion matrix defined.\n");
        } else {
            stack_entry_fsm(top, |f| cmatrix_print(f));
        }
    }
}

// [spec:foma:def:iface.iface-print-defined-fn]
// [spec:foma:sem:iface.iface-print-defined-fn]
// [spec:foma:def:foma.iface-print-defined-fn]
// [spec:foma:sem:foma.iface-print-defined-fn]
pub fn iface_print_defined() {
    G_DEFINES.with(|g| {
        let g = g.borrow();
        if g.is_none() {
            print!("No defined symbols.\n");
        }
        // Falls through to the loop even when NULL (a no-op over an empty list).
        let mut d = g.as_deref();
        while let Some(node) = d {
            if let Some(name) = node.name.as_deref() {
                print!("{}\t", name);
                print_stats(node.net.as_deref().unwrap());
            }
            d = node.next.as_deref();
        }
    });
    G_DEFINES_F.with(|g| {
        let g = g.borrow();
        let mut d = g.as_deref();
        while let Some(node) = d {
            if let Some(name) = node.name.as_deref() {
                // "%s@%i)\t" — literal unmatched ')' then TAB (format-string quirk)
                print!("{}@{})\t", name, node.numargs);
                print!("{}\n", node.regex.as_deref().unwrap_or(""));
            }
            d = node.next.as_deref();
        }
    });
}

// [spec:foma:def:iface.iface-print-name-fn]
// [spec:foma:sem:iface.iface-print-name-fn]
// [spec:foma:def:foma.iface-print-name-fn]
// [spec:foma:sem:foma.iface-print-name-fn]
pub fn iface_print_name() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let name = stack_entry_fsm(top, |f| f.name.clone());
        print!("{}\n", name);
    }
}

// [spec:foma:def:iface.iface-quit-fn]
// [spec:foma:sem:iface.iface-quit-fn]
// [spec:foma:def:foma.iface-quit-fn]
// [spec:foma:sem:foma.iface-quit-fn]
pub fn iface_quit() {
    G_DEFINES.with(|g| {
        let mut g = g.borrow_mut();
        // remove_defined(g_defines, NULL) — NULL name destroys every defined net.
        if let Some(d) = g.as_deref_mut() {
            remove_defined(d, None);
        }
    });
    while stack_isempty() == 0 {
        let net = stack_pop().unwrap();
        fsm_destroy(net);
    }
    std::process::exit(0);
}

// ====================================================================
// SLICE 2 of 2: the SECOND HALF of iface.c (everything after iface_quit,
// in C-file order), plus the four callees slice 1 had left as stubs
// (iface_stack_check, print_stats, print_dot, print_net). The static
// helpers sigptr, print_net, print_dot, print_sigma, print_mem_size carry
// only per-file iface.* ids; print_stats and view_net additionally carry
// the foma.h prototype ids.
// ====================================================================

// [spec:foma:def:iface.iface-random-lower-fn]
// [spec:foma:sem:iface.iface-random-lower-fn]
// [spec:foma:def:foma.iface-random-lower-fn]
// [spec:foma:sem:foma.iface-random-lower-fn]
pub fn iface_random_lower(limit: i32) {
    iface_apply_random(apply_random_lower, limit);
}

// [spec:foma:def:iface.iface-random-upper-fn]
// [spec:foma:sem:iface.iface-random-upper-fn]
// [spec:foma:def:foma.iface-random-upper-fn]
// [spec:foma:sem:foma.iface-random-upper-fn]
pub fn iface_random_upper(limit: i32) {
    iface_apply_random(apply_random_upper, limit);
}

// [spec:foma:def:iface.iface-random-words-fn]
// [spec:foma:sem:iface.iface-random-words-fn]
// [spec:foma:def:foma.iface-random-words-fn]
// [spec:foma:sem:foma.iface-random-words-fn]
pub fn iface_random_words(limit: i32) {
    iface_apply_random(apply_random_words, limit);
}

// [spec:foma:def:iface.iface-apply-random-fn]
// [spec:foma:sem:iface.iface-apply-random-fn]
// [spec:foma:def:foma.iface-apply-random-fn]
// [spec:foma:sem:foma.iface-apply-random-fn]
// C: `void iface_apply_random(char *(*applyer)(struct apply_handle *h), int limit)` —
// the applyer function pointer becomes a Rust fn pointer of the same shape.
pub fn iface_apply_random(applyer: fn(&mut ApplyHandle) -> Option<String>, limit: i32) {
    let limit = if limit == -1 {
        G_LIST_RANDOM_LIMIT.with(|v| v.get())
    } else {
        limit
    };
    if iface_stack_check(1) != 0 {
        // calloc(limit, sizeof(struct apply_results {char *string; int count;}))
        let mut results: Vec<(Option<String>, i32)> = vec![(None, 0); limit as usize];
        let ah = stack_get_ah().unwrap();
        stack_entry_ah(ah, |h| iface_apply_set_params(h));
        let mut i = limit;
        while i > 0 {
            let result = stack_entry_ah(ah, |h| applyer(h));
            if let Some(result) = result {
                for slot in results.iter_mut() {
                    if slot.0.is_none() {
                        // strdup(result)
                        slot.0 = Some(result.clone());
                        slot.1 = 1;
                        break;
                    } else if slot.0.as_deref() == Some(result.as_str()) {
                        slot.1 += 1;
                        break;
                    }
                }
            }
            i -= 1;
        }
        for slot in results.iter() {
            if let Some(s) = &slot.0 {
                print!("[{}] {}\n", slot.1, s);
                // free(tempresults->string) — String dropped at scope end.
            }
        }
        // free(results) — Vec dropped at scope end.
        stack_entry_ah(ah, |h| apply_reset_enumerator(h));
    }
}

// [spec:foma:def:iface.iface-print-sigma-fn]
// [spec:foma:sem:iface.iface-print-sigma-fn]
// [spec:foma:def:foma.iface-print-sigma-fn]
// [spec:foma:sem:foma.iface-print-sigma-fn]
pub fn iface_print_sigma() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |f| print_sigma(f.sigma.as_deref(), &mut std::io::stdout()));
    }
}

// [spec:foma:def:iface.iface-print-stats-fn]
// [spec:foma:sem:iface.iface-print-stats-fn]
// [spec:foma:def:foma.iface-print-stats-fn]
// [spec:foma:sem:foma.iface-print-stats-fn]
pub fn iface_print_stats() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |f| print_stats(f));
    }
}

// [spec:foma:def:iface.iface-print-shortest-string-fn]
// [spec:foma:sem:iface.iface-print-shortest-string-fn]
// [spec:foma:def:foma.iface-print-shortest-string-fn]
// [spec:foma:sem:foma.iface-print-shortest-string-fn]
pub fn iface_print_shortest_string() {
    /* L -  ?+  [[L .o. [?:"@TMP@"]*].l .o. "@TMP@":?*].l; */
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let mut one = stack_entry_fsm(top, |f| fsm_copy(f));
        if stack_entry_fsm(top, |f| f.arity) == 1 {
            let result = fsm_minimize(fsm_minus(
                fsm_copy(&mut one),
                fsm_concat(
                    fsm_kleene_plus(fsm_identity()),
                    fsm_lower(fsm_compose(
                        fsm_lower(fsm_compose(
                            fsm_copy(&mut one),
                            fsm_kleene_star(fsm_cross_product(fsm_identity(), fsm_symbol("@TMP@"))),
                        )),
                        fsm_kleene_star(fsm_cross_product(fsm_symbol("@TMP@"), fsm_identity())),
                    )),
                ),
            ));
            let mut ah = apply_init(&result);
            let word = apply_words(&mut ah);
            if let Some(w) = &word {
                print!("{}\n", w);
            }
            apply_clear(ah);
            fsm_destroy(result);
            // C leaks the initial fsm_copy `one` here; dropped (freed) at scope end.
        } else {
            let mut onel = fsm_lower(fsm_copy(&mut one));
            let mut oneu = fsm_upper(one);
            let result_u = fsm_minimize(fsm_minus(
                fsm_copy(&mut oneu),
                fsm_concat(
                    fsm_kleene_plus(fsm_identity()),
                    fsm_lower(fsm_compose(
                        fsm_lower(fsm_compose(
                            fsm_copy(&mut oneu),
                            fsm_kleene_star(fsm_cross_product(fsm_identity(), fsm_symbol("@TMP@"))),
                        )),
                        fsm_kleene_star(fsm_cross_product(fsm_symbol("@TMP@"), fsm_identity())),
                    )),
                ),
            ));
            let result_l = fsm_minimize(fsm_minus(
                fsm_copy(&mut onel),
                fsm_concat(
                    fsm_kleene_plus(fsm_identity()),
                    fsm_lower(fsm_compose(
                        fsm_lower(fsm_compose(
                            fsm_copy(&mut onel),
                            fsm_kleene_star(fsm_cross_product(fsm_identity(), fsm_symbol("@TMP@"))),
                        )),
                        fsm_kleene_star(fsm_cross_product(fsm_symbol("@TMP@"), fsm_identity())),
                    )),
                ),
            ));
            fsm_destroy(oneu);
            fsm_destroy(onel);
            let mut ah = apply_init(&result_u);
            let word = apply_words(&mut ah);
            // C: if (word == NULL) word = ""; printf("Upper: %s\n", word);
            print!("Upper: {}\n", word.as_deref().unwrap_or(""));
            apply_clear(ah);
            fsm_destroy(result_u);
            let mut ah = apply_init(&result_l);
            let word = apply_words(&mut ah);
            print!("Lower: {}\n", word.as_deref().unwrap_or(""));
            apply_clear(ah);
            fsm_destroy(result_l);
        }
    }
}

// [spec:foma:def:iface.iface-print-shortest-string-size-fn]
// [spec:foma:sem:iface.iface-print-shortest-string-size-fn]
// [spec:foma:def:foma.iface-print-shortest-string-size-fn]
// [spec:foma:sem:foma.iface-print-shortest-string-size-fn]
pub fn iface_print_shortest_string_size() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let mut one = stack_entry_fsm(top, |f| fsm_copy(f));
        /* [L .o. [?:a]*].l; */
        if stack_entry_fsm(top, |f| f.arity) == 1 {
            let result = fsm_minimize(fsm_lower(fsm_compose(
                one,
                fsm_kleene_star(fsm_cross_product(fsm_identity(), fsm_symbol("a"))),
            )));
            print!("Shortest acyclic path length: {}\n", result.statecount - 1);
            // Result net never fsm_destroy'd in C (leak); dropped at scope end.
        } else {
            let onel = fsm_lower(fsm_copy(&mut one));
            let oneu = fsm_upper(one);
            let result_u = fsm_minimize(fsm_lower(fsm_compose(
                oneu,
                fsm_kleene_star(fsm_cross_product(fsm_identity(), fsm_symbol("a"))),
            )));
            let result_l = fsm_minimize(fsm_lower(fsm_compose(
                onel,
                fsm_kleene_star(fsm_cross_product(fsm_identity(), fsm_symbol("a"))),
            )));
            print!("Shortest acyclic upper path length: {}\n", result_u.statecount - 1);
            print!("Shortest acyclic lower path length: {}\n", result_l.statecount - 1);
        }
    }
}

// [spec:foma:def:iface.iface-read-att-fn]
// [spec:foma:sem:iface.iface-read-att-fn]
// [spec:foma:def:foma.iface-read-att-fn]
// [spec:foma:sem:foma.iface-read-att-fn]
pub fn iface_read_att(filename: &str) -> i32 {
    print!("Reading AT&T file: {}\n", filename);
    match read_att(filename) {
        None => {
            eprint!("{}: ", filename);
            perror("Error opening file");
            1
        }
        Some(tempnet) => {
            stack_add(tempnet);
            0
        }
    }
}

// [spec:foma:def:iface.iface-read-prolog-fn]
// [spec:foma:sem:iface.iface-read-prolog-fn]
// [spec:foma:def:foma.iface-read-prolog-fn]
// [spec:foma:sem:foma.iface-read-prolog-fn]
pub fn iface_read_prolog(filename: &str) -> i32 {
    print!("Reading prolog: {}\n", filename);
    match fsm_read_prolog(filename) {
        None => {
            eprint!("{}: ", filename);
            perror("Error opening file");
            1
        }
        Some(tempnet) => {
            stack_add(tempnet);
            0
        }
    }
}

// [spec:foma:def:iface.iface-read-spaced-text-fn]
// [spec:foma:sem:iface.iface-read-spaced-text-fn]
// [spec:foma:def:foma.iface-read-spaced-text-fn]
// [spec:foma:sem:foma.iface-read-spaced-text-fn]
pub fn iface_read_spaced_text(filename: &str) -> i32 {
    match fsm_read_spaced_text_file(filename) {
        None => {
            eprint!("{}: ", filename);
            perror("File error");
            1
        }
        Some(net) => {
            stack_add(fsm_topsort(fsm_minimize(net)));
            0
        }
    }
}

// [spec:foma:def:iface.iface-read-text-fn]
// [spec:foma:sem:iface.iface-read-text-fn]
// [spec:foma:def:foma.iface-read-text-fn]
// [spec:foma:sem:foma.iface-read-text-fn]
pub fn iface_read_text(filename: &str) -> i32 {
    match fsm_read_text_file(filename) {
        None => {
            eprint!("{}: ", filename);
            perror("File error");
            1
        }
        Some(net) => {
            stack_add(fsm_topsort(fsm_minimize(net)));
            0
        }
    }
}

// [spec:foma:def:iface.iface-stack-check-fn]
// [spec:foma:sem:iface.iface-stack-check-fn]
// [spec:foma:def:foma.iface-stack-check-fn]
// [spec:foma:sem:foma.iface-stack-check-fn]
pub fn iface_stack_check(size: i32) -> i32 {
    if stack_size() < size {
        print!(
            "Not enough networks on stack. Operation requires at least {}.\n",
            size
        );
        return 0;
    }
    1
}

// [spec:foma:def:iface.iface-substitute-symbol-fn]
// [spec:foma:sem:iface.iface-substitute-symbol-fn]
// [spec:foma:def:foma.iface-substitute-symbol-fn]
// [spec:foma:sem:foma.iface-substitute-symbol-fn]
pub fn iface_substitute_symbol(original: &str, substitute: &str) {
    if iface_stack_check(1) != 0 {
        // DEVIATION from C: C dequotes the caller's `char *` buffers in place; the
        // args are &str here, so local byte copies are dequoted instead (observably
        // identical — the printed strings and the fsm op both use the dequoted text).
        let mut original = original.as_bytes().to_vec();
        let mut substitute = substitute.as_bytes().to_vec();
        dequote_string(&mut original);
        dequote_string(&mut substitute);
        let original = String::from_utf8_lossy(&original).into_owned();
        let substitute = String::from_utf8_lossy(&substitute).into_owned();
        stack_add(fsm_topsort(fsm_minimize(fsm_substitute_symbol(
            stack_pop().unwrap(),
            &original,
            &substitute,
        ))));
        print!("Substituted '{}' for '{}'.\n", substitute, original);
    }
}

// [spec:foma:def:iface.iface-substitute-defined-fn]
// [spec:foma:sem:iface.iface-substitute-defined-fn]
// [spec:foma:def:foma.iface-substitute-defined-fn]
// [spec:foma:sem:foma.iface-substitute-defined-fn]
pub fn iface_substitute_defined(original: &str, substitute: &str) {
    if iface_stack_check(1) != 0 {
        // DEVIATION from C: see iface_substitute_symbol — dequote on local copies.
        let mut original = original.as_bytes().to_vec();
        let mut substitute = substitute.as_bytes().to_vec();
        dequote_string(&mut original);
        dequote_string(&mut substitute);
        let original = String::from_utf8_lossy(&original).into_owned();
        let substitute = String::from_utf8_lossy(&substitute).into_owned();
        G_DEFINES.with(|g| {
            let mut g = g.borrow_mut();
            // find_defined(g_defines, substitute): NULL g_defines ↔ not found.
            let subnet = match g.as_deref_mut() {
                Some(defs) => find_defined(defs, &substitute),
                None => None,
            };
            match subnet {
                None => {
                    print!("No defined network '{}'.\n", substitute);
                }
                Some(subnet) => {
                    let top = stack_find_top().unwrap();
                    if stack_entry_fsm(top, |f| fsm_symbol_occurs(f, &original, M_UPPER + M_LOWER))
                        == 0
                    {
                        print!("Symbol '{}' does not occur.\n", original);
                    } else {
                        let newnet =
                            stack_entry_fsm(top, |f| fsm_substitute_label(f, &original, subnet));
                        // C: stack_pop() — the popped net is NOT fsm_destroy'd (latent
                        // leak); here the returned Box is dropped (freed) instead.
                        let _ = stack_pop();
                        print!("Substituted network '{}' for '{}'.\n", substitute, original);
                        stack_add(fsm_topsort(fsm_minimize(newnet)));
                    }
                }
            }
        });
    }
}

// [spec:foma:def:iface.iface-upper-words-fn]
// [spec:foma:sem:iface.iface-upper-words-fn]
// [spec:foma:def:foma.iface-upper-words-fn]
// [spec:foma:sem:foma.iface-upper-words-fn]
pub fn iface_upper_words(limit: i32) {
    let limit = if limit == -1 {
        G_LIST_LIMIT.with(|v| v.get())
    } else {
        limit
    };
    if iface_stack_check(1) != 0 {
        let ah = stack_get_ah().unwrap();
        stack_entry_ah(ah, |h| iface_apply_set_params(h));
        let mut i = limit;
        while i > 0 {
            let result = stack_entry_ah(ah, |h| apply_upper_words(h));
            match result {
                None => break,
                Some(r) => print!("{}\n", r),
            }
            i -= 1;
        }
        stack_entry_ah(ah, |h| apply_reset_enumerator(h));
    }
}

// [spec:foma:def:iface.iface-prune-fn]
// [spec:foma:sem:iface.iface-prune-fn]
// [spec:foma:def:foma.iface-prune-fn]
// [spec:foma:sem:foma.iface-prune-fn]
pub fn iface_prune() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_topsort(fsm_coaccessible(stack_pop().unwrap())));
    }
}

// [spec:foma:def:iface.iface-reverse-fn]
// [spec:foma:sem:iface.iface-reverse-fn]
// [spec:foma:def:foma.iface-reverse-fn]
// [spec:foma:sem:foma.iface-reverse-fn]
pub fn iface_reverse() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_topsort(fsm_determinize(fsm_reverse(stack_pop().unwrap()))));
    }
}

// [spec:foma:def:iface.iface-rotate-fn]
// [spec:foma:sem:iface.iface-rotate-fn]
// [spec:foma:def:foma.iface-rotate-fn]
// [spec:foma:sem:foma.iface-rotate-fn]
pub fn iface_rotate() {
    if iface_stack_check(1) != 0 {
        stack_rotate();
    }
}

// [spec:foma:def:iface.iface-save-defined-fn]
// [spec:foma:sem:iface.iface-save-defined-fn]
// [spec:foma:def:foma.iface-save-defined-fn]
// [spec:foma:sem:foma.iface-save-defined-fn]
pub fn iface_save_defined(filename: &str) {
    G_DEFINES.with(|g| {
        let mut g = g.borrow_mut();
        // save_defined(g_defines, filename): the C helper prints "No defined
        // networks.\n" to stderr when g_defines is NULL; a &mut can't be NULL, so
        // (per io.rs save_defined) that check lives at this call site.
        match g.as_deref_mut() {
            None => {
                eprint!("No defined networks.\n");
            }
            Some(d) => {
                save_defined(d, filename);
            }
        }
    });
}

// [spec:foma:def:iface.iface-save-stack-fn]
// [spec:foma:sem:iface.iface-save-stack-fn]
// [spec:foma:def:foma.iface-save-stack-fn]
// [spec:foma:sem:foma.iface-save-stack-fn]
pub fn iface_save_stack(filename: &str) {
    if iface_stack_check(1) != 0 {
        // gzopen(filename, "wb") — File::create + GzEncoder.
        let file = match File::create(filename) {
            Ok(f) => f,
            Err(_) => {
                print!("Error opening file {} for writing.\n", filename);
                return;
            }
        };
        print!("Writing to file {}.\n", filename);
        let mut outfile = GzEncoder::new(file, Compression::default());
        // for (stack_ptr = stack_find_bottom(); stack_ptr->next != NULL; stack_ptr = stack_ptr->next)
        let mut stack_ptr = stack_find_bottom().unwrap();
        while stack_entry_next(stack_ptr).is_some() {
            stack_entry_fsm(stack_ptr, |f| foma_net_print(f, &mut outfile));
            stack_ptr = stack_entry_next(stack_ptr).unwrap();
        }
        // gzclose(outfile)
        let _ = outfile.finish();
    }
}

// C strncmp(a, b, 8): compares at most 8 bytes as unsigned char, stopping early
// when a shared NUL is reached; == 0 iff the first 8 bytes (or up to a common
// NUL) match. Unannotated plumbing for the variable-name lookup (the 8-char
// prefix match is the documented latent bug in iface_{set,show}_variable).
fn strncmp8(a: &str, b: &str) -> i32 {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    for i in 0..8 {
        let ca = ab.get(i).copied().unwrap_or(0);
        let cb = bb.get(i).copied().unwrap_or(0);
        if ca != cb {
            return ca as i32 - cb as i32;
        }
        if ca == 0 {
            return 0;
        }
    }
    0
}

// C strtol(value, &endptr, 10) semantics used by iface_set_variable's FVAR_INT
// branch. Returns (result truncated to `long`=i64, endptr==value i.e. no digits
// consumed, errno==ERANGE i.e. out of long range). Unannotated plumbing.
fn c_strtol_base10(value: &str) -> (i64, bool, bool) {
    let bytes = value.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let mut neg = false;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        neg = bytes[i] == b'-';
        i += 1;
    }
    let mut any = false;
    let mut acc: i64 = 0;
    let mut range = false;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        any = true;
        let d = (bytes[i] - b'0') as i64;
        if !range {
            match acc.checked_mul(10).and_then(|v| v.checked_add(d)) {
                Some(v) => acc = v,
                None => range = true,
            }
        }
        i += 1;
    }
    let result = if range {
        if neg { i64::MIN } else { i64::MAX }
    } else if neg {
        -acc
    } else {
        acc
    };
    (result, !any, range)
}

// [spec:foma:def:iface.iface-show-variables-fn]
// [spec:foma:sem:iface.iface-show-variables-fn]
// [spec:foma:def:foma.iface-show-variables-fn]
// [spec:foma:sem:foma.iface-show-variables-fn]
pub fn iface_show_variables() {
    for gv in global_vars() {
        // "%-17.17s" — left-justified, padded/truncated to exactly 17 chars.
        if gv.r#type == FVAR_BOOL {
            let v = match &gv.ptr {
                GvPtr::Int(c) => c.with(|x| x.get()),
                GvPtr::Str(_) => 0,
            };
            print!("{:<17.17}: {}\n", gv.name, if v == 1 { "ON" } else { "OFF" });
        }
        if gv.r#type == FVAR_INT {
            let v = match &gv.ptr {
                GvPtr::Int(c) => c.with(|x| x.get()),
                GvPtr::Str(_) => 0,
            };
            print!("{:<17.17}: {}\n", gv.name, v);
        }
        if gv.r#type == FVAR_STRING {
            let v = match &gv.ptr {
                GvPtr::Str(rc) => rc.with(|s| s.borrow().clone()),
                GvPtr::Int(_) => String::new(),
            };
            print!("{:<17.17}: {}\n", gv.name, v);
        }
    }
}

// [spec:foma:def:iface.iface-show-variable-fn]
// [spec:foma:sem:iface.iface-show-variable-fn]
// [spec:foma:def:foma.iface-show-variable-fn]
// [spec:foma:sem:foma.iface-show-variable-fn]
pub fn iface_show_variable(name: &str) {
    for gv in global_vars() {
        if strncmp8(name, gv.name) == 0 {
            // Latent bug reproduced: prints ON/OFF from `*(int*)ptr == 1` for EVERY
            // variable type. For FVAR_INT only value 1 shows ON; for FVAR_STRING C
            // reinterprets the char* pointer bytes as int (garbage).
            let val = match &gv.ptr {
                GvPtr::Int(c) => c.with(|x| x.get()),
                // DEVIATION from C: safe Rust can't reinterpret the string pointer
                // bytes as an int; modelled as a non-1 value (practically "OFF").
                GvPtr::Str(_) => 0,
            };
            print!("{} = {}\n", gv.name, if val == 1 { "ON" } else { "OFF" });
            return;
        }
    }
    print!("*There is no global variable '{}'.\n", name);
}

// [spec:foma:def:iface.iface-set-variable-fn]
// [spec:foma:sem:iface.iface-set-variable-fn]
// [spec:foma:def:foma.iface-set-variable-fn]
// [spec:foma:sem:foma.iface-set-variable-fn]
pub fn iface_set_variable(name: &str, value: &str) {
    for gv in global_vars() {
        if strncmp8(name, gv.name) == 0 {
            if gv.r#type == FVAR_BOOL {
                let j: i32;
                if value == "ON" || value == "1" {
                    j = 1;
                } else if value == "OFF" || value == "0" {
                    j = 0;
                } else {
                    print!("Invalid value '{}' for variable '{}'\n", value, gv.name);
                    return;
                }
                if let GvPtr::Int(c) = &gv.ptr {
                    c.with(|x| x.set(j));
                }
                let cur = match &gv.ptr {
                    GvPtr::Int(c) => c.with(|x| x.get()),
                    GvPtr::Str(_) => 0,
                };
                print!(
                    "variable {} = {}\n",
                    gv.name,
                    if cur == 1 { "ON" } else { "OFF" }
                );
                return;
            }
            if gv.r#type == FVAR_STRING {
                // *ptr = strdup(value): C leaks the old string; here it is replaced.
                if let GvPtr::Str(rc) = &gv.ptr {
                    rc.with(|s| *s.borrow_mut() = value.to_string());
                }
                print!("variable {} = {}\n", gv.name, value);
                return;
            }
            if gv.r#type == FVAR_INT {
                let (result, no_digits, range) = c_strtol_base10(value);
                // j = (int)strtol(...) — truncation to int.
                let j = result as i32;
                if range || no_digits || j < 0 {
                    print!("invalid value {} for variable {}\n", value, gv.name);
                    return;
                } else {
                    print!("variable {} = {}\n", gv.name, j);
                    if let GvPtr::Int(c) = &gv.ptr {
                        c.with(|x| x.set(j));
                    }
                    return;
                }
            }
        }
    }
    print!("*There is no global variable '{}'.\n", name);
}

// [spec:foma:def:iface.iface-shuffle-fn]
// [spec:foma:sem:iface.iface-shuffle-fn]
// [spec:foma:def:foma.iface-shuffle-fn]
// [spec:foma:sem:foma.iface-shuffle-fn]
pub fn iface_shuffle() {
    if iface_stack_check(2) != 0 {
        while stack_size() > 1 {
            // C: fsm_shuffle(stack_pop(), stack_pop()) — two pops in one expression
            // (C order unspecified); Rust evaluates left-to-right. Shuffle is
            // commutative, so the resulting language is the same.
            let a = stack_pop().unwrap();
            let b = stack_pop().unwrap();
            stack_add(fsm_minimize(fsm_shuffle(a, b)));
        }
    }
}

// [spec:foma:def:iface.iface-sigma-net-fn]
// [spec:foma:sem:iface.iface-sigma-net-fn]
// [spec:foma:def:foma.iface-sigma-net-fn]
// [spec:foma:sem:foma.iface-sigma-net-fn]
pub fn iface_sigma_net() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_sigma_net(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-sort-input-fn]
// [spec:foma:sem:iface.iface-sort-input-fn]
// [spec:foma:def:foma.iface-sort-input-fn]
// [spec:foma:sem:foma.iface-sort-input-fn]
pub fn iface_sort_input() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |f| fsm_sort_arcs(f, 1));
    }
}

// [spec:foma:def:iface.iface-sort-output-fn]
// [spec:foma:sem:iface.iface-sort-output-fn]
// [spec:foma:def:foma.iface-sort-output-fn]
// [spec:foma:sem:foma.iface-sort-output-fn]
pub fn iface_sort_output() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |f| fsm_sort_arcs(f, 2));
    }
}

// [spec:foma:def:iface.iface-sort-fn]
// [spec:foma:sem:iface.iface-sort-fn]
// [spec:foma:def:foma.iface-sort-fn]
// [spec:foma:sem:foma.iface-sort-fn]
pub fn iface_sort() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |f| sigma_sort(f));
        stack_add(fsm_topsort(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-test-equivalent-fn]
// [spec:foma:sem:iface.iface-test-equivalent-fn]
// [spec:foma:def:foma.iface-test-equivalent-fn]
// [spec:foma:sem:foma.iface-test-equivalent-fn]
pub fn iface_test_equivalent() {
    if iface_stack_check(2) != 0 {
        let top = stack_find_top().unwrap();
        let second = stack_find_second().unwrap();
        let mut one = stack_entry_fsm(top, |f| fsm_copy(f));
        let mut two = stack_entry_fsm(second, |f| fsm_copy(f));
        fsm_count(&mut one);
        fsm_count(&mut two);
        // Latent leak in C: the two copies are never fsm_destroy'd; here they are
        // consumed (freed) by fsm_equivalent — no-op observable difference.
        iface_print_bool(fsm_equivalent(one, two));
    }
}

// [spec:foma:def:iface.iface-test-functional-fn]
// [spec:foma:sem:iface.iface-test-functional-fn]
// [spec:foma:def:foma.iface-test-functional-fn]
// [spec:foma:sem:foma.iface-test-functional-fn]
pub fn iface_test_functional() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let r = stack_entry_fsm(top, |f| fsm_isfunctional(f));
        iface_print_bool(r);
    }
}

// [spec:foma:def:iface.iface-test-identity-fn]
// [spec:foma:sem:iface.iface-test-identity-fn]
// [spec:foma:def:foma.iface-test-identity-fn]
// [spec:foma:sem:foma.iface-test-identity-fn]
pub fn iface_test_identity() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let r = stack_entry_fsm(top, |f| fsm_isidentity(f));
        iface_print_bool(r);
    }
}

// [spec:foma:def:iface.iface-test-nonnull-fn]
// [spec:foma:sem:iface.iface-test-nonnull-fn]
// [spec:foma:def:foma.iface-test-nonnull-fn]
// [spec:foma:sem:foma.iface-test-nonnull-fn]
pub fn iface_test_nonnull() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        // C: iface_print_bool(!fsm_isempty(...)) — logical NOT of the int result.
        let e = stack_entry_fsm(top, |f| fsm_isempty(f));
        iface_print_bool((e == 0) as i32);
    }
}

// [spec:foma:def:iface.iface-test-null-fn]
// [spec:foma:sem:iface.iface-test-null-fn]
// [spec:foma:def:foma.iface-test-null-fn]
// [spec:foma:sem:foma.iface-test-null-fn]
pub fn iface_test_null() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let r = stack_entry_fsm(top, |f| fsm_isempty(f));
        iface_print_bool(r);
    }
}

// [spec:foma:def:iface.iface-test-unambiguous-fn]
// [spec:foma:sem:iface.iface-test-unambiguous-fn]
// [spec:foma:def:foma.iface-test-unambiguous-fn]
// [spec:foma:sem:foma.iface-test-unambiguous-fn]
pub fn iface_test_unambiguous() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let r = stack_entry_fsm(top, |f| fsm_isunambiguous(f));
        iface_print_bool(r);
    }
}

// [spec:foma:def:iface.iface-test-lower-universal-fn]
// [spec:foma:sem:iface.iface-test-lower-universal-fn]
// [spec:foma:def:foma.iface-test-lower-universal-fn]
// [spec:foma:sem:foma.iface-test-lower-universal-fn]
pub fn iface_test_lower_universal() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let mut tmp = fsm_complement(fsm_lower(stack_entry_fsm(top, |f| fsm_copy(f))));
        iface_print_bool(fsm_isempty(&mut tmp));
        fsm_destroy(tmp);
    }
}

// [spec:foma:def:iface.iface-test-sequential-fn]
// [spec:foma:sem:iface.iface-test-sequential-fn]
// [spec:foma:def:foma.iface-test-sequential-fn]
// [spec:foma:sem:foma.iface-test-sequential-fn]
pub fn iface_test_sequential() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let r = stack_entry_fsm(top, |f| fsm_issequential(f));
        iface_print_bool(r);
    }
}

// [spec:foma:def:iface.iface-test-upper-universal-fn]
// [spec:foma:sem:iface.iface-test-upper-universal-fn]
// [spec:foma:def:foma.iface-test-upper-universal-fn]
// [spec:foma:sem:foma.iface-test-upper-universal-fn]
pub fn iface_test_upper_universal() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        let mut tmp = fsm_complement(fsm_upper(stack_entry_fsm(top, |f| fsm_copy(f))));
        iface_print_bool(fsm_isempty(&mut tmp));
        fsm_destroy(tmp);
    }
}

// [spec:foma:def:iface.iface-turn-fn]
// [spec:foma:sem:iface.iface-turn-fn]
// [spec:foma:def:foma.iface-turn-fn]
// [spec:foma:sem:foma.iface-turn-fn]
pub fn iface_turn() {
    // Latent bug reproduced: "turn stack" calls stack_rotate() (byte-for-byte the
    // same as iface_rotate), NOT stack_turn(); it only swaps top/bottom fsms.
    if iface_stack_check(1) != 0 {
        stack_rotate();
    }
}

// [spec:foma:def:iface.iface-twosided-flags-fn]
// [spec:foma:sem:iface.iface-twosided-flags-fn]
// [spec:foma:def:foma.iface-twosided-flags-fn]
// [spec:foma:sem:foma.iface-twosided-flags-fn]
pub fn iface_twosided_flags() {
    if iface_stack_check(1) != 0 {
        stack_add(flag_twosided(stack_pop().unwrap()));
    }
}

// [spec:foma:def:iface.iface-union-fn]
// [spec:foma:sem:iface.iface-union-fn]
// [spec:foma:def:foma.iface-union-fn]
// [spec:foma:sem:foma.iface-union-fn]
pub fn iface_union() {
    if iface_stack_check(2) != 0 {
        while stack_size() > 1 {
            // C: fsm_union(stack_pop(), stack_pop()) — pops in one expression (C
            // order unspecified); union is commutative. Minimized, NOT topsorted.
            let a = stack_pop().unwrap();
            let b = stack_pop().unwrap();
            stack_add(fsm_minimize(fsm_union(a, b)));
        }
    }
}

// [spec:foma:def:iface.iface-upper-side-fn]
// [spec:foma:sem:iface.iface-upper-side-fn]
// [spec:foma:def:foma.iface-upper-side-fn]
// [spec:foma:sem:foma.iface-upper-side-fn]
pub fn iface_upper_side() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_topsort(fsm_minimize(fsm_upper(stack_pop().unwrap()))));
    }
}

// [spec:foma:def:iface.iface-view-fn]
// [spec:foma:sem:iface.iface-view-fn]
// [spec:foma:def:foma.iface-view-fn]
// [spec:foma:sem:foma.iface-view-fn]
pub fn iface_view() {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |f| view_net(f));
    }
}

// [spec:foma:def:iface.iface-words-file-fn]
// [spec:foma:sem:iface.iface-words-file-fn]
// [spec:foma:def:foma.iface-words-file-fn]
// [spec:foma:sem:foma.iface-words-file-fn]
pub fn iface_words_file(filename: &str, r#type: i32) {
    /* type 0 (words), 1 (upper-words), 2 (lower-words) */
    // C: `static char *(*applyer)(...) = &apply_words;` — a function-local static.
    // Latent bug: type 0 never resets it, so after any type 1/2 call a later type-0
    // call reuses the previous upper/lower enumerator. Reproduced via a thread_local.
    thread_local! {
        static APPLYER: Cell<fn(&mut ApplyHandle) -> Option<String>> =
            Cell::new(apply_words as fn(&mut ApplyHandle) -> Option<String>);
    }
    if r#type == 1 {
        APPLYER.with(|a| a.set(apply_upper_words));
    }
    if r#type == 2 {
        APPLYER.with(|a| a.set(apply_lower_words));
    }
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        if stack_entry_fsm(top, |f| f.pathcount) == PATHCOUNT_CYCLIC {
            print!("FSM is cyclic: can't write all words to file.\n");
            return;
        }
        print!("Writing to {}.\n", filename);
        let mut outfile = match File::create(filename) {
            Ok(f) => f,
            Err(_) => {
                perror("Error opening file");
                return;
            }
        };
        let ah = stack_get_ah().unwrap();
        stack_entry_ah(ah, |h| iface_apply_set_params(h));
        let applyer = APPLYER.with(|a| a.get());
        loop {
            let result = stack_entry_ah(ah, |h| applyer(h));
            match result {
                None => break,
                Some(r) => {
                    let _ = write!(outfile, "{}\n", r);
                }
            }
        }
        stack_entry_ah(ah, |h| apply_reset_enumerator(h));
        // fclose(outfile) — dropped at scope end.
    }
}

// [spec:foma:def:iface.iface-words-fn]
// [spec:foma:sem:iface.iface-words-fn]
// [spec:foma:def:foma.iface-words-fn]
// [spec:foma:sem:foma.iface-words-fn]
pub fn iface_words(limit: i32) {
    let limit = if limit == -1 {
        G_LIST_LIMIT.with(|v| v.get())
    } else {
        limit
    };
    if iface_stack_check(1) != 0 {
        let ah = stack_get_ah().unwrap();
        stack_entry_ah(ah, |h| iface_apply_set_params(h));
        let mut i = limit;
        while i > 0 {
            let result = stack_entry_ah(ah, |h| apply_words(h));
            match result {
                None => break,
                Some(r) => print!("{}\n", r),
            }
            i -= 1;
        }
        stack_entry_ah(ah, |h| apply_reset_enumerator(h));
    }
}

/* Splits string of upper:lower pairs with space separator into two strings */
/* e.g. a:b c:d e 0:g => ace,bdeg  */

// [spec:foma:def:iface.iface-split-string-fn]
// [spec:foma:sem:iface.iface-split-string-fn]
// [spec:foma:def:foma.iface-split-string-fn]
// [spec:foma:sem:foma.iface-split-string-fn]
pub fn iface_split_string(result: &[u8], string: &mut Vec<u8>) {
    let space = 1u8;
    let epsilon = 2u8;
    let separator = 3u8;
    /* Simulate: SEPARATOR \SPACE+ @-> 0 .o. SPACE|SEPARATOR|EPSILON -> 0 */
    /*           to extract only the upper side of `result`.             */
    // Two-state filter (C's goto zero/one). End-of-Vec is the NUL terminator.
    let mut i = 0usize;
    let mut state = 0; // 0 = "zero" (initial), 1 = "one"
    loop {
        let c = result.get(i).copied().unwrap_or(0);
        if state == 0 {
            if c == 0 {
                break;
            } else if c == space || c == epsilon {
                i += 1;
            } else if c == separator {
                i += 1;
                state = 1;
            } else {
                string.push(c); // strncat(string, result+i, 1)
                i += 1;
            }
        } else if c == 0 {
            break;
        } else if c == space {
            i += 1;
            state = 0;
        } else {
            i += 1;
        }
    }
}

// [spec:foma:def:iface.iface-split-result-fn]
// [spec:foma:sem:iface.iface-split-result-fn]
// [spec:foma:def:foma.iface-split-result-fn]
// [spec:foma:sem:foma.iface-split-result-fn]
pub fn iface_split_result(result: &mut Vec<u8>, upper: &mut Vec<u8>, lower: &mut Vec<u8>) {
    // DEVIATION from C: C does calloc(strlen(result), 1) for *upper and *lower —
    // one byte short of room for the NUL, so a filter-free result writes one byte
    // past the allocation. Growable Vecs (starting empty) have no OOB hazard.
    upper.clear();
    lower.clear();
    /* Split string into upper by filtering the input side, and lower by the */
    /* same filter but on the reversed string.                               */
    iface_split_string(result, upper);
    xstrrev(Some(result));
    iface_split_string(result, lower);
    xstrrev(Some(lower));
    xstrrev(Some(result));
}

// [spec:foma:def:iface.iface-pairs-call-fn]
// [spec:foma:sem:iface.iface-pairs-call-fn]
// [spec:foma:def:foma.iface-pairs-call-fn]
// [spec:foma:sem:foma.iface-pairs-call-fn]
pub fn iface_pairs_call(limit: i32, random: i32) {
    let limit = if limit == -1 {
        G_LIST_LIMIT.with(|v| v.get())
    } else {
        limit
    };
    if iface_stack_check(1) != 0 {
        let ah = stack_get_ah().unwrap();
        stack_entry_ah(ah, |h| apply_set_show_flags(h, G_SHOW_FLAGS.with(|v| v.get())));
        stack_entry_ah(ah, |h| apply_set_obey_flags(h, G_OBEY_FLAGS.with(|v| v.get())));
        stack_entry_ah(ah, |h| apply_set_space_symbol(h, "\u{1}"));
        stack_entry_ah(ah, |h| apply_set_epsilon(h, "\u{2}"));
        stack_entry_ah(ah, |h| apply_set_separator(h, "\u{3}"));
        let mut i = limit;
        while i > 0 {
            let result = if random == 1 {
                stack_entry_ah(ah, |h| apply_random_words(h))
            } else {
                stack_entry_ah(ah, |h| apply_words(h))
            };
            let result = match result {
                None => break,
                Some(r) => r,
            };
            let mut result = result.into_bytes();
            let mut upper = Vec::new();
            let mut lower = Vec::new();
            iface_split_result(&mut result, &mut upper, &mut lower);
            // printf("%s\t%s\n", upper, lower) — raw bytes (may be UTF-8-corrupted).
            let mut out = std::io::stdout();
            let _ = out.write_all(&upper);
            let _ = out.write_all(b"\t");
            let _ = out.write_all(&lower);
            let _ = out.write_all(b"\n");
            i -= 1;
        }
        stack_entry_ah(ah, |h| apply_set_space_symbol(h, " "));
        stack_entry_ah(ah, |h| apply_set_epsilon(h, "0"));
        stack_entry_ah(ah, |h| apply_set_separator(h, ":"));
        stack_entry_ah(ah, |h| apply_reset_enumerator(h));
    }
}

// [spec:foma:def:iface.iface-random-pairs-fn]
// [spec:foma:sem:iface.iface-random-pairs-fn]
// [spec:foma:def:foma.iface-random-pairs-fn]
// [spec:foma:sem:foma.iface-random-pairs-fn]
pub fn iface_random_pairs(limit: i32) {
    // Latent quirk: limit == -1 becomes g_list_limit (inside iface_pairs_call),
    // NOT g_list_random_limit like the other random commands.
    iface_pairs_call(limit, 1);
}

// [spec:foma:def:iface.iface-pairs-fn]
// [spec:foma:sem:iface.iface-pairs-fn]
// [spec:foma:def:foma.iface-pairs-fn]
// [spec:foma:sem:foma.iface-pairs-fn]
pub fn iface_pairs(limit: i32) {
    iface_pairs_call(limit, 0);
}

// [spec:foma:def:iface.iface-pairs-file-fn]
// [spec:foma:sem:iface.iface-pairs-file-fn]
// [spec:foma:def:foma.iface-pairs-file-fn]
// [spec:foma:sem:foma.iface-pairs-file-fn]
pub fn iface_pairs_file(filename: &str) {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        if stack_entry_fsm(top, |f| f.pathcount) == PATHCOUNT_CYCLIC {
            print!("FSM is cyclic: can't write all pairs to file.\n");
            return;
        }
        print!("Writing to {}.\n", filename);
        let mut outfile = match File::create(filename) {
            Ok(f) => f,
            Err(_) => {
                perror("Error opening file");
                return;
            }
        };
        let ah = stack_get_ah().unwrap();
        stack_entry_ah(ah, |h| apply_set_show_flags(h, G_SHOW_FLAGS.with(|v| v.get())));
        stack_entry_ah(ah, |h| apply_set_obey_flags(h, G_OBEY_FLAGS.with(|v| v.get())));
        stack_entry_ah(ah, |h| apply_set_space_symbol(h, "\u{1}"));
        stack_entry_ah(ah, |h| apply_set_epsilon(h, "\u{2}"));
        stack_entry_ah(ah, |h| apply_set_separator(h, "\u{3}"));
        loop {
            let result = stack_entry_ah(ah, |h| apply_words(h));
            let result = match result {
                None => break,
                Some(r) => r,
            };
            let mut result = result.into_bytes();
            let mut upper = Vec::new();
            let mut lower = Vec::new();
            iface_split_result(&mut result, &mut upper, &mut lower);
            let _ = outfile.write_all(&upper);
            let _ = outfile.write_all(b"\t");
            let _ = outfile.write_all(&lower);
            let _ = outfile.write_all(b"\n");
        }
        stack_entry_ah(ah, |h| apply_set_space_symbol(h, " "));
        stack_entry_ah(ah, |h| apply_set_epsilon(h, "0"));
        stack_entry_ah(ah, |h| apply_set_separator(h, ":"));
        stack_entry_ah(ah, |h| apply_reset_enumerator(h));
        // fclose(outfile) — dropped at scope end.
    }
}

// [spec:foma:def:iface.iface-write-att-fn]
// [spec:foma:sem:iface.iface-write-att-fn]
// [spec:foma:def:foma.iface-write-att-fn]
// [spec:foma:sem:foma.iface-write-att-fn]
pub fn iface_write_att(filename: Option<&str>) -> i32 {
    if iface_stack_check(1) == 0 {
        return 1;
    }
    let top = stack_find_top().unwrap();
    let mut outfile: Box<dyn Write> = match filename {
        None => Box::new(std::io::stdout()),
        Some(name) => {
            print!("Writing AT&T file: {}\n", name);
            match File::create(name) {
                Ok(f) => Box::new(f),
                Err(_) => {
                    eprint!("{}: ", name);
                    perror("File error opening.");
                    return 1;
                }
            }
        }
    };
    stack_entry_fsm(top, |f| net_print_att(f, &mut *outfile));
    // fclose only when filename != NULL; stdout is not closed. Both drop here.
    0
}

// [spec:foma:def:iface.iface-write-prolog-fn]
// [spec:foma:sem:iface.iface-write-prolog-fn]
// [spec:foma:def:foma.iface-write-prolog-fn]
// [spec:foma:sem:foma.iface-write-prolog-fn]
pub fn iface_write_prolog(filename: Option<&str>) {
    if iface_stack_check(1) != 0 {
        let top = stack_find_top().unwrap();
        stack_entry_fsm(top, |f| foma_write_prolog(f, filename));
    }
}

// [spec:foma:def:iface.iface-zero-plus-fn]
// [spec:foma:sem:iface.iface-zero-plus-fn]
// [spec:foma:def:foma.iface-zero-plus-fn]
// [spec:foma:sem:foma.iface-zero-plus-fn]
pub fn iface_zero_plus() {
    if iface_stack_check(1) != 0 {
        stack_add(fsm_topsort(fsm_minimize(fsm_kleene_star(stack_pop().unwrap()))));
    }
}

// [spec:foma:def:iface.sigptr-fn]
// [spec:foma:sem:iface.sigptr-fn]
// Static helper (C: `static char *sigptr`). Returns an owned display string; C
// returns borrowed static/sigma pointers or a leaked malloc'd "NONE(%i)", but
// the owned String here is observably identical.
pub(crate) fn sigptr(sigma: Option<&Sigma>, number: i32) -> String {
    if number == EPSILON {
        return "0".to_string();
    }
    if number == UNKNOWN {
        return "?".to_string();
    }
    if number == IDENTITY {
        return "@".to_string();
    }
    let mut s = sigma;
    while let Some(node) = s {
        if node.number == number {
            let sym = node.symbol.as_deref().unwrap_or("");
            if sym == "0" {
                return "\"0\"".to_string();
            }
            if sym == "?" {
                return "\"?\"".to_string();
            }
            if sym == "\n" {
                return "\\n".to_string();
            }
            if sym == "\r" {
                return "\\r".to_string();
            }
            return sym.to_string();
        }
        s = node.next.as_deref();
    }
    // malloc(40) + snprintf "NONE(%i)" — leaked in C.
    format!("NONE({})", number)
}

// [spec:foma:def:iface.print-net-fn]
// [spec:foma:sem:iface.print-net-fn]
pub(crate) fn print_net(net: &mut Fsm, filename: Option<&str>) -> i32 {
    let mut out: Box<dyn Write> = match filename {
        None => Box::new(std::io::stdout()),
        Some(name) => match File::create(name) {
            Ok(f) => Box::new(f),
            Err(_) => {
                print!("Error writing to file {}. Using stdout.\n", name);
                Box::new(std::io::stdout())
            }
        },
    };
    // C prints this unconditionally after the fopen block (even after fallback).
    if let Some(name) = filename {
        print!("Writing network to file {}.\n", name);
    }
    fsm_count(net);
    let mut finals = vec![0i32; net.statecount as usize];
    let mut i = 0usize;
    loop {
        let state_no = net.states[i].state_no;
        if state_no == -1 {
            break;
        }
        let final_state = net.states[i].final_state;
        let in_ = net.states[i].r#in;
        let out_ = net.states[i].out;
        finals[state_no as usize] = if final_state == 1 { 1 } else { 0 };
        if in_ != out_ {
            net.arity = 2;
        }
        i += 1;
    }
    print_sigma(net.sigma.as_deref(), &mut *out);
    let _ = write!(out, "Net: {}\n", net.name);
    let _ = write!(out, "Flags: ");
    if net.is_deterministic == YES {
        let _ = write!(out, "deterministic ");
    }
    if net.is_pruned == YES {
        let _ = write!(out, "pruned ");
    }
    if net.is_minimized == YES {
        let _ = write!(out, "minimized ");
    }
    if net.is_epsilon_free == YES {
        let _ = write!(out, "epsilon_free ");
    }
    if net.is_loop_free != 0 {
        let _ = write!(out, "loop_free ");
    }
    if net.arcs_sorted_in != 0 {
        let _ = write!(out, "arcs_sorted_in ");
    }
    if net.arcs_sorted_out != 0 {
        let _ = write!(out, "arcs_sorted_out ");
    }
    let _ = write!(out, "\n");
    let _ = write!(out, "Arity: {}\n", net.arity);
    let mut previous_state: i32 = -1;
    let mut i = 0usize;
    loop {
        let state_no = net.states[i].state_no;
        if state_no == -1 {
            break;
        }
        let start_state = net.states[i].start_state;
        let final_state = net.states[i].final_state;
        let in_ = net.states[i].r#in as i32;
        let out_ = net.states[i].out as i32;
        let target = net.states[i].target;
        if state_no != previous_state {
            if start_state != 0 {
                let _ = write!(out, "S");
            }
            if final_state != 0 {
                let _ = write!(out, "f");
            }
            if in_ == -1 {
                let _ = write!(out, "s{}:\t(no arcs).\n", state_no);
                i += 1;
                continue;
            } else {
                let _ = write!(out, "s{}:\t", state_no);
            }
        }
        previous_state = state_no;
        if in_ == out_ {
            if in_ == IDENTITY {
                let _ = write!(out, "@ -> ");
            } else if in_ == UNKNOWN {
                let _ = write!(out, "?:? -> ");
            } else {
                let _ = write!(out, "{} -> ", sigptr(net.sigma.as_deref(), in_));
            }
        } else {
            let _ = write!(
                out,
                "<{}:{}> -> ",
                sigptr(net.sigma.as_deref(), in_),
                sigptr(net.sigma.as_deref(), out_)
            );
        }
        if finals[target as usize] == 1 {
            let _ = write!(out, "f");
        }
        let _ = write!(out, "s{}", target);
        if net.states[i + 1].state_no == state_no {
            let _ = write!(out, ", ");
        } else {
            let _ = write!(out, ".\n");
        }
        i += 1;
    }
    // fclose only when filename != NULL; free finals. All drop at scope end.
    0
}

// [spec:foma:def:iface.print-mem-size-fn]
// [spec:foma:sem:iface.print-mem-size-fn]
pub(crate) fn print_mem_size(net: &Fsm) {
    // DEVIATION from C: the byte total uses C's LP64 sizeof(struct sigma/fsm/
    // fsm_state) = 24 / 128 / 16. Rust's own struct layouts differ (String/Vec/
    // Option<Box>), so the C ABI sizes are hardcoded to keep the printed size
    // byte-identical on a 64-bit build.
    const SIZEOF_SIGMA: u32 = 24;
    const SIZEOF_FSM: u32 = 128;
    const SIZEOF_FSM_STATE: u32 = 16;
    let mut s: u32 = 0;
    let mut sig = net.sigma.as_deref();
    while let Some(node) = sig {
        if node.number == -1 {
            break;
        }
        let symlen = node.symbol.as_deref().unwrap_or("").len() as u32;
        s = s.wrapping_add(symlen).wrapping_add(1).wrapping_add(SIZEOF_SIGMA);
        sig = node.next.as_deref();
    }
    s = s.wrapping_add(SIZEOF_FSM);
    s = s.wrapping_add(SIZEOF_FSM_STATE.wrapping_mul(net.linecount as u32));
    let sf = s as f32;
    let size: String;
    if s < 1024 {
        size = format!("{} bytes. ", s);
    } else if s >= 1024 && s < 1048576 {
        size = format!("{:.1} kB. ", (sf / 1024.0f32) as f64);
    } else if s >= 1048576 && s < 1073741824 {
        size = format!("{:.1} MB. ", (sf / 1048576.0f32) as f64);
    } else {
        size = format!("{:.1} GB. ", (sf / 1073741824.0f32) as f64);
    }
    print!("{}", size);
    let _ = std::io::stdout().flush();
}

// [spec:foma:def:iface.print-stats-fn]
// [spec:foma:sem:iface.print-stats-fn]
// [spec:foma:def:foma.print-stats-fn]
// [spec:foma:sem:foma.print-stats-fn]
pub fn print_stats(net: &Fsm) -> i32 {
    print_mem_size(net);
    if net.statecount == 1 {
        print!("1 state, ");
    } else {
        print!("{} states, ", net.statecount);
    }
    if net.arccount == 1 {
        print!("1 arc, ");
    } else {
        print!("{} arcs, ", net.arccount);
    }
    if net.pathcount == 1 {
        print!("1 path");
    } else if net.pathcount == -1 {
        print!("Cyclic");
    } else if net.pathcount == -2 {
        // more than %lld paths with LLONG_MAX
        print!("more than {} paths", i64::MAX);
    } else if net.pathcount == -3 {
        print!("unknown number of paths");
    } else {
        print!("{} paths", net.pathcount);
    }
    print!(".\n");
    0
}

// [spec:foma:def:iface.print-sigma-fn]
// [spec:foma:sem:iface.print-sigma-fn]
pub(crate) fn print_sigma(sigma: Option<&Sigma>, out: &mut dyn Write) -> i32 {
    let mut size = 0;
    let _ = write!(out, "Sigma:");
    let mut s = sigma;
    while let Some(node) = s {
        if node.number > 2 {
            let _ = write!(out, " {}", node.symbol.as_deref().unwrap_or(""));
            size += 1;
        }
        if node.number == IDENTITY {
            let _ = write!(out, " {}", "@");
        }
        if node.number == UNKNOWN {
            let _ = write!(out, " {}", "?");
        }
        s = node.next.as_deref();
    }
    let _ = write!(out, "\n");
    let _ = write!(out, "Size: {}.\n", size);
    1
}

// [spec:foma:def:iface.print-dot-fn]
// [spec:foma:sem:iface.print-dot-fn]
pub(crate) fn print_dot(net: &mut Fsm, filename: Option<&str>) -> i32 {
    fsm_count(net);
    let mut finals = vec![0i16; net.statecount as usize];
    let mut i = 0usize;
    loop {
        let state_no = net.states[i].state_no;
        if state_no == -1 {
            break;
        }
        finals[state_no as usize] = if net.states[i].final_state == 1 { 1 } else { 0 };
        i += 1;
    }
    let mut dotfile: Box<dyn Write> = match filename {
        // C: `dotfile = fopen(filename,"w");` with NO NULL check (latent crash on
        // failure). DEVIATION from C: expect() panics at the nearest safe point.
        Some(name) => Box::new(File::create(name).expect("Error opening dot file")),
        None => Box::new(std::io::stdout()),
    };
    let _ = write!(dotfile, "digraph A {{\nrankdir = LR;\n");
    for i in 0..net.statecount {
        if finals[i as usize] != 0 {
            let _ = write!(dotfile, "node [shape=doublecircle,style=filled] {}\n", i);
        } else {
            let _ = write!(dotfile, "node [shape=circle,style=filled] {}\n", i);
        }
    }
    // C: calloc(linecount, sizeof(printed)) allocates sizeof(POINTER) per line
    // (over-allocation bug, harmless); here a per-line flag Vec of linecount.
    let mut printed = vec![0i16; net.linecount as usize];
    let mut i = 0usize;
    loop {
        let state_no_i = net.states[i].state_no;
        if state_no_i == -1 {
            break;
        }
        let target_i = net.states[i].target;
        if target_i == -1 || printed[i] == 1 {
            i += 1;
            continue;
        }
        let _ = write!(dotfile, "{} -> {} [label=\"", state_no_i, target_i);
        let mut linelen = 0i32;
        let mut j = i;
        while net.states[j].state_no == state_no_i {
            let target_j = net.states[j].target;
            if target_i == target_j && printed[j] == 0 {
                printed[j] = 1;
                let in_j = net.states[j].r#in as i32;
                let out_j = net.states[j].out as i32;
                if in_j == out_j && out_j != UNKNOWN {
                    let sig = sigptr(net.sigma.as_deref(), in_j);
                    let _ = dotfile.write_all(&escape_string(sig.as_bytes(), b'"'));
                    linelen += sig.len() as i32;
                } else {
                    let sig_in = sigptr(net.sigma.as_deref(), in_j);
                    let sig_out = sigptr(net.sigma.as_deref(), out_j);
                    let _ = dotfile.write_all(b"<");
                    let _ = dotfile.write_all(&escape_string(sig_in.as_bytes(), b'"'));
                    let _ = dotfile.write_all(b":");
                    let _ = dotfile.write_all(&escape_string(sig_out.as_bytes(), b'"'));
                    let _ = dotfile.write_all(b">");
                    linelen += sig_in.len() as i32 + sig_out.len() as i32 + 3;
                }
                if linelen > 12 {
                    let _ = write!(dotfile, "\\n");
                    linelen = 0;
                } else {
                    let _ = write!(dotfile, " ");
                }
            }
            j += 1;
        }
        let _ = write!(dotfile, "\"];\n");
        i += 1;
    }
    // free(finals); free(printed).
    let _ = write!(dotfile, "}}\n");
    // fclose only when filename != NULL — dropped at scope end.
    1
}

// [spec:foma:def:iface.view-net-fn]
// [spec:foma:sem:iface.view-net-fn]
// [spec:foma:def:foma.view-net-fn]
// [spec:foma:sem:foma.view-net-fn]
pub(crate) fn view_net(net: &mut Fsm) -> i32 {
    // DEVIATION from C: no tempnam(); a unique temp path is built under the system
    // temp dir from the pid + a per-thread counter (observably a unique file).
    fn tempnam_foma() -> String {
        thread_local! { static CTR: Cell<u64> = const { Cell::new(0) }; }
        let n = CTR.with(|c| {
            let v = c.get();
            c.set(v + 1);
            v
        });
        std::env::temp_dir()
            .join(format!("foma{}_{}", std::process::id(), n))
            .to_string_lossy()
            .into_owned()
    }
    let dotname = format!("{}.dot", tempnam_foma());
    print_dot(net, Some(&dotname));
    let pngname = tempnam_foma();
    // DEVIATION from C: system(cmd) → `/bin/sh -c "<cmd>"` via std::process::Command
    // (a spawn failure ↔ C's system() == -1; the exit status is otherwise ignored).
    let cmd1 = if cfg!(target_os = "macos") {
        format!("dot -Tpng {} > {}.png ", dotname, pngname)
    } else {
        format!("dot -Tpng {} > {} ", dotname, pngname)
    };
    if std::process::Command::new("sh")
        .arg("-c")
        .arg(&cmd1)
        .status()
        .is_err()
    {
        print!("Error writing tempfile.\n");
    }
    let cmd2 = if cfg!(target_os = "macos") {
        format!("/usr/bin/open {}.png 2>/dev/null &", pngname)
    } else {
        format!("/usr/bin/xdg-open {} 2>/dev/null &", pngname)
    };
    if std::process::Command::new("sh")
        .arg("-c")
        .arg(&cmd2)
        .status()
        .is_err()
    {
        print!("Error opening viewer.\n");
    }
    // free(pngname); free(dotname) — temp files are never deleted (as in C).
    1
}

// ====================================================================
// SLICE 1 tests (iface.c top-of-file .. iface_quit, C-file order).
// These functions mutate the CLI network stack and print. Unit tests
// assert the STACK/NET effects: stack size, pop order, and the top
// net's language (via crate::apply / fsm_equivalent). Functions whose
// only observable effect is printing (help / apropos / warranty /
// print_*) are asserted for return value + non-panic on representative
// states — byte-exact stdout is covered by the sibling integration
// tests in crates/foma/tests/. Every test starts with stack_init()
// (thread-local stack + g_* globals start fresh per test thread).
//
// iface_quit only ever exit(0)s, so it has no in-process observable
// path; its /test facet lives on the integration assertion in
// crates/foma/tests/cli_slice1.rs (spawns `foma -q`, feeds "quit",
// asserts exit 0). Slice 2 appends its own tests below this block.
// ====================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::define::{add_defined, defined_networks_init};
    use crate::regex::fsm_parse_regex;
    use crate::stack::stack_init;
    use crate::types::Fsm;

    /// Push a compiled regex onto the CLI stack (fixture).
    fn push(re: &str) {
        stack_add(fsm_parse_regex(re, None, None).unwrap());
    }

    /// Push a topsorted regex (fixture). fsm_count leaves pathcount untouched, so
    /// a cyclic net only gets pathcount == PATHCOUNT_CYCLIC after fsm_topsort (the
    /// REPL topsorts regex results; a bare fsm_parse_regex leaves it UNKNOWN).
    fn push_topsorted(re: &str) {
        stack_add(fsm_topsort(fsm_parse_regex(re, None, None).unwrap()));
    }

    /// Pop the top net and test recognizer-language equality against `re`.
    fn top_is(re: &str) -> bool {
        let expected = fsm_parse_regex(re, None, None).unwrap();
        fsm_equivalent(stack_pop().unwrap(), expected) == 1
    }

    /// Does `net` accept `w` on the input (down) side?
    fn accepts_down(net: &Fsm, w: &str) -> bool {
        let mut h = apply_init(net);
        let r = apply_down(&mut h, Some(w));
        apply_clear(h);
        r.is_some()
    }

    /// First apply-down output for input `w` (transducer image).
    fn down1(net: &Fsm, w: &str) -> Option<String> {
        let mut h = apply_init(net);
        let r = apply_down(&mut h, Some(w));
        apply_clear(h);
        r
    }

    /// First apply-up output for input `w`.
    fn up1(net: &Fsm, w: &str) -> Option<String> {
        let mut h = apply_init(net);
        let r = apply_up(&mut h, Some(w));
        apply_clear(h);
        r
    }

    // Print-only help family: byte-exact output is integration-tested; here we
    // pin the return type (unit) and non-panic on empty and populated states.
    // [spec:foma:sem:iface.iface-help-fn/test]
    // [spec:foma:sem:foma.iface-help-fn/test]
    // [spec:foma:sem:iface.iface-apropos-fn/test]
    // [spec:foma:sem:foma.iface-apropos-fn/test]
    // [spec:foma:sem:iface.iface-help-search-fn/test]
    // [spec:foma:sem:foma.iface-help-search-fn/test]
    // [spec:foma:sem:iface.iface-print-bool-fn/test]
    // [spec:foma:sem:iface.iface-warranty-fn/test]
    // [spec:foma:sem:foma.iface-warranty-fn/test]
    #[test]
    fn help_family_prints_without_touching_the_stack() {
        stack_init();
        iface_help();
        iface_apropos("net"); // some matches
        iface_apropos("\u{1}zznomatchzz"); // no match → prints nothing
        iface_help_search("compose"); // matching entry
        iface_help_search("zznomatchzz"); // no match
        iface_print_bool(1);
        iface_print_bool(0);
        iface_warranty();
        // None of these read or write the stack.
        assert_eq!(stack_size(), 0);
    }

    // foma_net_print is re-exported from io.c; verify the re-export reaches the
    // real serializer (returns 1, writes the foma save-format frame). Byte-exact
    // content is covered by io.rs's own tests.
    // [spec:foma:sem:iface.foma-net-print-fn/test]
    #[test]
    fn foma_net_print_writes_save_format_and_returns_1() {
        let net = fsm_parse_regex("a", None, None).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        assert_eq!(foma_net_print(&net, &mut buf), 1);
        let s = String::from_utf8_lossy(&buf);
        assert!(s.starts_with("##foma-net 1.0##"), "got: {:?}", &s[..s.len().min(40)]);
        assert!(s.contains("##sigma##"));
        assert!(s.trim_end().ends_with("##end##"));
    }

    // iface_apply_set_params copies the four apply globals onto the handle in
    // order: print_space, print_pairs, show_flags, obey_flags.
    // [spec:foma:sem:iface.iface-apply-set-params-fn/test]
    // [spec:foma:sem:foma.iface-apply-set-params-fn/test]
    #[test]
    fn apply_set_params_copies_the_four_globals() {
        let net = fsm_parse_regex("a", None, None).unwrap();
        let mut h = apply_init(&net);
        G_PRINT_SPACE.with(|v| v.set(1));
        G_PRINT_PAIRS.with(|v| v.set(1));
        G_SHOW_FLAGS.with(|v| v.set(1));
        G_OBEY_FLAGS.with(|v| v.set(0));
        iface_apply_set_params(&mut h);
        assert_eq!(h.print_space, 1);
        assert_eq!(h.print_pairs, 1);
        assert_eq!(h.show_flags, 1);
        assert_eq!(h.obey_flags, 0);
        apply_clear(h);
    }

    // apply up/down via the iface entry points: net is NOT consumed (size
    // unchanged), and the empty-stack guard makes it a no-op.
    // [spec:foma:sem:iface.iface-apply-down-fn/test]
    // [spec:foma:sem:foma.iface-apply-down-fn/test]
    // [spec:foma:sem:iface.iface-apply-up-fn/test]
    // [spec:foma:sem:foma.iface-apply-up-fn/test]
    #[test]
    fn apply_down_and_up_keep_the_net_and_refuse_empty_stack() {
        stack_init();
        // Empty stack: iface_stack_check(1) refuses; nothing pushed/popped.
        iface_apply_down("a");
        iface_apply_up("a");
        assert_eq!(stack_size(), 0);
        push("a:b"); // transducer a -> b
        iface_apply_down("a"); // prints "b"; net not consumed
        assert_eq!(stack_size(), 1);
        iface_apply_up("b"); // prints "a"; net not consumed
        assert_eq!(stack_size(), 1);
        // The net is unchanged and still maps a -> b.
        assert_eq!(down1(&stack_pop().unwrap(), "a"), Some("b".to_string()));
    }

    // apply med: configures the cached med handle (heap max 4194305, med-limit
    // g_med_limit=3, med-cutoff g_med_cutoff=15) and does not consume the net;
    // empty stack refuses.
    // [spec:foma:sem:iface.iface-apply-med-fn/test]
    // [spec:foma:sem:foma.iface-apply-med-fn/test]
    #[test]
    fn apply_med_configures_handle_and_refuses_empty_stack() {
        stack_init();
        iface_apply_med("cat"); // empty stack: no-op
        assert_eq!(stack_size(), 0);
        push("c a t");
        iface_apply_med("cat");
        assert_eq!(stack_size(), 1); // net not consumed
        let amedh = stack_get_med_ah().unwrap();
        let (limit, cutoff, heap) =
            stack_entry_amedh(amedh, |m| (m.med_limit, m.med_cutoff, m.med_max_heap_size));
        assert_eq!(limit, 3);
        assert_eq!(cutoff, 15);
        assert_eq!(heap, 4194304 + 1);
    }

    // apply-file: invalid direction returns 1; empty stack returns 0; a bad
    // input path returns 1; a good run returns 0, writes results to the output
    // file, and leaves the net on the stack.
    // [spec:foma:sem:iface.iface-apply-file-fn/test]
    // [spec:foma:sem:foma.iface-apply-file-fn/test]
    #[test]
    fn apply_file_direction_stack_and_roundtrip() {
        let dir = std::env::temp_dir();
        let inpath = dir.join("foma_s1_applyfile_in.txt");
        let outpath = dir.join("foma_s1_applyfile_out.txt");
        std::fs::write(&inpath, "cat\n").unwrap();

        stack_init();
        // Invalid direction: rejected before the stack/file checks.
        assert_eq!(iface_apply_file(inpath.to_str().unwrap(), None, 0), 1);
        // Valid direction, empty stack: iface_stack_check(1) fails → 0.
        assert_eq!(iface_apply_file(inpath.to_str().unwrap(), None, AP_D), 0);

        push("c a t"); // acceptor for "cat" over sigma {c,a,t}
        // Bad input path with a populated stack: open failure → 1.
        assert_eq!(iface_apply_file("/no/such/foma/path", None, AP_D), 1);
        // Good run writing to a file.
        let rc = iface_apply_file(inpath.to_str().unwrap(), Some(outpath.to_str().unwrap()), AP_D);
        assert_eq!(rc, 0);
        assert_eq!(stack_size(), 1); // net not consumed
        let out = std::fs::read_to_string(&outpath).unwrap();
        assert!(out.contains("cat"), "output was: {:?}", out);
    }

    // close sigma: pop + push topsort(minimize(close_sigma(net,0))); size
    // unchanged, language preserved for a net with no unknown symbols. Refusal
    // path leaves the (empty) stack unchanged.
    // [spec:foma:sem:iface.iface-close-fn/test]
    // [spec:foma:sem:foma.iface-close-fn/test]
    #[test]
    fn close_preserves_language_and_refuses_empty() {
        stack_init();
        iface_close();
        assert_eq!(stack_size(), 0);
        push("a | b");
        iface_close();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a | b"));
    }

    // compact sigma: mutates top in place then pop + push topsort(minimize);
    // size unchanged, language preserved.
    // [spec:foma:sem:iface.iface-compact-fn/test]
    // [spec:foma:sem:foma.iface-compact-fn/test]
    #[test]
    fn compact_preserves_language() {
        stack_init();
        push("a | b");
        iface_compact();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a | b"));
    }

    // complete net: pop + push fsm_complete(net). The accepted language is
    // unchanged (the added sink state is non-final); size unchanged.
    // [spec:foma:sem:iface.iface-complete-fn/test]
    // [spec:foma:sem:foma.iface-complete-fn/test]
    #[test]
    fn complete_keeps_language_and_adds_no_words() {
        stack_init();
        push("a");
        iface_complete();
        assert_eq!(stack_size(), 1);
        let net = stack_pop().unwrap();
        assert!(accepts_down(&net, "a"));
        assert!(!accepts_down(&net, "aa"));
        assert!(!accepts_down(&net, ""));
    }

    // determinize net: pop + push fsm_determinize(net); no minimize/topsort.
    // Size unchanged, language preserved.
    // [spec:foma:sem:iface.iface-determinize-fn/test]
    // [spec:foma:sem:foma.iface-determinize-fn/test]
    #[test]
    fn determinize_preserves_language() {
        stack_init();
        push("a | b");
        iface_determinize();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a | b"));
    }

    // compose net: folds the ENTIRE stack down to one; the net nearer the top
    // is the first (upper) composition operand. Refuses with <2 nets.
    // [spec:foma:sem:iface.iface-compose-fn/test]
    // [spec:foma:sem:foma.iface-compose-fn/test]
    #[test]
    fn compose_folds_whole_stack_and_top_is_upper_operand() {
        stack_init();
        // <2 nets: refusal, stack unchanged.
        push("a");
        iface_compose();
        assert_eq!(stack_size(), 1);
        // 3 identity nets fold to a single net (fold-until-one).
        push("a");
        push("a");
        assert_eq!(stack_size(), 3);
        iface_compose();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a"));
        // Pop order: compose(top, second) = (a:b) .o. (b:c) = a:c.
        stack_init();
        push("b:c"); // bottom / second operand
        push("a:b"); // top / first (upper) operand
        iface_compose();
        assert_eq!(stack_size(), 1);
        assert_eq!(down1(&stack_pop().unwrap(), "a"), Some("c".to_string()));
    }

    // concatenate: folds the whole stack; the top net is the LEFT operand each
    // step, so [a,b,c] (bottom->top) yields "cba". Refuses with <2 nets.
    // [spec:foma:sem:iface.iface-conc-fn/test]
    // [spec:foma:sem:foma.iface-conc-fn/test]
    #[test]
    fn concatenate_folds_with_top_as_left_operand() {
        stack_init();
        push("a");
        iface_conc(); // <2 nets: refusal
        assert_eq!(stack_size(), 1);
        push("b");
        push("c"); // stack bottom->top: a, b, c
        assert_eq!(stack_size(), 3);
        iface_conc();
        assert_eq!(stack_size(), 1);
        assert!(top_is("c b a"));
    }

    // crossproduct net: SINGLE step (does not fold); top is the upper operand.
    // [spec:foma:sem:iface.iface-crossproduct-fn/test]
    // [spec:foma:sem:foma.iface-crossproduct-fn/test]
    #[test]
    fn crossproduct_is_single_step_top_is_upper() {
        stack_init();
        push("z"); // bottom, untouched
        push("b"); // second / lower operand
        push("c"); // top / upper operand
        iface_crossproduct();
        // Only the top two were consumed and one pushed: 3 -> 2.
        assert_eq!(stack_size(), 2);
        assert_eq!(down1(&stack_pop().unwrap(), "c"), Some("b".to_string()));
        assert!(top_is("z")); // bottom net still present
    }

    // intersect net: folds the whole stack (commutative). Refuses with <2 nets.
    // [spec:foma:sem:iface.iface-intersect-fn/test]
    // [spec:foma:sem:foma.iface-intersect-fn/test]
    #[test]
    fn intersect_folds_whole_stack() {
        stack_init();
        push("a | b");
        iface_intersect(); // <2 nets: refusal
        assert_eq!(stack_size(), 1);
        push("b | c");
        push("b"); // three sets whose intersection is {b}
        assert_eq!(stack_size(), 3);
        iface_intersect();
        assert_eq!(stack_size(), 1);
        assert!(top_is("b"));
    }

    // ignore net: SINGLE step; the top net is the base language A in A/B.
    // [spec:foma:sem:iface.iface-ignore-fn/test]
    // [spec:foma:sem:foma.iface-ignore-fn/test]
    #[test]
    fn ignore_is_single_step_base_is_top() {
        stack_init();
        push("z"); // bottom, untouched
        push("x"); // second: ignored material B
        push("a"); // top: base language A
        iface_ignore();
        assert_eq!(stack_size(), 2); // single step 3 -> 2
        let net = stack_pop().unwrap();
        assert!(accepts_down(&net, "a")); // base A with zero B's
        assert!(!accepts_down(&net, "x")); // B alone is not in A/B
        assert!(top_is("z"));
    }

    // invert net: pop + push fsm_invert(net); swaps upper/lower sides.
    // [spec:foma:sem:iface.iface-invert-fn/test]
    // [spec:foma:sem:foma.iface-invert-fn/test]
    #[test]
    fn invert_swaps_sides() {
        stack_init();
        push("a:b"); // maps a -> b
        iface_invert();
        assert_eq!(stack_size(), 1);
        // Inverted net is b:a; apply up "a" (lower) yields "b" (upper).
        assert_eq!(up1(&stack_pop().unwrap(), "a"), Some("b".to_string()));
    }

    // lower-side net: pop + push topsort(minimize(fsm_lower(net))).
    // [spec:foma:sem:iface.iface-lower-side-fn/test]
    // [spec:foma:sem:foma.iface-lower-side-fn/test]
    #[test]
    fn lower_side_takes_lower_projection() {
        stack_init();
        push("a:b");
        iface_lower_side();
        assert_eq!(stack_size(), 1);
        assert!(top_is("b"));
    }

    // letter machine: pop + push topsort(minimize(fsm_letter_machine(net)));
    // single-char symbols are unchanged in language.
    // [spec:foma:sem:iface.iface-letter-machine-fn/test]
    // [spec:foma:sem:foma.iface-letter-machine-fn/test]
    #[test]
    fn letter_machine_preserves_single_char_language() {
        stack_init();
        push("a b");
        iface_letter_machine();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a b"));
    }

    // minimize net: forces g_minimal = 1 for the op, then RESTORES the saved
    // value; size unchanged, language preserved.
    // [spec:foma:sem:iface.iface-minimize-fn/test]
    // [spec:foma:sem:foma.iface-minimize-fn/test]
    #[test]
    fn minimize_restores_g_minimal_and_preserves_language() {
        stack_init();
        G_MINIMAL.with(|v| v.set(0)); // user turned `minimal` OFF
        push("a | b");
        iface_minimize();
        assert_eq!(G_MINIMAL.with(|v| v.get()), 0); // restored to saved value
        assert_eq!(stack_size(), 1);
        assert!(top_is("a | b"));
    }

    // negate net: pop + push topsort(minimize(complement(net))); complement is
    // over the net's own sigma.
    // [spec:foma:sem:iface.iface-negate-fn/test]
    // [spec:foma:sem:foma.iface-negate-fn/test]
    #[test]
    fn negate_complements_over_sigma() {
        stack_init();
        push("a"); // sigma {a}
        iface_negate();
        assert_eq!(stack_size(), 1);
        let net = stack_pop().unwrap();
        assert!(!accepts_down(&net, "a")); // "a" excluded
        assert!(accepts_down(&net, "")); // epsilon in complement
        assert!(accepts_down(&net, "aa")); // aa in complement
    }

    // one-plus net (Kleene plus): pop + push topsort(minimize(kleene_plus)).
    // [spec:foma:sem:iface.iface-one-plus-fn/test]
    // [spec:foma:sem:foma.iface-one-plus-fn/test]
    #[test]
    fn one_plus_is_kleene_plus() {
        stack_init();
        push("a");
        iface_one_plus();
        assert_eq!(stack_size(), 1);
        let net = stack_pop().unwrap();
        assert!(accepts_down(&net, "a"));
        assert!(accepts_down(&net, "aa"));
        assert!(!accepts_down(&net, "")); // plus requires >= 1
    }

    // eliminate flags / eliminate flag <name>: pop + push flag_eliminate(net,
    // None|Some(name)); on a flagless net the language is preserved.
    // [spec:foma:sem:iface.iface-eliminate-flags-fn/test]
    // [spec:foma:sem:foma.iface-eliminate-flags-fn/test]
    // [spec:foma:sem:iface.iface-eliminate-flag-fn/test]
    // [spec:foma:sem:foma.iface-eliminate-flag-fn/test]
    #[test]
    fn eliminate_flags_and_flag_wire_to_flag_eliminate() {
        stack_init();
        iface_eliminate_flags(); // empty: refusal
        iface_eliminate_flag("X"); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a b");
        iface_eliminate_flags();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a b"));
        push("a b");
        iface_eliminate_flag("X");
        assert_eq!(stack_size(), 1);
        assert!(top_is("a b"));
    }

    // extract ambiguous / unambiguous / ambiguous-upper (domain): each pops +
    // pushes its extraction; size unchanged.
    // [spec:foma:sem:iface.iface-extract-ambiguous-fn/test]
    // [spec:foma:sem:foma.iface-extract-ambiguous-fn/test]
    // [spec:foma:sem:iface.iface-extract-unambiguous-fn/test]
    // [spec:foma:sem:foma.iface-extract-unambiguous-fn/test]
    // [spec:foma:sem:iface.iface-ambiguous-upper-fn/test]
    // [spec:foma:sem:foma.iface-ambiguous-upper-fn/test]
    #[test]
    fn extract_ambiguous_unambiguous_and_domain() {
        // extract ambiguous of an ambiguous transducer keeps its paths.
        stack_init();
        push("a:b | a:c");
        iface_extract_ambiguous();
        assert_eq!(stack_size(), 1);
        assert!(accepts_down(&stack_pop().unwrap(), "a"));
        // extract ambiguous of an unambiguous transducer is empty.
        push("a:b");
        iface_extract_ambiguous();
        assert!(!accepts_down(&stack_pop().unwrap(), "a"));

        // extract unambiguous of an unambiguous transducer keeps it (a -> b).
        push("a:b");
        iface_extract_unambiguous();
        assert_eq!(stack_size(), 1);
        assert_eq!(down1(&stack_pop().unwrap(), "a"), Some("b".to_string()));
        // extract unambiguous of an ambiguous transducer is empty.
        push("a:b | a:c");
        iface_extract_unambiguous();
        assert!(!accepts_down(&stack_pop().unwrap(), "a"));

        // ambiguous upper = domain of ambiguous inputs.
        push("a:b | a:c");
        iface_ambiguous_upper();
        assert_eq!(stack_size(), 1);
        assert!(accepts_down(&stack_pop().unwrap(), "a"));
        push("a:b"); // unambiguous → empty domain
        iface_ambiguous_upper();
        assert!(!accepts_down(&stack_pop().unwrap(), "a"));
    }

    // extract number: scans to the first ASCII digit then atoi; minus signs are
    // skipped so negatives read positive; no digit → 0; stops at first non-digit.
    // [spec:foma:sem:iface.iface-extract-number-fn/test]
    // [spec:foma:sem:foma.iface-extract-number-fn/test]
    #[test]
    fn extract_number_scans_to_first_digit() {
        assert_eq!(iface_extract_number("abc-5"), 5);
        assert_eq!(iface_extract_number("42abc"), 42);
        assert_eq!(iface_extract_number("v2.3"), 2);
        assert_eq!(iface_extract_number("hello"), 0);
        assert_eq!(iface_extract_number(""), 0);
        assert_eq!(iface_extract_number("007"), 7);
    }

    // factorize / sequentialize: pop + push fsm_bimachine / fsm_sequentialize;
    // size unchanged, refuses on empty.
    // [spec:foma:sem:iface.iface-factorize-fn/test]
    // [spec:foma:sem:foma.iface-factorize-fn/test]
    // [spec:foma:sem:iface.iface-sequentialize-fn/test]
    // [spec:foma:sem:foma.iface-sequentialize-fn/test]
    #[test]
    fn factorize_and_sequentialize_are_single_net_ops() {
        stack_init();
        iface_factorize(); // empty: refusal
        iface_sequentialize(); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a b");
        iface_sequentialize();
        assert_eq!(stack_size(), 1);
        // sequentialize of an acyclic acceptor preserves its language.
        assert!(top_is("a b"));
        push("a b c");
        iface_factorize();
        assert_eq!(stack_size(), 1); // bimachine factorization is one net op
    }

    // label net: pop + push fsm_sigma_pairs_net(net); size unchanged.
    // [spec:foma:sem:iface.iface-label-net-fn/test]
    // [spec:foma:sem:foma.iface-label-net-fn/test]
    #[test]
    fn label_net_extracts_attested_pairs() {
        stack_init();
        iface_label_net(); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a:b");
        iface_label_net();
        assert_eq!(stack_size(), 1);
        // The label net accepts the single attested pair a:b.
        assert_eq!(down1(&stack_pop().unwrap(), "a"), Some("b".to_string()));
    }

    // pop stack: empty prints "Stack is empty." (no iface_stack_check) and
    // leaves the stack empty; otherwise pops + fsm_destroy's the top.
    // [spec:foma:sem:iface.iface-pop-fn/test]
    // [spec:foma:sem:foma.iface-pop-fn/test]
    #[test]
    fn pop_removes_top_or_reports_empty() {
        stack_init();
        iface_pop(); // empty: message only
        assert_eq!(stack_size(), 0);
        push("a");
        push("b");
        iface_pop(); // removes "b"
        assert_eq!(stack_size(), 1);
        assert!(top_is("a")); // "a" is now the top
    }

    // print lower-words: caches the apply handle, enumerates (prints), resets
    // the enumerator, and does NOT consume the net; empty refuses.
    // [spec:foma:sem:iface.iface-lower-words-fn/test]
    // [spec:foma:sem:foma.iface-lower-words-fn/test]
    #[test]
    fn lower_words_prints_without_consuming() {
        stack_init();
        iface_lower_words(-1); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a:b | a:c");
        iface_lower_words(-1); // g_list_limit; prints "b","c"
        iface_lower_words(1); // explicit small limit
        assert_eq!(stack_size(), 1); // net not consumed
    }

    // name net + print name: name net strncpy's <=40 bytes into the top net's
    // name (no NUL when >=40 → truncated to 40 bytes) and calls print name; the
    // net stays on the stack.
    // [spec:foma:sem:iface.iface-name-net-fn/test]
    // [spec:foma:sem:foma.iface-name-net-fn/test]
    // [spec:foma:sem:iface.iface-print-name-fn/test]
    // [spec:foma:sem:foma.iface-print-name-fn/test]
    #[test]
    fn name_net_sets_name_truncating_at_40_then_prints() {
        stack_init();
        iface_name_net("nope"); // empty: refusal, no panic
        iface_print_name(); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a");
        iface_name_net("hello");
        assert_eq!(stack_size(), 1); // not popped
        let top = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(top, |f| f.name.clone()), "hello");
        iface_print_name(); // prints "hello"
        // >= 40 bytes: truncated to exactly 40 bytes (no NUL terminator in C).
        let long = "x".repeat(45);
        iface_name_net(&long);
        let top = stack_find_top().unwrap();
        let name = stack_entry_fsm(top, |f| f.name.clone());
        assert_eq!(name.len(), 40);
        assert_eq!(name, "x".repeat(40));
    }

    // print dot: requires >=1; writes dot to stdout (None) or a file (Some);
    // net not consumed; empty refuses.
    // [spec:foma:sem:iface.iface-print-dot-fn/test]
    // [spec:foma:sem:foma.iface-print-dot-fn/test]
    #[test]
    fn print_dot_writes_and_keeps_net() {
        stack_init();
        iface_print_dot(None); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a b");
        iface_print_dot(None); // to stdout
        let dotpath = std::env::temp_dir().join("foma_s1_printdot.dot");
        iface_print_dot(Some(dotpath.to_str().unwrap()));
        assert_eq!(stack_size(), 1); // net not consumed
        assert!(dotpath.exists());
    }

    // print net: named lookup (found / not found via g_defines) and the stack
    // top path; nothing is consumed.
    // [spec:foma:sem:iface.iface-print-net-fn/test]
    // [spec:foma:sem:foma.iface-print-net-fn/test]
    #[test]
    fn print_net_named_and_top_paths() {
        stack_init();
        // netname None + empty stack: refusal.
        iface_print_net(None, None);
        assert_eq!(stack_size(), 0);
        // netname Some with g_defines None (default): "No defined network".
        iface_print_net(Some("Foo"), None);
        // Populate g_defines with Foo, then print it by name.
        G_DEFINES.with(|g| *g.borrow_mut() = Some(defined_networks_init()));
        let def = fsm_parse_regex("x y", None, None).unwrap();
        G_DEFINES.with(|g| {
            let mut g = g.borrow_mut();
            add_defined(g.as_deref_mut().unwrap(), Some(def), "Foo");
        });
        iface_print_net(Some("Foo"), None); // found → prints
        // netname None + populated stack: prints the top net.
        push("a b");
        iface_print_net(None, None);
        assert_eq!(stack_size(), 1); // stack untouched
    }

    // print cmatrix / export cmatrix: with no confusion matrix both print
    // "No confusion matrix defined." and do not consume the net; empty refuses.
    // The cmatrix-present + byte-exact output is covered by the spelling/io
    // integration tests.
    // [spec:foma:sem:iface.iface-print-cmatrix-fn/test]
    // [spec:foma:sem:foma.iface-print-cmatrix-fn/test]
    // [spec:foma:sem:iface.iface-print-cmatrix-att-fn/test]
    // [spec:foma:sem:foma.iface-print-cmatrix-att-fn/test]
    #[test]
    fn print_cmatrix_reports_missing_matrix() {
        stack_init();
        iface_print_cmatrix(); // empty: refusal
        iface_print_cmatrix_att(None); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a b"); // no confusion matrix attached
        iface_print_cmatrix();
        iface_print_cmatrix_att(None);
        assert_eq!(stack_size(), 1); // net not consumed
    }

    // print defined: g_defines None prints "No defined symbols."; a populated
    // list prints each entry. Neither touches the stack.
    // [spec:foma:sem:iface.iface-print-defined-fn/test]
    // [spec:foma:sem:foma.iface-print-defined-fn/test]
    #[test]
    fn print_defined_handles_empty_and_populated() {
        stack_init();
        iface_print_defined(); // g_defines None: "No defined symbols."
        G_DEFINES.with(|g| *g.borrow_mut() = Some(defined_networks_init()));
        let def = fsm_parse_regex("x y", None, None).unwrap();
        G_DEFINES.with(|g| {
            let mut g = g.borrow_mut();
            add_defined(g.as_deref_mut().unwrap(), Some(def), "Foo");
        });
        iface_print_defined(); // prints "Foo\t<stats>"
        assert_eq!(stack_size(), 0);
    }

    // load defined: round-trip through a temp file. The file is written with
    // io::save_defined (the same primitive iface_save_defined delegates to;
    // that iface wrapper is slice 2's), then iface_load_defined restores the
    // definitions into a fresh g_defines. A missing file → load reports
    // "File error" and adds nothing.
    // [spec:foma:sem:iface.iface-load-defined-fn/test]
    // [spec:foma:sem:foma.iface-load-defined-fn/test]
    #[test]
    fn load_defined_restores_saved_definitions() {
        let path = std::env::temp_dir().join("foma_s1_defined.gz");
        let p = path.to_str().unwrap();
        // Missing file: load reports "File error" (g_defines initialized so the
        // helper is actually invoked) and leaves the table empty.
        G_DEFINES.with(|g| *g.borrow_mut() = Some(defined_networks_init()));
        iface_load_defined("/no/such/foma/defined");
        let missing = G_DEFINES.with(|g| {
            let mut g = g.borrow_mut();
            find_defined(g.as_deref_mut().unwrap(), "Foo").is_some()
        });
        assert!(!missing);

        // Define Foo = [x y] and write the file via the io primitive.
        let def = fsm_parse_regex("x y", None, None).unwrap();
        G_DEFINES.with(|g| {
            let mut g = g.borrow_mut();
            add_defined(g.as_deref_mut().unwrap(), Some(def), "Foo");
            save_defined(g.as_deref_mut().unwrap(), p);
        });
        // Reset g_defines and load the file back.
        G_DEFINES.with(|g| *g.borrow_mut() = Some(defined_networks_init()));
        iface_load_defined(p);
        // Foo is restored and equals [x y].
        let restored = G_DEFINES.with(|g| {
            let mut g = g.borrow_mut();
            find_defined(g.as_deref_mut().unwrap(), "Foo").map(|f| fsm_copy(f))
        });
        let restored = restored.expect("Foo should be restored");
        let expected = fsm_parse_regex("x y", None, None).unwrap();
        assert_eq!(fsm_equivalent(restored, expected), 1);
    }

    // load stack: reads every net from a multi-net binary file and pushes them
    // in file order, so the LAST net in the file ends up on top. A missing file
    // is reported and the stack is left unchanged.
    // [spec:foma:sem:iface.iface-load-stack-fn/test]
    // [spec:foma:sem:foma.iface-load-stack-fn/test]
    #[test]
    fn load_stack_pushes_in_file_order_last_on_top() {
        let path = std::env::temp_dir().join("foma_s1_stack.gz");
        let p = path.to_str().unwrap();
        // Build a bottom->top save file (a, b, c) exactly as iface_save_stack
        // would: nets on the CLI stack have had fsm_count run by stack_add (so
        // linecount is current), then each is foma_net_print'd into one gzip
        // stream.
        {
            let file = File::create(p).unwrap();
            let mut gz = GzEncoder::new(file, Compression::default());
            for r in ["a", "b", "c"] {
                let mut net = fsm_parse_regex(r, None, None).unwrap();
                fsm_count(&mut net);
                foma_net_print(&net, &mut gz);
            }
            gz.finish().unwrap();
        }
        stack_init();
        // Missing file: reported, stack unchanged.
        iface_load_stack("/no/such/foma/stack");
        assert_eq!(stack_size(), 0);
        // Real load: a, b, c pushed in file order → top is "c", bottom is "a".
        iface_load_stack(p);
        assert_eq!(stack_size(), 3);
        assert!(top_is("c")); // pops "c"
        assert!(top_is("b"));
        assert!(top_is("a"));
    }

    // ================================================================
    // SLICE 2 tests (iface_quit .. end of iface.c, C-file order), plus
    // the four static callees slice 1 had stubbed. Same method as slice
    // 1: stack-effect assertions (size + top-net language via crate::apply
    // / fsm_equivalent), temp-file round-trips for read/write/save, and
    // return-value pins for the pure/test_* functions. Print-only paths
    // assert return value + non-panic + stack preservation; byte-exact
    // stdout is left to the sibling integration tests.
    // ================================================================

    // random lower/upper/words + iface_apply_random: each caches the apply
    // handle, prints tallied random paths, resets the enumerator, and does NOT
    // consume the net; empty stack refuses. Output is non-deterministic so only
    // the stack effect is pinned here.
    // [spec:foma:sem:iface.iface-random-lower-fn/test]
    // [spec:foma:sem:foma.iface-random-lower-fn/test]
    // [spec:foma:sem:iface.iface-random-upper-fn/test]
    // [spec:foma:sem:foma.iface-random-upper-fn/test]
    // [spec:foma:sem:iface.iface-random-words-fn/test]
    // [spec:foma:sem:foma.iface-random-words-fn/test]
    // [spec:foma:sem:iface.iface-apply-random-fn/test]
    // [spec:foma:sem:foma.iface-apply-random-fn/test]
    #[test]
    fn random_family_prints_without_consuming() {
        stack_init();
        iface_random_lower(-1); // empty: refusal
        iface_random_upper(-1);
        iface_random_words(-1);
        iface_apply_random(apply_random_words, 3);
        assert_eq!(stack_size(), 0);
        push("a:b | c:d");
        iface_random_lower(-1); // g_list_random_limit
        iface_random_upper(3); // explicit limit
        iface_random_words(2);
        iface_apply_random(apply_random_words, 4);
        assert_eq!(stack_size(), 1); // net not consumed
        assert!(top_is("a:b | c:d"));
    }

    // print sigma / print stats: both read the top net and preserve it; empty
    // stack refuses. print_stats returns 0 and drives print_mem_size.
    // [spec:foma:sem:iface.iface-print-sigma-fn/test]
    // [spec:foma:sem:foma.iface-print-sigma-fn/test]
    // [spec:foma:sem:iface.iface-print-stats-fn/test]
    // [spec:foma:sem:foma.iface-print-stats-fn/test]
    #[test]
    fn print_sigma_and_stats_keep_net() {
        stack_init();
        iface_print_sigma(); // empty: refusal
        iface_print_stats();
        assert_eq!(stack_size(), 0);
        push("a b");
        iface_print_sigma();
        iface_print_stats();
        assert_eq!(stack_size(), 1); // net not consumed
        assert!(top_is("a b"));
    }

    // print shortest-string / -size: both branches (arity 1 acceptor and arity 2
    // transducer) run without consuming the net; empty refuses. Byte-exact output
    // is integration-tested.
    // [spec:foma:sem:iface.iface-print-shortest-string-fn/test]
    // [spec:foma:sem:foma.iface-print-shortest-string-fn/test]
    // [spec:foma:sem:iface.iface-print-shortest-string-size-fn/test]
    // [spec:foma:sem:foma.iface-print-shortest-string-size-fn/test]
    #[test]
    fn print_shortest_string_both_arities_keep_net() {
        stack_init();
        iface_print_shortest_string(); // empty: refusal
        iface_print_shortest_string_size();
        assert_eq!(stack_size(), 0);
        push("a b b"); // arity 1
        iface_print_shortest_string();
        iface_print_shortest_string_size();
        assert_eq!(stack_size(), 1); // net not consumed
        let _ = stack_pop();
        push("[a:b] | [a a:c]"); // arity 2 transducer
        iface_print_shortest_string();
        iface_print_shortest_string_size();
        assert_eq!(stack_size(), 1); // net not consumed
    }

    // read att / prolog: a print + stack_add wrapper over the io reader. Round-trip
    // through the matching writer (net_print_att / foma_write_prolog); a bad path
    // returns 1 and leaves the stack unchanged.
    // [spec:foma:sem:iface.iface-read-att-fn/test]
    // [spec:foma:sem:foma.iface-read-att-fn/test]
    // [spec:foma:sem:iface.iface-read-prolog-fn/test]
    // [spec:foma:sem:foma.iface-read-prolog-fn/test]
    #[test]
    fn read_att_and_prolog_roundtrip_and_error() {
        let dir = std::env::temp_dir();
        let attp = dir.join("foma_s2_read.att");
        let att = attp.to_str().unwrap();
        {
            let net = fsm_parse_regex("a b", None, None).unwrap();
            let mut f = File::create(att).unwrap();
            net_print_att(&net, &mut f);
        }
        let plp = dir.join("foma_s2_read.prolog");
        let pl = plp.to_str().unwrap();
        {
            let mut net = fsm_parse_regex("a b", None, None).unwrap();
            foma_write_prolog(&mut net, Some(pl));
        }
        stack_init();
        assert_eq!(iface_read_att("/no/such/foma/att"), 1);
        assert_eq!(iface_read_prolog("/no/such/foma/prolog"), 1);
        assert_eq!(stack_size(), 0);
        assert_eq!(iface_read_att(att), 0);
        assert_eq!(stack_size(), 1);
        assert!(top_is("a b"));
        assert_eq!(iface_read_prolog(pl), 0);
        assert_eq!(stack_size(), 1);
        assert!(top_is("a b"));
    }

    // read text / spaced-text: compile a newline word list / space-separated
    // symbol lines into an automaton (topsort(minimize)); pushed on the stack. A
    // bad path returns 1.
    // [spec:foma:sem:iface.iface-read-text-fn/test]
    // [spec:foma:sem:foma.iface-read-text-fn/test]
    // [spec:foma:sem:iface.iface-read-spaced-text-fn/test]
    // [spec:foma:sem:foma.iface-read-spaced-text-fn/test]
    #[test]
    fn read_text_and_spaced_text() {
        let dir = std::env::temp_dir();
        let rtp = dir.join("foma_s2_text.txt");
        let rt = rtp.to_str().unwrap();
        std::fs::write(rt, "cat\ndog\n").unwrap();
        let rstp = dir.join("foma_s2_spaced.txt");
        let rst = rstp.to_str().unwrap();
        std::fs::write(rst, "a b c\n").unwrap();

        stack_init();
        assert_eq!(iface_read_text("/no/such/foma/text"), 1);
        assert_eq!(iface_read_spaced_text("/no/such/foma/spaced"), 1);
        assert_eq!(stack_size(), 0);
        assert_eq!(iface_read_text(rt), 0);
        assert_eq!(stack_size(), 1);
        let net = stack_pop().unwrap();
        assert!(accepts_down(&net, "cat"));
        assert!(accepts_down(&net, "dog"));
        assert!(!accepts_down(&net, "cow"));
        assert_eq!(iface_read_spaced_text(rst), 0);
        assert_eq!(stack_size(), 1);
        assert!(accepts_down(&stack_pop().unwrap(), "abc"));
    }

    // stack check: returns 1 when >= size nets are present, else prints the
    // "Not enough networks" message and returns 0.
    // [spec:foma:sem:iface.iface-stack-check-fn/test]
    // [spec:foma:sem:foma.iface-stack-check-fn/test]
    #[test]
    fn stack_check_counts_the_stack() {
        stack_init();
        assert_eq!(iface_stack_check(0), 1); // 0 >= 0 with an empty stack
        assert_eq!(iface_stack_check(1), 0);
        push("a");
        push("b");
        assert_eq!(iface_stack_check(2), 1);
        assert_eq!(iface_stack_check(3), 0);
    }

    // substitute symbol / defined: symbol dequotes both args and replaces the
    // `original` symbol with `substitute`; defined replaces every arc labelled
    // `original` with the named defined network. Both pop+push a single net;
    // the "No defined network" and "does not occur" guards leave it untouched.
    // [spec:foma:sem:iface.iface-substitute-symbol-fn/test]
    // [spec:foma:sem:foma.iface-substitute-symbol-fn/test]
    // [spec:foma:sem:iface.iface-substitute-defined-fn/test]
    // [spec:foma:sem:foma.iface-substitute-defined-fn/test]
    #[test]
    fn substitute_symbol_and_defined() {
        stack_init();
        iface_substitute_symbol("a", "x"); // empty: refusal
        iface_substitute_defined("a", "X");
        assert_eq!(stack_size(), 0);

        push("a b");
        iface_substitute_symbol("a", "x"); // replace symbol a with x
        assert_eq!(stack_size(), 1);
        assert!(top_is("x b"));

        // substitute defined: X = [p] replaces the arc labelled `a`.
        G_DEFINES.with(|g| *g.borrow_mut() = Some(defined_networks_init()));
        let def = fsm_parse_regex("p", None, None).unwrap();
        G_DEFINES.with(|g| {
            let mut g = g.borrow_mut();
            add_defined(g.as_deref_mut().unwrap(), Some(def), "X");
        });
        push("a b");
        iface_substitute_defined("a", "Nope"); // no such defined net → unchanged
        assert_eq!(stack_size(), 1);
        iface_substitute_defined("zzz", "X"); // symbol does not occur → unchanged
        assert_eq!(stack_size(), 1);
        iface_substitute_defined("a", "X"); // success → a replaced by [p]
        assert_eq!(stack_size(), 1);
        assert!(top_is("p b"));
    }

    // upper-words / words: cache the apply handle, enumerate up to `limit`
    // (g_list_limit for -1), reset the enumerator, and preserve the net; empty
    // refuses.
    // [spec:foma:sem:iface.iface-upper-words-fn/test]
    // [spec:foma:sem:foma.iface-upper-words-fn/test]
    // [spec:foma:sem:iface.iface-words-fn/test]
    // [spec:foma:sem:foma.iface-words-fn/test]
    #[test]
    fn upper_words_and_words_keep_net() {
        stack_init();
        iface_upper_words(-1); // empty: refusal
        iface_words(-1);
        assert_eq!(stack_size(), 0);
        push("a:b | a:c");
        iface_upper_words(-1);
        iface_words(1);
        iface_words(-1);
        assert_eq!(stack_size(), 1); // net not consumed
    }

    // prune / reverse / zero-plus / upper-side: each pops + pushes a single-net
    // op. Language checks pin the transformation.
    // [spec:foma:sem:iface.iface-prune-fn/test]
    // [spec:foma:sem:foma.iface-prune-fn/test]
    // [spec:foma:sem:iface.iface-reverse-fn/test]
    // [spec:foma:sem:foma.iface-reverse-fn/test]
    // [spec:foma:sem:iface.iface-zero-plus-fn/test]
    // [spec:foma:sem:foma.iface-zero-plus-fn/test]
    // [spec:foma:sem:iface.iface-upper-side-fn/test]
    // [spec:foma:sem:foma.iface-upper-side-fn/test]
    #[test]
    fn prune_reverse_zero_plus_upper_side() {
        stack_init();
        iface_prune(); // empty: refusal
        iface_reverse();
        iface_zero_plus();
        iface_upper_side();
        assert_eq!(stack_size(), 0);

        push("a b"); // coaccessible already → language unchanged
        iface_prune();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a b"));

        push("a b"); // reverse of "ab" accepts "ba"
        iface_reverse();
        let net = stack_pop().unwrap();
        assert!(accepts_down(&net, "ba"));
        assert!(!accepts_down(&net, "ab"));

        push("a"); // Kleene star
        iface_zero_plus();
        let net = stack_pop().unwrap();
        assert!(accepts_down(&net, ""));
        assert!(accepts_down(&net, "aa"));

        push("a:b"); // upper projection
        iface_upper_side();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a"));
    }

    // rotate / turn: both wire to stack_rotate (turn is the reproduced latent bug
    // — it does NOT reverse the stack). rotate swaps the TOP and BOTTOM fsm fields
    // only; single-net and empty are no-ops.
    // [spec:foma:sem:iface.iface-rotate-fn/test]
    // [spec:foma:sem:foma.iface-rotate-fn/test]
    // [spec:foma:sem:iface.iface-turn-fn/test]
    // [spec:foma:sem:foma.iface-turn-fn/test]
    #[test]
    fn rotate_and_turn_swap_top_and_bottom() {
        stack_init();
        iface_rotate(); // empty: refusal (guard), no-op
        iface_turn();
        assert_eq!(stack_size(), 0);

        // bottom->top: a, b, c. rotate swaps a<->c fsms → top a, mid b, bottom c.
        push("a");
        push("b");
        push("c");
        iface_rotate();
        assert!(top_is("a")); // former bottom now on top
        assert!(top_is("b"));
        assert!(top_is("c"));

        // turn behaves identically (byte-for-byte the same stack_rotate call).
        push("a");
        push("b");
        push("c");
        iface_turn();
        assert!(top_is("a"));
        assert!(top_is("b"));
        assert!(top_is("c"));

        push("x"); // single net: unchanged
        iface_rotate();
        assert!(top_is("x"));
    }

    // save defined / save stack: save_defined reports "No defined networks" when
    // g_defines is empty, else round-trips through iface_load_defined; save_stack
    // writes every net bottom→top and round-trips through iface_load_stack.
    // [spec:foma:sem:iface.iface-save-defined-fn/test]
    // [spec:foma:sem:foma.iface-save-defined-fn/test]
    // [spec:foma:sem:iface.iface-save-stack-fn/test]
    // [spec:foma:sem:foma.iface-save-stack-fn/test]
    #[test]
    fn save_defined_and_save_stack_roundtrip() {
        let dir = std::env::temp_dir();
        let dp = dir.join("foma_s2_saved.gz");
        let d = dp.to_str().unwrap();
        // g_defines None (default): "No defined networks." and nothing written.
        let _ = std::fs::remove_file(d);
        iface_save_defined(d);
        assert!(!std::path::Path::new(d).exists());
        // Populate, save, reset, load back.
        G_DEFINES.with(|g| *g.borrow_mut() = Some(defined_networks_init()));
        let def = fsm_parse_regex("x y", None, None).unwrap();
        G_DEFINES.with(|g| {
            let mut g = g.borrow_mut();
            add_defined(g.as_deref_mut().unwrap(), Some(def), "Foo");
        });
        iface_save_defined(d);
        G_DEFINES.with(|g| *g.borrow_mut() = Some(defined_networks_init()));
        iface_load_defined(d);
        let restored = G_DEFINES.with(|g| {
            let mut g = g.borrow_mut();
            find_defined(g.as_deref_mut().unwrap(), "Foo").map(|f| fsm_copy(f))
        });
        let expected = fsm_parse_regex("x y", None, None).unwrap();
        assert_eq!(fsm_equivalent(restored.expect("Foo restored"), expected), 1);

        // save stack: writes bottom→top (a, b, c); load pushes in file order.
        let sp = dir.join("foma_s2_savestack.gz");
        let s = sp.to_str().unwrap();
        stack_init();
        let _ = std::fs::remove_file(s);
        iface_save_stack(s); // empty: refusal, nothing written
        assert!(!std::path::Path::new(s).exists());
        push("a");
        push("b");
        push("c");
        iface_save_stack(s);
        assert_eq!(stack_size(), 3); // stack not consumed
        stack_init();
        iface_load_stack(s);
        assert_eq!(stack_size(), 3);
        assert!(top_is("c"));
        assert!(top_is("b"));
        assert!(top_is("a"));
    }

    // set variable: BOOL accepts ON/OFF/1/0 (else "Invalid value"); the name match
    // is strncmp over 8 bytes (documented latent bug: any name sharing a real
    // variable's first 8 bytes hits it). INT uses strtol truncation and rejects
    // no-digit / range / negative. STRING replaces the value verbatim. show
    // variable(s) print without touching the stack or the values; the show_variable
    // ON/OFF-for-every-type formatter bug is byte-exact and integration-tested.
    // [spec:foma:sem:iface.iface-set-variable-fn/test]
    // [spec:foma:sem:foma.iface-set-variable-fn/test]
    // [spec:foma:sem:iface.iface-show-variable-fn/test]
    // [spec:foma:sem:foma.iface-show-variable-fn/test]
    // [spec:foma:sem:iface.iface-show-variables-fn/test]
    // [spec:foma:sem:foma.iface-show-variables-fn/test]
    #[test]
    fn set_and_show_variables() {
        stack_init();
        // BOOL: ON/OFF/1/0 all recognised.
        iface_set_variable("minimal", "OFF");
        assert_eq!(G_MINIMAL.with(|v| v.get()), 0);
        iface_set_variable("minimal", "ON");
        assert_eq!(G_MINIMAL.with(|v| v.get()), 1);
        iface_set_variable("minimal", "0");
        assert_eq!(G_MINIMAL.with(|v| v.get()), 0);
        iface_set_variable("minimal", "1");
        assert_eq!(G_MINIMAL.with(|v| v.get()), 1);
        iface_set_variable("minimal", "bogus"); // invalid → unchanged
        assert_eq!(G_MINIMAL.with(|v| v.get()), 1);

        // strncmp-8 latent bug: "hopcroft-XYZ" shares 8 bytes with "hopcroft-min".
        iface_set_variable("hopcroft-XYZ", "OFF");
        assert_eq!(G_MINIMIZE_HOPCROFT.with(|v| v.get()), 0);

        // INT: strtol truncation; no-digit / negative are rejected.
        iface_set_variable("med-limit", "7");
        assert_eq!(G_MED_LIMIT.with(|v| v.get()), 7);
        iface_set_variable("med-limit", "abc"); // no digits → unchanged
        assert_eq!(G_MED_LIMIT.with(|v| v.get()), 7);
        iface_set_variable("med-limit", "-3"); // negative → unchanged
        assert_eq!(G_MED_LIMIT.with(|v| v.get()), 7);

        // STRING: verbatim replace.
        iface_set_variable("att-epsilon", "@E@");
        assert_eq!(G_ATT_EPSILON.with(|s| s.borrow().clone()), "@E@");

        // Unknown variable: message only, no panic.
        iface_set_variable("zzznope", "ON");

        // show variable / variables: non-panic, no stack effect.
        iface_show_variable("med-limit"); // formatter bug prints OFF (value 7 != 1)
        iface_show_variable("att-epsilon");
        iface_show_variable("minimal");
        iface_show_variable("zzznope"); // "no global variable"
        iface_show_variables();
        assert_eq!(stack_size(), 0);
    }

    // shuffle / union: both fold the entire stack (commutative); union minimizes
    // without topsort. Refuse with < 2 nets.
    // [spec:foma:sem:iface.iface-shuffle-fn/test]
    // [spec:foma:sem:foma.iface-shuffle-fn/test]
    // [spec:foma:sem:iface.iface-union-fn/test]
    // [spec:foma:sem:foma.iface-union-fn/test]
    #[test]
    fn shuffle_and_union_fold_whole_stack() {
        stack_init();
        push("a");
        iface_shuffle(); // < 2: refusal
        iface_union();
        assert_eq!(stack_size(), 1);

        push("b"); // shuffle(a,b) = {ab, ba}
        iface_shuffle();
        assert_eq!(stack_size(), 1);
        let net = stack_pop().unwrap();
        assert!(accepts_down(&net, "ab"));
        assert!(accepts_down(&net, "ba"));

        push("a");
        push("b");
        push("c"); // union folds to a|b|c
        iface_union();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a | b | c"));
    }

    // sigma net: pop + push fsm_sigma_net; accepts every single symbol of the
    // alphabet.
    // [spec:foma:sem:iface.iface-sigma-net-fn/test]
    // [spec:foma:sem:foma.iface-sigma-net-fn/test]
    #[test]
    fn sigma_net_accepts_single_symbols() {
        stack_init();
        iface_sigma_net(); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a b"); // sigma {a, b}
        iface_sigma_net();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a | b"));
    }

    // sort in / out: mutate the top net's arc order in place (no pop/push);
    // sort net: sigma_sort + pop + push topsort. All preserve the language.
    // [spec:foma:sem:iface.iface-sort-input-fn/test]
    // [spec:foma:sem:foma.iface-sort-input-fn/test]
    // [spec:foma:sem:iface.iface-sort-output-fn/test]
    // [spec:foma:sem:foma.iface-sort-output-fn/test]
    // [spec:foma:sem:iface.iface-sort-fn/test]
    // [spec:foma:sem:foma.iface-sort-fn/test]
    #[test]
    fn sort_family_preserves_language() {
        stack_init();
        iface_sort_input(); // empty: refusal
        iface_sort_output();
        iface_sort();
        assert_eq!(stack_size(), 0);
        push("a b | b a");
        iface_sort_input();
        assert_eq!(stack_size(), 1);
        iface_sort_output();
        assert_eq!(stack_size(), 1);
        iface_sort();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a b | b a"));
    }

    // test_* family: each reads the top net (equivalent reads the top two),
    // prints the predicate via iface_print_bool, and preserves the stack. The
    // printed value is pinned by calling the same predicate the iface fn uses.
    // [spec:foma:sem:iface.iface-test-functional-fn/test]
    // [spec:foma:sem:foma.iface-test-functional-fn/test]
    // [spec:foma:sem:iface.iface-test-identity-fn/test]
    // [spec:foma:sem:foma.iface-test-identity-fn/test]
    // [spec:foma:sem:iface.iface-test-unambiguous-fn/test]
    // [spec:foma:sem:foma.iface-test-unambiguous-fn/test]
    // [spec:foma:sem:iface.iface-test-sequential-fn/test]
    // [spec:foma:sem:foma.iface-test-sequential-fn/test]
    // [spec:foma:sem:iface.iface-test-null-fn/test]
    // [spec:foma:sem:foma.iface-test-null-fn/test]
    // [spec:foma:sem:iface.iface-test-nonnull-fn/test]
    // [spec:foma:sem:foma.iface-test-nonnull-fn/test]
    // [spec:foma:sem:iface.iface-test-lower-universal-fn/test]
    // [spec:foma:sem:foma.iface-test-lower-universal-fn/test]
    // [spec:foma:sem:iface.iface-test-upper-universal-fn/test]
    // [spec:foma:sem:foma.iface-test-upper-universal-fn/test]
    // [spec:foma:sem:iface.iface-test-equivalent-fn/test]
    // [spec:foma:sem:foma.iface-test-equivalent-fn/test]
    #[test]
    fn test_family_pins_predicate_and_preserves_stack() {
        stack_init();
        // Every test_* refuses on an empty stack.
        iface_test_functional();
        iface_test_identity();
        iface_test_unambiguous();
        iface_test_sequential();
        iface_test_null();
        iface_test_nonnull();
        iface_test_lower_universal();
        iface_test_upper_universal();
        assert_eq!(stack_size(), 0);

        // functional: true for a:b, false for a:b|a:c.
        push("a:b");
        let t = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(t, |f| fsm_isfunctional(f)), 1);
        iface_test_functional();
        assert_eq!(stack_size(), 1);
        let _ = stack_pop();
        push("[a:b] | [a:c]");
        let t = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(t, |f| fsm_isfunctional(f)), 0);
        iface_test_functional();
        let _ = stack_pop();

        // identity: true for a, false for a:b.
        push("a");
        let t = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(t, |f| fsm_isidentity(f)), 1);
        iface_test_identity();
        let _ = stack_pop();
        push("a:b");
        let t = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(t, |f| fsm_isidentity(f)), 0);
        iface_test_identity();
        let _ = stack_pop();

        // unambiguous: true for a:b, false for a:b|a:c.
        push("a:b");
        let t = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(t, |f| fsm_isunambiguous(f)), 1);
        iface_test_unambiguous();
        let _ = stack_pop();
        push("[a:b] | [a:c]");
        let t = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(t, |f| fsm_isunambiguous(f)), 0);
        iface_test_unambiguous();
        let _ = stack_pop();

        // sequential: true for the acyclic acceptor "a b".
        push("a b");
        let t = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(t, |f| fsm_issequential(f)), 1);
        iface_test_sequential();
        let _ = stack_pop();

        // null / nonnull: empty language vs. non-empty.
        push("[a] - [a]"); // empty
        let t = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(t, |f| fsm_isempty(f)), 1);
        iface_test_null();
        iface_test_nonnull();
        assert_eq!(stack_size(), 1);
        let _ = stack_pop();
        push("a"); // non-empty
        let t = stack_find_top().unwrap();
        assert_eq!(stack_entry_fsm(t, |f| fsm_isempty(f)), 0);
        iface_test_null();
        iface_test_nonnull();
        let _ = stack_pop();

        // lower-/upper-universal: compound (copy + complement + isempty), so only
        // preservation is pinned here. ?* is universal; "a" is not.
        push("?*");
        iface_test_lower_universal();
        iface_test_upper_universal();
        assert_eq!(stack_size(), 1);
        let _ = stack_pop();
        push("a");
        iface_test_lower_universal();
        iface_test_upper_universal();
        assert_eq!(stack_size(), 1);
        let _ = stack_pop();

        // equivalent: reads the top two, preserves both. (a|b)≡(b|a) true; a≢b.
        push("a | b");
        push("b | a");
        assert_eq!(stack_size(), 2);
        iface_test_equivalent();
        assert_eq!(stack_size(), 2);
        let one = stack_entry_fsm(stack_find_top().unwrap(), |f| fsm_copy(f));
        let two = stack_entry_fsm(stack_find_second().unwrap(), |f| fsm_copy(f));
        assert_eq!(fsm_equivalent(one, two), 1);
        stack_init();
        push("a");
        push("b");
        iface_test_equivalent();
        assert_eq!(stack_size(), 2);
    }

    // twosided flag-diacritics: pop + push flag_twosided; a flagless net keeps
    // its language.
    // [spec:foma:sem:iface.iface-twosided-flags-fn/test]
    // [spec:foma:sem:foma.iface-twosided-flags-fn/test]
    #[test]
    fn twosided_flags_preserves_flagless_language() {
        stack_init();
        iface_twosided_flags(); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a b");
        iface_twosided_flags();
        assert_eq!(stack_size(), 1);
        assert!(top_is("a b"));
    }

    // view: guarded by iface_stack_check(1); with an empty stack it returns
    // without calling view_net (which would spawn dot + a viewer). This guard-path
    // test also stands in for view_net's facet — no external process is spawned.
    // [spec:foma:sem:iface.iface-view-fn/test]
    // [spec:foma:sem:foma.iface-view-fn/test]
    // [spec:foma:sem:iface.view-net-fn/test]
    // [spec:foma:sem:foma.view-net-fn/test]
    #[test]
    fn view_refuses_empty_stack_without_spawning() {
        stack_init();
        iface_view(); // empty: refusal → view_net not reached, no viewer spawned
        assert_eq!(stack_size(), 0);
    }

    // words to file: type 0/1/2 select the word/upper/lower enumerator; a cyclic
    // net prints the "can't write" message and writes nothing; an acyclic net
    // writes each enumerated word. The net is preserved.
    // [spec:foma:sem:iface.iface-words-file-fn/test]
    // [spec:foma:sem:foma.iface-words-file-fn/test]
    #[test]
    fn words_file_writes_per_type_and_refuses_cyclic() {
        let dir = std::env::temp_dir();
        let p0 = dir.join("foma_s2_words0.txt");
        let p1 = dir.join("foma_s2_words1.txt");
        let p2 = dir.join("foma_s2_words2.txt");
        let pc = dir.join("foma_s2_wordsc.txt");
        let _ = std::fs::remove_file(&pc);

        stack_init();
        iface_words_file(p0.to_str().unwrap(), 0); // empty: refusal
        assert_eq!(stack_size(), 0);

        push("a b"); // acyclic acceptor → whole word "ab"
        iface_words_file(p0.to_str().unwrap(), 0);
        assert_eq!(stack_size(), 1); // net not consumed
        assert_eq!(std::fs::read_to_string(&p0).unwrap(), "ab\n");
        let _ = stack_pop();

        push("a:b"); // transducer → upper "a", lower "b"
        iface_words_file(p1.to_str().unwrap(), 1);
        assert_eq!(std::fs::read_to_string(&p1).unwrap(), "a\n");
        iface_words_file(p2.to_str().unwrap(), 2);
        assert_eq!(std::fs::read_to_string(&p2).unwrap(), "b\n");
        let _ = stack_pop();

        push_topsorted("a*"); // cyclic → refuses before opening the file
        iface_words_file(pc.to_str().unwrap(), 0);
        assert_eq!(stack_size(), 1);
        assert!(!std::path::Path::new(&pc).exists());
    }

    // pairs / pairs_call / random_pairs: print input:output pairs (splitting the
    // pair encoding), preserve the net, and reset the enumerator; empty refuses.
    // random_pairs routes through pairs_call(limit, 1) and inherits the g_list_limit
    // quirk for -1.
    // [spec:foma:sem:iface.iface-pairs-fn/test]
    // [spec:foma:sem:foma.iface-pairs-fn/test]
    // [spec:foma:sem:iface.iface-pairs-call-fn/test]
    // [spec:foma:sem:foma.iface-pairs-call-fn/test]
    // [spec:foma:sem:iface.iface-random-pairs-fn/test]
    // [spec:foma:sem:foma.iface-random-pairs-fn/test]
    #[test]
    fn pairs_family_prints_without_consuming() {
        stack_init();
        iface_pairs(-1); // empty: refusal
        iface_pairs_call(2, 0);
        iface_random_pairs(-1);
        assert_eq!(stack_size(), 0);
        push("a:b | c:d");
        iface_pairs(-1);
        iface_pairs_call(2, 0);
        iface_pairs_call(2, 1); // random path
        iface_random_pairs(-1);
        assert_eq!(stack_size(), 1); // net not consumed
        assert!(top_is("a:b | c:d"));
    }

    // pairs to file: cyclic refuses (message, no file); acyclic writes
    // upper\tlower per pair. Net preserved.
    // [spec:foma:sem:iface.iface-pairs-file-fn/test]
    // [spec:foma:sem:foma.iface-pairs-file-fn/test]
    #[test]
    fn pairs_file_writes_pairs_and_refuses_cyclic() {
        let dir = std::env::temp_dir();
        let pf = dir.join("foma_s2_pairs.txt");
        let pc = dir.join("foma_s2_pairs_cyclic.txt");
        let _ = std::fs::remove_file(&pc);
        stack_init();
        iface_pairs_file(pf.to_str().unwrap()); // empty: refusal
        assert_eq!(stack_size(), 0);
        push("a:b");
        iface_pairs_file(pf.to_str().unwrap());
        assert_eq!(stack_size(), 1);
        assert_eq!(std::fs::read_to_string(&pf).unwrap(), "a\tb\n");
        let _ = stack_pop();
        push_topsorted("[a:b]*"); // cyclic transducer → refuses, no file
        iface_pairs_file(pc.to_str().unwrap());
        assert_eq!(stack_size(), 1);
        assert!(!std::path::Path::new(&pc).exists());
    }

    // split string / split result: extract the upper side of the pair encoding
    // (space=1, epsilon=2, separator=3); split_result additionally reverses to
    // pull the lower side. Doc example: a:b c:d e 0:g → ace,bdeg.
    // [spec:foma:sem:iface.iface-split-string-fn/test]
    // [spec:foma:sem:foma.iface-split-string-fn/test]
    // [spec:foma:sem:iface.iface-split-result-fn/test]
    // [spec:foma:sem:foma.iface-split-result-fn/test]
    #[test]
    fn split_string_and_result_extract_sides() {
        // Encoding of [a:b, c:d, e (identity), 0:g (epsilon:g)]:
        //   a SEP b SPACE c SEP d SPACE e SPACE EPS SEP g
        let input: Vec<u8> = vec![b'a', 3, b'b', 1, b'c', 3, b'd', 1, b'e', 1, 2, 3, b'g'];
        let mut upper = Vec::new();
        iface_split_string(&input, &mut upper);
        assert_eq!(upper, b"ace");

        let mut result = input.clone();
        let mut upper = Vec::new();
        let mut lower = Vec::new();
        iface_split_result(&mut result, &mut upper, &mut lower);
        assert_eq!(upper, b"ace");
        assert_eq!(lower, b"bdeg");
        // split_result restores `result` to its original bytes (double reverse).
        assert_eq!(result, input);
    }

    // write att / write prolog: watt returns 1 on an empty stack, else writes an
    // AT&T/prolog file (net preserved). Round-trip through the matching io reader.
    // [spec:foma:sem:iface.iface-write-att-fn/test]
    // [spec:foma:sem:foma.iface-write-att-fn/test]
    // [spec:foma:sem:iface.iface-write-prolog-fn/test]
    // [spec:foma:sem:foma.iface-write-prolog-fn/test]
    #[test]
    fn write_att_and_prolog_roundtrip() {
        let dir = std::env::temp_dir();
        let attp = dir.join("foma_s2_write.att");
        let att = attp.to_str().unwrap();
        let plp = dir.join("foma_s2_write.prolog");
        let pl = plp.to_str().unwrap();

        stack_init();
        assert_eq!(iface_write_att(Some(att)), 1); // empty: returns 1

        push("a b");
        assert_eq!(iface_write_att(Some(att)), 0);
        assert_eq!(stack_size(), 1); // net not consumed
        let back = read_att(att).unwrap();
        let expected = fsm_parse_regex("a b", None, None).unwrap();
        assert_eq!(fsm_equivalent(back, expected), 1);

        iface_write_prolog(Some(pl));
        assert_eq!(stack_size(), 1);
        let back = fsm_read_prolog(pl).unwrap();
        let expected = fsm_parse_regex("a b", None, None).unwrap();
        assert_eq!(fsm_equivalent(back, expected), 1);
    }

    // sigptr (static): the three reserved arc labels map to 0/?/@; a matched
    // symbol is returned verbatim except the quoted "0"/"?" and escaped \n/\r
    // special cases; an unknown number yields NONE(n).
    // [spec:foma:sem:iface.sigptr-fn/test]
    #[test]
    fn sigptr_maps_reserved_and_special_symbols() {
        let sig = Sigma {
            number: 3,
            symbol: Some("0".to_string()),
            next: Some(Box::new(Sigma {
                number: 4,
                symbol: Some("?".to_string()),
                next: Some(Box::new(Sigma {
                    number: 5,
                    symbol: Some("\n".to_string()),
                    next: Some(Box::new(Sigma {
                        number: 6,
                        symbol: Some("\r".to_string()),
                        next: Some(Box::new(Sigma {
                            number: 7,
                            symbol: Some("hello".to_string()),
                            next: None,
                        })),
                    })),
                })),
            })),
        };
        assert_eq!(sigptr(Some(&sig), EPSILON), "0");
        assert_eq!(sigptr(Some(&sig), UNKNOWN), "?");
        assert_eq!(sigptr(Some(&sig), IDENTITY), "@");
        assert_eq!(sigptr(Some(&sig), 3), "\"0\"");
        assert_eq!(sigptr(Some(&sig), 4), "\"?\"");
        assert_eq!(sigptr(Some(&sig), 5), "\\n");
        assert_eq!(sigptr(Some(&sig), 6), "\\r");
        assert_eq!(sigptr(Some(&sig), 7), "hello");
        assert_eq!(sigptr(Some(&sig), 99), "NONE(99)");
    }

    // print_sigma (static): writes "Sigma:" + the >2 symbols + "@"/"?" then a
    // "Size:" line, and returns 1. Byte-exact on the sigma of "a b".
    // [spec:foma:sem:iface.print-sigma-fn/test]
    #[test]
    fn print_sigma_static_formats_alphabet() {
        let net = fsm_parse_regex("a b", None, None).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        assert_eq!(print_sigma(net.sigma.as_deref(), &mut buf), 1);
        let s = String::from_utf8(buf).unwrap();
        assert!(s.starts_with("Sigma:"), "got {:?}", s);
        assert!(s.contains(" a"));
        assert!(s.contains(" b"));
        assert!(s.trim_end().ends_with("Size: 2."), "got {:?}", s);
    }

    // print_stats (static) + print_mem_size (static): print_stats drives
    // print_mem_size and returns 0.
    // [spec:foma:sem:iface.print-stats-fn/test]
    // [spec:foma:sem:foma.print-stats-fn/test]
    // [spec:foma:sem:iface.print-mem-size-fn/test]
    #[test]
    fn print_stats_static_returns_zero() {
        let mut net = fsm_parse_regex("a b | c", None, None).unwrap();
        fsm_count(&mut net);
        assert_eq!(print_stats(&net), 0);
        // exercise print_mem_size directly as well (side-effect only).
        print_mem_size(&net);
    }

    // print_net (static): writes the Sigma/Net/Flags/Arity/arc dump to stdout
    // (None) or a file (Some), returns 0, and does not touch the stack.
    // [spec:foma:sem:iface.print-net-fn/test]
    #[test]
    fn print_net_static_writes_dump() {
        let mut net = fsm_parse_regex("a b", None, None).unwrap();
        assert_eq!(print_net(&mut net, None), 0); // to stdout
        let p = std::env::temp_dir().join("foma_s2_printnet.txt");
        let mut net = fsm_parse_regex("a b", None, None).unwrap();
        assert_eq!(print_net(&mut net, Some(p.to_str().unwrap())), 0);
        let s = std::fs::read_to_string(&p).unwrap();
        assert!(s.contains("Sigma:"));
        assert!(s.contains("Net:"));
        assert!(s.contains("Arity:"));
    }

    // print_dot (static): writes a Graphviz digraph and returns 1.
    // [spec:foma:sem:iface.print-dot-fn/test]
    #[test]
    fn print_dot_static_writes_digraph() {
        let mut net = fsm_parse_regex("a b", None, None).unwrap();
        assert_eq!(print_dot(&mut net, None), 1); // to stdout
        let p = std::env::temp_dir().join("foma_s2_printdot.dot");
        let mut net = fsm_parse_regex("a b", None, None).unwrap();
        assert_eq!(print_dot(&mut net, Some(p.to_str().unwrap())), 1);
        let s = std::fs::read_to_string(&p).unwrap();
        assert!(s.starts_with("digraph A {"), "got {:?}", &s[..s.len().min(20)]);
        assert!(s.trim_end().ends_with("}"));
    }
}

# foma → Rust porting conventions (Wave 2: literal, bug-for-bug)

These conventions bind every Wave-2 translation. The goal is a 1:1 port:
a reviewer diffing a C function against its Rust twin must see a
line-by-line correspondence. Idiom is Wave 4's job; fixes land after the
port is green. **Reproduce bugs faithfully** (off-by-ones, overflow,
signed-char hashing, order of side effects) unless memory-unsafe to do
so — see "Deviations" below.

## Layout

- One crate: `crates/foma`. One Rust module per C file:
  `foma/structures.c` → `crates/foma/src/structures.rs`, etc.
- Types declared in the C headers (`foma.h`, `fomalib.h`,
  `fomalibconf.h`, `lexc.h`) live in `crates/foma/src/types.rs`.
  Types declared inside a `.c` file (e.g. determinize's nhash tables)
  live in that file's module.
- CLI binaries: `src/bin/foma.rs`, `src/bin/flookup.rs`,
  `src/bin/cgflookup.rs` (w2-cli concern).

## Type mappings (match C widths exactly — sem rules document truncation)

| C | Rust |
|---|---|
| `int` | `i32` |
| `unsigned int` | `u32` |
| `short int` | `i16` |
| `unsigned short` | `u16` |
| `char` (numeric/flag) | `i8` (or `u8` when the C treats it as a byte) |
| `long long` | `i64` |
| `size_t` | `usize` |
| `_Bool` | `bool` (watch documented `_Bool` truncation quirks: reproduce via `!= 0` on the same expression the C truncates) |
| `char *` (owned string) | `String` / `Option<String>` |
| `char *` (byte buffer) | `Vec<u8>` |
| `char name[40]` | `String` capped at 40 bytes; reproduce the "no NUL when ≥ 40" quirk as truncation to 40 bytes |

Hashing/char arithmetic: where a sem rule documents signed-char
sign-extension (`sh_hashf`, `trie_hashf`, `lexc_symbol_hash`,
`fsm_construct_hash_sym`), iterate `s.as_bytes()` and cast each byte
`as i8 as i32` to reproduce it. Where a rule documents wrapping overflow
(e.g. `triplethash_hashf`), use `wrapping_mul`/`wrapping_add` on `i32`.

## Memory model

- Malloc'd arrays → `Vec<T>`. Keep the C's explicit capacity/length
  bookkeeping fields when the algorithm reads them (don't substitute
  `vec.len()` where the C tracks a separate counter — that's idiom).
- The fsm line table stays a sentinel-terminated `Vec<FsmState>` with a
  final `state_no == -1` sentinel line, exactly as in C. Iteration is by
  index, mirroring pointer walks.
- Owned singly/doubly linked lists → `Option<Box<Node>>` chains with the
  same insert/delete order. Where the C keeps a dummy head, keep it.
- Handles (`apply_handle`, `fsm_construct_handle`, …) → owned structs
  passed `&mut`.
- C functions that *consume* (free) their `struct fsm *` arguments take
  `Box<Fsm>` by value; functions that borrow take `&Fsm`/`&mut Fsm`.
  The per-function sem rule states which convention each function uses —
  follow it exactly.
- File-static mutable globals → module-level
  `thread_local! { static NAME: RefCell<T> = ... }`. Keep the C names
  (upper-cased). Non-reentrancy is part of the contract; do not
  redesign into handle-passing where the C didn't (Wave 4).

## Control flow

- Keep loop shapes, early returns, and statement order. `goto` →
  labelled `loop`/`break 'label` with the same targets.
- Keep function names (already snake_case) and parameter names.
- No traits, no iterator chains, no combining functions. One C function
  = one Rust `pub fn` (file-statics become `pub(crate) fn`).

## Deviations (memory-unsafe C behavior)

Safe Rust cannot reproduce use-after-free / double-free / OOB reads.
For each such documented bug (they are flagged in the sem rules):
reproduce the *observable* result the C exhibits in practice where
possible; otherwise implement the nearest safe behavior, add
`// DEVIATION from C (<one line why>)` at the site, and note it in the
concern's commit message. Never silently "fix" logic bugs that are
memory-safe (e.g. `flag_eliminate`'s `|`-for-`&` filter — port it as-is).

## Stubs for not-yet-ported callees

Concerns land in dependency order, but the C has call cycles across
files. When your module calls a function whose concern hasn't landed,
add `pub fn name(...) -> ... { todo!("ported by <concern-id>") }` in
that function's *home module* (create the module file if needed) —
WITHOUT spec annotations. The owning concern replaces the stub and adds
the annotations. `cargo check` must pass after every concern.

## Annotations (what the Wave-2 gate counts)

Above every ported item, carry its manifest ids as line comments:

```rust
// [spec:foma:def:structures.fsm-create-fn]
// [spec:foma:sem:structures.fsm-create-fn]
pub fn fsm_create(name: &str) -> Box<Fsm> { ... }
```

Header prototypes got their own rule ids (they did NOT dedup with the
per-file impl ids). A function declared in a header carries BOTH id
families at its single Rust site, e.g. `apply_init` carries
`apply.apply-init-fn` (def+sem) AND `fomalib.apply-init-fn` (def+sem).
Before annotating, grep `docs/spec/port/foma/{fomalib,foma,fomalibconf,lexc}.md`
for the function's name to find its header-layer ids. Types from headers
carry the header id (e.g. `fomalib.fsm`) plus any duplicate per-file id.

## Verification per concern

1. `cargo check` clean (warnings acceptable in Wave 2 if C-faithful).
2. Every symbol in the concern shows `tgt_impl: true` in
   `nplan port status` (annotation present in target source).
3. No Wave-4 idiom crept in.

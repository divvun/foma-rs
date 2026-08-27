//! foma/regex.y + regex.l (the regex compiler).
//!
//! Wave-2 wiring rather than literal translation: foma's flex/bison grammar is
//! replaced by the `nfst-xre` parser, which produces a typed `XreExpr` AST. We
//! walk that AST and call the same construction functions the C grammar's
//! semantic actions would, so the OBSERVABLE net for a given regex matches the
//! C compositions bug-for-bug (see docs/port/rust-conventions.md).
//!
//! Mapping authority is foma/regex.y (production -> construction call) and
//! foma/regex.l (symbol handling, defined-symbol substitution, @"file" loads,
//! NAME(args) function application). Where nfst-xre's AST covers syntax the C
//! grammar lacks (merge ops, weights, .-u./.-l., @pl"…"), we return None with a
//! diagnostic. Where the C grammar covers syntax nfst-xre cannot lex (the
//! `_foo(` internal builtins, quantifiers ∀/∃, VAR logic, right/interleave
//! quotients, `.f` flag-eliminate, two-level `|||` replace), those simply never
//! reach us as AST nodes (they lex/parse to something else or error out).

use crate::options::FomaOptions;

use nfst_xre::{
    BinaryOp, ContextMark, MappingKind, MappingPair, MappingSide, ReadKind, ReplaceArrow,
    ReplaceRule, RestrContext, SpannedXre, SubstituteWhat, UnaryOp, XreExpr,
};

use crate::constructions::{
    fsm_add_loop, fsm_add_sink, fsm_close_sigma, fsm_complement, fsm_compose, fsm_concat,
    fsm_concat_m_n, fsm_concat_n, fsm_contains, fsm_contains_one, fsm_contains_opt_one,
    fsm_context_restrict, fsm_cross_product, fsm_equal_substrings, fsm_flatten, fsm_follows,
    fsm_ignore, fsm_intersect, fsm_invert, fsm_kleene_plus, fsm_kleene_star, fsm_left_rewr,
    fsm_lenient_compose, fsm_letter_machine, fsm_mark_fsm_tail, fsm_minus, fsm_network_to_char,
    fsm_optionality, fsm_precedes, fsm_priority_union_lower, fsm_priority_union_upper,
    fsm_quotient_left, fsm_shuffle, fsm_substitute_label, fsm_substitute_symbol, fsm_symbol,
    fsm_term_negation, fsm_union, fsm_universal,
};
use crate::define::{add_defined, find_defined, find_defined_function, remove_defined};
use crate::determinize::fsm_determinize;
use crate::extract::{fsm_lower, fsm_upper};
use crate::io::{file_to_mem, fsm_read_binary_file, fsm_read_spaced_text_file, fsm_read_text_file};
use crate::minimize::fsm_minimize;
use crate::reverse::fsm_reverse;
use crate::rewrite::fsm_rewrite;
use crate::structures::{
    Quantifiers, add_quantifier, count_quantifiers, find_quantifier, fsm_boolean, fsm_copy,
    fsm_destroy, fsm_empty_string, fsm_extract_ambiguous, fsm_extract_ambiguous_domain,
    fsm_extract_nonidentity, fsm_extract_unambiguous, fsm_identity, fsm_isempty, fsm_isfunctional,
    fsm_isidentity, fsm_isunambiguous, fsm_logical_eq, fsm_logical_precedence, fsm_lowerdet,
    fsm_lowerdeteps, fsm_markallfinal, fsm_quantifier, purge_quantifier, union_quantifiers,
};
use crate::trie::{
    THASH_TABLESIZE, fsm_trie_done, fsm_trie_end_word, fsm_trie_init_sized, fsm_trie_symbol,
};
use crate::types::{
    ArrowType, DefinedFunctions, DefinedNetworks, Fsm, Fsmcontexts, Fsmrules, OP_IGNORE_ALL,
    OP_IGNORE_INTERNAL, ReplaceDir, RewriteSet,
};
use crate::utf8::replace_equal_len;
use smol_str::SmolStr;

/* C: `#define MAX_PARSE_DEPTH 100` — the self-recursion guard for my_yyparse. */
const MAX_PARSE_DEPTH: i32 = 100;

/// The parse-scoped state that C kept in the file-static `g_parse_depth`
/// (regex.l, the self-recursion guard) and `g_internal_sym` (regex.y, the
/// running counter for the unique temporary symbol names function application
/// synthesizes). Both survive the nested `my_yyparse` reparse a function
/// application triggers, so one `&mut ParseState` is threaded through the whole
/// recursive walk. A fresh `ParseState` is created for each top-level parse.
struct ParseState {
    /* C: `int g_parse_depth = 0;` */
    depth: i32,
    /* C: `unsigned int g_internal_sym = 23482342;` */
    internal_sym: u32,
    /* C kept the bound first-order variables in a file-static list that the
    LEXER consulted: an identifier returned VAR (not NET) iff find_quantifier
    matched, and `=` returned EQUALS only while count_quantifiers() > 0.
    nfst-xre is a context-free lexer and cannot make that call, so the table
    lives here and the classification happens during the tree walk — a binder
    is registered before its body is built, which reproduces the scope the C
    lexer produced for well-formed input. */
    quantifiers: Quantifiers,
}

impl ParseState {
    fn new() -> ParseState {
        ParseState {
            quantifiers: Quantifiers::default(),
            depth: 0,
            internal_sym: 23482342,
        }
    }
}

// [spec:foma:def:fomalib.fsm-parse-regex-fn]
// [spec:foma:sem:fomalib.fsm-parse-regex-fn]
pub fn fsm_parse_regex(
    opts: &FomaOptions,
    regex: &str,
    defined_nets: Option<&mut DefinedNetworks>,
    defined_funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    /* C: strcpy a copy of `regex` with ";" appended, my_yyparse it at line 1,
    and on success return fsm_minimize(opts, current_parse). nfst-xre tolerates the
    optional trailing ";" itself, so no copy is needed. */
    let mut ps = ParseState::new();
    let current_parse = my_yyparse(opts, &mut ps, regex, defined_nets, defined_funcs)?;
    Some(fsm_minimize(opts, current_parse))
}

// [spec:foma:def:foma.my-yyparse-fn]
// [spec:foma:sem:foma.my-yyparse-fn]
fn my_yyparse(
    opts: &FomaOptions,
    ps: &mut ParseState,
    regex: &str,
    defined_nets: Option<&mut DefinedNetworks>,
    defined_funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    /* C: depth-limited reentrant driver. The C also saves/restores the global
    parser state (rewrite/contexts/rules/rewrite_rules) around the nested
    yyparse; this port builds those structures locally on the stack, so only
    the depth guard is observable. Returns the net the parse deposits in
    current_parse (unminimized — fsm_parse_regex/@re do the minimize). */
    if ps.depth >= MAX_PARSE_DEPTH {
        tracing::error!("Exceeded parser stack depth.  Self-recursive call?");
        return None;
    }
    ps.depth += 1;
    let result = my_yyparse_inner(opts, ps, regex, defined_nets, defined_funcs);
    ps.depth -= 1;
    result
}

fn my_yyparse_inner(
    opts: &FomaOptions,
    ps: &mut ParseState,
    regex: &str,
    defined_nets: Option<&mut DefinedNetworks>,
    defined_funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    let exprs = match nfst_xre::parse_all(regex) {
        Ok(e) => e,
        Err(e) => {
            /* C's my_yyparse returns non-zero on a syntax error; yyerror has
            already printed a "***...at '...'" diagnostic. */
            let msg = e
                .diagnostics
                .first()
                .map(|d| d.message.clone())
                .unwrap_or_else(|| "syntax error".to_string());
            tracing::error!("Syntax error: {}", msg);
            return None;
        }
    };
    /* C grammar: `start: regex | regex start` with `regex: network END
    { current_parse = $1; }` — current_parse ends up as the LAST network
    parsed. */
    let last = match exprs.last() {
        Some(e) => e,
        None => {
            tracing::error!("Syntax error: empty regular expression");
            return None;
        }
    };
    build_net(opts, ps, &last.value, defined_nets, defined_funcs)
}

/// Walk one AST node to a network, mirroring the regex.y semantic action for
/// the corresponding production.
fn build_net(
    opts: &FomaOptions,
    ps: &mut ParseState,
    expr: &XreExpr,
    mut nets: Option<&mut DefinedNetworks>,
    mut funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    match expr {
        // ──────────────── atoms ────────────────
        XreExpr::Symbol(s) => {
            /* regex.l NONRESERVED path: substitute a defined net when the
            symbol names one; otherwise it is a literal single symbol. (`0`
            and `?` arrive as Epsilon/Any; a Symbol("0")/Symbol("?") means the
            user escaped it as %0/%?, so it is NOT special-cased here.) */
            if let Some(n) = nets.as_deref_mut() {
                if let Some(found) = find_defined(n, s) {
                    return Some(fsm_copy(found));
                }
            }
            Some(fsm_symbol(s))
        }
        XreExpr::Curly(s) => {
            /* regex.l {BRACED}: nfst-xre delivers the braces' interior, which
            is exactly fsm_explode's payload */
            Some(crate::constructions::fsm_explode(s))
        }
        XreExpr::Epsilon => Some(fsm_empty_string()),
        XreExpr::Any => Some(fsm_identity()),
        XreExpr::BoundaryMarker => Some(fsm_symbol(".#.")),

        // ──────────────── label combinators ────────────────
        XreExpr::Pair { upper, lower } => {
            /* `:` HIGH_CROSS_PRODUCT: fsm_cross_product(upper, lower). */
            let u = build_net(
                opts,
                ps,
                &upper.value,
                nets.as_deref_mut(),
                funcs.as_deref_mut(),
            )?;
            let l = build_net(
                opts,
                ps,
                &lower.value,
                nets.as_deref_mut(),
                funcs.as_deref_mut(),
            )?;
            Some(fsm_cross_product(opts, u, l))
        }
        XreExpr::Weighted { .. } => {
            tracing::error!("Syntax error: weights (::w) are not supported");
            None
        }
        XreExpr::ContainmentWithWeight { .. } => {
            tracing::error!("Syntax error: weighted containment ($::w) is not supported");
            None
        }

        XreExpr::ReadFile { kind, path } => build_read_file(opts, ps, *kind, path, nets, funcs),

        XreExpr::FunctionCall { name, args } => match name.strip_prefix('_') {
            /* `_` is not a NAME_CH, so a user-defined function can never carry
            this prefix: every `_xxx(` is one of regex.l's builtin keywords. */
            Some(builtin) => build_builtin(opts, ps, builtin, args, nets, funcs),
            None => function_apply(opts, ps, name, args, nets, funcs),
        },

        // ──────────────── grouping ────────────────
        XreExpr::Group(inner) => build_net(opts, ps, &inner.value, nets, funcs),
        XreExpr::Optional(inner) => {
            /* regex.y LPAREN network RPAREN:
                 if (count_quantifiers()) $$ = $2; else $$ = fsm_optionality($2);
            inside a quantified formula the parens are plain grouping. */
            let n = build_net(opts, ps, &inner.value, nets, funcs)?;
            if count_quantifiers(&ps.quantifiers) > 0 {
                Some(n)
            } else {
                Some(fsm_optionality(opts, n))
            }
        }
        XreExpr::BracketedDotted(_) => {
            /* `[. E .]` outside a replacement mapping is a syntax error in the C
            grammar (LDOT/RDOT only appear inside rule productions). */
            tracing::error!("Syntax error: [. .] is only valid as a replacement mapping side");
            None
        }

        // ──────────────── unary ────────────────
        XreExpr::Unary(op, inner) => build_unary(opts, ps, *op, &inner.value, nets, funcs),

        // ──────────────── binary ────────────────
        /* A concatenation spine may be a first-order formula rather than a
        plain concatenation (regex.y network5/network7). Try that reading
        first; nothing in it fires without a binder in scope. */
        XreExpr::Binary(BinaryOp::Concatenate, _, _) => {
            let mut items = Vec::new();
            concat_spine(expr, &mut items);
            build_items(opts, ps, &items, nets, funcs)
        }
        XreExpr::Binary(op, l, r) => {
            /* Fast path: a union of literal strings compiles straight to a
            trie/DAWG, skipping the O(n^2) pairwise-union fold and the
            determinization of the resulting epsilon-laden NFA. */
            if *op == BinaryOp::Union {
                let mut words = Vec::new();
                if collect_union_words(expr, &mut words) && words.len() >= 2 {
                    let has_defined = match nets.as_deref_mut() {
                        Some(n) => words.iter().flatten().any(|s| find_defined(n, s).is_some()),
                        None => false,
                    };
                    if !has_defined {
                        return Some(build_dawg(&words));
                    }
                }
            }
            build_binary(opts, ps, *op, &l.value, &r.value, nets, funcs)
        }

        // ──────────────── iteration ────────────────
        XreExpr::RepeatN(inner, n) => {
            /* NCONCAT: fsm_concat_n(net, n). */
            let net = build_net(opts, ps, &inner.value, nets, funcs)?;
            Some(fsm_concat_n(opts, net, *n as i32))
        }
        XreExpr::RepeatNPlus(inner, n) => {
            /* MORENCONCAT (`^>N`): concat(concat_n(copy,n), kleene_plus(copy)). */
            let mut net = build_net(opts, ps, &inner.value, nets, funcs)?;
            let res = fsm_concat(
                opts,
                fsm_concat_n(opts, fsm_copy(&mut net), *n as i32),
                fsm_kleene_plus(opts, fsm_copy(&mut net)),
            );
            fsm_destroy(net);
            Some(res)
        }
        XreExpr::RepeatNMinus(inner, n) => {
            /* LESSNCONCAT (`^<N`): fsm_concat_m_n(net, 0, n-1). */
            let net = build_net(opts, ps, &inner.value, nets, funcs)?;
            Some(fsm_concat_m_n(opts, net, 0, *n as i32 - 1))
        }
        XreExpr::RepeatNToK(inner, n, k) => {
            /* MNCONCAT (`^N,K`): fsm_concat_m_n(net, n, k). */
            let net = build_net(opts, ps, &inner.value, nets, funcs)?;
            Some(fsm_concat_m_n(opts, net, *n as i32, *k as i32))
        }

        // ──────────────── replace / restriction / substitute ────────────────
        XreExpr::Replace { rules, .. } => build_replace(opts, ps, rules, nets, funcs),
        XreExpr::Restriction { body, contexts } => {
            build_restriction(opts, ps, &body.value, contexts, nets, funcs)
        }
        XreExpr::Substitute { haystack, what } => {
            build_substitute(opts, ps, &haystack.value, what, nets, funcs)
        }
    }
}

/// Flatten a left-nested concatenation spine into its operands, left to right.
fn concat_spine<'a>(expr: &'a XreExpr, out: &mut Vec<&'a XreExpr>) {
    if let XreExpr::Binary(BinaryOp::Concatenate, l, r) = expr {
        concat_spine(&l.value, out);
        concat_spine(&r.value, out);
    } else {
        out.push(expr);
    }
}

/// Strip any unary operators off `expr`, returning them outermost-first along
/// with the operand. `~(\u{2203}y)` parses as Complement applied to the binder alone,
/// so the operators have to be lifted over the quantification they really
/// scope over.
fn peel_unary(expr: &XreExpr) -> (Vec<UnaryOp>, &XreExpr) {
    let mut ops = Vec::new();
    let mut cur = expr;
    while let XreExpr::Unary(op, inner) = cur {
        ops.push(*op);
        cur = &inner.value;
    }
    (ops, cur)
}

/// A `(\u{2200}x)` / `(\u{2203}x)` binder. nfst-xre lexes `\u{2200}x` as a single Symbol
/// (both are NAME_CH) and the parens as `Optional`, so a binder arrives as
/// `Optional(Symbol("\u{2200}x"))`. Returns (is_universal, variable name).
fn binder_of(expr: &XreExpr) -> Option<(bool, &str)> {
    let XreExpr::Optional(inner) = expr else {
        return None;
    };
    let XreExpr::Symbol(s) = &inner.value else {
        return None;
    };
    let mut cs = s.chars();
    let universal = match cs.next() {
        Some('\u{2200}') => true,
        Some('\u{2203}') => false,
        _ => return None,
    };
    let name = cs.as_str();
    (!name.is_empty()).then_some((universal, name))
}

/// The name of a currently-bound first-order variable, if `expr` is one. This
/// is the tree-walk equivalent of the C lexer returning VAR rather than NET
/// when `find_quantifier` matched.
fn bound_var<'a>(ps: &ParseState, expr: &'a XreExpr) -> Option<&'a str> {
    let XreExpr::Symbol(s) = expr else {
        return None;
    };
    find_quantifier(&ps.quantifiers, s).map(|_| s.as_str())
}

/// Build a concatenation spine: as a first-order formula when it is one,
/// otherwise as a plain concatenation.
fn build_items(
    opts: &FomaOptions,
    ps: &mut ParseState,
    items: &[&XreExpr],
    mut nets: Option<&mut DefinedNetworks>,
    mut funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    if let Some(r) = build_logic_spine(opts, ps, items, nets.as_deref_mut(), funcs.as_deref_mut()) {
        return r;
    }
    let mut acc: Option<Fsm> = None;
    for item in items {
        match build_net(opts, ps, item, nets.as_deref_mut(), funcs.as_deref_mut()) {
            Some(n) => {
                acc = Some(match acc {
                    Some(a) => fsm_concat(opts, a, n),
                    None => n,
                })
            }
            None => {
                if let Some(a) = acc {
                    fsm_destroy(a);
                }
                return None;
            }
        }
    }
    Some(acc.unwrap_or_else(fsm_empty_string))
}

/// foma's first-order-logic sublanguage, which C resolved in the lexer through
/// the live quantifier table. `None` means "not a formula" and the caller falls
/// back to plain concatenation; nothing here fires without a binder in scope.
fn build_logic_spine(
    opts: &FomaOptions,
    ps: &mut ParseState,
    items: &[&XreExpr],
    nets: Option<&mut DefinedNetworks>,
    funcs: Option<&mut DefinedFunctions>,
) -> Option<Option<Fsm>> {
    if items.is_empty() {
        return None;
    }

    /* A leading binder scopes over the whole rest of the spine. regex.y:
      UQUANT LPAREN network RPAREN
        -> ~[[Q(x) & ~F] with x -> 0]
      EQUANT network
        -> [Q(x) & F] with x -> 0
    then purge_quantifier(x). Q(x) = \x* x \x* x \x* pins the variable's
    two position markers; substituting them away projects the formula back
    onto the object alphabet. Unary operators written before the binder
    (`~(∃y)…`) actually scope over the quantification, so lift them. */
    let (ops, head) = peel_unary(items[0]);
    if let Some((universal, name)) = binder_of(head) {
        let name = name.to_string();
        add_quantifier(&mut ps.quantifiers, &name);
        let body = build_items(opts, ps, &items[1..], nets, funcs);
        let q = fsm_quantifier(opts, &name);
        purge_quantifier(&mut ps.quantifiers, &name);
        let Some(body) = body else {
            fsm_destroy(q);
            return Some(None);
        };
        let inner = if universal {
            fsm_intersect(opts, q, fsm_complement(opts, body))
        } else {
            fsm_intersect(opts, q, body)
        };
        let projected = fsm_substitute_symbol(inner, &name, "@_EPSILON_SYMBOL_@");
        let mut out = if universal {
            fsm_complement(opts, projected)
        } else {
            projected
        };
        for op in ops.into_iter().rev() {
            out = apply_unary(opts, op, out);
        }
        return Some(Some(out));
    }

    /* The infix relations exist only while a variable is bound. */
    if count_quantifiers(&ps.quantifiers) == 0 || items.len() < 3 {
        return None;
    }

    /* regex.y `VAR IN network5`: [$[x N x]] / union_quantifiers. */
    if let (Some(v), XreExpr::Symbol(op)) = (bound_var(ps, items[0]), items[1])
        && op == "\u{2208}"
    {
        let v = v.to_string();
        let rest = build_items(opts, ps, &items[2..], nets, funcs)?;
        let bracketed = fsm_concat(opts, fsm_symbol(&v), fsm_concat(opts, rest, fsm_symbol(&v)));
        return Some(Some(fsm_ignore(
            opts,
            fsm_contains(opts, bracketed),
            union_quantifiers(&ps.quantifiers),
            OP_IGNORE_ALL,
        )));
    }

    /* The variable-to-variable relations are exactly three operands wide. */
    if items.len() != 3 {
        return None;
    }
    let (Some(v1), Some(v2)) = (bound_var(ps, items[0]), bound_var(ps, items[2])) else {
        return None;
    };
    let XreExpr::Symbol(op) = items[1] else {
        return None;
    };
    let (v1, v2) = (v1.to_string(), v2.to_string());
    Some(Some(match op.as_str() {
        "=" => fsm_logical_eq(opts, &ps.quantifiers, &v1, &v2),
        "\u{2260}" => fsm_complement(opts, fsm_logical_eq(opts, &ps.quantifiers, &v1, &v2)),
        "\u{227A}" => fsm_logical_precedence(opts, &ps.quantifiers, &v1, &v2),
        /* x \u{227B} y is precedence with the operands swapped */
        "\u{227B}" => fsm_logical_precedence(opts, &ps.quantifiers, &v2, &v1),
        _ => return None,
    }))
}

/// The unary operators, split out of `build_unary` so the logic layer can
/// re-apply operators it had to lift over a quantifier.
fn apply_unary(opts: &FomaOptions, op: UnaryOp, net: Fsm) -> Fsm {
    match op {
        /* network9 KLEENE_STAR: fsm_kleene_star(fsm_minimize(net)) */
        UnaryOp::Star => fsm_kleene_star(opts, fsm_minimize(opts, net)),
        UnaryOp::Plus => fsm_kleene_plus(opts, net),
        /* network9 REVERSE: fsm_determinize(fsm_reverse(net)) */
        UnaryOp::Reverse => fsm_determinize(fsm_reverse(net)),
        UnaryOp::Invert => fsm_invert(net),
        UnaryOp::UpperProject => fsm_upper(net),
        UnaryOp::LowerProject => fsm_lower(net),
        UnaryOp::Complement => fsm_complement(opts, net),
        UnaryOp::TermComplement => fsm_term_negation(opts, net),
        UnaryOp::Containment => fsm_contains(opts, net),
        UnaryOp::ContainmentOnce => fsm_contains_one(opts, net),
        UnaryOp::ContainmentOpt => fsm_contains_opt_one(opts, net),
    }
}

fn build_unary(
    opts: &FomaOptions,
    ps: &mut ParseState,
    op: UnaryOp,
    inner: &XreExpr,
    nets: Option<&mut DefinedNetworks>,
    funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    let net = build_net(opts, ps, inner, nets, funcs)?;
    Some(apply_unary(opts, op, net))
}

/// Collect the branch words of a union-of-strings AST. Returns true with the
/// words filled if `expr` is a union whose every leaf is a plain concatenation
/// of literal `Symbol`s; false if any leaf has other structure, in which case
/// the caller falls back to the general union.
fn collect_union_words(expr: &XreExpr, words: &mut Vec<Vec<SmolStr>>) -> bool {
    match expr {
        XreExpr::Binary(BinaryOp::Union, l, r) => {
            collect_union_words(&l.value, words) && collect_union_words(&r.value, words)
        }
        XreExpr::Group(inner) => collect_union_words(&inner.value, words),
        _ => {
            let mut w = Vec::new();
            if word_symbols(expr, &mut w) && !w.is_empty() {
                words.push(w);
                true
            } else {
                false
            }
        }
    }
}

/// The symbols of a single string branch, or false if `expr` is anything other
/// than a concatenation of literal `Symbol`s (an epsilon, `?`, pair, star,
/// nested operator, …), which the general union path handles instead.
fn word_symbols(expr: &XreExpr, out: &mut Vec<SmolStr>) -> bool {
    match expr {
        XreExpr::Symbol(s) => {
            out.push(s.clone());
            true
        }
        XreExpr::Binary(BinaryOp::Concatenate, l, r) => {
            word_symbols(&l.value, out) && word_symbols(&r.value, out)
        }
        XreExpr::Group(inner) => word_symbols(&inner.value, out),
        _ => false,
    }
}

/// Build the automaton for a set of literal words as a trie; the caller's final
/// `fsm_minimize` collapses it to the minimal DAWG (the trie is already
/// deterministic, so minimization skips determinization).
fn build_dawg(words: &[Vec<SmolStr>]) -> Fsm {
    /* Size the trie hash to the arc count (chaining absorbs the load) rather
    than pay fsm_trie_init's 1M-bucket default, whose zero-fill would dwarf the
    build for a modest word set. The result is minimized, so the table size is
    invisible in the output. */
    let total: usize = words.iter().map(Vec::len).sum();
    let size = total
        .saturating_mul(2)
        .clamp(1024, THASH_TABLESIZE as usize) as u32;
    let mut th = fsm_trie_init_sized(size);
    for word in words {
        for sym in word {
            fsm_trie_symbol(&mut th, sym, sym);
        }
        fsm_trie_end_word(&mut th);
    }
    fsm_trie_done(th)
}

fn build_binary(
    opts: &FomaOptions,
    ps: &mut ParseState,
    op: BinaryOp,
    left: &XreExpr,
    right: &XreExpr,
    mut nets: Option<&mut DefinedNetworks>,
    mut funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    /* regex.y `VAR PRECEDES VAR` / `VAR FOLLOWS VAR`. The ASCII spellings `<`
    and `>` reach us as Before/After; between two bound variables they are the
    logical relations instead. (The Unicode spellings ≺/≻ arrive as symbols in
    a concatenation spine and are handled there.) */
    if matches!(op, BinaryOp::Before | BinaryOp::After)
        && count_quantifiers(&ps.quantifiers) > 0
        && let Some(v2) = bound_var(ps, right).map(str::to_string)
    {
        let rel = |ps: &ParseState, v1: &str, v2: &str| {
            let (lo, hi) = if matches!(op, BinaryOp::Before) {
                (v1, v2)
            } else {
                (v2, v1)
            };
            fsm_logical_precedence(opts, &ps.quantifiers, lo, hi)
        };
        if let Some(v1) = bound_var(ps, left).map(str::to_string) {
            return Some(rel(ps, &v1, &v2));
        }
        /* xre binds `<`/`>` looser than `&`, where regex.y puts the VAR
        relation at network5 — tighter. So `A & x < y` reaches us as
        `[A & x] < y`; re-associate it to `A & [x < y]`. This can only fire
        when both operands are bound variables, so ordinary network `<` is
        untouched. */
        if let XreExpr::Binary(op2, ll, lr) = left
            && matches!(
                op2,
                BinaryOp::Intersect | BinaryOp::Union | BinaryOp::Concatenate
            )
            && let Some(v1) = bound_var(ps, &lr.value).map(str::to_string)
        {
            let rest = build_net(
                opts,
                ps,
                &ll.value,
                nets.as_deref_mut(),
                funcs.as_deref_mut(),
            )?;
            let r = rel(ps, &v1, &v2);
            return Some(match op2 {
                BinaryOp::Intersect => fsm_intersect(opts, rest, r),
                BinaryOp::Union => fsm_union(opts, rest, r),
                _ => fsm_concat(opts, rest, r),
            });
        }
    }
    let l = build_net(opts, ps, left, nets.as_deref_mut(), funcs.as_deref_mut())?;
    let r = build_net(opts, ps, right, nets, funcs)?;
    match op {
        BinaryOp::Concatenate => Some(fsm_concat(opts, l, r)),
        BinaryOp::Compose => Some(fsm_compose(opts, l, r)),
        BinaryOp::LenientCompose => Some(fsm_lenient_compose(opts, l, r)),
        BinaryOp::CrossProduct => Some(fsm_cross_product(opts, l, r)),
        BinaryOp::Union => Some(fsm_union(opts, l, r)),
        BinaryOp::Intersect => Some(fsm_intersect(opts, l, r)),
        BinaryOp::Subtract => Some(fsm_minus(opts, l, r)),
        BinaryOp::UpperPriorityUnion => Some(fsm_priority_union_upper(opts, l, r)),
        BinaryOp::LowerPriorityUnion => Some(fsm_priority_union_lower(opts, l, r)),
        BinaryOp::Ignoring => Some(fsm_ignore(opts, l, r, OP_IGNORE_ALL)),
        BinaryOp::IgnoreInternally => Some(fsm_ignore(opts, l, r, OP_IGNORE_INTERNAL)),
        BinaryOp::LeftQuotient => Some(fsm_quotient_left(opts, l, r)),
        BinaryOp::Shuffle => Some(fsm_shuffle(opts, l, r)),
        /* PRECEDES/FOLLOWS borrow (do not consume) their operands. */
        BinaryOp::Before => {
            let mut l = l;
            let mut r = r;
            let res = fsm_precedes(opts, &mut l, &mut r);
            fsm_destroy(l);
            fsm_destroy(r);
            Some(res)
        }
        BinaryOp::After => {
            let mut l = l;
            let mut r = r;
            let res = fsm_follows(opts, &mut l, &mut r);
            fsm_destroy(l);
            fsm_destroy(r);
            Some(res)
        }
        /* Operators nfst-xre can lex but the foma grammar has no production
        for: merges, upper/lower minus. */
        BinaryOp::MergeRight
        | BinaryOp::MergeLeft
        | BinaryOp::UpperSubtract
        | BinaryOp::LowerSubtract => {
            tracing::error!("Syntax error: operator not supported by foma regex grammar");
            fsm_destroy(l);
            fsm_destroy(r);
            None
        }
    }
}

fn build_read_file(
    opts: &FomaOptions,
    ps: &mut ParseState,
    kind: ReadKind,
    path: &str,
    nets: Option<&mut DefinedNetworks>,
    funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    match kind {
        /* regex.l @"…"/@bin"…": fsm_read_binary_file */
        ReadKind::Binary => match fsm_read_binary_file(path).ok() {
            Some(n) => Some(n),
            None => {
                tracing::error!("Error reading binary file '{}'", path);
                None
            }
        },
        ReadKind::Text => match fsm_read_text_file(path) {
            Some(n) => Some(n),
            None => {
                tracing::error!("Error reading text file '{}'", path);
                None
            }
        },
        ReadKind::Spaced => match fsm_read_spaced_text_file(path) {
            Some(n) => Some(n),
            None => {
                tracing::error!("Error reading spaced text file '{}'", path);
                None
            }
        },
        /* regex.l @re"…": file_to_mem then fsm_parse_regex_string (parse +
        minimize). */
        ReadKind::Regex => {
            let bytes = match file_to_mem(path).ok() {
                Some(b) => b,
                None => {
                    tracing::error!("Error reading regex file '{}'", path);
                    return None;
                }
            };
            let s = match String::from_utf8(bytes) {
                Ok(s) => s,
                Err(_) => {
                    tracing::error!("Error: regex file '{}' is not valid UTF-8", path);
                    return None;
                }
            };
            Some(fsm_minimize(opts, my_yyparse(opts, ps, &s, nets, funcs)?))
        }
        ReadKind::Prolog => {
            tracing::error!("Syntax error: @pl\"…\" prolog files are not supported");
            None
        }
    }
}

// ───────────────────────── function application ─────────────────────────

/// regex.y function_apply: look up the function body regex by (name, numargs),
/// substitute each `@ARGUMENTNN@` with a unique temporary symbol, temporarily
/// define each argument net under that symbol, reparse the substituted regex,
/// then remove the temporaries.
/// The `_xxx(...)` builtin family (regex.l's hardcoded function keywords, each
/// a fixed-arity production in regex.y). Arity is checked here rather than in
/// the grammar, since nfst-xre lexes them as ordinary function names.
fn build_builtin(
    opts: &FomaOptions,
    ps: &mut ParseState,
    name: &str,
    args: &[SpannedXre],
    mut nets: Option<&mut DefinedNetworks>,
    mut funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    let arity = match name {
        "isunambiguous" | "isidentity" | "isfunctional" | "notid" | "lm" | "loweruniq"
        | "loweruniqeps" | "allfinal" | "unambpart" | "ambpart" | "ambdom" | "addsink"
        | "close" | "closeu" => 1,
        "marktail" | "addfinalloop" | "addnonfinalloop" | "addloop" | "leftrewr" | "flatten" => 2,
        "eq" | "sublabel" => 3,
        "S" => 2,
        _ => {
            tracing::error!("Syntax error: unknown builtin function _{}(", name);
            return None;
        }
    };
    if args.len() != arity {
        tracing::error!(
            "Syntax error: _{}( takes {} argument(s), got {}",
            name,
            arity,
            args.len()
        );
        return None;
    }

    /* _S( classifies each argument as a bound variable or a network before
    building anything, so it cannot use the generic argument loop below.
    regex.y ~378-380, one production per (VAR|network) combination:
        _S(v1, v2) = ?* v1 ?* v1 UQ v2 ?* v2 ?*
        _S(v1, N)  = ?* v1 ?* v1 [N / UQ] ?*
        _S(N, v2)  = ?* [N / UQ] v2 ?* v2 ?*
    with UQ = union_quantifiers(). There is no network/network form: at least
    one side must be a variable bound by an enclosing quantifier. */
    if name == "S" {
        let a = bound_var(ps, &args[0].value).map(str::to_string);
        let b = bound_var(ps, &args[1].value).map(str::to_string);
        if a.is_none() && b.is_none() {
            tracing::error!(
                "Syntax error: _S( needs a variable bound by an enclosing quantifier on at least one side"
            );
            return None;
        }
        let left = match &a {
            /* ?* v ?* v — the variable's two position markers */
            Some(v) => fsm_concat(
                opts,
                fsm_universal(),
                fsm_concat(
                    opts,
                    fsm_symbol(v),
                    fsm_concat(opts, fsm_universal(), fsm_symbol(v)),
                ),
            ),
            None => {
                let n = build_net(
                    opts,
                    ps,
                    &args[0].value,
                    nets.as_deref_mut(),
                    funcs.as_deref_mut(),
                )?;
                fsm_concat(
                    opts,
                    fsm_universal(),
                    fsm_ignore(opts, n, union_quantifiers(&ps.quantifiers), OP_IGNORE_ALL),
                )
            }
        };
        let right = match &b {
            Some(v) => {
                let tail = fsm_concat(
                    opts,
                    fsm_symbol(v),
                    fsm_concat(
                        opts,
                        fsm_universal(),
                        fsm_concat(opts, fsm_symbol(v), fsm_universal()),
                    ),
                );
                /* with variables on both sides the halves are joined by UQ */
                if a.is_some() {
                    fsm_concat(opts, union_quantifiers(&ps.quantifiers), tail)
                } else {
                    tail
                }
            }
            None => {
                let n = build_net(opts, ps, &args[1].value, nets, funcs)?;
                fsm_concat(
                    opts,
                    fsm_ignore(opts, n, union_quantifiers(&ps.quantifiers), OP_IGNORE_ALL),
                    fsm_universal(),
                )
            }
        };
        return Some(fsm_concat(opts, left, right));
    }

    let mut nets_built: Vec<Fsm> = Vec::new();
    for a in args {
        match build_net(
            opts,
            ps,
            &a.value,
            nets.as_deref_mut(),
            funcs.as_deref_mut(),
        ) {
            Some(n) => nets_built.push(n),
            None => {
                for n in nets_built {
                    fsm_destroy(n);
                }
                return None;
            }
        }
    }
    let mut it = nets_built.into_iter();
    let mut first = it.next().expect("arity >= 1");

    Some(match name {
        /* Predicates: regex.y wraps the boolean in fsm_boolean (empty string
        for true, empty set for false). */
        "isunambiguous" => {
            let r = fsm_boolean(fsm_isunambiguous(opts, &mut first) as i32);
            fsm_destroy(first);
            r
        }
        "isidentity" => {
            let r = fsm_boolean(fsm_isidentity(opts, &mut first) as i32);
            fsm_destroy(first);
            r
        }
        "isfunctional" => {
            let r = fsm_boolean(fsm_isfunctional(opts, &mut first) as i32);
            fsm_destroy(first);
            r
        }
        "notid" => fsm_extract_nonidentity(opts, first),
        "lm" => fsm_letter_machine(opts, first),
        "loweruniq" => fsm_lowerdet(opts, first),
        "loweruniqeps" => fsm_lowerdeteps(opts, first),
        "allfinal" => fsm_markallfinal(first),
        "unambpart" => fsm_extract_unambiguous(opts, first),
        "ambpart" => fsm_extract_ambiguous(opts, first),
        "ambdom" => fsm_extract_ambiguous_domain(opts, first),
        "addsink" => fsm_add_sink(first, 1),
        "close" => fsm_close_sigma(opts, first, 0),
        "closeu" => fsm_close_sigma(opts, first, 1),
        "marktail" => fsm_mark_fsm_tail(first, &it.next().expect("arity 2")),
        /* fsm_add_loop's `finals` selector: 1 = final states only,
        0 = non-final only, 2 = every state. */
        "addfinalloop" => fsm_add_loop(first, &it.next().expect("arity 2"), 1),
        "addnonfinalloop" => fsm_add_loop(first, &it.next().expect("arity 2"), 0),
        "addloop" => fsm_add_loop(first, &it.next().expect("arity 2"), 2),
        "leftrewr" => fsm_left_rewr(opts, first, it.next().expect("arity 2")),
        "flatten" => fsm_flatten(opts, first, it.next().expect("arity 2"))?,
        "eq" => {
            let mut left = it.next().expect("arity 3");
            let mut right = it.next().expect("arity 3");
            let r = fsm_equal_substrings(opts, first, &mut left, &mut right);
            fsm_destroy(left);
            fsm_destroy(right);
            r
        }
        /* regex.y: fsm_substitute_label($2, fsm_network_to_char($4), $6) — the
        label to replace is named by a network, of whose alphabet C takes the
        last (highest-numbered) symbol. An empty alphabet has no such symbol,
        so there is nothing to substitute and the net comes back unchanged. */
        "sublabel" => {
            let label_net = it.next().expect("arity 3");
            let mut replacement = it.next().expect("arity 3");
            let label = fsm_network_to_char(&label_net);
            fsm_destroy(label_net);
            let mut first = first;
            let r = match label.as_deref() {
                Some(l) => fsm_substitute_label(opts, &mut first, l, &mut replacement),
                None => fsm_copy(&mut first),
            };
            fsm_destroy(first);
            fsm_destroy(replacement);
            r
        }
        _ => unreachable!("arity table and dispatch cover the same names"),
    })
}

fn function_apply(
    opts: &FomaOptions,
    ps: &mut ParseState,
    name: &str,
    args: &[SpannedXre],
    mut nets: Option<&mut DefinedNetworks>,
    mut funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    let numargs = args.len() as i32;
    let body = match funcs.as_deref() {
        Some(f) => match find_defined_function(f, name, numargs) {
            Some(s) => s.to_string(),
            None => {
                tracing::error!("function {}@{}) not defined!", name, numargs);
                return None;
            }
        },
        None => {
            tracing::error!("function {}@{}) not defined!", name, numargs);
            return None;
        }
    };

    /* Build each argument net (C had these already built during the parse of
    the NAME(args) call). */
    let mut arg_nets: Vec<Fsm> = Vec::new();
    for a in args {
        arg_nets.push(build_net(
            opts,
            ps,
            &a.value,
            nets.as_deref_mut(),
            funcs.as_deref_mut(),
        )?);
    }

    let mut regex_bytes = body.into_bytes();
    let mut created: Vec<String> = Vec::new();
    for (i, argnet) in arg_nets.into_iter().enumerate() {
        let gsym = ps.internal_sym;
        /* C: sprintf(repstr, "%012X", g_internal_sym);
              sprintf(oldstr, "@ARGUMENT%02i@", i+1); — both 12 bytes wide, so
        streqrep's equal-length in-place replacement is valid. */
        let repstr = format!("{:012X}", gsym);
        let oldstr = format!("@ARGUMENT{:02}@", i + 1);
        replace_equal_len(&mut regex_bytes, oldstr.as_bytes(), repstr.as_bytes());
        match nets.as_deref_mut() {
            Some(n) => {
                add_defined(n, Some(argnet), &repstr);
            }
            None => {
                /* No registry to hold the temporary (only the internal
                None-table callers hit this; they never use functions). */
                fsm_destroy(argnet);
            }
        }
        created.push(repstr);
        ps.internal_sym = gsym.wrapping_add(1);
    }

    let regex_str = match String::from_utf8(regex_bytes) {
        Ok(s) => s,
        Err(_) => {
            if let Some(n) = nets.as_deref_mut() {
                for r in &created {
                    remove_defined(n, Some(r));
                }
            }
            tracing::error!("function {} produced a non-UTF-8 expansion", name);
            return None;
        }
    };

    let result = my_yyparse(opts, ps, &regex_str, nets.as_deref_mut(), funcs);

    if let Some(n) = nets {
        for r in &created {
            remove_defined(n, Some(r));
        }
    }
    result
}

// ───────────────────────── replacement rules ─────────────────────────

fn arrow_to_type(arrow: ReplaceArrow) -> ArrowType {
    match arrow {
        ReplaceArrow::Right => ArrowType::RIGHT,
        ReplaceArrow::OptionalRight => ArrowType::RIGHT | ArrowType::OPTIONAL,
        ReplaceArrow::Left => ArrowType::LEFT,
        ReplaceArrow::OptionalLeft => ArrowType::LEFT | ArrowType::OPTIONAL,
        ReplaceArrow::LeftRight => ArrowType::LEFT | ArrowType::RIGHT,
        ReplaceArrow::OptionalLeftRight => ArrowType::LEFT | ArrowType::RIGHT | ArrowType::OPTIONAL,
        ReplaceArrow::LtrLongest => {
            ArrowType::RIGHT | ArrowType::LONGEST_MATCH | ArrowType::LEFT_TO_RIGHT
        }
        ReplaceArrow::LtrShortest => {
            ArrowType::RIGHT | ArrowType::SHORTEST_MATCH | ArrowType::LEFT_TO_RIGHT
        }
        ReplaceArrow::RtlLongest => {
            ArrowType::RIGHT | ArrowType::LONGEST_MATCH | ArrowType::RIGHT_TO_LEFT
        }
        ReplaceArrow::RtlShortest => {
            ArrowType::RIGHT | ArrowType::SHORTEST_MATCH | ArrowType::RIGHT_TO_LEFT
        }
    }
}

fn mark_to_dir(mark: ContextMark) -> ReplaceDir {
    match mark {
        ContextMark::UpperUpper => ReplaceDir::Upward,
        ContextMark::LowerUpper => ReplaceDir::Rightward,
        ContextMark::UpperLower => ReplaceDir::Leftward,
        ContextMark::LowerLower => ReplaceDir::Downward,
    }
}

fn build_replace(
    opts: &FomaOptions,
    ps: &mut ParseState,
    rules: &[ReplaceRule],
    mut nets: Option<&mut DefinedNetworks>,
    mut funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    /* Each ReplaceRule (a `,,`-separated block) becomes one rewrite_set node;
    each MappingPair inside becomes one (or two, for dotted) Fsmrules node.
    Rule/set ordering is observably irrelevant (the sets are unioned /
    intersected / subtracted and all internal rule markers are erased at the
    end), so we build in source order. */
    let mut set_nodes: Vec<RewriteSet> = Vec::new();
    for rule in rules {
        let mut rule_nodes: Vec<Fsmrules> = Vec::new();
        for mapping in &rule.mappings {
            /* regex.y stores an arrow per Fsmrules node, so a parallel list
            may mix them: `a -> b, c (->) d` keeps `c` optional while `a` stays
            obligatory, both under the shared context. */
            build_mapping(
                opts,
                ps,
                mapping,
                arrow_to_type(mapping.arrow),
                &mut rule_nodes,
                nets.as_deref_mut(),
                funcs.as_deref_mut(),
            )?;
        }
        let (contexts_chain, direction) = match &rule.contexts {
            Some(rc) => {
                let dir = mark_to_dir(rc.mark);
                let mut ctx_nodes: Vec<Fsmcontexts> = Vec::new();
                for item in &rc.items {
                    /* regex.y add_context_pair: a missing side stores
                    fsm_empty_string() (never NULL). */
                    let left = match &item.left {
                        Some(e) => build_net(
                            opts,
                            ps,
                            &e.value,
                            nets.as_deref_mut(),
                            funcs.as_deref_mut(),
                        )?,
                        None => fsm_empty_string(),
                    };
                    let right = match &item.right {
                        Some(e) => build_net(
                            opts,
                            ps,
                            &e.value,
                            nets.as_deref_mut(),
                            funcs.as_deref_mut(),
                        )?,
                        None => fsm_empty_string(),
                    };
                    ctx_nodes.push(Fsmcontexts {
                        left: Some(left),
                        right: Some(right),
                        next: None,
                        cpleft: None,
                        cpright: None,
                    });
                }
                (link_fsmcontexts(ctx_nodes), Some(dir))
            }
            None => (None, None),
        };
        set_nodes.push(RewriteSet {
            rewrite_rules: link_fsmrules(rule_nodes),
            rewrite_contexts: contexts_chain,
            next: None,
            rule_direction: direction,
        });
    }

    let mut head = link_rewritesets(set_nodes)?;
    /* networkA: fsm_rewrite(rewrite_rules); clear_rewrite_ruleset(...). */
    let net = fsm_rewrite(opts, &mut head);
    /* clear_rewrite_ruleset — the owned chain drops here. */
    drop(head);
    Some(net)
}

fn build_restriction(
    opts: &FomaOptions,
    ps: &mut ParseState,
    body: &XreExpr,
    contexts: &[RestrContext],
    mut nets: Option<&mut DefinedNetworks>,
    mut funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    /* n0 CRESTRICT n0: fsm_context_restrict(body, contexts). */
    let x = build_net(opts, ps, body, nets.as_deref_mut(), funcs.as_deref_mut())?;
    let mut ctx_nodes: Vec<Fsmcontexts> = Vec::new();
    for item in contexts {
        let left = match &item.left {
            Some(e) => build_net(
                opts,
                ps,
                &e.value,
                nets.as_deref_mut(),
                funcs.as_deref_mut(),
            )?,
            None => fsm_empty_string(),
        };
        let right = match &item.right {
            Some(e) => build_net(
                opts,
                ps,
                &e.value,
                nets.as_deref_mut(),
                funcs.as_deref_mut(),
            )?,
            None => fsm_empty_string(),
        };
        ctx_nodes.push(Fsmcontexts {
            left: Some(left),
            right: Some(right),
            next: None,
            cpleft: None,
            cpright: None,
        });
    }
    Some(fsm_context_restrict(opts, x, link_fsmcontexts(ctx_nodes)))
}

fn build_substitute(
    opts: &FomaOptions,
    ps: &mut ParseState,
    haystack: &XreExpr,
    what: &SubstituteWhat,
    nets: Option<&mut DefinedNetworks>,
    funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    let net = build_net(opts, ps, haystack, nets, funcs)?;
    match what {
        /* sub1 sub2: fsm_substitute_symbol(net, subval1, subval2) — exactly one
        symbol to one symbol. */
        SubstituteWhat::Symbol {
            needle,
            replacement,
        } => {
            if replacement.len() != 1 {
                tracing::error!(
                    "Syntax error: substitution replaces a symbol with exactly one symbol"
                );
                fsm_destroy(net);
                return None;
            }
            Some(fsm_substitute_symbol(net, needle, &replacement[0]))
        }
        SubstituteWhat::Pair { .. } => {
            tracing::error!("Syntax error: pair substitution (a:b) is not supported");
            fsm_destroy(net);
            None
        }
    }
}

fn build_mapping(
    opts: &FomaOptions,
    ps: &mut ParseState,
    m: &MappingPair,
    arrow_type: ArrowType,
    out: &mut Vec<Fsmrules>,
    mut nets: Option<&mut DefinedNetworks>,
    mut funcs: Option<&mut DefinedFunctions>,
) -> Option<()> {
    match &m.upper {
        MappingSide::Expr(e) => {
            let upper = build_net(
                opts,
                ps,
                &e.value,
                nets.as_deref_mut(),
                funcs.as_deref_mut(),
            )?;
            let (r, r2) = build_rhs(opts, ps, &m.kind, nets.as_deref_mut(), funcs.as_deref_mut())?;
            add_rule(opts, out, upper, r, r2, arrow_type);
        }
        MappingSide::Dotted(Some(e)) => {
            /* LDOT n0 RDOT ARROW ...: add_rule with ArrowType::DOTTED. */
            let upper = build_net(
                opts,
                ps,
                &e.value,
                nets.as_deref_mut(),
                funcs.as_deref_mut(),
            )?;
            let (r, r2) = build_rhs(opts, ps, &m.kind, nets.as_deref_mut(), funcs.as_deref_mut())?;
            add_rule(opts, out, upper, r, r2, arrow_type | ArrowType::DOTTED);
        }
        MappingSide::Dotted(None) => {
            /* LDOT RDOT ARROW n0: add_eprule with ArrowType::DOTTED. */
            let (r, r2) = build_rhs(opts, ps, &m.kind, nets, funcs)?;
            add_eprule(out, r, r2, arrow_type | ArrowType::DOTTED);
        }
    }
    Some(())
}

/// The right-hand side(s) of a mapping: (right, right2).
type RhsPair = (Option<Fsm>, Option<Fsm>);

fn build_rhs(
    opts: &FomaOptions,
    ps: &mut ParseState,
    kind: &MappingKind,
    mut nets: Option<&mut DefinedNetworks>,
    mut funcs: Option<&mut DefinedFunctions>,
) -> Option<RhsPair> {
    match kind {
        MappingKind::Plain { lower } => {
            let r = build_side(opts, ps, lower, nets.as_deref_mut(), funcs.as_deref_mut())?;
            Some((Some(r), None))
        }
        MappingKind::Markup { pre, post } => {
            /* n0 ARROW [n0] TRIPLE_DOT [n0]: right = pre|0, right2 = post|0. */
            let r = match pre {
                Some(s) => build_side(opts, ps, s, nets.as_deref_mut(), funcs.as_deref_mut())?,
                None => fsm_empty_string(),
            };
            let r2 = match post {
                Some(s) => build_side(opts, ps, s, nets, funcs)?,
                None => fsm_empty_string(),
            };
            Some((Some(r), Some(r2)))
        }
    }
}

fn build_side(
    opts: &FomaOptions,
    ps: &mut ParseState,
    s: &MappingSide,
    nets: Option<&mut DefinedNetworks>,
    funcs: Option<&mut DefinedFunctions>,
) -> Option<Fsm> {
    match s {
        MappingSide::Expr(e) => build_net(opts, ps, &e.value, nets, funcs),
        MappingSide::Dotted(Some(e)) => build_net(opts, ps, &e.value, nets, funcs),
        MappingSide::Dotted(None) => Some(fsm_empty_string()),
    }
}

/// regex.y add_rule: build the Fsmrules node(s) for one mapping. For dotted
/// rules the main rule has ArrowType::DOTTED stripped (and its LHS loses the empty
/// string); an extra rule keeping ArrowType::DOTTED is emitted only when the LHS
/// could match the empty string.
fn add_rule(
    opts: &FomaOptions,
    out: &mut Vec<Fsmrules>,
    l: Fsm,
    r: Option<Fsm>,
    r2: Option<Fsm>,
    ty: ArrowType,
) {
    if !ty.contains(ArrowType::DOTTED) {
        out.push(Fsmrules {
            left: Some(l),
            right: r,
            right2: r2,
            cross_product: None,
            next: None,
            arrow_type: ty,
            dotted: 0,
        });
        return;
    }

    let mut l = l;
    let main_left = fsm_minus(opts, fsm_copy(&mut l), fsm_empty_string());
    let mut main = Fsmrules {
        left: Some(main_left),
        right: r,
        right2: r2,
        cross_product: None,
        next: None,
        arrow_type: ty - ArrowType::DOTTED,
        dotted: 0,
    };

    /* test = L ∩ [] : add the empty-[..] rule only if non-empty. */
    let mut test = fsm_intersect(opts, l, fsm_empty_string());
    if !fsm_isempty(opts, &mut test) {
        let test_right = main.right.as_mut().map(fsm_copy);
        let test_right2 = main.right2.as_mut().map(fsm_copy);
        out.push(Fsmrules {
            left: Some(test),
            right: test_right,
            right2: test_right2,
            cross_product: None,
            next: None,
            arrow_type: ty,
            dotted: 0,
        });
    } else {
        fsm_destroy(test);
    }
    out.push(main);
}

/// regex.y add_eprule: `[..] -> R (... R2)` — LHS is the empty string, and the
/// arrow_type keeps ArrowType::DOTTED (unlike add_rule's main rule).
fn add_eprule(out: &mut Vec<Fsmrules>, r: Option<Fsm>, r2: Option<Fsm>, ty: ArrowType) {
    out.push(Fsmrules {
        left: Some(fsm_empty_string()),
        right: r,
        right2: r2,
        cross_product: None,
        next: None,
        arrow_type: ty,
        dotted: 0,
    });
}

fn link_fsmrules(mut nodes: Vec<Fsmrules>) -> Option<Box<Fsmrules>> {
    let mut head: Option<Box<Fsmrules>> = None;
    while let Some(mut node) = nodes.pop() {
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn link_fsmcontexts(mut nodes: Vec<Fsmcontexts>) -> Option<Box<Fsmcontexts>> {
    let mut head: Option<Box<Fsmcontexts>> = None;
    while let Some(mut node) = nodes.pop() {
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn link_rewritesets(mut nodes: Vec<RewriteSet>) -> Option<Box<RewriteSet>> {
    let mut head: Option<Box<RewriteSet>> = None;
    while let Some(mut node) = nodes.pop() {
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

#[cfg(test)]
mod tests {
    use crate::constructions::{fsm_count, fsm_equivalent};
    use crate::define::{
        Defined, add_defined, add_defined_function, defined_functions_init, defined_networks_init,
    };
    use crate::options::FomaOptions;
    use crate::topsort::fsm_topsort;
    use crate::types::Fsm;

    /// C foma's `print size` numbers are produced downstream of
    /// fsm_parse_regex: the CLI regex command runs fsm_topsort (which sets
    /// pathcount) and stack_add runs fsm_count — mirror that pipeline here.
    fn counted(net: Fsm) -> (i32, i32, i64) {
        let mut net = fsm_topsort(net);
        fsm_count(&mut net);
        (net.statecount, net.arccount, net.pathcount)
    }

    /// The internal regexes that rewrite.rs feeds to fsm_parse_regex and then
    /// `.unwrap()`s. If nfst-xre cannot parse any of these, the rewrite
    /// compiler would panic at runtime — so guard the parse here.
    const REWRITE_INTERNAL_REGEXES: &[&str] = &[
        r#""@O@" "@0@" "@#@" "@ID@""#,
        r#"[?:0]^4 [?:0 ?:0 ? ?]* [?:0]^4"#,
        r#"["@I[@"|"@I[]@"] ["@I[@"|"@I[]@"|"@I]@"|"@I@"|"@O@"]* ["@O@"|"@I[@"|"@I[]@"] ["@I[@"|"@I[]@"|"@I]@"|"@I@"|"@O@"]*"#,
        r#"[? ? ? ?]* [? ? [?-"@0@"] ?]"#,
        r#"[? ? ? ?]* [? ? ? [?-"@0@"]]"#,
        r#"["@I[@"] \["@I]@"]*"#,
        r#""@O@" ["@O@"]* ["@I[@"|"@I[]@"] ["@I[@"|"@I[]@"|"@I]@"|"@I@"|"@O@"]*"#,
        r#"~[[? ?]* "@0@" "@0@" [? ?]*]"#,
        r#"[? ? | "@UNK@" "@UNK@":"@ID@" ]*"#,
        r#"["@I[]@" ? ? ? | "@I[@" ? ? ? ["@I@" ? ? ?]* "@I]@" ? [?-"@0@"] ? ] ["@I]@" ? "@0@" ?]* | 0"#,
        r#"~[[? ? "@0@" ?]*]"#,
    ];

    #[test]
    fn rewrite_internal_regexes_parse() {
        for src in REWRITE_INTERNAL_REGEXES {
            let r = nfst_xre::parse_all(src);
            assert!(
                r.is_ok(),
                "nfst-xre failed to parse {:?}: {:?}",
                src,
                r.err()
            );
        }
    }

    #[test]
    fn end_to_end_compiles() {
        let opts = &FomaOptions::default();
        /* Exercise the full walk + construction pipeline (no defined tables). */
        let cases = [
            "cat",
            "c a t",
            "a | b",
            "a & b",
            "a - b",
            "a*",
            "a+",
            "a:b",
            "[a b]*",
            "a .o. b",
            "~a",
            "$a",
            "a^3",
            "a^{2,4}",
            "\\a",
            "a .x. b",
            ".#. a .#.",
            "{cat}",
            "a b c ;",
        ];
        for src in cases {
            let net = super::fsm_parse_regex(opts, src, None, None);
            assert!(net.is_some(), "failed to compile regex: {:?}", src);
        }
    }

    /// Enumerate the words of the net `src` compiles to, sorted.
    fn words_of(opts: &FomaOptions, src: &str) -> Vec<String> {
        use crate::apply::{apply_init, apply_words};
        let net = super::fsm_parse_regex(opts, src, None, None).expect("regex compiles");
        let mut h = apply_init(&net);
        let mut v = Vec::new();
        while let Some(w) = apply_words(&mut h) {
            v.push(w);
        }
        v.sort();
        v.dedup();
        v
    }

    /// Enumerate the lower-side outputs of `word` through `src`, sorted.
    fn down_all(opts: &FomaOptions, src: &str, word: &str) -> Vec<String> {
        use crate::apply::{apply_down, apply_init};
        let net = super::fsm_parse_regex(opts, src, None, None).expect("regex compiles");
        let mut h = apply_init(&net);
        let mut v = Vec::new();
        let mut r = apply_down(&mut h, Some(word));
        while let Some(w) = r {
            v.push(w);
            r = apply_down(&mut h, None);
        }
        v.sort();
        v.dedup();
        v
    }

    // A parallel rule list may mix obligatory and optional arrows: regex.y
    // stores an arrow per rule, so `c` alternates optionally while `a` is
    // replaced obligatorily, both under the one shared context.
    // Regression for divvun/foma-rs#4.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn parallel_rule_list_mixes_obligatory_and_optional_arrows() {
        let opts = &FomaOptions::default();
        let src = "[ a -> b, c (->) d || _ e ]";
        assert_eq!(down_all(opts, src, "ae"), vec!["be"]);
        assert_eq!(down_all(opts, src, "ace"), vec!["ace", "ade"]);
    }

    // The `_xxx(` builtin family (regex.l's hardcoded function keywords).
    // `_eq` is the reduplication operator from foma's own docs: it keeps only
    // the paths whose `%<`-delimited substrings are all equal.
    // Regression for divvun/foma-rs#3.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn builtin_eq_filters_to_equal_substrings() {
        use crate::apply::{apply_init, apply_up};
        let opts = &FomaOptions::default();
        let src = "_eq([%< [{cat}|{dog}] %> (%- %< [?-%<-%>]+ %>)], %<, %>) .o. %<|%> -> 0";
        let net = super::fsm_parse_regex(opts, src, None, None).expect("_eq compiles");
        let mut h = apply_init(&net);
        assert_eq!(
            apply_up(&mut h, Some("cat-cat")).as_deref(),
            Some("<cat>-<cat>")
        );
        let mut h = apply_init(&net);
        assert_eq!(
            apply_up(&mut h, Some("dog-dog")).as_deref(),
            Some("<dog>-<dog>")
        );
        /* Unequal halves are not reduplication, so no path survives. */
        let mut h = apply_init(&net);
        assert_eq!(apply_up(&mut h, Some("cat-dog")), None);
    }

    // Predicates return fsm_boolean: the empty-string net for true, the empty
    // set for false. The rest of the family compiles and dispatches.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn builtin_family_compiles() {
        let opts = &FomaOptions::default();
        let t = super::fsm_parse_regex(opts, "_isfunctional(a:b)", None, None).unwrap();
        assert_eq!(counted(t).2, 1, "_isfunctional(a:b) is true (empty string)");
        let f = super::fsm_parse_regex(opts, "_isfunctional(a:b | a:c)", None, None).unwrap();
        assert_eq!(
            counted(f).2,
            0,
            "_isfunctional(a:b|a:c) is false (empty set)"
        );

        for src in [
            "_isidentity(a)",
            "_isunambiguous(a:b)",
            "_notid(a:a | a:b)",
            "_lm({abc})",
            "_loweruniq(a:b)",
            "_loweruniqeps(a:b)",
            "_allfinal(a b)",
            "_unambpart(a:b)",
            "_ambpart(a:b | a:c)",
            "_ambdom(a:b | a:c)",
            "_addsink(a)",
            "_close(a ?)",
            "_closeu(a ?)",
            "_marktail(a, b)",
            "_addfinalloop(a, b)",
            "_addnonfinalloop(a, b)",
            "_addloop(a, b)",
            "_leftrewr(a, b:c)",
            "_flatten(a:b, x)",
            "_sublabel(a b c, b, x)",
        ] {
            assert!(
                super::fsm_parse_regex(opts, src, None, None).is_some(),
                "failed to compile builtin: {src}"
            );
        }
    }

    // Wrong arity is rejected rather than silently mis-dispatched.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    /// Assert two regexes compile to the same language.
    fn assert_equiv(opts: &FomaOptions, lhs: &str, rhs: &str) {
        let a = super::fsm_parse_regex(opts, lhs, None, None)
            .unwrap_or_else(|| panic!("lhs failed to compile: {lhs}"));
        let b = super::fsm_parse_regex(opts, rhs, None, None)
            .unwrap_or_else(|| panic!("rhs failed to compile: {rhs}"));
        assert!(
            fsm_equivalent(opts, a, b),
            "not equivalent:\n  {lhs}\n  {rhs}"
        );
    }

    // foma's first-order-logic sublanguage: a variable denotes a substring,
    // marked by the two occurrences of its symbol that fsm_quantifier pins
    // (\x* x \x* x \x*); the binder intersects the body with that constraint
    // and then substitutes the markers away.
    //
    // The reference is foma's own help text (iface.c:217-218), which gives
    //   $.A == (∃x)(x ∈ A ∧ ¬(∃y)(y ∈ A ∧ ¬(x = y)))
    // in both the ∃ and ∀ phrasings. NOTE: upstream C foma does not currently
    // satisfy this — its lexer folds the formula body into a single symbol, so
    // it compiles the two sides to a 2-state and a 372-state network and
    // fsm_equivalent reports them unequal. These identities are the spec the C
    // implementation documents but does not meet.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn logic_quantifiers_match_documented_identities() {
        let opts = &FomaOptions::default();
        /* $.A — contains exactly one A — in both documented phrasings. */
        assert_equiv(
            opts,
            "$.[a|b]",
            "(\u{2203}x)(x \u{2208} [a|b] & ~(\u{2203}y)(y \u{2208} [a|b] & ~(x = y)))",
        );
        assert_equiv(
            opts,
            "$.[a|b]",
            "(\u{2203}x)(x \u{2208} [a|b] & (\u{2200}y)(~(y \u{2208} [a|b] & ~(x = y))))",
        );
        /* plain containment, and its negation */
        assert_equiv(opts, "$[a b]", "(\u{2203}x)(x \u{2208} [a b])");
        assert_equiv(opts, "~$[a]", "~(\u{2203}x)(x \u{2208} a)");
    }

    // The variable-to-variable relations. `<`/`>` and the Unicode `\u{227A}`/`\u{227B}`
    // are the same relation; `\u{2260}` is the negation of `=`.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:structures.fsm-logical-precedence-fn/test]
    // [spec:foma:sem:structures.fsm-logical-eq-fn/test]
    #[test]
    fn logic_relations_match_documented_identities() {
        let opts = &FomaOptions::default();
        let contains_a_then_b = "$[a ?* b]";
        /* xre binds `<` looser than `&`, where regex.y binds the VAR relation
        tighter, so the unparenthesized ASCII form exercises the
        re-association path; the Unicode spellings bind tightly already. */
        assert_equiv(
            opts,
            contains_a_then_b,
            "(\u{2203}x)(\u{2203}y)(x \u{2208} a & y \u{2208} b & x < y)",
        );
        assert_equiv(
            opts,
            contains_a_then_b,
            "(\u{2203}x)(\u{2203}y)(x \u{2208} a & y \u{2208} b & x \u{227A} y)",
        );
        assert_equiv(
            opts,
            contains_a_then_b,
            "(\u{2203}x)(\u{2203}y)(x \u{2208} a & y \u{2208} b & y \u{227B} x)",
        );
        /* \u{2260}: "an a, and no other a distinct from it" is again exactly-one-a */
        assert_equiv(
            opts,
            "$.[a] & $[a]",
            "(\u{2203}x)(x \u{2208} a & ~(\u{2203}y)(y \u{2208} a & x \u{2260} y))",
        );
    }

    // _S(x, y): y immediately succeeds x, so requiring an `a` and a `b` in
    // that relation is exactly "contains the substring a b".
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    #[test]
    fn builtin_successor_of_relates_adjacent_variables() {
        let opts = &FomaOptions::default();
        assert_equiv(
            opts,
            "$[a b]",
            "(\u{2203}x)(\u{2203}y)(x \u{2208} a & y \u{2208} b & _S(x, y))",
        );
    }

    // Outside a formula nothing above may fire: `(A)` stays optionality and
    // `<`/`>` stay the before/after network operators.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    #[test]
    fn logic_layer_is_inert_without_a_binder() {
        let opts = &FomaOptions::default();
        assert_equiv(opts, "(a) b", "[a b] | b");
        assert_eq!(
            counted(super::fsm_parse_regex(opts, "a < b", None, None).unwrap()).0,
            2
        );
        assert_eq!(
            counted(super::fsm_parse_regex(opts, "$.[a|b]", None, None).unwrap()).0,
            2
        );
    }

    #[test]
    fn builtin_arity_is_checked() {
        let opts = &FomaOptions::default();
        assert!(super::fsm_parse_regex(opts, "_lm(a, b)", None, None).is_none());
        assert!(super::fsm_parse_regex(opts, "_eq(a, b)", None, None).is_none());
        assert!(super::fsm_parse_regex(opts, "_sublabel(a, b)", None, None).is_none());
    }

    // regex.y: fsm_substitute_label($2, fsm_network_to_char($4), $6) — splice a
    // network in for every arc carrying one label. The label is named by a
    // network, of whose alphabet C takes the LAST (highest-numbered) symbol, so
    // `[a|b|c]` names `c`, not `a`. Verified against C foma built from source.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn builtin_sublabel_splices_a_network_for_one_label() {
        let opts = &FomaOptions::default();
        assert_eq!(
            words_of(opts, "_sublabel(a b c, b, [x|y])"),
            vec!["axc", "ayc"]
        );
        /* sigma of [a|b|c] is {a,b,c}; the last entry names the label. */
        assert_eq!(words_of(opts, "_sublabel(a b c, [a|b|c], x)"), vec!["abx"]);
        /* a label absent from the net leaves it unchanged */
        assert_eq!(words_of(opts, "_sublabel(a b c, z, x)"), vec!["abc"]);
    }

    // _S( needs a quantifier-bound variable, which requires foma's
    // first-order-logic sublanguage; it reports that instead of mis-dispatching.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    #[test]
    fn builtin_successor_of_is_reported_unsupported() {
        let opts = &FomaOptions::default();
        assert!(super::fsm_parse_regex(opts, "_S(a, b)", None, None).is_none());
    }

    #[test]
    fn replace_and_restriction_compile() {
        let opts = &FomaOptions::default();
        /* The rewrite batch API and context-restriction paths. */
        assert!(super::fsm_parse_regex(opts, "a -> b", None, None).is_some());
        assert!(super::fsm_parse_regex(opts, "a -> b || c _ d", None, None).is_some());
        assert!(super::fsm_parse_regex(opts, "a -> b, c -> d", None, None).is_some());
        assert!(super::fsm_parse_regex(opts, "[. a .] -> b", None, None).is_some());
        assert!(super::fsm_parse_regex(opts, "a => b _ c", None, None).is_some());
        assert!(super::fsm_parse_regex(opts, "a @-> b || _ c", None, None).is_some());
    }

    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn syntax_error_returns_none() {
        let opts = &FomaOptions::default();
        /* C: yyparse returns non-zero, my_yyparse propagates it, and
        fsm_parse_regex returns NULL. */
        assert!(super::fsm_parse_regex(opts, "[ a b", None, None).is_none());
        assert!(super::fsm_parse_regex(opts, "a |", None, None).is_none());
    }

    // fsm_parse_regex returns fsm_minimize(current_parse): the shapes below
    // are C foma's `print size` for the same regexes (2 states/1 arc/1 path;
    // 2/2/2; 3/3/2 — the last only after minimization).
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn parse_success_yields_minimized_c_shapes() {
        let opts = &FomaOptions::default();
        let expect = [
            ("a", 2, 1, 1i64),
            ("a|b", 2, 2, 2),
            ("[a b]|[a c]", 3, 3, 2),
        ];
        for (src, states, arcs, paths) in expect {
            let net = super::fsm_parse_regex(opts, src, None, None).unwrap();
            assert_eq!(counted(net), (states, arcs, paths), "shape of {:?}", src);
        }
    }

    // Grammar start rule `start: regex | regex start`: current_parse is the
    // LAST `;`-terminated network in the string.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn semicolon_separated_regexes_keep_last() {
        let opts = &FomaOptions::default();
        let net = super::fsm_parse_regex(opts, "a b ; x y z ;", None, None).unwrap();
        let expected = super::fsm_parse_regex(opts, "x y z", None, None).unwrap();
        assert!(fsm_equivalent(opts, net, expected));
        let net = super::fsm_parse_regex(opts, "a ; b", None, None).unwrap();
        let expected = super::fsm_parse_regex(opts, "b", None, None).unwrap();
        assert!(fsm_equivalent(opts, net, expected));
    }

    // regex.l NONRESERVED: a symbol naming a defined net is substituted with
    // a copy of that net (via the defined_nets table); without the table the
    // same name is a literal single symbol.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn defined_net_names_substitute_from_the_table() {
        let opts = &FomaOptions::default();
        let mut nets = defined_networks_init();
        let def = super::fsm_parse_regex(opts, "x y", None, None).unwrap();
        assert_eq!(add_defined(&mut nets, Some(def), "Foo"), Defined::New);
        // With the table: "Foo" compiles to the defined net [x y]
        // (C foma: 3 states, 2 arcs, 1 path).
        let net = super::fsm_parse_regex(opts, "Foo", Some(&mut nets), None).unwrap();
        assert_eq!(counted(net), (3, 2, 1));
        let net = super::fsm_parse_regex(opts, "Foo", Some(&mut nets), None).unwrap();
        let expected = super::fsm_parse_regex(opts, "x y", None, None).unwrap();
        assert!(fsm_equivalent(opts, net, expected));
        // Without the table: "Foo" is one literal (multichar) symbol.
        let net = super::fsm_parse_regex(opts, "Foo", None, None).unwrap();
        assert_eq!(counted(net), (2, 1, 1));
    }

    // regex.y function_apply: F(a) expands the stored body regex with the
    // argument bound to a temporary defined symbol and reparses it.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    // [spec:foma:sem:fomalib.fsm-parse-regex-fn/test]
    #[test]
    fn defined_function_application_expands_body() {
        let opts = &FomaOptions::default();
        let mut nets = defined_networks_init();
        let mut funcs = defined_functions_init();
        // define F(X) [X X];  — stored body references @ARGUMENT01@.
        add_defined_function(opts, &mut funcs, "F", "[@ARGUMENT01@ @ARGUMENT01@]", 1);
        let net = super::fsm_parse_regex(opts, "F(a)", Some(&mut nets), Some(&mut funcs)).unwrap();
        // C foma: regex F(a); => 3 states, 2 arcs, 1 path (== [a a]).
        assert_eq!(counted(net), (3, 2, 1));
        let net = super::fsm_parse_regex(opts, "F(a)", Some(&mut nets), Some(&mut funcs)).unwrap();
        let expected = super::fsm_parse_regex(opts, "a a", None, None).unwrap();
        assert!(fsm_equivalent(opts, net, expected));
        // Undefined functions fail.
        assert!(super::fsm_parse_regex(opts, "G(a)", Some(&mut nets), Some(&mut funcs)).is_none());
    }

    // my_yyparse's g_parse_depth guard: at MAX_PARSE_DEPTH (100) nested
    // reparses it prints "Exceeded parser stack depth.  Self-recursive call?"
    // and fails instead of recursing forever.
    // [spec:foma:sem:foma.my-yyparse-fn/test]
    #[test]
    fn parse_depth_guard_stops_self_recursive_defines() {
        let opts = &FomaOptions::default();
        let mut nets = defined_networks_init();
        let mut funcs = defined_functions_init();
        // define F(X) F(X); — every application reparses another F(...) call.
        add_defined_function(opts, &mut funcs, "F", "F(@ARGUMENT01@)", 1);
        assert!(super::fsm_parse_regex(opts, "F(a)", Some(&mut nets), Some(&mut funcs)).is_none());
        // Each top-level parse gets a fresh depth counter, so the parser still
        // works after the guard trips.
        assert!(super::fsm_parse_regex(opts, "a", None, None).is_some());
    }
}

# foma/rewrite.c

> [spec:foma:def:rewrite.fsm-clear-contexts-fn]
> void fsm_clear_contexts(struct fsmcontexts *contexts)

> [spec:foma:sem:rewrite.fsm-clear-contexts-fn]
> Frees a linked list of struct fsmcontexts: for each node, fsm_destroy()s its left, right,
> cpleft, and cpright nets (fsm_destroy tolerates NULL), saves next, then free()s the node.
> Iterates to the end of the list; no return value.

> [spec:foma:def:rewrite.fsm-rewrite-fn]
> struct fsm *fsm_rewrite(struct rewrite_set *all_rules)

> [spec:foma:sem:rewrite.fsm-rewrite-fn]
> Master rewrite-rule compiler: turns a linked list of parallel rulesets (struct rewrite_set, each
> with rules {left,right,right2,arrow_type} sharing rewrite_contexts and a rule_direction) into one
> 2-tape transducer. Works in a flattened 4-tape encoding: each logical position is a block of 4
> consecutive symbols — tape1 = position class (@O@ outside a rewrite, @I[@ open, @I@ inside, @I]@
> close, @I[]@ open+close), tape2 = rule-number marker or @0@, tape3 = input symbol, tape4 = output
> symbol; @0@ = epsilon, @ID@ = "same symbol as tape3 here", @#@ = word boundary. Arrow-type bits:
> ARROW_RIGHT 1, ARROW_LEFT 2, ARROW_OPTIONAL 4, ARROW_DOTTED 8, ARROW_LONGEST_MATCH 16,
> ARROW_SHORTEST_MATCH 32. Steps:
> 1. num_rules = total rule count over all rulesets. calloc a rewrite_batch rb; rb->namestrings[i]
> (char[8] each) = sprintf "@#%04i@" of i+1 — rules are numbered 1..n in traversal order.
> 2. rb->ISyms = min(@I@ | @I[]@ | @I[@ | @I]@); rb->Rulenames = fold min(union(...)) of all rule
> markers over fsm_empty_set(); rb->ANY = fsm_identity() (single ? symbol) run through
> `[spec:foma:sem:rewrite.rewrite-add-special-syms-fn]` so ? never matches an auxiliary symbol.
> 3. Call rewrite_add_special_syms on every rule's left, right, right2 and every context's
> left/right (this also renames .#. to @#@ in their sigmas).
> 4. Cross-products, per rule with running rule_number: if right == NULL (transducer center T(x)):
> CP = `[spec:foma:sem:rewrite.rewrite-cp-transducer-fn]`(copy(left), #); then rules->right :=
> min(lower(copy(left))), rules->left := min(upper(copy(left))) (the original left net is leaked),
> and special syms are added to both projections. Else if right2 == NULL (plain A -> B):
> CP = `[spec:foma:sem:rewrite.rewrite-cp-fn]`(copy(left), copy(right), #). Else (A -> B ... C):
> CP = `[spec:foma:sem:rewrite.rewrite-cp-markup-fn]`(copy(left), copy(right), copy(right2), #).
> Store fsm_copy(CP) in rules->cross_product; RuleCP = min(union over all rules of CP).
> 5. Base = min(Boundary (RuleCP | Outside)* Boundary) where Boundary is the literal 4-block
> regex `"@O@" "@0@" "@#@" "@ID@"` and Outside = @O@ @0@ ANY @ID@.
> 6. Per context of each ruleset, compute cpleft/cpright from copies of left/right per
> rule_direction: OP_UPWARD_REPLACE(1) upper/upper; OP_RIGHTWARD_REPLACE(2) lower/upper;
> OP_LEFTWARD_REPLACE(3) upper/lower; OP_DOWNWARD_REPLACE(4) lower/lower; OP_TWO_LEVEL_REPLACE(5)
> `[spec:foma:sem:rewrite.rewrite-two-level-fn]`(left,0) / (right,1); "upper"/"lower" meaning
> `[spec:foma:sem:rewrite.rewrite-upper-fn]` / `[spec:foma:sem:rewrite.rewrite-lower-fn]`.
> 7. Then per rule, in the same numbering order:
> (a) if ARROW_DOTTED: with Center = copy(cross_product), Any4 = rewrite_any_4tape, EP =
> rewrite_epextend: Base := Base & ~[Any4 Center ~[EP Any4]] & ~[~[Any4 EP] Center Any4] — a
> dotted (epenthesis) center may only occur immediately flanked by EP material on both sides.
> (b) if the ruleset has contexts: Base := Base &
> `[spec:foma:sem:rewrite.rewr-context-restrict-fn]`(rb, cross_product, contexts).
> (c) violation language C := empty set; if ARROW_RIGHT and not ARROW_OPTIONAL, C |=
> rewr_unrewritten(min(left - [])); if ARROW_LEFT and not OPTIONAL, C |=
> rewr_unrewritten(min(right - [])); if ARROW_LONGEST_MATCH: for ARROW_RIGHT, C |=
> rewr_notleftmost(rewrite_upper(copy(left))) | rewr_notlongest(rewrite_upper(copy(left))), and
> for ARROW_LEFT the same over rewrite_lower(copy(right)); if ARROW_SHORTEST_MATCH: identical but
> with rewr_notshortest in place of rewr_notlongest.
> (d) if the ruleset has NO contexts: Base := Base - rewr_contains(EP EP) when (ARROW_DOTTED and
> not OPTIONAL), else Base := Base - rewr_contains(copy(C)).
> (e) otherwise, per context: when (ARROW_DOTTED and not OPTIONAL), LeftExtend = min((Any4 cpleft)
> & (Any4 EP)), RightExtend = min((EP Any4) & (cpright Any4)), Base := Base -
> rewr_contains(min(LeftExtend RightExtend)); else Base := Base -
> rewr_contains(copy(cpleft) copy(C) copy(cpright)). Destroy C after each rule.
> 8. Project to 2 tapes: Base := min(lower(Base .o. regex `[?:0]^4 [?:0 ?:0 ? ?]* [?:0]^4`)) —
> deletes the two 4-symbol boundary blocks and tapes 1-2 of every interior block, leaving an
> alternating input/output symbol string; then Base := fsm_unflatten(Base, "@0@", "@ID@"),
> rebuilding a real transducer with @0@ read as epsilon and @ID@ as identity.
> 9. Remove the 8 special symbols (see `[spec:foma:sem:rewrite.rewrite-add-special-syms-fn]`) and
> all rule-name markers from Base->sigma via sigma_remove; fsm_compact(Base); sigma_sort(Base);
> `[spec:foma:sem:rewrite.rewrite-cleanup-fn]`(rb); return Base.

> [spec:foma:def:rewrite.rewr-contains-fn]
> struct fsm *rewr_contains(struct rewrite_batch *rb, struct fsm *lang)

> [spec:foma:sem:rewrite.rewr-contains-fn]
> Returns min(Any4 lang Any4), with Any4 = fresh copies from
> `[spec:foma:sem:rewrite.rewrite-any-4tape-fn]`: every well-formed 4-tape string containing a
> factor in lang. Despite the "NotContain" comment in the source, NO complement is taken — callers
> subtract the result from Base instead. Consumes lang.

> [spec:foma:def:rewrite.rewr-context-restrict-fn]
> struct fsm *rewr_context_restrict(struct rewrite_batch *rb, struct fsm *X, struct fsmcontexts *LR)

> [spec:foma:sem:rewrite.rewr-context-restrict-fn]
> Compiles the context restriction X => L1 _ R1, ..., Ln _ Rn over the 4-tape encoding using the
> auxiliary marker symbol @VARX@ (classic marker construction). Var = fsm_symbol("@VARX@");
> Notvar = rewrite_any_4tape(rb) minus fsm_contains(@VARX@) — well-formed 4-tape strings free of
> the marker (NOT the generic ~$[@VARX@]). NewX = copy of X with @VARX@ added to its sigma (and
> sigma_sort) if absent, so ? in X never matches the marker; the same sigma-add is applied to each
> context copy. For every context pair: Left = copy(cpleft), or fsm_empty_string() when
> pairs->left is NULL; Right = copy(cpright), or empty string when pairs->right is NULL;
> UnionP |= Left Var Notvar Var Right. UnionL = Notvar Var NewX Var Notvar. Result = UnionL -
> (Notvar UnionP Notvar): marked occurrences of X NOT properly surrounded by some (Li, Ri). If
> @VARX@ is present in Result's sigma, substitute it by @_EPSILON_SYMBOL_@ and complement;
> otherwise just complement. Destroys UnionP, Var, Notvar, NewX; X and the contexts' cpleft/cpright
> nets are NOT consumed (all uses are copies). Returns the complemented (i.e. constraint) language.

> [spec:foma:def:rewrite.rewr-notleftmost-fn]
> struct fsm *rewr_notleftmost(struct rewrite_batch *rb, struct fsm *lang, int rule_number, int arrow_type)

> [spec:foma:sem:rewrite.rewr-notleftmost-fn]
> Violation language for leftmost application of rule rule_number. nl = regex `"@O@" ["@O@"]*
> ["@I[@"|"@I[]@"] ["@I[@"|"@I[]@"|"@I]@"|"@I@"|"@O@"]*` (on tape 1: at least one outside block
> precedes the first opening bracket) lifted to tape 1 of 4 via
> `[spec:foma:sem:rewrite.rewrite-tape-m-to-n-of-k-fn]`(nl,1,1,4). Refined by rulenum, built with
> combinators over full blocks: min([@O@ ? ? ?] [@O@ ? ? ?]* [@I[@|@I[]@] RULE ?*) — all blocks
> before the opener are Outside blocks and the opening block's tape-2 carries this rule's marker.
> nl &= rulenum. flt = regex `[? ? ? ?]* [? ? [?-"@0@"] ?]` when arrow_type has ARROW_RIGHT (the
> final block's tape-3, the upper symbol, must not be @0@), else the tape-4 variant `[? ? ? ?]*
> [? ? ? [?-"@0@"]]`. Returns min(nl & copy(lang) & flt); lang itself is not consumed (callers in
> `[spec:foma:sem:rewrite.fsm-rewrite-fn]` leak it).

> [spec:foma:def:rewrite.rewr-notlongest-fn]
> struct fsm *rewr_notlongest(struct rewrite_batch *rb, struct fsm *lang, int rule_number, int arrow_type)

> [spec:foma:sem:rewrite.rewr-notlongest-fn]
> Violation language for longest-match of rule rule_number: strings where a match opens and later
> material (outside symbols or another opening) shows the match could have been extended. nl =
> regex `["@I[@"|"@I[]@"] ["@I[@"|"@I[]@"|"@I]@"|"@I@"|"@O@"]* ["@O@"|"@I[@"|"@I[]@"]
> ["@I[@"|"@I[]@"|"@I]@"|"@I@"|"@O@"]*` lifted to tape 1 of 4 by
> `[spec:foma:sem:rewrite.rewrite-tape-m-to-n-of-k-fn]`(nl,1,1,4); intersected with rulenum =
> min(? RULE ? ? ?*) — the first block's tape-2 symbol is this rule's marker. flt = regex
> `[? ? ? ?]* [? ? [?-"@0@"] ?]` for ARROW_RIGHT (the language may not end in @0@ on tape 3), else
> `[? ? ? ?]* [? ? ? [?-"@0@"]]` (tape 4). Returns min(nl & copy(lang) & flt); lang not consumed.

> [spec:foma:def:rewrite.rewr-notshortest-fn]
> struct fsm *rewr_notshortest(struct rewrite_batch *rb, struct fsm *lang, int rule_number)

> [spec:foma:sem:rewrite.rewr-notshortest-fn]
> Violation language for shortest-match of rule rule_number: a match opens with @I[@ and no
> closing bracket ever follows (so a shorter completed match was available). ns = regex
> `["@I[@"] \["@I]@"]*` lifted to tape 1 of 4 via
> `[spec:foma:sem:rewrite.rewrite-tape-m-to-n-of-k-fn]`(ns,1,1,4); intersected with rulenum =
> min(? RULE ? ? ?*) (first block's tape-2 is this rule's marker). Returns min(ns & copy(lang)).
> Unlike notlongest/notleftmost there is no trailing-@0@ filter and no arrow_type parameter; lang
> is not consumed.

> [spec:foma:def:rewrite.rewr-unrewritten-fn]
> struct fsm *rewr_unrewritten(struct rewrite_batch *rb, struct fsm *lang)

> [spec:foma:sem:rewrite.rewr-unrewritten-fn]
> Maps a 1-tape language X to its "left entirely outside any rewrite" 4-tape encoding: composes
> lang with the filter ([0:"@O@"] [0:"@0@"] ANY [0:"@ID@"])* (ANY = rb->ANY) and takes the lower
> projection — every symbol x of X becomes the block @O@ @0@ x @ID@. Returns the minimized result;
> consumes lang. Used to forbid unrewritten centers for obligatory rules.

> [spec:foma:def:rewrite.rewrite-add-special-syms-fn]
> void rewrite_add_special_syms(struct rewrite_batch *rb, struct fsm *net)

> [spec:foma:sem:rewrite.rewrite-add-special-syms-fn]
> Mutates net in place (no-op when net is NULL). First sigma_substitute(".#.", "@#@") — the parser
> boundary symbol is renamed to the internal boundary, because fsm_merge_sigma treats .#.
> specially, which is unwanted here. Then, for each entry of the file-global table
> specialsymbols = {"@0@","@O@","@I@","@I[@","@I[]@","@I]@","@ID@","@#@"}, sigma_add it if
> sigma_find returns -1; then unconditionally sigma_add all rb->num_rules rule-name markers
> ("@#0001@"...); finally sigma_sort(net). Ensures ? / ?:? in user nets never match the auxiliary
> alphabet during the construction.

> [spec:foma:def:rewrite.rewrite-align-fn]
> struct fsm *rewrite_align(struct fsm *upper, struct fsm *lower)

> [spec:foma:sem:rewrite.rewrite-align-fn]
> Aligns two 1-tape languages into a flattened 2-tape (alternating symbol-pair) language. first =
> `[spec:foma:sem:rewrite.rewrite-tape-m-to-n-of-k-fn]`(upper "@0@"*, 1,1,2) — tape 1 spells a word
> of upper padded with trailing @0@; second = tape_m_to_n_of_k(lower "@0@"*, 2,2,2); third = regex
> `~[[? ?]* "@0@" "@0@" [? ?]*]` — no pair position may be epsilon on both tapes. align =
> min(third & first & second). Then identity handling: rename @_IDENTITY_SYMBOL_@ to @UNK@
> (fsm_substitute_symbol), compose with regex `[? ? | "@UNK@" "@UNK@":"@ID@"]*` and take the lower
> projection — wherever BOTH members of a pair were the identity symbol, the second becomes @ID@,
> recording an identity correspondence; finally rename @UNK@ back to @_IDENTITY_SYMBOL_@. Returns
> the minimized result; consumes upper and lower.

> [spec:foma:def:rewrite.rewrite-align-markup-fn]
> struct fsm *rewrite_align_markup(struct fsm *upper, struct fsm *lower1, struct fsm *lower2)

> [spec:foma:sem:rewrite.rewrite-align-markup-fn]
> Alignment for markup rules A -> B ... C, producing the flattened 2-tape pair language
> min([Tape1of2("@0@"*) & Tape2of2(lower1)] [Tape1of2(upper) & Tape2of2("@ID@"*)]
> [Tape1of2("@0@"*) & Tape2of2(lower2)]) — B (lower1) is inserted before the center (upper side
> all epsilon), every center symbol maps to @ID@ (copied unchanged to the output), and C (lower2)
> is inserted after. Each TapeXof2 is built via
> `[spec:foma:sem:rewrite.rewrite-tape-m-to-n-of-k-fn]`. Then applies exactly the same
> @UNK@/@ID@ identity-marking pass as `[spec:foma:sem:rewrite.rewrite-align-fn]` (substitute
> @_IDENTITY_SYMBOL_@ to @UNK@, compose with `[? ? | "@UNK@" "@UNK@":"@ID@"]*`, lower, substitute
> back). Returns minimized; consumes upper, lower1, and lower2.

> [spec:foma:def:rewrite.rewrite-any-4tape-fn]
> struct fsm *rewrite_any_4tape(struct rewrite_batch *rb)

> [spec:foma:sem:rewrite.rewrite-any-4tape-fn]
> The language of ALL well-formed 4-tape block strings, lazily built once and cached in
> rb->Any4Tape; every call returns fsm_copy of the cache. Formula:
> min(( [@O@ @0@ (ANY|@#@) @ID@] | [ISyms Rulenames (ANY|@0@) (ANY|@ID@|@0@)] )*) — an outside
> block carries a real symbol or the boundary on tape 3, copied by @ID@ on tape 4; an inside block
> carries any I-symbol on tape 1, any rule marker on tape 2, a real-or-epsilon tape 3 and a
> real/@ID@/epsilon tape 4. ANY is rb->ANY, ISyms/Rulenames the cached unions from rb.

> [spec:foma:def:rewrite.rewrite-batch]
> struct rewrite_batch {
>   struct rewrite_set *rewrite_set;
>   struct fsm *Rulenames;
>   struct fsm *ISyms;
>   struct fsm *ANY;
>   struct fsm *IOpen;
>   struct fsm *IClose;
>   struct fsm *ITape;
>   struct fsm *Any4Tape;
>   struct fsm *Epextend;
>   int num_rules;
>   char (*namestrings)[8];
> }

> [spec:foma:def:rewrite.rewrite-cleanup-fn]
> void rewrite_cleanup(struct rewrite_batch *rb)

> [spec:foma:sem:rewrite.rewrite-cleanup-fn]
> Tears down the rewrite_batch: fsm_destroy each of Rulenames, ISyms, ANY, IOpen, IClose, ITape,
> Any4Tape, Epextend if non-NULL (IOpen and IClose are declared but never assigned anywhere, so
> they are always NULL), free(rb->namestrings) if non-NULL, then free(rb). Note it does not touch
> rb->rewrite_set — the caller's rule structures survive.

> [spec:foma:def:rewrite.rewrite-cp-fn]
> struct fsm *rewrite_cp(struct rewrite_batch *rb, struct fsm *upper, struct fsm *lower, int rule_number)

> [spec:foma:sem:rewrite.rewrite-cp-fn]
> 4-tape cross-product of one A -> B rule center: Aligned =
> `[spec:foma:sem:rewrite.rewrite-align-fn]`(upper, lower) lifted to tapes 3-4 of 4 via
> `[spec:foma:sem:rewrite.rewrite-tape-m-to-n-of-k-fn]`(Aligned, 3, 4, 4); threetape =
> min(Aligned & `[spec:foma:sem:rewrite.rewrite-itape-fn]`(rb)) imposing the tape-1 bracket
> discipline; rulenumtape = tape_m_to_n_of_k(min(RULE_rule_number*), 2, 2, 4) putting this rule's
> marker on tape 2 of every block. Returns min(threetape & rulenumtape). Consumes upper and lower.

> [spec:foma:def:rewrite.rewrite-cp-markup-fn]
> struct fsm *rewrite_cp_markup(struct rewrite_batch *rb, struct fsm *upper, struct fsm *lower1, struct fsm *lower2, int rule_number)

> [spec:foma:sem:rewrite.rewrite-cp-markup-fn]
> Identical to `[spec:foma:sem:rewrite.rewrite-cp-fn]` except the alignment step is
> `[spec:foma:sem:rewrite.rewrite-align-markup-fn]`(upper, lower1, lower2) (for A -> B ... C
> rules): lift the alignment to tapes 3-4 of 4, intersect with rewrite_itape(rb), intersect with
> this rule's marker spelled on tape 2 of every block, minimize and return. Consumes all three
> arguments.

> [spec:foma:def:rewrite.rewrite-cp-transducer-fn]
> struct fsm *rewrite_cp_transducer(struct rewrite_batch *rb, struct fsm *t, int rule_number)

> [spec:foma:sem:rewrite.rewrite-cp-transducer-fn]
> Cross-product for a rule whose center is already a transducer (T(x)-type rule): instead of
> aligning two languages, Aligned = fsm_flatten(t, fsm_symbol("@0@")) — flattens the transducer
> into the 2-tape pair-string form with @0@ standing for epsilon. Then exactly as
> `[spec:foma:sem:rewrite.rewrite-cp-fn]`: lift to tapes 3-4 of 4, intersect with
> rewrite_itape(rb), intersect with the rule marker on tape 2 (min(RULE*) lifted to tape 2 of 4),
> minimize and return. Consumes t.

> [spec:foma:def:rewrite.rewrite-epextend-fn]
> struct fsm *rewrite_epextend(struct rewrite_batch *rb)

> [spec:foma:sem:rewrite.rewrite-epextend-fn]
> The "epenthesis extension" language EP: the block sequences that may legitimately sit
> immediately next to a dotted-rule ([..]) center. Lazily built, cached in rb->Epextend, copy
> returned per call. EP = one | two | three where:
> one = a single outside block @O@ @0@ (ANY|@#@) @ID@;
> two = a single-block match @I[]@ Rulenames ANY (@0@|@ID@|ANY) — tape-3 is a REAL symbol here;
> three = (threea threeb* threec) & ~[[? ? "@0@" ?]*], with threea = @I[@ Rulenames (ANY|@0@)
> (@0@|@ID@|ANY), threeb = the same block shape with @I@, threec = the same with @I]@ — a complete
> multi-block match whose tape-3 (upper) side is not entirely epsilon (the ~[[? ? "@0@" ?]*]
> intersection). All pieces use rb->ANY / rb->Rulenames; result minimized. (threeb carries the
> Kleene star itself; threea/threec occur exactly once.)

> [spec:foma:def:rewrite.rewrite-itape-fn]
> struct fsm *rewrite_itape(struct rewrite_batch *rb)

> [spec:foma:sem:rewrite.rewrite-itape-fn]
> The tape-1 bracket discipline of a single rewrite region over full 4-symbol blocks, lazily
> parsed once and cached in rb->ITape; a copy is returned per call. Literal regex:
> `["@I[]@" ? ? ? | "@I[@" ? ? ? ["@I@" ? ? ?]* "@I]@" ? [?-"@0@"] ?] ["@I]@" ? "@0@" ?]* | 0`
> — either a one-block match (@I[]@), or an opening block, any number of inside blocks, and a
> closing block whose tape-3 (upper symbol) is not epsilon; in both cases optionally followed by
> extra @I]@-marked blocks whose tape-3 IS @0@ (trailing epsilon-input output material); or the
> empty string.

> [spec:foma:def:rewrite.rewrite-lower-fn]
> struct fsm *rewrite_lower(struct rewrite_batch *rb, struct fsm *lower)

> [spec:foma:sem:rewrite.rewrite-lower-fn]
> Encodes a 1-tape language as its appearance on the OUTPUT side of the 4-tape encoding (used for
> lower-side contexts). Filter = min((One | Two | Three)*) where, writing 0:X for inserted
> material: One = [0:@O@] [0:@0@] (@#@|ANY) [0:@ID@] — an outside block consuming one argument
> symbol on tape 3, whose output equals it via @ID@; Two = [0:ISyms] [0:Rulenames] [0:(ANY|@0@)]
> ANY — an inside block consuming the argument symbol on tape 4, tape 3 arbitrary; Three =
> [0:ISyms] [0:Rulenames] [0:ANY] [0:@0@] — a fully inserted inside block whose tape-4 is epsilon
> (contributes no output symbol). Returns min(lower(lower_arg .o. Filter)). Consumes the argument.

> [spec:foma:def:rewrite.rewrite-tape-m-to-n-of-k-fn]
> struct fsm *rewrite_tape_m_to_n_of_k(struct fsm *lang, int m, int n, int k)

> [spec:foma:sem:rewrite.rewrite-tape-m-to-n-of-k-fn]
> Lifts a language over tape positions m..n (1-indexed, inclusive) into the k-tape block encoding:
> returns min(lower(lang .o. ( [0:?]^(m-1) ?^(n-m+1) [0:?]^(k-n) )*)) built with fsm_concat_n and
> fsm_cross_product(empty_string, identity) — each k-symbol block has positions m..n spelled by
> successive symbols of lang and every other position filled with an arbitrary inserted symbol.
> m == n selects a single tape (e.g. (X,1,1,4) = "X on tape 1 of 4"). Consumes lang.

> [spec:foma:def:rewrite.rewrite-two-level-fn]
> struct fsm *rewrite_two_level(struct rewrite_batch *rb, struct fsm *lang, int rightside)

> [spec:foma:sem:rewrite.rewrite-two-level-fn]
> Encodes a two-level context (the context is itself a transducer relating upper and lower):
> Lower = `[spec:foma:sem:rewrite.rewrite-lower-fn]`(min(fsm_lower(copy(lang)))), Upper =
> `[spec:foma:sem:rewrite.rewrite-upper-fn]`(min(fsm_upper(lang))). With Any4 = fresh copies from
> `[spec:foma:sem:rewrite.rewrite-any-4tape-fn]`: for a right context (rightside == 1) returns
> min((Lower Any4) & (Upper Any4)); for a left context (rightside == 0) returns
> min((Any4 Lower) & (Any4 Upper)) — both projections are anchored at the edge adjacent to the
> rewrite site so they constrain the same stretch. Consumes lang.

> [spec:foma:def:rewrite.rewrite-upper-fn]
> struct fsm *rewrite_upper(struct rewrite_batch *rb, struct fsm *upper)

> [spec:foma:sem:rewrite.rewrite-upper-fn]
> Encodes a 1-tape language as its appearance on the INPUT side of the 4-tape encoding (used for
> upper-side contexts). Filter = min((One | Two | Three)*) with 0:X = inserted: One = [0:@O@]
> [0:@0@] (@#@|ANY) [0:@ID@] — outside block consuming one argument symbol on tape 3; Two =
> [0:ISyms] [0:Rulenames] [0:@0@] [0:ANY] — fully inserted inside block whose tape-3 is epsilon
> (contributes no input symbol, arbitrary output); Three = [0:ISyms] [0:Rulenames] ANY
> [0:(@0@|ANY|@ID@)] — inside block consuming the argument symbol on tape 3, arbitrary tape 4.
> Returns min(lower(upper .o. Filter)). Consumes the argument.


# foma/rewrite.c

> [spec:foma:def:rewrite.fsm-clear-contexts-fn]
> void fsm_clear_contexts(struct fsmcontexts *contexts)

> [spec:foma:sem:rewrite.fsm-clear-contexts-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.fsm-rewrite-fn]
> struct fsm *fsm_rewrite(struct rewrite_set *all_rules)

> [spec:foma:sem:rewrite.fsm-rewrite-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewr-contains-fn]
> struct fsm *rewr_contains(struct rewrite_batch *rb, struct fsm *lang)

> [spec:foma:sem:rewrite.rewr-contains-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewr-context-restrict-fn]
> struct fsm *rewr_context_restrict(struct rewrite_batch *rb, struct fsm *X, struct fsmcontexts *LR)

> [spec:foma:sem:rewrite.rewr-context-restrict-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewr-notleftmost-fn]
> struct fsm *rewr_notleftmost(struct rewrite_batch *rb, struct fsm *lang, int rule_number, int arrow_type)

> [spec:foma:sem:rewrite.rewr-notleftmost-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewr-notlongest-fn]
> struct fsm *rewr_notlongest(struct rewrite_batch *rb, struct fsm *lang, int rule_number, int arrow_type)

> [spec:foma:sem:rewrite.rewr-notlongest-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewr-notshortest-fn]
> struct fsm *rewr_notshortest(struct rewrite_batch *rb, struct fsm *lang, int rule_number)

> [spec:foma:sem:rewrite.rewr-notshortest-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewr-unrewritten-fn]
> struct fsm *rewr_unrewritten(struct rewrite_batch *rb, struct fsm *lang)

> [spec:foma:sem:rewrite.rewr-unrewritten-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-add-special-syms-fn]
> void rewrite_add_special_syms(struct rewrite_batch *rb, struct fsm *net)

> [spec:foma:sem:rewrite.rewrite-add-special-syms-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-align-fn]
> struct fsm *rewrite_align(struct fsm *upper, struct fsm *lower)

> [spec:foma:sem:rewrite.rewrite-align-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-align-markup-fn]
> struct fsm *rewrite_align_markup(struct fsm *upper, struct fsm *lower1, struct fsm *lower2)

> [spec:foma:sem:rewrite.rewrite-align-markup-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-any-4tape-fn]
> struct fsm *rewrite_any_4tape(struct rewrite_batch *rb)

> [spec:foma:sem:rewrite.rewrite-any-4tape-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

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
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-cp-fn]
> struct fsm *rewrite_cp(struct rewrite_batch *rb, struct fsm *upper, struct fsm *lower, int rule_number)

> [spec:foma:sem:rewrite.rewrite-cp-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-cp-markup-fn]
> struct fsm *rewrite_cp_markup(struct rewrite_batch *rb, struct fsm *upper, struct fsm *lower1, struct fsm *lower2, int rule_number)

> [spec:foma:sem:rewrite.rewrite-cp-markup-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-cp-transducer-fn]
> struct fsm *rewrite_cp_transducer(struct rewrite_batch *rb, struct fsm *t, int rule_number)

> [spec:foma:sem:rewrite.rewrite-cp-transducer-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-epextend-fn]
> struct fsm *rewrite_epextend(struct rewrite_batch *rb)

> [spec:foma:sem:rewrite.rewrite-epextend-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-itape-fn]
> struct fsm *rewrite_itape(struct rewrite_batch *rb)

> [spec:foma:sem:rewrite.rewrite-itape-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-lower-fn]
> struct fsm *rewrite_lower(struct rewrite_batch *rb, struct fsm *lower)

> [spec:foma:sem:rewrite.rewrite-lower-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-tape-m-to-n-of-k-fn]
> struct fsm *rewrite_tape_m_to_n_of_k(struct fsm *lang, int m, int n, int k)

> [spec:foma:sem:rewrite.rewrite-tape-m-to-n-of-k-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-two-level-fn]
> struct fsm *rewrite_two_level(struct rewrite_batch *rb, struct fsm *lang, int rightside)

> [spec:foma:sem:rewrite.rewrite-two-level-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:rewrite.rewrite-upper-fn]
> struct fsm *rewrite_upper(struct rewrite_batch *rb, struct fsm *upper)

> [spec:foma:sem:rewrite.rewrite-upper-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.


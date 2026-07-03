# foma/constructions.c

> [spec:foma:def:constructions.add-fsm-arc-fn]
> int add_fsm_arc(struct fsm_state *fsm, int offset, int state_no, int in, int out, int target, int final_state, int start_state)

> [spec:foma:sem:constructions.add-fsm-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.add-to-mergesigma-fn]
> struct mergesigma *add_to_mergesigma(struct mergesigma *msigma, struct sigma *sigma, short presence)

> [spec:foma:sem:constructions.add-to-mergesigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.copy-mergesigma-fn]
> struct sigma *copy_mergesigma(struct mergesigma *mergesigma)

> [spec:foma:sem:constructions.copy-mergesigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-add-loop-fn]
> struct fsm *fsm_add_loop(struct fsm *net, struct fsm *marker, int finals)

> [spec:foma:sem:constructions.fsm-add-loop-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-add-sink-fn]
> struct fsm *fsm_add_sink(struct fsm *net, int final)

> [spec:foma:sem:constructions.fsm-add-sink-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-add-to-states-fn]
> static void fsm_add_to_states(struct fsm *net, int add)

> [spec:foma:sem:constructions.fsm-add-to-states-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-bimachine-fn]
> struct fsm *fsm_bimachine(struct fsm *net)

> [spec:foma:sem:constructions.fsm-bimachine-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-close-sigma-fn]
> struct fsm *fsm_close_sigma(struct fsm *net, int mode)

> [spec:foma:sem:constructions.fsm-close-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-compact-fn]
> void fsm_compact(struct fsm *net)

> [spec:foma:sem:constructions.fsm-compact-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-complement-fn]
> struct fsm *fsm_complement(struct fsm *net)

> [spec:foma:sem:constructions.fsm-complement-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-complete-fn]
> struct fsm *fsm_complete(struct fsm *net)

> [spec:foma:sem:constructions.fsm-complete-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-completes-fn]
> struct fsm *fsm_completes(struct fsm *net, int operation)

> [spec:foma:sem:constructions.fsm-completes-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-compose-fn]
> struct fsm *fsm_compose(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-compose-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-concat-fn]
> struct fsm *fsm_concat(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-concat-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-concat-m-n-fn]
> struct fsm *fsm_concat_m_n(struct fsm *net1, int m, int n)

> [spec:foma:sem:constructions.fsm-concat-m-n-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-concat-n-fn]
> struct fsm *fsm_concat_n(struct fsm *net1, int n)

> [spec:foma:sem:constructions.fsm-concat-n-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-contains-fn]
> struct fsm *fsm_contains(struct fsm *net)

> [spec:foma:sem:constructions.fsm-contains-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-contains-one-fn]
> struct fsm *fsm_contains_one(struct fsm *net)

> [spec:foma:sem:constructions.fsm-contains-one-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-contains-opt-one-fn]
> struct fsm *fsm_contains_opt_one(struct fsm *net)

> [spec:foma:sem:constructions.fsm-contains-opt-one-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-context-restrict-fn]
> struct fsm *fsm_context_restrict(struct fsm *X, struct fsmcontexts *LR)

> [spec:foma:sem:constructions.fsm-context-restrict-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-count-fn]
> void fsm_count(struct fsm *net)

> [spec:foma:sem:constructions.fsm-count-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-count-states-fn]
> int fsm_count_states(struct fsm_state *fsm)

> [spec:foma:sem:constructions.fsm-count-states-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-cross-product-fn]
> struct fsm *fsm_cross_product(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-cross-product-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-equal-substrings-fn]
> struct fsm *fsm_equal_substrings(struct fsm *net, struct fsm *left, struct fsm *right)

> [spec:foma:sem:constructions.fsm-equal-substrings-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-equivalent-fn]
> int fsm_equivalent(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-equivalent-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-escape-fn]
> struct fsm *fsm_escape(char *symbol)

> [spec:foma:sem:constructions.fsm-escape-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-explode-fn]
> struct fsm *fsm_explode(char *symbol)

> [spec:foma:sem:constructions.fsm-explode-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-flatten-fn]
> struct fsm *fsm_flatten(struct fsm *net, struct fsm *epsilon)

> [spec:foma:sem:constructions.fsm-flatten-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-follows-fn]
> struct fsm *fsm_follows(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-follows-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-ignore-fn]
> struct fsm *fsm_ignore(struct fsm *net1, struct fsm *net2, int operation)

> [spec:foma:sem:constructions.fsm-ignore-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-intersect-fn]
> struct fsm *fsm_intersect(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-intersect-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-invert-fn]
> struct fsm *fsm_invert(struct fsm *net)

> [spec:foma:sem:constructions.fsm-invert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-kleene-closure-fn]
> struct fsm *fsm_kleene_closure(struct fsm *net, int operation)

> [spec:foma:sem:constructions.fsm-kleene-closure-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-kleene-plus-fn]
> struct fsm *fsm_kleene_plus(struct fsm *net)

> [spec:foma:sem:constructions.fsm-kleene-plus-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-kleene-star-fn]
> struct fsm *fsm_kleene_star(struct fsm *net)

> [spec:foma:sem:constructions.fsm-kleene-star-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-left-rewr-fn]
> struct fsm *fsm_left_rewr(struct fsm *net, struct fsm *rewr)

> [spec:foma:sem:constructions.fsm-left-rewr-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-lenient-compose-fn]
> struct fsm *fsm_lenient_compose(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-lenient-compose-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-letter-machine-fn]
> struct fsm *fsm_letter_machine(struct fsm *net)

> [spec:foma:sem:constructions.fsm-letter-machine-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-mark-fsm-tail-fn]
> struct fsm *fsm_mark_fsm_tail(struct fsm *net, struct fsm *marker)

> [spec:foma:sem:constructions.fsm-mark-fsm-tail-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-merge-sigma-fn]
> void fsm_merge_sigma(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-merge-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-minus-fn]
> struct fsm *fsm_minus(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-minus-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-network-to-char-fn]
> char *fsm_network_to_char(struct fsm *net)

> [spec:foma:sem:constructions.fsm-network-to-char-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-optionality-fn]
> struct fsm *fsm_optionality(struct fsm *net)

> [spec:foma:sem:constructions.fsm-optionality-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-precedes-fn]
> struct fsm *fsm_precedes(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-precedes-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-priority-union-lower-fn]
> struct fsm *fsm_priority_union_lower(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-priority-union-lower-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-priority-union-upper-fn]
> struct fsm *fsm_priority_union_upper(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-priority-union-upper-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-quotient-interleave-fn]
> struct fsm *fsm_quotient_interleave(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-quotient-interleave-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-quotient-left-fn]
> struct fsm *fsm_quotient_left(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-quotient-left-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-quotient-right-fn]
> struct fsm *fsm_quotient_right(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-quotient-right-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-sequentialize-fn]
> struct fsm *fsm_sequentialize(struct fsm *net)

> [spec:foma:sem:constructions.fsm-sequentialize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-shuffle-fn]
> struct fsm *fsm_shuffle(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-shuffle-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-simple-replace-fn]
> struct fsm *fsm_simple_replace(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-simple-replace-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-sort-lines-fn]
> void fsm_sort_lines(struct fsm *net)

> [spec:foma:sem:constructions.fsm-sort-lines-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-substitute-label-fn]
> struct fsm *fsm_substitute_label(struct fsm *net, char *original, struct fsm *substitute)

> [spec:foma:sem:constructions.fsm-substitute-label-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-substitute-symbol-fn]
> struct fsm *fsm_substitute_symbol(struct fsm *net, char *original, char *substitute)

> [spec:foma:sem:constructions.fsm-substitute-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-symbol-fn]
> struct fsm *fsm_symbol(char *symbol)

> [spec:foma:sem:constructions.fsm-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-symbol-occurs-fn]
> int fsm_symbol_occurs(struct fsm *net, char *symbol, int side)

> [spec:foma:sem:constructions.fsm-symbol-occurs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-term-negation-fn]
> struct fsm *fsm_term_negation(struct fsm *net1)

> [spec:foma:sem:constructions.fsm-term-negation-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-unflatten-fn]
> struct fsm *fsm_unflatten(struct fsm *net, char *epsilon_sym, char *repeat_sym)

> [spec:foma:sem:constructions.fsm-unflatten-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-union-fn]
> struct fsm *fsm_union(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:constructions.fsm-union-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-universal-fn]
> struct fsm *fsm_universal()

> [spec:foma:sem:constructions.fsm-universal-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.fsm-update-flags-fn]
> void fsm_update_flags(struct fsm *net, int det, int pru, int min, int eps, int loop, int completed)

> [spec:foma:sem:constructions.fsm-update-flags-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.init-state-pointers-fn]
> struct state_arr *init_state_pointers(struct fsm_state *fsm_state)

> [spec:foma:sem:constructions.init-state-pointers-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.mergesigma]
> struct mergesigma {
>   char *symbol;
>   unsigned char presence;
>   int number;
>   struct mergesigma *next;
> }

> [spec:foma:def:constructions.sort-cmp-fn]
> int sort_cmp(const void *a, const void *b)

> [spec:foma:sem:constructions.sort-cmp-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.state-arr]
> struct state_arr {
>   int final;
>   int start;
>   struct fsm_state *transitions;
> }

> [spec:foma:def:constructions.triplet-hash-find-fn]
> int triplet_hash_find(struct triplethash *th, int a, int b, int c)

> [spec:foma:sem:constructions.triplet-hash-find-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.triplet-hash-free-fn]
> void triplet_hash_free(struct triplethash *th)

> [spec:foma:sem:constructions.triplet-hash-free-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.triplet-hash-init-fn]
> struct triplethash *triplet_hash_init()

> [spec:foma:sem:constructions.triplet-hash-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.triplet-hash-insert-fn]
> int triplet_hash_insert(struct triplethash *th, int a, int b, int c)

> [spec:foma:sem:constructions.triplet-hash-insert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.triplet-hash-insert-with-key-fn]
> void triplet_hash_insert_with_key(struct triplethash *th, int a, int b, int c, int key)

> [spec:foma:sem:constructions.triplet-hash-insert-with-key-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.triplet-hash-rehash-fn]
> void triplet_hash_rehash(struct triplethash *th)

> [spec:foma:sem:constructions.triplet-hash-rehash-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.triplethash]
> struct triplethash {
>   struct triplethash_triplets *triplets;
>   unsigned int tablesize;
>   int occupancy;
> }

> [spec:foma:def:constructions.triplethash-hashf-fn]
> unsigned int triplethash_hashf(int a, int b, int c)

> [spec:foma:sem:constructions.triplethash-hashf-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:constructions.triplethash-triplets]
> struct triplethash_triplets {
>   int a;
>   int b;
>   int c;
>   int key;
> }


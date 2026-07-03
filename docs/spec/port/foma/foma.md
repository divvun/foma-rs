# foma/foma.c, foma/foma.h

> [spec:foma:def:foma.add-history-fn]
> extern int add_history (const char *)

> [spec:foma:sem:foma.add-history-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.add-quantifier-fn]
> void add_quantifier (char *string)

> [spec:foma:sem:foma.add-quantifier-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.clear-quantifiers-fn]
> void clear_quantifiers()

> [spec:foma:sem:foma.clear-quantifiers-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.count-quantifiers-fn]
> int count_quantifiers()

> [spec:foma:sem:foma.count-quantifiers-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.find-quantifier-fn]
> char *find_quantifier(char *string)

> [spec:foma:sem:foma.find-quantifier-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.fsm-options]
> struct _fsm_options {
>   _Bool skip_word_boundary_marker;
> }

> [spec:foma:def:foma.iface-ambiguous-upper-fn]
> void iface_ambiguous_upper(void)

> [spec:foma:sem:foma.iface-ambiguous-upper-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-apply-down-fn]
> void iface_apply_down(char *word)

> [spec:foma:sem:foma.iface-apply-down-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-apply-file-fn]
> int iface_apply_file(char *infilename, char *outfilename, int direction)

> [spec:foma:sem:foma.iface-apply-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-apply-med-fn]
> void iface_apply_med(char *word)

> [spec:foma:sem:foma.iface-apply-med-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-apply-random-fn]
> void iface_apply_random(char *(*applyer)(struct apply_handle *h), int limit)

> [spec:foma:sem:foma.iface-apply-random-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-apply-set-params-fn]
> void iface_apply_set_params(struct apply_handle *h)

> [spec:foma:sem:foma.iface-apply-set-params-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-apply-up-fn]
> void iface_apply_up(char *word)

> [spec:foma:sem:foma.iface-apply-up-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-apropos-fn]
> void iface_apropos(char *s)

> [spec:foma:sem:foma.iface-apropos-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-close-fn]
> void iface_close(void)

> [spec:foma:sem:foma.iface-close-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-compact-fn]
> void iface_compact(void)

> [spec:foma:sem:foma.iface-compact-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-complete-fn]
> void iface_complete(void)

> [spec:foma:sem:foma.iface-complete-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-compose-fn]
> void iface_compose(void)

> [spec:foma:sem:foma.iface-compose-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-conc-fn]
> void iface_conc(void)

> [spec:foma:sem:foma.iface-conc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-crossproduct-fn]
> void iface_crossproduct(void)

> [spec:foma:sem:foma.iface-crossproduct-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-determinize-fn]
> void iface_determinize(void)

> [spec:foma:sem:foma.iface-determinize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-eliminate-flag-fn]
> void iface_eliminate_flag(char *name)

> [spec:foma:sem:foma.iface-eliminate-flag-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-eliminate-flags-fn]
> void iface_eliminate_flags(void)

> [spec:foma:sem:foma.iface-eliminate-flags-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-extract-ambiguous-fn]
> void iface_extract_ambiguous(void)

> [spec:foma:sem:foma.iface-extract-ambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-extract-number-fn]
> int iface_extract_number(char *s)

> [spec:foma:sem:foma.iface-extract-number-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-extract-unambiguous-fn]
> void iface_extract_unambiguous(void)

> [spec:foma:sem:foma.iface-extract-unambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-factorize-fn]
> void iface_factorize(void)

> [spec:foma:sem:foma.iface-factorize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-help-fn]
> void iface_help(void)

> [spec:foma:sem:foma.iface-help-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-help-search-fn]
> void iface_help_search(char *s)

> [spec:foma:sem:foma.iface-help-search-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-ignore-fn]
> void iface_ignore(void)

> [spec:foma:sem:foma.iface-ignore-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-intersect-fn]
> void iface_intersect(void)

> [spec:foma:sem:foma.iface-intersect-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-invert-fn]
> void iface_invert(void)

> [spec:foma:sem:foma.iface-invert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-label-net-fn]
> void iface_label_net(void)

> [spec:foma:sem:foma.iface-label-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-letter-machine-fn]
> void iface_letter_machine(void)

> [spec:foma:sem:foma.iface-letter-machine-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-load-defined-fn]
> void iface_load_defined(char *filename)

> [spec:foma:sem:foma.iface-load-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-load-stack-fn]
> void iface_load_stack(char *filename)

> [spec:foma:sem:foma.iface-load-stack-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-lower-side-fn]
> void iface_lower_side(void)

> [spec:foma:sem:foma.iface-lower-side-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-lower-words-fn]
> void iface_lower_words(int limit)

> [spec:foma:sem:foma.iface-lower-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-minimize-fn]
> void iface_minimize(void)

> [spec:foma:sem:foma.iface-minimize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-name-net-fn]
> void iface_name_net(char *name)

> [spec:foma:sem:foma.iface-name-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-negate-fn]
> void iface_negate(void)

> [spec:foma:sem:foma.iface-negate-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-one-plus-fn]
> void iface_one_plus(void)

> [spec:foma:sem:foma.iface-one-plus-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-pairs-file-fn]
> void iface_pairs_file(char *filename)

> [spec:foma:sem:foma.iface-pairs-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-pairs-fn]
> void iface_pairs(int limit)

> [spec:foma:sem:foma.iface-pairs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-pop-fn]
> void iface_pop(void)

> [spec:foma:sem:foma.iface-pop-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-cmatrix-att-fn]
> void iface_print_cmatrix_att(char *filename)

> [spec:foma:sem:foma.iface-print-cmatrix-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-cmatrix-fn]
> void iface_print_cmatrix(void)

> [spec:foma:sem:foma.iface-print-cmatrix-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-defined-fn]
> void iface_print_defined(void)

> [spec:foma:sem:foma.iface-print-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-dot-fn]
> void iface_print_dot(char *filename)

> [spec:foma:sem:foma.iface-print-dot-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-name-fn]
> void iface_print_name(void)

> [spec:foma:sem:foma.iface-print-name-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-net-fn]
> void iface_print_net(char *netname, char *filename)

> [spec:foma:sem:foma.iface-print-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-shortest-string-fn]
> void iface_print_shortest_string()

> [spec:foma:sem:foma.iface-print-shortest-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-shortest-string-size-fn]
> void iface_print_shortest_string_size()

> [spec:foma:sem:foma.iface-print-shortest-string-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-sigma-fn]
> void iface_print_sigma(void)

> [spec:foma:sem:foma.iface-print-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-print-stats-fn]
> void iface_print_stats(void)

> [spec:foma:sem:foma.iface-print-stats-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-prune-fn]
> void iface_prune(void)

> [spec:foma:sem:foma.iface-prune-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-quit-fn]
> void iface_quit(void)

> [spec:foma:sem:foma.iface-quit-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-random-lower-fn]
> void iface_random_lower(int limit)

> [spec:foma:sem:foma.iface-random-lower-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-random-pairs-fn]
> void iface_random_pairs(int limit)

> [spec:foma:sem:foma.iface-random-pairs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-random-upper-fn]
> void iface_random_upper(int limit)

> [spec:foma:sem:foma.iface-random-upper-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-random-words-fn]
> void iface_random_words(int limit)

> [spec:foma:sem:foma.iface-random-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-read-att-fn]
> int iface_read_att(char *filename)

> [spec:foma:sem:foma.iface-read-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-read-prolog-fn]
> int iface_read_prolog(char *filename)

> [spec:foma:sem:foma.iface-read-prolog-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-read-spaced-text-fn]
> int iface_read_spaced_text(char *filename)

> [spec:foma:sem:foma.iface-read-spaced-text-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-read-text-fn]
> int iface_read_text(char *filename)

> [spec:foma:sem:foma.iface-read-text-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-reverse-fn]
> void iface_reverse(void)

> [spec:foma:sem:foma.iface-reverse-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-rotate-fn]
> void iface_rotate(void)

> [spec:foma:sem:foma.iface-rotate-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-save-defined-fn]
> void iface_save_defined(char *filename)

> [spec:foma:sem:foma.iface-save-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-save-stack-fn]
> void iface_save_stack(char *filename)

> [spec:foma:sem:foma.iface-save-stack-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-sequentialize-fn]
> void iface_sequentialize(void)

> [spec:foma:sem:foma.iface-sequentialize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-set-variable-fn]
> void iface_set_variable(char *name, char *value)

> [spec:foma:sem:foma.iface-set-variable-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-show-variable-fn]
> void iface_show_variable(char *name)

> [spec:foma:sem:foma.iface-show-variable-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-show-variables-fn]
> void iface_show_variables(void)

> [spec:foma:sem:foma.iface-show-variables-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-shuffle-fn]
> void iface_shuffle(void)

> [spec:foma:sem:foma.iface-shuffle-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-sigma-net-fn]
> void iface_sigma_net()

> [spec:foma:sem:foma.iface-sigma-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-sort-fn]
> void iface_sort(void)

> [spec:foma:sem:foma.iface-sort-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-sort-input-fn]
> void iface_sort_input(void)

> [spec:foma:sem:foma.iface-sort-input-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-sort-output-fn]
> void iface_sort_output(void)

> [spec:foma:sem:foma.iface-sort-output-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-stack-check-fn]
> int iface_stack_check(int size)

> [spec:foma:sem:foma.iface-stack-check-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-substitute-defined-fn]
> void iface_substitute_defined (char *original, char *substitute)

> [spec:foma:sem:foma.iface-substitute-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-substitute-symbol-fn]
> void iface_substitute_symbol (char *original, char *substitute)

> [spec:foma:sem:foma.iface-substitute-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-test-equivalent-fn]
> void iface_test_equivalent(void)

> [spec:foma:sem:foma.iface-test-equivalent-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-test-functional-fn]
> void iface_test_functional(void)

> [spec:foma:sem:foma.iface-test-functional-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-test-identity-fn]
> void iface_test_identity(void)

> [spec:foma:sem:foma.iface-test-identity-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-test-lower-universal-fn]
> void iface_test_lower_universal(void)

> [spec:foma:sem:foma.iface-test-lower-universal-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-test-nonnull-fn]
> void iface_test_nonnull(void)

> [spec:foma:sem:foma.iface-test-nonnull-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-test-null-fn]
> void iface_test_null(void)

> [spec:foma:sem:foma.iface-test-null-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-test-sequential-fn]
> void iface_test_sequential(void)

> [spec:foma:sem:foma.iface-test-sequential-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-test-unambiguous-fn]
> void iface_test_unambiguous(void)

> [spec:foma:sem:foma.iface-test-unambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-test-upper-universal-fn]
> void iface_test_upper_universal(void)

> [spec:foma:sem:foma.iface-test-upper-universal-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-turn-fn]
> void iface_turn(void)

> [spec:foma:sem:foma.iface-turn-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-twosided-flags-fn]
> void iface_twosided_flags(void)

> [spec:foma:sem:foma.iface-twosided-flags-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-union-fn]
> void iface_union(void)

> [spec:foma:sem:foma.iface-union-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-upper-side-fn]
> void iface_upper_side(void)

> [spec:foma:sem:foma.iface-upper-side-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-upper-words-fn]
> void iface_upper_words(int limit)

> [spec:foma:sem:foma.iface-upper-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-view-fn]
> void iface_view(void)

> [spec:foma:sem:foma.iface-view-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-warranty-fn]
> void iface_warranty(void)

> [spec:foma:sem:foma.iface-warranty-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-words-file-fn]
> void iface_words_file(char *filename, int type)

> [spec:foma:sem:foma.iface-words-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-words-fn]
> void iface_words(int limit)

> [spec:foma:sem:foma.iface-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-write-att-fn]
> int iface_write_att(char *filename)

> [spec:foma:sem:foma.iface-write-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-write-prolog-fn]
> void iface_write_prolog(char *filename)

> [spec:foma:sem:foma.iface-write-prolog-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.iface-zero-plus-fn]
> void iface_zero_plus(void)

> [spec:foma:sem:foma.iface-zero-plus-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.main-fn]
> int main(int argc, char *argv[])

> [spec:foma:sem:foma.main-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.my-completion-fn]
> static char **my_completion(const char *text, int start, int end)

> [spec:foma:sem:foma.my-completion-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.my-generator-fn]
> char *my_generator(const char *text, int state)

> [spec:foma:sem:foma.my-generator-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.my-yyparse-fn]
> extern int my_yyparse(char *my_string)

> [spec:foma:sem:foma.my-yyparse-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.print-help-fn]
> void print_help()

> [spec:foma:sem:foma.print-help-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.print-stats-fn]
> int print_stats(struct fsm *net)

> [spec:foma:sem:foma.print-stats-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.purge-quantifier-fn]
> void purge_quantifier (char *string)

> [spec:foma:sem:foma.purge-quantifier-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.rl-gets-fn]
> char *rl_gets(char *prompt)

> [spec:foma:sem:foma.rl-gets-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-add-fn]
> int stack_add(struct fsm *fsm)

> [spec:foma:sem:foma.stack-add-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-clear-fn]
> int stack_clear()

> [spec:foma:sem:foma.stack-clear-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-entry]
> struct stack_entry {
>   int number;
>   struct apply_handle *ah;
>   struct apply_med_handle *amedh;
>   struct fsm *fsm;
>   struct stack_entry *next;
>   struct stack_entry *previous;
> }

> [spec:foma:def:foma.stack-find-bottom-fn]
> struct stack_entry *stack_find_bottom()

> [spec:foma:sem:foma.stack-find-bottom-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-find-second-fn]
> struct stack_entry *stack_find_second()

> [spec:foma:sem:foma.stack-find-second-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-find-top-fn]
> struct stack_entry *stack_find_top()

> [spec:foma:sem:foma.stack-find-top-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-get-ah-fn]
> struct apply_handle *stack_get_ah()

> [spec:foma:sem:foma.stack-get-ah-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-get-med-ah-fn]
> struct apply_med_handle *stack_get_med_ah()

> [spec:foma:sem:foma.stack-get-med-ah-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-init-fn]
> int stack_init()

> [spec:foma:sem:foma.stack-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-isempty-fn]
> int stack_isempty()

> [spec:foma:sem:foma.stack-isempty-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-pop-fn]
> struct fsm *stack_pop()

> [spec:foma:sem:foma.stack-pop-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-print-fn]
> int stack_print()

> [spec:foma:sem:foma.stack-print-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-rotate-fn]
> int stack_rotate()

> [spec:foma:sem:foma.stack-rotate-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-size-fn]
> int stack_size()

> [spec:foma:sem:foma.stack-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.stack-turn-fn]
> int stack_turn()

> [spec:foma:sem:foma.stack-turn-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.union-quantifiers-fn]
> struct fsm *union_quantifiers()

> [spec:foma:sem:foma.union-quantifiers-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.view-net-fn]
> int view_net(struct fsm *net)

> [spec:foma:sem:foma.view-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:foma.xprintf-fn]
> void xprintf(char *string)

> [spec:foma:sem:foma.xprintf-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.


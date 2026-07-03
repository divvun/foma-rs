# foma/iface.c

> [spec:foma:def:iface.foma-net-print-fn]
> extern int foma_net_print(struct fsm *net, gzFile outfile)

> [spec:foma:sem:iface.foma-net-print-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.g-v]
> struct g_v {
>   void *ptr;
>   char *name;
>   int type;
> }

> [spec:foma:def:iface.global-help]
> struct global_help {
>   char *name;
>   char *help;
>   char *longhelp;
> }

> [spec:foma:def:iface.iface-ambiguous-upper-fn]
> void iface_ambiguous_upper()

> [spec:foma:sem:iface.iface-ambiguous-upper-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-apply-down-fn]
> void iface_apply_down(char *word)

> [spec:foma:sem:iface.iface-apply-down-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-apply-file-fn]
> int iface_apply_file(char *infilename, char *outfilename, int direction)

> [spec:foma:sem:iface.iface-apply-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-apply-med-fn]
> void iface_apply_med(char *word)

> [spec:foma:sem:iface.iface-apply-med-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-apply-random-fn]
> void iface_apply_random(char *(*applyer)(struct apply_handle *h), int limit)

> [spec:foma:sem:iface.iface-apply-random-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-apply-set-params-fn]
> void iface_apply_set_params(struct apply_handle *h)

> [spec:foma:sem:iface.iface-apply-set-params-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-apply-up-fn]
> void iface_apply_up(char *word)

> [spec:foma:sem:iface.iface-apply-up-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-apropos-fn]
> void iface_apropos(char *s)

> [spec:foma:sem:iface.iface-apropos-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-close-fn]
> void iface_close()

> [spec:foma:sem:iface.iface-close-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-compact-fn]
> void iface_compact()

> [spec:foma:sem:iface.iface-compact-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-complete-fn]
> void iface_complete()

> [spec:foma:sem:iface.iface-complete-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-compose-fn]
> void iface_compose()

> [spec:foma:sem:iface.iface-compose-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-conc-fn]
> void iface_conc()

> [spec:foma:sem:iface.iface-conc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-crossproduct-fn]
> void iface_crossproduct()

> [spec:foma:sem:iface.iface-crossproduct-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-determinize-fn]
> void iface_determinize()

> [spec:foma:sem:iface.iface-determinize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-eliminate-flag-fn]
> void iface_eliminate_flag(char *name)

> [spec:foma:sem:iface.iface-eliminate-flag-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-eliminate-flags-fn]
> void iface_eliminate_flags()

> [spec:foma:sem:iface.iface-eliminate-flags-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-extract-ambiguous-fn]
> void iface_extract_ambiguous()

> [spec:foma:sem:iface.iface-extract-ambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-extract-number-fn]
> int iface_extract_number(char *s)

> [spec:foma:sem:iface.iface-extract-number-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-extract-unambiguous-fn]
> void iface_extract_unambiguous()

> [spec:foma:sem:iface.iface-extract-unambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-factorize-fn]
> void iface_factorize()

> [spec:foma:sem:iface.iface-factorize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-help-fn]
> void iface_help()

> [spec:foma:sem:iface.iface-help-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-help-search-fn]
> void iface_help_search(char *s)

> [spec:foma:sem:iface.iface-help-search-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-ignore-fn]
> void iface_ignore()

> [spec:foma:sem:iface.iface-ignore-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-intersect-fn]
> void iface_intersect()

> [spec:foma:sem:iface.iface-intersect-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-invert-fn]
> void iface_invert()

> [spec:foma:sem:iface.iface-invert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-label-net-fn]
> void iface_label_net()

> [spec:foma:sem:iface.iface-label-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-letter-machine-fn]
> void iface_letter_machine()

> [spec:foma:sem:iface.iface-letter-machine-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-load-defined-fn]
> void iface_load_defined(char *filename)

> [spec:foma:sem:iface.iface-load-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-load-stack-fn]
> void iface_load_stack(char *filename)

> [spec:foma:sem:iface.iface-load-stack-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-lower-side-fn]
> void iface_lower_side()

> [spec:foma:sem:iface.iface-lower-side-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-lower-words-fn]
> void iface_lower_words(int limit)

> [spec:foma:sem:iface.iface-lower-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-minimize-fn]
> void iface_minimize()

> [spec:foma:sem:iface.iface-minimize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-name-net-fn]
> void iface_name_net(char *name)

> [spec:foma:sem:iface.iface-name-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-negate-fn]
> void iface_negate()

> [spec:foma:sem:iface.iface-negate-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-one-plus-fn]
> void iface_one_plus()

> [spec:foma:sem:iface.iface-one-plus-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-pairs-call-fn]
> void iface_pairs_call(int limit, int random)

> [spec:foma:sem:iface.iface-pairs-call-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-pairs-file-fn]
> void iface_pairs_file(char *filename)

> [spec:foma:sem:iface.iface-pairs-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-pairs-fn]
> void iface_pairs(int limit)

> [spec:foma:sem:iface.iface-pairs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-pop-fn]
> void iface_pop()

> [spec:foma:sem:iface.iface-pop-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-bool-fn]
> void iface_print_bool(int value)

> [spec:foma:sem:iface.iface-print-bool-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-cmatrix-att-fn]
> void iface_print_cmatrix_att(char *filename)

> [spec:foma:sem:iface.iface-print-cmatrix-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-cmatrix-fn]
> void iface_print_cmatrix()

> [spec:foma:sem:iface.iface-print-cmatrix-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-defined-fn]
> void iface_print_defined()

> [spec:foma:sem:iface.iface-print-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-dot-fn]
> void iface_print_dot(char *filename)

> [spec:foma:sem:iface.iface-print-dot-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-name-fn]
> void iface_print_name()

> [spec:foma:sem:iface.iface-print-name-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-net-fn]
> void iface_print_net(char *netname, char *filename)

> [spec:foma:sem:iface.iface-print-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-shortest-string-fn]
> void iface_print_shortest_string()

> [spec:foma:sem:iface.iface-print-shortest-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-shortest-string-size-fn]
> void iface_print_shortest_string_size()

> [spec:foma:sem:iface.iface-print-shortest-string-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-sigma-fn]
> void iface_print_sigma()

> [spec:foma:sem:iface.iface-print-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-print-stats-fn]
> void iface_print_stats()

> [spec:foma:sem:iface.iface-print-stats-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-prune-fn]
> void iface_prune()

> [spec:foma:sem:iface.iface-prune-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-quit-fn]
> void iface_quit()

> [spec:foma:sem:iface.iface-quit-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-random-lower-fn]
> void iface_random_lower(int limit)

> [spec:foma:sem:iface.iface-random-lower-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-random-pairs-fn]
> void iface_random_pairs(int limit)

> [spec:foma:sem:iface.iface-random-pairs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-random-upper-fn]
> void iface_random_upper(int limit)

> [spec:foma:sem:iface.iface-random-upper-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-random-words-fn]
> void iface_random_words(int limit)

> [spec:foma:sem:iface.iface-random-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-read-att-fn]
> int iface_read_att(char *filename)

> [spec:foma:sem:iface.iface-read-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-read-prolog-fn]
> int iface_read_prolog(char *filename)

> [spec:foma:sem:iface.iface-read-prolog-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-read-spaced-text-fn]
> int iface_read_spaced_text(char *filename)

> [spec:foma:sem:iface.iface-read-spaced-text-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-read-text-fn]
> int iface_read_text(char *filename)

> [spec:foma:sem:iface.iface-read-text-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-reverse-fn]
> void iface_reverse()

> [spec:foma:sem:iface.iface-reverse-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-rotate-fn]
> void iface_rotate()

> [spec:foma:sem:iface.iface-rotate-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-save-defined-fn]
> void iface_save_defined(char *filename)

> [spec:foma:sem:iface.iface-save-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-save-stack-fn]
> void iface_save_stack(char *filename)

> [spec:foma:sem:iface.iface-save-stack-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-sequentialize-fn]
> void iface_sequentialize()

> [spec:foma:sem:iface.iface-sequentialize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-set-variable-fn]
> void iface_set_variable(char *name, char *value)

> [spec:foma:sem:iface.iface-set-variable-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-show-variable-fn]
> void iface_show_variable(char *name)

> [spec:foma:sem:iface.iface-show-variable-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-show-variables-fn]
> void iface_show_variables()

> [spec:foma:sem:iface.iface-show-variables-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-shuffle-fn]
> void iface_shuffle()

> [spec:foma:sem:iface.iface-shuffle-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-sigma-net-fn]
> void iface_sigma_net()

> [spec:foma:sem:iface.iface-sigma-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-sort-fn]
> void iface_sort()

> [spec:foma:sem:iface.iface-sort-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-sort-input-fn]
> void iface_sort_input()

> [spec:foma:sem:iface.iface-sort-input-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-sort-output-fn]
> void iface_sort_output()

> [spec:foma:sem:iface.iface-sort-output-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-split-result-fn]
> void iface_split_result(char *result, char **upper, char **lower)

> [spec:foma:sem:iface.iface-split-result-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-split-string-fn]
> void iface_split_string(char *result, char *string)

> [spec:foma:sem:iface.iface-split-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-stack-check-fn]
> int iface_stack_check (int size)

> [spec:foma:sem:iface.iface-stack-check-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-substitute-defined-fn]
> void iface_substitute_defined (char *original, char *substitute)

> [spec:foma:sem:iface.iface-substitute-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-substitute-symbol-fn]
> void iface_substitute_symbol (char *original, char *substitute)

> [spec:foma:sem:iface.iface-substitute-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-test-equivalent-fn]
> void iface_test_equivalent()

> [spec:foma:sem:iface.iface-test-equivalent-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-test-functional-fn]
> void iface_test_functional()

> [spec:foma:sem:iface.iface-test-functional-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-test-identity-fn]
> void iface_test_identity()

> [spec:foma:sem:iface.iface-test-identity-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-test-lower-universal-fn]
> void iface_test_lower_universal()

> [spec:foma:sem:iface.iface-test-lower-universal-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-test-nonnull-fn]
> void iface_test_nonnull()

> [spec:foma:sem:iface.iface-test-nonnull-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-test-null-fn]
> void iface_test_null()

> [spec:foma:sem:iface.iface-test-null-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-test-sequential-fn]
> void iface_test_sequential()

> [spec:foma:sem:iface.iface-test-sequential-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-test-unambiguous-fn]
> void iface_test_unambiguous()

> [spec:foma:sem:iface.iface-test-unambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-test-upper-universal-fn]
> void iface_test_upper_universal()

> [spec:foma:sem:iface.iface-test-upper-universal-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-turn-fn]
> void iface_turn()

> [spec:foma:sem:iface.iface-turn-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-twosided-flags-fn]
> void iface_twosided_flags()

> [spec:foma:sem:iface.iface-twosided-flags-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-union-fn]
> void iface_union()

> [spec:foma:sem:iface.iface-union-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-upper-side-fn]
> void iface_upper_side()

> [spec:foma:sem:iface.iface-upper-side-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-upper-words-fn]
> void iface_upper_words(int limit)

> [spec:foma:sem:iface.iface-upper-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-view-fn]
> void iface_view()

> [spec:foma:sem:iface.iface-view-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-warranty-fn]
> void iface_warranty()

> [spec:foma:sem:iface.iface-warranty-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-words-file-fn]
> void iface_words_file(char *filename, int type)

> [spec:foma:sem:iface.iface-words-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-words-fn]
> void iface_words(int limit)

> [spec:foma:sem:iface.iface-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-write-att-fn]
> int iface_write_att(char *filename)

> [spec:foma:sem:iface.iface-write-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-write-prolog-fn]
> void iface_write_prolog(char *filename)

> [spec:foma:sem:iface.iface-write-prolog-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.iface-zero-plus-fn]
> void iface_zero_plus()

> [spec:foma:sem:iface.iface-zero-plus-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.print-dot-fn]
> static int print_dot(struct fsm *net, char *filename)

> [spec:foma:sem:iface.print-dot-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.print-mem-size-fn]
> void print_mem_size(struct fsm *net)

> [spec:foma:sem:iface.print-mem-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.print-net-fn]
> static int print_net(struct fsm *net, char *filename)

> [spec:foma:sem:iface.print-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.print-sigma-fn]
> static int print_sigma(struct sigma *sigma, FILE *out)

> [spec:foma:sem:iface.print-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.print-stats-fn]
> int print_stats(struct fsm *net)

> [spec:foma:sem:iface.print-stats-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.sigptr-fn]
> static char *sigptr(struct sigma *sigma, int number)

> [spec:foma:sem:iface.sigptr-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:iface.view-net-fn]
> static int view_net(struct fsm *net)

> [spec:foma:sem:iface.view-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.


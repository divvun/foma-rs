# foma/fomalib.h

> [spec:foma:def:fomalib.add-defined-fn]
> FEXPORT int add_defined(struct defined_networks *def, struct fsm *net, char *string)

> [spec:foma:sem:fomalib.add-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.add-defined-function-fn]
> FEXPORT int add_defined_function (struct defined_functions *deff, char *name, char *regex, int numargs)

> [spec:foma:sem:fomalib.add-defined-function-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-clear-fn]
> FEXPORT void apply_clear(struct apply_handle *h)

> [spec:foma:sem:fomalib.apply-clear-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-down-fn]
> FEXPORT char *apply_down(struct apply_handle *h, char *word)

> [spec:foma:sem:fomalib.apply-down-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-index-fn]
> FEXPORT void apply_index(struct apply_handle *h, int inout, int densitycutoff, int mem_limit, int flags_only)

> [spec:foma:sem:fomalib.apply-index-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-init-fn]
> apply_handle *apply_init(struct fsm *net)

> [spec:foma:sem:fomalib.apply-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-lower-words-fn]
> FEXPORT char *apply_lower_words(struct apply_handle *h)

> [spec:foma:sem:fomalib.apply-lower-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-clear-fn]
> FEXPORT void apply_med_clear(struct apply_med_handle *h)

> [spec:foma:sem:fomalib.apply-med-clear-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-fn]
> FEXPORT char *apply_med(struct apply_med_handle *medh, char *word)

> [spec:foma:sem:fomalib.apply-med-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-get-cost-fn]
> FEXPORT int apply_med_get_cost(struct apply_med_handle *medh)

> [spec:foma:sem:fomalib.apply-med-get-cost-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-get-instring-fn]
> FEXPORT char *apply_med_get_instring(struct apply_med_handle *medh)

> [spec:foma:sem:fomalib.apply-med-get-instring-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-get-outstring-fn]
> FEXPORT char *apply_med_get_outstring(struct apply_med_handle *medh)

> [spec:foma:sem:fomalib.apply-med-get-outstring-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-init-fn]
> apply_med_handle *apply_med_init(struct fsm *net)

> [spec:foma:sem:fomalib.apply-med-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-set-align-symbol-fn]
> FEXPORT void apply_med_set_align_symbol(struct apply_med_handle *medh, char *align)

> [spec:foma:sem:fomalib.apply-med-set-align-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-set-heap-max-fn]
> FEXPORT void apply_med_set_heap_max(struct apply_med_handle *medh, int max)

> [spec:foma:sem:fomalib.apply-med-set-heap-max-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-set-med-cutoff-fn]
> FEXPORT void apply_med_set_med_cutoff(struct apply_med_handle *medh, int max)

> [spec:foma:sem:fomalib.apply-med-set-med-cutoff-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-med-set-med-limit-fn]
> FEXPORT void apply_med_set_med_limit(struct apply_med_handle *medh, int max)

> [spec:foma:sem:fomalib.apply-med-set-med-limit-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-random-lower-fn]
> FEXPORT char *apply_random_lower(struct apply_handle *h)

> [spec:foma:sem:fomalib.apply-random-lower-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-random-upper-fn]
> FEXPORT char *apply_random_upper(struct apply_handle *h)

> [spec:foma:sem:fomalib.apply-random-upper-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-random-words-fn]
> FEXPORT char *apply_random_words(struct apply_handle *h)

> [spec:foma:sem:fomalib.apply-random-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-reset-enumerator-fn]
> FEXPORT void apply_reset_enumerator(struct apply_handle *h)

> [spec:foma:sem:fomalib.apply-reset-enumerator-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-set-epsilon-fn]
> FEXPORT void apply_set_epsilon(struct apply_handle *h, char *symbol)

> [spec:foma:sem:fomalib.apply-set-epsilon-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-set-obey-flags-fn]
> FEXPORT void apply_set_obey_flags(struct apply_handle *h, int value)

> [spec:foma:sem:fomalib.apply-set-obey-flags-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-set-print-pairs-fn]
> FEXPORT void apply_set_print_pairs(struct apply_handle *h, int value)

> [spec:foma:sem:fomalib.apply-set-print-pairs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-set-print-space-fn]
> FEXPORT void apply_set_print_space(struct apply_handle *h, int value)

> [spec:foma:sem:fomalib.apply-set-print-space-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-set-separator-fn]
> FEXPORT void apply_set_separator(struct apply_handle *h, char *symbol)

> [spec:foma:sem:fomalib.apply-set-separator-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-set-show-flags-fn]
> FEXPORT void apply_set_show_flags(struct apply_handle *h, int value)

> [spec:foma:sem:fomalib.apply-set-show-flags-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-set-space-symbol-fn]
> FEXPORT void apply_set_space_symbol(struct apply_handle *h, char *space)

> [spec:foma:sem:fomalib.apply-set-space-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-up-fn]
> FEXPORT char *apply_up(struct apply_handle *h, char *word)

> [spec:foma:sem:fomalib.apply-up-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-upper-words-fn]
> FEXPORT char *apply_upper_words(struct apply_handle *h)

> [spec:foma:sem:fomalib.apply-upper-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.apply-words-fn]
> FEXPORT char *apply_words(struct apply_handle *h)

> [spec:foma:sem:fomalib.apply-words-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.cmatrix-default-delete-fn]
> FEXPORT void cmatrix_default_delete(struct fsm *net, int cost)

> [spec:foma:sem:fomalib.cmatrix-default-delete-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.cmatrix-default-insert-fn]
> FEXPORT void cmatrix_default_insert(struct fsm *net, int cost)

> [spec:foma:sem:fomalib.cmatrix-default-insert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.cmatrix-default-substitute-fn]
> FEXPORT void cmatrix_default_substitute(struct fsm *net, int cost)

> [spec:foma:sem:fomalib.cmatrix-default-substitute-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.cmatrix-init-fn]
> FEXPORT void cmatrix_init(struct fsm *net)

> [spec:foma:sem:fomalib.cmatrix-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.cmatrix-print-att-fn]
> FEXPORT void cmatrix_print_att(struct fsm *net, FILE *outfile)

> [spec:foma:sem:fomalib.cmatrix-print-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.cmatrix-print-fn]
> FEXPORT void cmatrix_print(struct fsm *net)

> [spec:foma:sem:fomalib.cmatrix-print-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.cmatrix-set-cost-fn]
> FEXPORT void cmatrix_set_cost(struct fsm *net, char *in, char *out, int cost)

> [spec:foma:sem:fomalib.cmatrix-set-cost-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.defined-functions]
> struct defined_functions {
>   char *name;
>   char *regex;
>   int numargs;
>   struct defined_functions *next;
> }

> [spec:foma:def:fomalib.defined-functions-init-fn]
> defined_functions *defined_functions_init(void)

> [spec:foma:sem:fomalib.defined-functions-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.defined-networks]
> struct defined_networks {
>   char *name;
>   struct fsm *net;
>   struct defined_networks *next;
> }

> [spec:foma:def:fomalib.defined-networks-init-fn]
> defined_networks *defined_networks_init(void)

> [spec:foma:sem:fomalib.defined-networks-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.defined-quantifiers]
> struct defined_quantifiers {
>   char *name;
>   struct defined_quantifiers *next;
> }

> [spec:foma:def:fomalib.file-to-mem-fn]
> FEXPORT char *file_to_mem(char *name)

> [spec:foma:sem:fomalib.file-to-mem-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.find-defined-fn]
> struct fsm *find_defined(struct defined_networks *def, char *string)

> [spec:foma:sem:fomalib.find-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.find-defined-function-fn]
> char *find_defined_function(struct defined_functions *deff, char *name, int numargs)

> [spec:foma:sem:fomalib.find-defined-function-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.flag-build-fn]
> FEXPORT int flag_build(int ftype, char *fname, char *fvalue, int fftype, char *ffname, char *ffvalue)

> [spec:foma:sem:fomalib.flag-build-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.flag-eliminate-fn]
> fsm *flag_eliminate(struct fsm *net, char *name)

> [spec:foma:sem:fomalib.flag-eliminate-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.flag-twosided-fn]
> fsm *flag_twosided(struct fsm *net)

> [spec:foma:sem:fomalib.flag-twosided-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.foma-net-print-fn]
> FEXPORT int foma_net_print(struct fsm *net, gzFile outfile)

> [spec:foma:sem:fomalib.foma-net-print-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.foma-write-prolog-fn]
> FEXPORT int foma_write_prolog(struct fsm *net, char *filename)

> [spec:foma:sem:fomalib.foma-write-prolog-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm]
> struct fsm {
>   char name[FSM_NAME_LEN];
>   int arity;
>   int arccount;
>   int statecount;
>   int linecount;
>   int finalcount;
>   long long pathcount;
>   int is_deterministic;
>   int is_pruned;
>   int is_minimized;
>   int is_epsilon_free;
>   int is_loop_free;
>   int is_completed;
>   int arcs_sorted_in;
>   int arcs_sorted_out;
>   struct fsm_state *states;
>   struct sigma *sigma;
>   struct medlookup *medlookup;
> }

> [spec:foma:def:fomalib.fsm-add-loop-fn]
> fsm *fsm_add_loop(struct fsm *net, struct fsm *marker, int finals)

> [spec:foma:sem:fomalib.fsm-add-loop-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-add-sink-fn]
> fsm *fsm_add_sink(struct fsm *net, int final)

> [spec:foma:sem:fomalib.fsm-add-sink-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-bimachine-fn]
> fsm *fsm_bimachine(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-bimachine-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-boolean-fn]
> fsm *fsm_boolean(int value)

> [spec:foma:sem:fomalib.fsm-boolean-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-clear-contexts-fn]
> FEXPORT void fsm_clear_contexts(struct fsmcontexts *contexts)

> [spec:foma:sem:fomalib.fsm-clear-contexts-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-close-sigma-fn]
> fsm *fsm_close_sigma(struct fsm *net, int mode)

> [spec:foma:sem:fomalib.fsm-close-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-coaccessible-fn]
> fsm *fsm_coaccessible(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-coaccessible-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-compact-fn]
> FEXPORT void fsm_compact(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-compact-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-complement-fn]
> fsm *fsm_complement(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-complement-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-complete-fn]
> fsm *fsm_complete(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-complete-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-compose-fn]
> fsm *fsm_compose(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-compose-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-concat-fn]
> fsm *fsm_concat(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-concat-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-concat-m-n-fn]
> fsm *fsm_concat_m_n(struct fsm *net1, int m, int n)

> [spec:foma:sem:fomalib.fsm-concat-m-n-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-concat-n-fn]
> fsm *fsm_concat_n(struct fsm *net1, int n)

> [spec:foma:sem:fomalib.fsm-concat-n-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-construct-add-arc-fn]
> FEXPORT void fsm_construct_add_arc(struct fsm_construct_handle *handle, int source, int target, char *in, char *out)

> [spec:foma:sem:fomalib.fsm-construct-add-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-construct-add-arc-nums-fn]
> FEXPORT void fsm_construct_add_arc_nums(struct fsm_construct_handle *handle, int source, int target, int in, int out)

> [spec:foma:sem:fomalib.fsm-construct-add-arc-nums-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-construct-add-symbol-fn]
> FEXPORT int fsm_construct_add_symbol(struct fsm_construct_handle *handle, char *symbol)

> [spec:foma:sem:fomalib.fsm-construct-add-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-construct-check-symbol-fn]
> FEXPORT int fsm_construct_check_symbol(struct fsm_construct_handle *handle, char *symbol)

> [spec:foma:sem:fomalib.fsm-construct-check-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-construct-copy-sigma-fn]
> FEXPORT void fsm_construct_copy_sigma(struct fsm_construct_handle *handle, struct sigma *sigma)

> [spec:foma:sem:fomalib.fsm-construct-copy-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-construct-done-fn]
> fsm *fsm_construct_done(struct fsm_construct_handle *handle)

> [spec:foma:sem:fomalib.fsm-construct-done-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-construct-init-fn]
> fsm_construct_handle *fsm_construct_init(char *name)

> [spec:foma:sem:fomalib.fsm-construct-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-construct-set-final-fn]
> FEXPORT void fsm_construct_set_final(struct fsm_construct_handle *handle, int state_no)

> [spec:foma:sem:fomalib.fsm-construct-set-final-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-construct-set-initial-fn]
> FEXPORT void fsm_construct_set_initial(struct fsm_construct_handle *handle, int state_no)

> [spec:foma:sem:fomalib.fsm-construct-set-initial-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-contains-fn]
> fsm *fsm_contains(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-contains-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-contains-one-fn]
> fsm *fsm_contains_one(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-contains-one-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-contains-opt-one-fn]
> fsm *fsm_contains_opt_one(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-contains-opt-one-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-context-restrict-fn]
> fsm *fsm_context_restrict(struct fsm *X, struct fsmcontexts *LR)

> [spec:foma:sem:fomalib.fsm-context-restrict-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-copy-fn]
> fsm *fsm_copy(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-copy-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-create-fn]
> fsm *fsm_create(char *name)

> [spec:foma:sem:fomalib.fsm-create-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-create-letter-lookup-fn]
> FEXPORT void fsm_create_letter_lookup(struct apply_med_handle *medh, struct fsm *net)

> [spec:foma:sem:fomalib.fsm-create-letter-lookup-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-cross-product-fn]
> fsm *fsm_cross_product(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-cross-product-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-destroy-fn]
> FEXPORT int fsm_destroy(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-destroy-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-determinize-fn]
> fsm *fsm_determinize(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-determinize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-empty-fn]
> fsm_state *fsm_empty()

> [spec:foma:sem:fomalib.fsm-empty-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-empty-set-fn]
> fsm *fsm_empty_set()

> [spec:foma:sem:fomalib.fsm-empty-set-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-empty-string-fn]
> fsm *fsm_empty_string()

> [spec:foma:sem:fomalib.fsm-empty-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-epsilon-remove-fn]
> fsm *fsm_epsilon_remove(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-epsilon-remove-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-equal-substrings-fn]
> fsm *fsm_equal_substrings(struct fsm *net, struct fsm *left, struct fsm *right)

> [spec:foma:sem:fomalib.fsm-equal-substrings-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-equivalent-fn]
> FEXPORT int fsm_equivalent(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-equivalent-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-escape-fn]
> fsm *fsm_escape(char *symbol)

> [spec:foma:sem:fomalib.fsm-escape-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-explode-fn]
> fsm *fsm_explode(char *symbol)

> [spec:foma:sem:fomalib.fsm-explode-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-extract-ambiguous-domain-fn]
> fsm *fsm_extract_ambiguous_domain(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-extract-ambiguous-domain-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-extract-ambiguous-fn]
> fsm *fsm_extract_ambiguous(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-extract-ambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-extract-nonidentity-fn]
> fsm *fsm_extract_nonidentity(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-extract-nonidentity-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-extract-unambiguous-fn]
> fsm *fsm_extract_unambiguous(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-extract-unambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-find-ambiguous-fn]
> fsm *fsm_find_ambiguous(struct fsm *net, int **extras)

> [spec:foma:sem:fomalib.fsm-find-ambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-flatten-fn]
> fsm *fsm_flatten(struct fsm *net, struct fsm *epsilon)

> [spec:foma:sem:fomalib.fsm-flatten-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-follows-fn]
> fsm *fsm_follows(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-follows-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-arc-in-fn]
> FEXPORT char *fsm_get_arc_in(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-arc-in-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-arc-num-in-fn]
> FEXPORT int fsm_get_arc_num_in(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-arc-num-in-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-arc-num-out-fn]
> FEXPORT int fsm_get_arc_num_out(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-arc-num-out-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-arc-out-fn]
> FEXPORT char *fsm_get_arc_out(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-arc-out-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-arc-source-fn]
> FEXPORT int fsm_get_arc_source(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-arc-source-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-arc-target-fn]
> FEXPORT int fsm_get_arc_target(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-arc-target-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-has-unknowns-fn]
> FEXPORT int fsm_get_has_unknowns(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-has-unknowns-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-library-version-string-fn]
> FEXPORT char *fsm_get_library_version_string()

> [spec:foma:sem:fomalib.fsm-get-library-version-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-next-arc-fn]
> FEXPORT int fsm_get_next_arc(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-next-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-next-final-fn]
> FEXPORT int fsm_get_next_final(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-next-final-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-next-initial-fn]
> FEXPORT int fsm_get_next_initial(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-next-initial-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-next-state-arc-fn]
> FEXPORT int fsm_get_next_state_arc(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-next-state-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-next-state-fn]
> FEXPORT int fsm_get_next_state(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-next-state-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-num-states-fn]
> FEXPORT int fsm_get_num_states(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-get-num-states-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-option-fn]
> FEXPORT void *fsm_get_option(unsigned long long option)

> [spec:foma:sem:fomalib.fsm-get-option-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-get-symbol-number-fn]
> FEXPORT int fsm_get_symbol_number(struct fsm_read_handle *handle, char *symbol)

> [spec:foma:sem:fomalib.fsm-get-symbol-number-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-identity-fn]
> fsm *fsm_identity()

> [spec:foma:sem:fomalib.fsm-identity-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-ignore-fn]
> fsm *fsm_ignore(struct fsm *net1, struct fsm *net2, int operation)

> [spec:foma:sem:fomalib.fsm-ignore-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-intersect-fn]
> fsm *fsm_intersect(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-intersect-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-invert-fn]
> fsm *fsm_invert(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-invert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-isempty-fn]
> FEXPORT int fsm_isempty(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-isempty-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-isfunctional-fn]
> FEXPORT int fsm_isfunctional(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-isfunctional-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-isidentity-fn]
> FEXPORT int fsm_isidentity(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-isidentity-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-issequential-fn]
> FEXPORT int fsm_issequential(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-issequential-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-isunambiguous-fn]
> FEXPORT int fsm_isunambiguous(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-isunambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-isuniversal-fn]
> FEXPORT int fsm_isuniversal(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-isuniversal-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-kleene-plus-fn]
> fsm *fsm_kleene_plus(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-kleene-plus-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-kleene-star-fn]
> fsm *fsm_kleene_star(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-kleene-star-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-left-rewr-fn]
> fsm *fsm_left_rewr(struct fsm *net, struct fsm *rewr)

> [spec:foma:sem:fomalib.fsm-left-rewr-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-lenient-compose-fn]
> fsm *fsm_lenient_compose(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-lenient-compose-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-letter-machine-fn]
> fsm *fsm_letter_machine(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-letter-machine-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-lexc-parse-file-fn]
> fsm *fsm_lexc_parse_file(char *myfile, int verbose)

> [spec:foma:sem:fomalib.fsm-lexc-parse-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-lexc-parse-string-fn]
> fsm *fsm_lexc_parse_string(char *mystring, int verbose)

> [spec:foma:sem:fomalib.fsm-lexc-parse-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-logical-eq-fn]
> fsm *fsm_logical_eq(char *string1, char *string2)

> [spec:foma:sem:fomalib.fsm-logical-eq-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-logical-precedence-fn]
> fsm *fsm_logical_precedence(char *string1, char *string2)

> [spec:foma:sem:fomalib.fsm-logical-precedence-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-lower-fn]
> fsm *fsm_lower(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-lower-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-lowerdet-fn]
> fsm *fsm_lowerdet(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-lowerdet-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-lowerdeteps-fn]
> fsm *fsm_lowerdeteps(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-lowerdeteps-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-mark-ambiguous-fn]
> fsm *fsm_mark_ambiguous(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-mark-ambiguous-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-mark-fsm-tail-fn]
> fsm *fsm_mark_fsm_tail(struct fsm *net, struct fsm *marker)

> [spec:foma:sem:fomalib.fsm-mark-fsm-tail-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-markallfinal-fn]
> fsm *fsm_markallfinal(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-markallfinal-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-merge-sigma-fn]
> FEXPORT void fsm_merge_sigma(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-merge-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-minimize-fn]
> fsm *fsm_minimize(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-minimize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-minus-fn]
> fsm *fsm_minus(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-minus-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-network-to-char-fn]
> FEXPORT char *fsm_network_to_char(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-network-to-char-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-optionality-fn]
> fsm *fsm_optionality(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-optionality-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-options]
> typedef enum

> [spec:foma:def:fomalib.fsm-parse-regex-fn]
> fsm *fsm_parse_regex(char *regex, struct defined_networks *defined_nets, struct defined_functions *defined_funcs)

> [spec:foma:sem:fomalib.fsm-parse-regex-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-precedes-fn]
> fsm *fsm_precedes(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-precedes-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-priority-union-lower-fn]
> fsm *fsm_priority_union_lower(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-priority-union-lower-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-priority-union-upper-fn]
> fsm *fsm_priority_union_upper(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-priority-union-upper-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-quantifier-fn]
> fsm *fsm_quantifier(char *string)

> [spec:foma:sem:fomalib.fsm-quantifier-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-quotient-interleave-fn]
> fsm *fsm_quotient_interleave(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-quotient-interleave-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-quotient-left-fn]
> fsm *fsm_quotient_left(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-quotient-left-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-quotient-right-fn]
> fsm *fsm_quotient_right(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-quotient-right-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-binary-file-fn]
> fsm *fsm_read_binary_file(char *filename)

> [spec:foma:sem:fomalib.fsm-read-binary-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-binary-file-multiple-fn]
> fsm *fsm_read_binary_file_multiple(fsm_read_binary_handle fsrh)

> [spec:foma:sem:fomalib.fsm-read-binary-file-multiple-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-done-fn]
> FEXPORT void fsm_read_done(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-read-done-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-handle]
> struct fsm_read_handle {
>   struct fsm_state *arcs_head;
>   struct fsm_state **states_head;
>   struct fsm_state *arcs_cursor;
>   int *finals_head;
>   int *finals_cursor;
>   struct fsm_state **states_cursor;
>   int *initials_head;
>   int *initials_cursor;
>   int current_state;
>   struct fsm_sigma_list *fsm_sigma_list;
>   int sigma_list_size;
>   struct fsm *net;
>   unsigned char *lookuptable;
>   _Bool has_unknowns;
> }

> [spec:foma:def:fomalib.fsm-read-init-fn]
> fsm_read_handle *fsm_read_init(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-read-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-is-final-fn]
> FEXPORT int fsm_read_is_final(struct fsm_read_handle *h, int state)

> [spec:foma:sem:fomalib.fsm-read-is-final-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-is-initial-fn]
> FEXPORT int fsm_read_is_initial(struct fsm_read_handle *h, int state)

> [spec:foma:sem:fomalib.fsm-read-is-initial-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-prolog-fn]
> fsm *fsm_read_prolog(char *filename)

> [spec:foma:sem:fomalib.fsm-read-prolog-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-reset-fn]
> FEXPORT void fsm_read_reset(struct fsm_read_handle *handle)

> [spec:foma:sem:fomalib.fsm-read-reset-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-spaced-text-file-fn]
> fsm *fsm_read_spaced_text_file(char *filename)

> [spec:foma:sem:fomalib.fsm-read-spaced-text-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-read-text-file-fn]
> fsm *fsm_read_text_file(char *filename)

> [spec:foma:sem:fomalib.fsm-read-text-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-reverse-fn]
> fsm *fsm_reverse(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-reverse-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-rewrite-fn]
> fsm *fsm_rewrite(struct rewrite_set *all_rules)

> [spec:foma:sem:fomalib.fsm-rewrite-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-sequentialize-fn]
> fsm *fsm_sequentialize(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-sequentialize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-set-option-fn]
> FEXPORT _Bool fsm_set_option(unsigned long long option, void *value)

> [spec:foma:sem:fomalib.fsm-set-option-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-shuffle-fn]
> fsm *fsm_shuffle(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-shuffle-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-sigma-destroy-fn]
> FEXPORT int fsm_sigma_destroy(struct sigma *sigma)

> [spec:foma:sem:fomalib.fsm-sigma-destroy-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-sigma-net-fn]
> fsm *fsm_sigma_net(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-sigma-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-sigma-pairs-net-fn]
> fsm *fsm_sigma_pairs_net(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-sigma-pairs-net-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-simple-replace-fn]
> fsm *fsm_simple_replace(struct fsm *net1, struct fsm *net2)

> [spec:foma:sem:fomalib.fsm-simple-replace-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-sort-arcs-fn]
> FEXPORT void fsm_sort_arcs(struct fsm *net, int direction)

> [spec:foma:sem:fomalib.fsm-sort-arcs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-state]
> struct fsm_state {
>   int state_no;
>   short int in;
>   short int out;
>   int target;
>   char final_state;
>   char start_state;
> }

> [spec:foma:def:fomalib.fsm-substitute-label-fn]
> fsm *fsm_substitute_label(struct fsm *net, char *original, struct fsm *substitute)

> [spec:foma:sem:fomalib.fsm-substitute-label-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-substitute-symbol-fn]
> fsm *fsm_substitute_symbol(struct fsm *net, char *original, char *substitute)

> [spec:foma:sem:fomalib.fsm-substitute-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-symbol-fn]
> fsm *fsm_symbol(char *symbol)

> [spec:foma:sem:fomalib.fsm-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-symbol-occurs-fn]
> FEXPORT int fsm_symbol_occurs(struct fsm *net, char *symbol, int side)

> [spec:foma:sem:fomalib.fsm-symbol-occurs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-term-negation-fn]
> fsm *fsm_term_negation(struct fsm *net1)

> [spec:foma:sem:fomalib.fsm-term-negation-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-topsort-fn]
> fsm *fsm_topsort(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-topsort-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-trie-add-word-fn]
> FEXPORT void fsm_trie_add_word(struct fsm_trie_handle *th, char *word)

> [spec:foma:sem:fomalib.fsm-trie-add-word-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-trie-done-fn]
> fsm *fsm_trie_done(struct fsm_trie_handle *th)

> [spec:foma:sem:fomalib.fsm-trie-done-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-trie-end-word-fn]
> FEXPORT void fsm_trie_end_word(struct fsm_trie_handle *th)

> [spec:foma:sem:fomalib.fsm-trie-end-word-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-trie-handle]
> struct fsm_trie_handle {
>   struct trie_states *trie_states;
>   unsigned int trie_cursor;
>   struct trie_hash *trie_hash;
>   unsigned int used_states;
>   unsigned int statesize;
>   struct sh_handle *sh_hash;
> }

> [spec:foma:def:fomalib.fsm-trie-init-fn]
> fsm_trie_handle *fsm_trie_init()

> [spec:foma:sem:fomalib.fsm-trie-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-trie-symbol-fn]
> FEXPORT void fsm_trie_symbol(struct fsm_trie_handle *th, char *insym, char *outsym)

> [spec:foma:sem:fomalib.fsm-trie-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-unflatten-fn]
> fsm *fsm_unflatten(struct fsm *net, char *epsilon_sym, char *repeat_sym)

> [spec:foma:sem:fomalib.fsm-unflatten-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-union-fn]
> fsm *fsm_union(struct fsm *net_1, struct fsm *net_2)

> [spec:foma:sem:fomalib.fsm-union-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-universal-fn]
> fsm *fsm_universal()

> [spec:foma:sem:fomalib.fsm-universal-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-upper-fn]
> fsm *fsm_upper(struct fsm *net)

> [spec:foma:sem:fomalib.fsm-upper-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsm-write-binary-file-fn]
> FEXPORT int fsm_write_binary_file(struct fsm *net, char *filename)

> [spec:foma:sem:fomalib.fsm-write-binary-file-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.fsmcontexts]
> struct fsmcontexts {
>   struct fsm *left;
>   struct fsm *right;
>   struct fsmcontexts *next;
>   struct fsm *cpleft;
>   struct fsm *cpright;
> }

> [spec:foma:def:fomalib.fsmrules]
> struct fsmrules {
>   struct fsm *left;
>   struct fsm *right;
>   struct fsm *right2;
>   struct fsm *cross_product;
>   struct fsmrules *next;
>   int arrow_type;
>   int dotted;
> }

> [spec:foma:def:fomalib.load-defined-fn]
> FEXPORT int load_defined(struct defined_networks *def, char *filename)

> [spec:foma:sem:fomalib.load-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.medlookup]
> struct medlookup {
>   int *confusion_matrix;
> }

> [spec:foma:def:fomalib.net-print-att-fn]
> FEXPORT int net_print_att(struct fsm *net, FILE *outfile)

> [spec:foma:sem:fomalib.net-print-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.read-att-fn]
> fsm *read_att(char *filename)

> [spec:foma:sem:fomalib.read-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.remove-defined-fn]
> int remove_defined (struct defined_networks *def, char *string)

> [spec:foma:sem:fomalib.remove-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.rewrite-set]
> struct rewrite_set {
>   struct fsmrules *rewrite_rules;
>   struct fsmcontexts *rewrite_contexts;
>   struct rewrite_set *next;
>   int rule_direction;
> }

> [spec:foma:def:fomalib.save-defined-fn]
> FEXPORT int save_defined(struct defined_networks *def, char *filename)

> [spec:foma:sem:fomalib.save-defined-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.save-stack-att-fn]
> FEXPORT int save_stack_att()

> [spec:foma:sem:fomalib.save-stack-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.sh-add-string-fn]
> char *sh_add_string(struct sh_handle *sh, char *string, int value)

> [spec:foma:sem:fomalib.sh-add-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.sh-done-fn]
> void sh_done(struct sh_handle *sh)

> [spec:foma:sem:fomalib.sh-done-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.sh-find-add-string-fn]
> char *sh_find_add_string(struct sh_handle *sh, char *string, int value)

> [spec:foma:sem:fomalib.sh-find-add-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.sh-find-string-fn]
> char *sh_find_string(struct sh_handle *sh, char *string)

> [spec:foma:sem:fomalib.sh-find-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.sh-get-value-fn]
> int sh_get_value(struct sh_handle *sh)

> [spec:foma:sem:fomalib.sh-get-value-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.sh-handle]
> struct sh_handle {
>   struct sh_hashtable *hash;
>   int lastvalue;
> }

> [spec:foma:def:fomalib.sh-hashtable]
> struct sh_hashtable {
>   char *string;
>   int value;
>   struct sh_hashtable *next;
> }

> [spec:foma:def:fomalib.sh-init-fn]
> struct sh_handle *sh_init()

> [spec:foma:sem:fomalib.sh-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.sigma]
> struct sigma {
>   int number;
>   char *symbol;
>   struct sigma *next;
> }

> [spec:foma:def:fomalib.sigma-copy-fn]
> sigma *sigma_copy(struct sigma *sigma)

> [spec:foma:sem:fomalib.sigma-copy-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalib.trie-hash]
> struct trie_hash {
>   char *insym;
>   char *outsym;
>   unsigned int sourcestate;
>   unsigned int targetstate;
>   struct trie_hash *next;
> }

> [spec:foma:def:fomalib.trie-states]
> struct trie_states {
>   _Bool is_final;
> }


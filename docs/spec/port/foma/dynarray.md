# foma/dynarray.c

> [spec:foma:def:dynarray.foma-reserved-symbols]
> struct foma_reserved_symbols {
>   char *symbol;
>   int number;
>   char *prints_as;
> }

> [spec:foma:def:dynarray.fsm-construct-add-arc-fn]
> void fsm_construct_add_arc(struct fsm_construct_handle *handle, int source, int target, char *in, char *out)

> [spec:foma:sem:dynarray.fsm-construct-add-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-add-arc-nums-fn]
> void fsm_construct_add_arc_nums(struct fsm_construct_handle *handle, int source, int target, int in, int out)

> [spec:foma:sem:dynarray.fsm-construct-add-arc-nums-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-add-symbol-fn]
> int fsm_construct_add_symbol(struct fsm_construct_handle *handle, char *symbol)

> [spec:foma:sem:dynarray.fsm-construct-add-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-check-size-fn]
> void fsm_construct_check_size(struct fsm_construct_handle *handle, int state_no)

> [spec:foma:sem:dynarray.fsm-construct-check-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-check-symbol-fn]
> int fsm_construct_check_symbol(struct fsm_construct_handle *handle, char *symbol)

> [spec:foma:sem:dynarray.fsm-construct-check-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-convert-sigma-fn]
> struct sigma *fsm_construct_convert_sigma(struct fsm_construct_handle *handle)

> [spec:foma:sem:dynarray.fsm-construct-convert-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-copy-sigma-fn]
> void fsm_construct_copy_sigma(struct fsm_construct_handle *handle, struct sigma *sigma)

> [spec:foma:sem:dynarray.fsm-construct-copy-sigma-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-done-fn]
> struct fsm *fsm_construct_done(struct fsm_construct_handle *handle)

> [spec:foma:sem:dynarray.fsm-construct-done-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-hash-sym-fn]
> unsigned int fsm_construct_hash_sym(char *symbol)

> [spec:foma:sem:dynarray.fsm-construct-hash-sym-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-init-fn]
> struct fsm_construct_handle *fsm_construct_init(char *name)

> [spec:foma:sem:dynarray.fsm-construct-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-set-final-fn]
> void fsm_construct_set_final(struct fsm_construct_handle *handle, int state_no)

> [spec:foma:sem:dynarray.fsm-construct-set-final-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-construct-set-initial-fn]
> void fsm_construct_set_initial(struct fsm_construct_handle *handle, int state_no)

> [spec:foma:sem:dynarray.fsm-construct-set-initial-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-arc-in-fn]
> char *fsm_get_arc_in(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-arc-in-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-arc-num-in-fn]
> int fsm_get_arc_num_in(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-arc-num-in-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-arc-num-out-fn]
> int fsm_get_arc_num_out(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-arc-num-out-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-arc-out-fn]
> char *fsm_get_arc_out(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-arc-out-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-arc-source-fn]
> int fsm_get_arc_source(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-arc-source-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-arc-target-fn]
> int fsm_get_arc_target(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-arc-target-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-has-unknowns-fn]
> int fsm_get_has_unknowns(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-has-unknowns-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-next-arc-fn]
> int fsm_get_next_arc(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-next-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-next-final-fn]
> int fsm_get_next_final(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-next-final-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-next-initial-fn]
> int fsm_get_next_initial(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-next-initial-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-next-state-arc-fn]
> int fsm_get_next_state_arc(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-next-state-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-next-state-fn]
> int fsm_get_next_state(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-next-state-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-num-states-fn]
> int fsm_get_num_states(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-get-num-states-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-get-symbol-number-fn]
> int fsm_get_symbol_number(struct fsm_read_handle *handle, char *symbol)

> [spec:foma:sem:dynarray.fsm-get-symbol-number-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-read-done-fn]
> void fsm_read_done(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-read-done-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-read-init-fn]
> struct fsm_read_handle *fsm_read_init(struct fsm *net)

> [spec:foma:sem:dynarray.fsm-read-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-read-is-final-fn]
> int fsm_read_is_final(struct fsm_read_handle *h, int state)

> [spec:foma:sem:dynarray.fsm-read-is-final-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-read-is-initial-fn]
> int fsm_read_is_initial(struct fsm_read_handle *h, int state)

> [spec:foma:sem:dynarray.fsm-read-is-initial-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-read-reset-fn]
> void fsm_read_reset(struct fsm_read_handle *handle)

> [spec:foma:sem:dynarray.fsm-read-reset-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-state-add-arc-fn]
> void fsm_state_add_arc(int state_no, int in, int out, int target, int final_state, int start_state)

> [spec:foma:sem:dynarray.fsm-state-add-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-state-close-fn]
> void fsm_state_close(struct fsm *net)

> [spec:foma:sem:dynarray.fsm-state-close-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-state-end-state-fn]
> void fsm_state_end_state()

> [spec:foma:sem:dynarray.fsm-state-end-state-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-state-init-fn]
> struct fsm_state *fsm_state_init(int sigma_size)

> [spec:foma:sem:dynarray.fsm-state-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.fsm-state-set-current-state-fn]
> void fsm_state_set_current_state(int state_no, int final_state, int start_state)

> [spec:foma:sem:dynarray.fsm-state-set-current-state-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:dynarray.sigma-lookup]
> struct sigma_lookup {
>   int target;
>   unsigned int mainloop;
> }


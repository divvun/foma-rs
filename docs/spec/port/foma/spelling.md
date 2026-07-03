# foma/spelling.c

> [spec:foma:def:spelling.apply-med-clear-fn]
> void apply_med_clear(struct apply_med_handle *medh)

> [spec:foma:sem:spelling.apply-med-clear-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.apply-med-fn]
> char *apply_med(struct apply_med_handle *medh, char *word)

> [spec:foma:sem:spelling.apply-med-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.apply-med-get-cost-fn]
> int apply_med_get_cost(struct apply_med_handle *medh)

> [spec:foma:sem:spelling.apply-med-get-cost-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.apply-med-get-instring-fn]
> char *apply_med_get_instring(struct apply_med_handle *medh)

> [spec:foma:sem:spelling.apply-med-get-instring-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.apply-med-get-outstring-fn]
> char *apply_med_get_outstring(struct apply_med_handle *medh)

> [spec:foma:sem:spelling.apply-med-get-outstring-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.apply-med-init-fn]
> struct apply_med_handle *apply_med_init(struct fsm *net)

> [spec:foma:sem:spelling.apply-med-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.apply-med-set-align-symbol-fn]
> void apply_med_set_align_symbol(struct apply_med_handle *medh, char *align)

> [spec:foma:sem:spelling.apply-med-set-align-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.apply-med-set-heap-max-fn]
> void apply_med_set_heap_max(struct apply_med_handle *medh, int max)

> [spec:foma:sem:spelling.apply-med-set-heap-max-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.apply-med-set-med-cutoff-fn]
> void apply_med_set_med_cutoff(struct apply_med_handle *medh, int max)

> [spec:foma:sem:spelling.apply-med-set-med-cutoff-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.apply-med-set-med-limit-fn]
> void apply_med_set_med_limit(struct apply_med_handle *medh, int max)

> [spec:foma:sem:spelling.apply-med-set-med-limit-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.calculate-h-fn]
> int calculate_h(struct apply_med_handle *medh, int *intword, int currpos, int state)

> [spec:foma:sem:spelling.calculate-h-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.cmatrix-default-delete-fn]
> void cmatrix_default_delete(struct fsm *net, int cost)

> [spec:foma:sem:spelling.cmatrix-default-delete-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.cmatrix-default-insert-fn]
> void cmatrix_default_insert(struct fsm *net, int cost)

> [spec:foma:sem:spelling.cmatrix-default-insert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.cmatrix-default-substitute-fn]
> void cmatrix_default_substitute(struct fsm *net, int cost)

> [spec:foma:sem:spelling.cmatrix-default-substitute-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.cmatrix-init-fn]
> void cmatrix_init(struct fsm *net)

> [spec:foma:sem:spelling.cmatrix-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.cmatrix-print-att-fn]
> void cmatrix_print_att(struct fsm *net, FILE *outfile)

> [spec:foma:sem:spelling.cmatrix-print-att-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.cmatrix-print-fn]
> void cmatrix_print(struct fsm *net)

> [spec:foma:sem:spelling.cmatrix-print-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.cmatrix-set-cost-fn]
> void cmatrix_set_cost(struct fsm *net, char *in, char *out, int cost)

> [spec:foma:sem:spelling.cmatrix-set-cost-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.fsm-create-letter-lookup-fn]
> void fsm_create_letter_lookup(struct apply_med_handle *medh, struct fsm *net)

> [spec:foma:sem:spelling.fsm-create-letter-lookup-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.letterbits-add-fn]
> void letterbits_add(int v, int symbol, uint8_t *ptr, int bytes_per_letter_array)

> [spec:foma:sem:spelling.letterbits-add-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.letterbits-copy-fn]
> void letterbits_copy(int source, int target, uint8_t *ptr, int bytes_per_letter_array)

> [spec:foma:sem:spelling.letterbits-copy-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.letterbits-union-fn]
> void letterbits_union(int v, int vp, uint8_t *ptr, int bytes_per_letter_array)

> [spec:foma:sem:spelling.letterbits-union-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.node-delete-min-fn]
> struct astarnode *node_delete_min(struct apply_med_handle *medh)

> [spec:foma:sem:spelling.node-delete-min-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.node-insert-fn]
> int node_insert(struct apply_med_handle *medh, int wordpos, int fsmstate, int g, int h, int in, int out, int parent)

> [spec:foma:sem:spelling.node-insert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.print-match-fn]
> void print_match(struct apply_med_handle *medh, struct astarnode *node, struct sigma *sigma, char *word)

> [spec:foma:sem:spelling.print-match-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.print-sym-fn]
> char *print_sym(int sym, struct sigma *sigma)

> [spec:foma:sem:spelling.print-sym-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:spelling.sccinfo]
> struct sccinfo {
>   int index;
>   int lowlink;
>   int on_t_stack;
> }


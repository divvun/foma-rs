# foma/determinize.c

> [spec:foma:def:determinize.add-fsm-arc-fn]
> extern int add_fsm_arc(struct fsm_state *fsm, int offset, int state_no, int in, int out, int target, int final_state, int start_state)

> [spec:foma:sem:determinize.add-fsm-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.add-t-ptr-fn]
> void add_T_ptr(int setnum, int setsize, unsigned int theset, int fs)

> [spec:foma:sem:determinize.add-t-ptr-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.e-closure-fn]
> INLINE static int e_closure(int states)

> [spec:foma:sem:determinize.e-closure-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.e-closure-free-fn]
> static void e_closure_free()

> [spec:foma:sem:determinize.e-closure-free-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.e-closure-memo]
> struct e_closure_memo {
>   int state;
>   int mark;
>   struct e_closure_memo *target;
>   struct e_closure_memo *next;
> }

> [spec:foma:def:determinize.fsm-determinize-fn]
> struct fsm *fsm_determinize(struct fsm *net)

> [spec:foma:sem:determinize.fsm-determinize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.fsm-epsilon-remove-fn]
> struct fsm *fsm_epsilon_remove(struct fsm *net)

> [spec:foma:sem:determinize.fsm-epsilon-remove-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.fsm-subset-fn]
> static struct fsm *fsm_subset(struct fsm *net, int operation)

> [spec:foma:sem:determinize.fsm-subset-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.hashf-fn]
> INLINE static int hashf(int *set, int setsize)

> [spec:foma:sem:determinize.hashf-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.init-fn]
> static void init(struct fsm *net)

> [spec:foma:sem:determinize.init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.init-trans-array-fn]
> static void init_trans_array(struct fsm *net)

> [spec:foma:sem:determinize.init-trans-array-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.initial-e-closure-fn]
> static int initial_e_closure(struct fsm *net)

> [spec:foma:sem:determinize.initial-e-closure-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.memoize-e-closure-fn]
> static void memoize_e_closure(struct fsm_state *fsm)

> [spec:foma:sem:determinize.memoize-e-closure-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.move-set-fn]
> static unsigned int move_set(int *set, int setsize)

> [spec:foma:sem:determinize.move-set-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.next-unmarked-fn]
> static int next_unmarked(void)

> [spec:foma:sem:determinize.next-unmarked-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.nhash-find-insert-fn]
> static int nhash_find_insert(int *set, int setsize)

> [spec:foma:sem:determinize.nhash-find-insert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.nhash-free-fn]
> static void nhash_free(struct nhash_list *nptr, int size)

> [spec:foma:sem:determinize.nhash-free-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.nhash-init-fn]
> static void nhash_init (int initial_size)

> [spec:foma:sem:determinize.nhash-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.nhash-insert-fn]
> static int nhash_insert(int hashval, int *set, int setsize)

> [spec:foma:sem:determinize.nhash-insert-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.nhash-list]
> struct nhash_list {
>   int setnum;
>   unsigned int size;
>   unsigned int set_offset;
>   struct nhash_list *next;
> }

> [spec:foma:def:determinize.nhash-rebuild-table-fn]
> static void nhash_rebuild_table ()

> [spec:foma:sem:determinize.nhash-rebuild-table-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.set-lookup-fn]
> INLINE static int set_lookup (int *lookup_table, int size)

> [spec:foma:sem:determinize.set-lookup-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.sigma-to-pairs-fn]
> static void sigma_to_pairs(struct fsm *net)

> [spec:foma:sem:determinize.sigma-to-pairs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.single-symbol-to-symbol-pair-fn]
> static void single_symbol_to_symbol_pair(int symbol, int *symbol_in, int *symbol_out)

> [spec:foma:sem:determinize.single-symbol-to-symbol-pair-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.symbol-pair-to-single-symbol-fn]
> static int symbol_pair_to_single_symbol(int in, int out)

> [spec:foma:sem:determinize.symbol-pair-to-single-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:determinize.t-memo]
> struct T_memo {
>   unsigned char finalstart;
>   unsigned int size;
>   unsigned int set_offset;
> }

> [spec:foma:def:determinize.trans-array]
> struct trans_array {
>   struct trans_list *transitions;
>   unsigned int size;
>   unsigned int tail;
> }

> [spec:foma:def:determinize.trans-list]
> struct trans_list {
>   int inout;
>   int target;
> }

> [spec:foma:def:determinize.trans-sort-cmp-fn]
> static int trans_sort_cmp(const void *a, const void *b)

> [spec:foma:sem:determinize.trans-sort-cmp-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.


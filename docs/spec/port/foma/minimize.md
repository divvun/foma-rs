# foma/minimize.c

> [spec:foma:def:minimize.agenda]
> struct agenda {
>   struct p *p;
>   struct agenda *next;
>   _Bool index;
> }

> [spec:foma:def:minimize.agenda-add-fn]
> static void agenda_add(struct p *pptr, int start)

> [spec:foma:sem:minimize.agenda-add-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.e]
> struct e {
>   struct p *group;
>   struct e *left;
>   struct e *right;
>   int inv_count;
> }

> [spec:foma:def:minimize.fsm-minimize-brz-fn]
> static struct fsm *fsm_minimize_brz(struct fsm *net)

> [spec:foma:sem:minimize.fsm-minimize-brz-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.fsm-minimize-fn]
> struct fsm *fsm_minimize(struct fsm *net)

> [spec:foma:sem:minimize.fsm-minimize-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.fsm-minimize-hop-fn]
> static struct fsm *fsm_minimize_hop(struct fsm *net)

> [spec:foma:sem:minimize.fsm-minimize-hop-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.generate-inverse-fn]
> static void generate_inverse(struct fsm *net)

> [spec:foma:sem:minimize.generate-inverse-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.init-pe-fn]
> static void init_PE()

> [spec:foma:sem:minimize.init-pe-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.p]
> struct p {
>   struct e *first_e;
>   struct e *last_e;
>   struct p *current_split;
>   struct p *next;
>   struct agenda *agenda;
>   int count;
>   int t_count;
>   int inv_count;
>   int inv_t_count;
> }

> [spec:foma:def:minimize.rebuild-machine-fn]
> static struct fsm *rebuild_machine(struct fsm *net)

> [spec:foma:sem:minimize.rebuild-machine-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.refine-states-fn]
> static INLINE int refine_states(int invstates)

> [spec:foma:sem:minimize.refine-states-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.sigma-to-pairs-fn]
> static void sigma_to_pairs(struct fsm *net)

> [spec:foma:sem:minimize.sigma-to-pairs-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.state-list]
> struct state_list {
>   int state;
>   struct state_list *next;
> }

> [spec:foma:def:minimize.statesym]
> struct statesym {
>   int target;
>   unsigned short int symbol;
>   struct state_list *states;
>   struct statesym *next;
> }

> [spec:foma:def:minimize.symbol-pair-to-single-symbol-fn]
> static INLINE int symbol_pair_to_single_symbol(int in, int out)

> [spec:foma:sem:minimize.symbol-pair-to-single-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:minimize.trans-array]
> struct trans_array {
>   struct trans_list *transitions;
>   unsigned int size;
>   unsigned int tail;
> }

> [spec:foma:def:minimize.trans-list]
> struct trans_list {
>   int inout;
>   int source;
> }

> [spec:foma:def:minimize.trans-sort-cmp-fn]
> static int trans_sort_cmp(const void *a, const void *b)

> [spec:foma:sem:minimize.trans-sort-cmp-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.


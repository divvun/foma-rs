# foma/cgflookup.c

> [spec:foma:def:cgflookup.app-print-fn]
> void app_print(char *result)

> [spec:foma:sem:cgflookup.app-print-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:cgflookup.applyer-fn]
> static char *(*applyer)(struct apply_handle *h, char *word) = &apply_up

> [spec:foma:sem:cgflookup.applyer-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:cgflookup.get-next-line-fn]
> char *get_next_line()

> [spec:foma:sem:cgflookup.get-next-line-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:cgflookup.handle-line-fn]
> void handle_line(char *s)

> [spec:foma:sem:cgflookup.handle-line-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:cgflookup.lookup-chain]
> struct lookup_chain {
>   struct fsm *net;
>   struct apply_handle *ah;
>   struct lookup_chain *next;
>   struct lookup_chain *prev;
> }

> [spec:foma:def:cgflookup.main-fn]
> int main(int argc, char *argv[])

> [spec:foma:sem:cgflookup.main-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.


# foma/flookup.c

> [spec:foma:def:flookup.app-print-fn]
> void app_print(char *result)

> [spec:foma:sem:flookup.app-print-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flookup.applyer-fn]
> static char *(*applyer)(struct apply_handle *h, char *word) = &apply_up

> [spec:foma:sem:flookup.applyer-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flookup.get-next-line-fn]
> char *get_next_line()

> [spec:foma:sem:flookup.get-next-line-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flookup.handle-line-fn]
> void handle_line(char *s)

> [spec:foma:sem:flookup.handle-line-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flookup.lookup-chain]
> struct lookup_chain {
>   struct fsm *net;
>   struct apply_handle *ah;
>   struct lookup_chain *next;
>   struct lookup_chain *prev;
> }

> [spec:foma:def:flookup.main-fn]
> int main(int argc, char *argv[])

> [spec:foma:sem:flookup.main-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flookup.server-init-fn]
> void server_init(void)

> [spec:foma:sem:flookup.server-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.


# foma/sigma.c

> [spec:foma:def:sigma.sigma-add-fn]
> int sigma_add (char *symbol, struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-add-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-add-number-fn]
> int sigma_add_number(struct sigma *sigma, char *symbol, int number)

> [spec:foma:sem:sigma.sigma-add-number-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-add-special-fn]
> int sigma_add_special (int symbol, struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-add-special-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-cleanup-fn]
> void sigma_cleanup (struct fsm *net, int force)

> [spec:foma:sem:sigma.sigma-cleanup-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-copy-fn]
> struct sigma *sigma_copy(struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-copy-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-create-fn]
> struct sigma *sigma_create()

> [spec:foma:sem:sigma.sigma-create-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-find-fn]
> int sigma_find(char *symbol, struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-find-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-find-number-fn]
> int sigma_find_number(int number, struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-find-number-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-max-fn]
> int sigma_max(struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-max-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-remove-fn]
> struct sigma *sigma_remove(char *symbol, struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-remove-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-remove-num-fn]
> struct sigma *sigma_remove_num(int num, struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-remove-num-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-size-fn]
> int sigma_size(struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-sort-fn]
> int sigma_sort(struct fsm *net)

> [spec:foma:sem:sigma.sigma-sort-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-string-fn]
> char *sigma_string(int number, struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-substitute-fn]
> int sigma_substitute(char *symbol, char *sub, struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-substitute-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.sigma-to-list-fn]
> struct fsm_sigma_list *sigma_to_list(struct sigma *sigma)

> [spec:foma:sem:sigma.sigma-to-list-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:sigma.ssort]
> struct ssort {
>   char *symbol;
>   int number;
> }

> [spec:foma:def:sigma.ssortcmp-fn]
> int ssortcmp(const void *_a, const void *_b)

> [spec:foma:sem:sigma.ssortcmp-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.


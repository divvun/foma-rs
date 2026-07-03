# foma/flags.c

> [spec:foma:def:flags.flag-build-fn]
> int flag_build(int ftype, char *fname, char *fvalue, int fftype, char *ffname, char *ffvalue)

> [spec:foma:sem:flags.flag-build-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-check-fn]
> int flag_check(char *s)

> [spec:foma:sem:flags.flag-check-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-create-symbol-fn]
> struct fsm *flag_create_symbol(int type, char *name, char *value)

> [spec:foma:sem:flags.flag-create-symbol-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-eliminate-fn]
> struct fsm *flag_eliminate(struct fsm *net, char *name)

> [spec:foma:sem:flags.flag-eliminate-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-extract-fn]
> struct flags *flag_extract (struct fsm *net)

> [spec:foma:sem:flags.flag-extract-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-get-name-fn]
> char *flag_get_name(char *string)

> [spec:foma:sem:flags.flag-get-name-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-get-type-fn]
> int flag_get_type(char *string)

> [spec:foma:sem:flags.flag-get-type-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-get-value-fn]
> char *flag_get_value(char *string)

> [spec:foma:sem:flags.flag-get-value-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-purge-fn]
> void flag_purge (struct fsm *net, char *name)

> [spec:foma:sem:flags.flag-purge-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-twosided-fn]
> struct fsm *flag_twosided(struct fsm *net)

> [spec:foma:sem:flags.flag-twosided-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flag-type-to-char-fn]
> char *flag_type_to_char (int type)

> [spec:foma:sem:flags.flag-type-to-char-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:flags.flags]
> struct flags {
>   int type;
>   char *name;
>   char *value;
>   struct flags *next;
> }


# foma/lexcread.c

> [spec:foma:def:lexcread.lexc-add-mc-fn]
> void lexc_add_mc(char *symbol)

> [spec:foma:sem:lexcread.lexc-add-mc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-add-network-fn]
> void lexc_add_network()

> [spec:foma:sem:lexcread.lexc-add-network-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-add-sigma-hash-fn]
> void lexc_add_sigma_hash(char *symbol, int number)

> [spec:foma:sem:lexcread.lexc-add-sigma-hash-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-add-state-fn]
> void lexc_add_state(struct states *s)

> [spec:foma:sem:lexcread.lexc-add-state-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-add-word-fn]
> void lexc_add_word()

> [spec:foma:sem:lexcread.lexc-add-word-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-cleanup-fn]
> void lexc_cleanup()

> [spec:foma:sem:lexcread.lexc-cleanup-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-clear-current-word-fn]
> void lexc_clear_current_word()

> [spec:foma:sem:lexcread.lexc-clear-current-word-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-deescape-string-fn]
> void lexc_deescape_string(char *name, char escape, int mode)

> [spec:foma:sem:lexcread.lexc-deescape-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-eq-paths-fn]
> int lexc_eq_paths(struct states *one, struct states *two)

> [spec:foma:sem:lexcread.lexc-eq-paths-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-find-delim-fn]
> char *lexc_find_delim(char *name, char delimiter, char escape)

> [spec:foma:sem:lexcread.lexc-find-delim-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-find-lex-state-fn]
> struct states *lexc_find_lex_state(char *name)

> [spec:foma:sem:lexcread.lexc-find-lex-state-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-find-mc-fn]
> int lexc_find_mc(char *symbol)

> [spec:foma:sem:lexcread.lexc-find-mc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-find-sigma-hash-fn]
> int lexc_find_sigma_hash(char *symbol)

> [spec:foma:sem:lexcread.lexc-find-sigma-hash-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-hashtable]
> struct lexc_hashtable {
>   char *symbol;
>   struct lexc_hashtable *next;
>   int sigma_number;
> }

> [spec:foma:def:lexcread.lexc-init-fn]
> void lexc_init()

> [spec:foma:sem:lexcread.lexc-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-medpad-fn]
> void lexc_medpad()

> [spec:foma:sem:lexcread.lexc-medpad-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-merge-states-fn]
> void lexc_merge_states()

> [spec:foma:sem:lexcread.lexc-merge-states-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-number-states-fn]
> void lexc_number_states()

> [spec:foma:sem:lexcread.lexc-number-states-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-pad-fn]
> void lexc_pad()

> [spec:foma:sem:lexcread.lexc-pad-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-set-current-lexicon-fn]
> void lexc_set_current_lexicon(char *name, int which)

> [spec:foma:sem:lexcread.lexc-set-current-lexicon-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-set-current-word-fn]
> void lexc_set_current_word(char *name)

> [spec:foma:sem:lexcread.lexc-set-current-word-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-set-network-fn]
> void lexc_set_network(struct fsm *net)

> [spec:foma:sem:lexcread.lexc-set-network-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-string-to-tokens-fn]
> void lexc_string_to_tokens(char *string, int *intarr)

> [spec:foma:sem:lexcread.lexc-string-to-tokens-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-suffix-hash-fn]
> static unsigned int lexc_suffix_hash(int offset)

> [spec:foma:sem:lexcread.lexc-suffix-hash-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-symbol-hash-fn]
> static unsigned int lexc_symbol_hash(char *s)

> [spec:foma:sem:lexcread.lexc-symbol-hash-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-to-fsm-fn]
> struct fsm *lexc_to_fsm()

> [spec:foma:sem:lexcread.lexc-to-fsm-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexc-update-unknowns-fn]
> void lexc_update_unknowns(int sigma_number)

> [spec:foma:sem:lexcread.lexc-update-unknowns-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.lexstates]
> struct lexstates {
>   char *name;
>   struct states *state;
>   struct lexstates *next;
>   unsigned char targeted;
>   unsigned char has_outgoing;
> }

> [spec:foma:def:lexcread.multichar-symbols]
> struct multichar_symbols {
>   char *symbol;
>   short int sigma_number;
>   struct multichar_symbols *next;
> }

> [spec:foma:def:lexcread.mystrncpy-fn]
> char *mystrncpy(char *dest, char *src, int len)

> [spec:foma:sem:lexcread.mystrncpy-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:lexcread.statelist]
> struct statelist {
>   struct states *state;
>   struct statelist *next;
>   char start;
>   char final;
> }

> [spec:foma:def:lexcread.states]
> struct states {
>   struct trans { short int in; short int out; struct states *target; struct trans *next; } *trans;
>   struct lexstates *lexstate;
>   int number;
>   unsigned int hashval;
>   unsigned char mergeable;
>   unsigned short int distance;
>   struct states *merge_with;
> }

> [spec:foma:def:lexcread.states.trans]
> struct trans {
>   short int in;
>   short int out;
>   struct states *target;
>   struct trans *next;
> }


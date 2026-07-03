# foma/fomalibconf.h

> [spec:foma:def:fomalibconf.add-fsm-arc-fn]
> int add_fsm_arc(struct fsm_state *fsm, int offset, int state_no, int in, int out, int target, int final_state, int start_state)

> [spec:foma:sem:fomalibconf.add-fsm-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.apply-handle]
> struct apply_handle {
>   int ptr;
>   int curr_ptr;
>   int ipos;
>   int opos;
>   int mode;
>   int printcount;
>   int *numlines;
>   int *statemap;
>   int *marks;
>   struct sigma_trie { int signum; struct sigma_trie *next; } *sigma_trie;
>   struct sigmatch_array { int signumber ; int consumes ; } *sigmatch_array;
>   struct sigma_trie_arrays { struct sigma_trie *arr; struct sigma_trie_arrays *next; } *sigma_trie_arrays;
>   int binsearch;
>   int indexed;
>   int state_has_index;
>   int sigma_size;
>   int sigmatch_array_size;
>   int current_instring_length;
>   int has_flags;
>   int obey_flags;
>   int show_flags;
>   int print_space;
>   char *space_symbol;
>   char *separator;
>   char *epsilon_symbol;
>   int print_pairs;
>   int apply_stack_ptr;
>   int apply_stack_top;
>   int oldflagneg;
>   int outstringtop;
>   int iterate_old;
>   int iterator;
>   uint8_t *flagstates;
>   char *outstring;
>   char *instring;
>   struct sigs { char *symbol; int length; } *sigs;
>   char *oldflagvalue;
>   struct fsm *last_net;
>   struct fsm_state *gstates;
>   struct sigma *gsigma;
>   struct apply_state_index { int fsmptr; struct apply_state_index *next; } **index_in, **index_out, *iptr;
>   struct flag_list { char *name; char *value; short neg; struct flag_list *next; } *flag_list;
>   struct flag_lookup { int type; char *name; char *value; } *flag_lookup;
>   struct searchstack { int offset; struct apply_state_index *iptr; int state_has_index; int opos; int ipos; int visitmark; char *flagname; char *flagvalue; int...;
> }

> [spec:foma:def:fomalibconf.apply-handle.apply-state-index]
> struct apply_state_index {
>   int fsmptr;
>   struct apply_state_index *next;
> }

> [spec:foma:def:fomalibconf.apply-handle.flag-list]
> struct flag_list {
>   char *name;
>   char *value;
>   short neg;
>   struct flag_list *next;
> }

> [spec:foma:def:fomalibconf.apply-handle.flag-lookup]
> struct flag_lookup {
>   int type;
>   char *name;
>   char *value;
> }

> [spec:foma:def:fomalibconf.apply-handle.searchstack]
> struct searchstack {
>   int offset;
>   struct apply_state_index *iptr;
>   int state_has_index;
>   int opos;
>   int ipos;
>   int visitmark;
>   char *flagname;
>   char *flagvalue;
>   int flagneg;
> }

> [spec:foma:def:fomalibconf.apply-handle.sigma-trie]
> struct sigma_trie {
>   int signum;
>   struct sigma_trie *next;
> }

> [spec:foma:def:fomalibconf.apply-handle.sigma-trie-arrays]
> struct sigma_trie_arrays {
>   struct sigma_trie *arr;
>   struct sigma_trie_arrays *next;
> }

> [spec:foma:def:fomalibconf.apply-handle.sigmatch-array]
> struct sigmatch_array {
>   int signumber;
>   int consumes;
> }

> [spec:foma:def:fomalibconf.apply-handle.sigs]
> struct sigs {
>   char *symbol;
>   int length;
> }

> [spec:foma:def:fomalibconf.apply-med-handle]
> struct apply_med_handle {
>   struct astarnode { short int wordpos; int fsmstate; short int f; short int g; short int h; int in; int out; int parent; } *agenda;
>   int bytes_per_letter_array;
>   uint8_t *letterbits;
>   uint8_t *nletterbits;
>   int astarcount;
>   int heapcount;
>   int heap_size;
>   int agenda_size;
>   int maxdepth;
>   int maxsigma;
>   int wordlen;
>   int utf8len;
>   int cost;
>   int nummatches;
>   int curr_state;
>   int curr_g;
>   int curr_pos;
>   int lines;
>   int curr_agenda_offset;
>   int curr_node_has_match;
>   int med_limit;
>   int med_cutoff;
>   int med_max_heap_size;
>   int nodes_expanded;
>   int *cm;
>   char *word;
>   char *instring;
>   int instring_length;
>   char *outstring;
>   int outstring_length;
>   char *align_symbol;
>   int *heap;
>   int *intword;
>   struct sh_handle *sigmahash;
>   struct state_array *state_array;
>   struct fsm *net;
>   struct fsm_state *curr_ptr;
>   _Bool hascm;
> }

> [spec:foma:def:fomalibconf.apply-med-handle.astarnode]
> struct astarnode {
>   short int wordpos;
>   int fsmstate;
>   short int f;
>   short int g;
>   short int h;
>   int in;
>   int out;
>   int parent;
> }

> [spec:foma:def:fomalibconf.decode-quoted-fn]
> void decode_quoted(char *s)

> [spec:foma:sem:fomalibconf.decode-quoted-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.dequote-string-fn]
> void dequote_string(char *s)

> [spec:foma:sem:fomalibconf.dequote-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.escape-string-fn]
> char *escape_string(char *string, char chr)

> [spec:foma:sem:fomalibconf.escape-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.find-arccount-fn]
> int find_arccount(struct fsm_state *fsm)

> [spec:foma:sem:fomalibconf.find-arccount-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.flag-check-fn]
> int flag_check(char *sm)

> [spec:foma:sem:fomalibconf.flag-check-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.flag-get-name-fn]
> char *flag_get_name(char *string)

> [spec:foma:sem:fomalibconf.flag-get-name-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.flag-get-type-fn]
> int flag_get_type(char *string)

> [spec:foma:sem:fomalibconf.flag-get-type-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.flag-get-value-fn]
> char *flag_get_value(char *string)

> [spec:foma:sem:fomalibconf.flag-get-value-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.fsm-construct-handle]
> struct fsm_construct_handle {
>   struct fsm_state_list *fsm_state_list;
>   int fsm_state_list_size;
>   struct fsm_sigma_list *fsm_sigma_list;
>   int fsm_sigma_list_size;
>   struct fsm_sigma_hash *fsm_sigma_hash;
>   int fsm_sigma_hash_size;
>   int maxstate;
>   int maxsigma;
>   int numfinals;
>   int hasinitial;
>   char *name;
> }

> [spec:foma:def:fomalibconf.fsm-count-fn]
> FEXPORT void fsm_count(struct fsm *net)

> [spec:foma:sem:fomalibconf.fsm-count-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.fsm-read-binary-handle]
> typedef void *fsm_read_binary_handle

> [spec:foma:def:fomalibconf.fsm-sigma-hash]
> struct fsm_sigma_hash {
>   char *symbol;
>   short int sym;
>   struct fsm_sigma_hash *next;
> }

> [spec:foma:def:fomalibconf.fsm-sigma-list]
> struct fsm_sigma_list {
>   char *symbol;
> }

> [spec:foma:def:fomalibconf.fsm-sort-lines-fn]
> void fsm_sort_lines(struct fsm *net)

> [spec:foma:sem:fomalibconf.fsm-sort-lines-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.fsm-state-add-arc-fn]
> void fsm_state_add_arc(int state_no, int in, int out, int target, int final_state, int start_state)

> [spec:foma:sem:fomalibconf.fsm-state-add-arc-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.fsm-state-close-fn]
> void fsm_state_close(struct fsm *net)

> [spec:foma:sem:fomalibconf.fsm-state-close-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.fsm-state-copy-fn]
> struct fsm_state *fsm_state_copy(struct fsm_state *fsm_state, int linecount)

> [spec:foma:sem:fomalibconf.fsm-state-copy-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.fsm-state-end-state-fn]
> void fsm_state_end_state()

> [spec:foma:sem:fomalibconf.fsm-state-end-state-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.fsm-state-init-fn]
> struct fsm_state *fsm_state_init(int sigma_size)

> [spec:foma:sem:fomalibconf.fsm-state-init-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.fsm-state-list]
> struct fsm_state_list {
>   _Bool used;
>   _Bool is_final;
>   _Bool is_initial;
>   short int num_trans;
>   int state_number;
>   struct fsm_trans_list *fsm_trans_list;
> }

> [spec:foma:def:fomalibconf.fsm-state-set-current-state-fn]
> void fsm_state_set_current_state(int state_no, int final_state, int start_state)

> [spec:foma:sem:fomalibconf.fsm-state-set-current-state-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.fsm-trans-list]
> struct fsm_trans_list {
>   short int in;
>   short int out;
>   int target;
>   struct fsm_trans_list *next;
> }

> [spec:foma:def:fomalibconf.fsm-update-flags-fn]
> void fsm_update_flags(struct fsm *net, int det, int pru, int min, int eps, int loop, int completed)

> [spec:foma:sem:fomalibconf.fsm-update-flags-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.int-stack-clear-fn]
> void int_stack_clear()

> [spec:foma:sem:fomalibconf.int-stack-clear-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.int-stack-find-fn]
> int int_stack_find (int entry)

> [spec:foma:sem:fomalibconf.int-stack-find-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.int-stack-isempty-fn]
> int int_stack_isempty()

> [spec:foma:sem:fomalibconf.int-stack-isempty-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.int-stack-isfull-fn]
> int int_stack_isfull()

> [spec:foma:sem:fomalibconf.int-stack-isfull-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.int-stack-pop-fn]
> int int_stack_pop()

> [spec:foma:sem:fomalibconf.int-stack-pop-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.int-stack-push-fn]
> void int_stack_push(int c)

> [spec:foma:sem:fomalibconf.int-stack-push-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.int-stack-size-fn]
> int int_stack_size()

> [spec:foma:sem:fomalibconf.int-stack-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.int-stack-status-fn]
> int int_stack_status()

> [spec:foma:sem:fomalibconf.int-stack-status-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.ishexstr-fn]
> int ishexstr(char *str)

> [spec:foma:sem:fomalibconf.ishexstr-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.map-firstlines-fn]
> struct state_array *map_firstlines(struct fsm *net)

> [spec:foma:sem:fomalibconf.map-firstlines-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.next-power-of-two-fn]
> int next_power_of_two(int v)

> [spec:foma:sem:fomalibconf.next-power-of-two-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.ptr-stack-clear-fn]
> void ptr_stack_clear()

> [spec:foma:sem:fomalibconf.ptr-stack-clear-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.ptr-stack-isempty-fn]
> int ptr_stack_isempty()

> [spec:foma:sem:fomalibconf.ptr-stack-isempty-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.ptr-stack-isfull-fn]
> int ptr_stack_isfull()

> [spec:foma:sem:fomalibconf.ptr-stack-isfull-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.ptr-stack-pop-fn]
> void *ptr_stack_pop()

> [spec:foma:sem:fomalibconf.ptr-stack-pop-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.ptr-stack-push-fn]
> void ptr_stack_push(void *ptr)

> [spec:foma:sem:fomalibconf.ptr-stack-push-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.remove-trailing-fn]
> char *remove_trailing(char *s, char c)

> [spec:foma:sem:fomalibconf.remove-trailing-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.round-up-to-power-of-two-fn]
> unsigned int round_up_to_power_of_two(unsigned int v)

> [spec:foma:sem:fomalibconf.round-up-to-power-of-two-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-add-fn]
> FEXPORT int sigma_add (char *symbol, struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-add-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-add-number-fn]
> FEXPORT int sigma_add_number(struct sigma *sigma, char *symbol, int number)

> [spec:foma:sem:fomalibconf.sigma-add-number-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-add-special-fn]
> FEXPORT int sigma_add_special (int symbol, struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-add-special-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-cleanup-fn]
> void sigma_cleanup (struct fsm *net, int force)

> [spec:foma:sem:fomalibconf.sigma-cleanup-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-create-fn]
> sigma *sigma_create ()

> [spec:foma:sem:fomalibconf.sigma-create-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-find-fn]
> int sigma_find (char *symbol, struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-find-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-find-number-fn]
> int sigma_find_number (int number, struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-find-number-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-max-fn]
> FEXPORT int sigma_max(struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-max-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-remove-fn]
> sigma *sigma_remove(char *symbol, struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-remove-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-remove-num-fn]
> sigma *sigma_remove_num(int num, struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-remove-num-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-size-fn]
> int sigma_size(struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-size-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-sort-fn]
> int sigma_sort (struct fsm *net)

> [spec:foma:sem:fomalibconf.sigma-sort-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-string-fn]
> FEXPORT char *sigma_string(int number, struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-string-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-substitute-fn]
> int sigma_substitute(char *orig, char *sub, struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-substitute-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sigma-to-list-fn]
> struct fsm_sigma_list *sigma_to_list(struct sigma *sigma)

> [spec:foma:sem:fomalibconf.sigma-to-list-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.sort-cmp-fn]
> int sort_cmp(const void *a, const void *b)

> [spec:foma:sem:fomalibconf.sort-cmp-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.state-array]
> struct state_array {
>   struct fsm_state *transitions;
> }

> [spec:foma:def:fomalibconf.streqrep-fn]
> char *streqrep(char *s, char *oldstring, char *newstring)

> [spec:foma:sem:fomalibconf.streqrep-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.strip-newline-fn]
> void strip_newline(char *s)

> [spec:foma:sem:fomalibconf.strip-newline-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.trim-fn]
> char *trim(char *string)

> [spec:foma:sem:fomalibconf.trim-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.utf8code16tostr-fn]
> unsigned char *utf8code16tostr(char *str)

> [spec:foma:sem:fomalibconf.utf8code16tostr-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.utf8iscombining-fn]
> int utf8iscombining(unsigned char *s)

> [spec:foma:sem:fomalibconf.utf8iscombining-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.utf8skip-fn]
> int utf8skip(char *str)

> [spec:foma:sem:fomalibconf.utf8skip-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.utf8strlen-fn]
> int utf8strlen(char *str)

> [spec:foma:sem:fomalibconf.utf8strlen-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.xprintf-fn]
> void xprintf(char *string)

> [spec:foma:sem:fomalibconf.xprintf-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.xstrrev-fn]
> char *xstrrev(char *str)

> [spec:foma:sem:fomalibconf.xstrrev-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.

> [spec:foma:def:fomalibconf.xxstrndup-fn]
> char *xxstrndup(const char *s, size_t n)

> [spec:foma:sem:fomalibconf.xxstrndup-fn]
> TODO(sem): what this does, step by step — precisely enough to
> re-implement from this rule alone, without reading the source.


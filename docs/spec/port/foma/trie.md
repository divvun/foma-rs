# foma/trie.c

> [spec:foma:def:trie.fsm-trie-add-word-fn]
> void fsm_trie_add_word(struct fsm_trie_handle *th, char *word)

> [spec:foma:sem:trie.fsm-trie-add-word-fn]
> Adds one word to the trie as a chain of identity (insym == outsym) transitions, one per
> UTF-8 character, then marks the reached state final. Steps: `wcopy = strdup(word)` as a
> scratch buffer; `len = strlen(word)` (byte length). Loop with counter `i` from 0 while
> `*word != '\0' && i < len`: let `k = utf8skip(word)` (number of UTF-8 continuation
> bytes of the character at `word`: 0 for ASCII, 1–3 for multibyte lead bytes, -1 for an
> invalid lead byte); copy the `k+1` bytes of the current character into `wcopy` and
> NUL-terminate at `wcopy[k+1]`; call `fsm_trie_symbol(th, wcopy, wcopy)`; advance
> `word += k + 1` and `i++`. (For an invalid byte `k = -1`, so `word` does not advance;
> the `i < len` bound still terminates the loop after `len` iterations.)
> After the loop, free `wcopy` and call `fsm_trie_end_word(th)` (mark current state
> final, reset cursor to the root, state 0). Cursor starts wherever it was — normally 0.

> [spec:foma:def:trie.fsm-trie-done-fn]
> struct fsm *fsm_trie_done(struct fsm_trie_handle *th)

> [spec:foma:sem:trie.fsm-trie-done-fn]
> Converts the accumulated trie into a `struct fsm` and frees the trie handle. Steps:
> `newh = fsm_construct_init("name")` (the network is literally named "name"). For each
> of the 1048573 hash buckets: walk the chain from the in-array head; for each node with
> `insym != NULL` call `fsm_construct_add_arc(newh, sourcestate, targetstate, insym,
> outsym)`; on a node with `insym == NULL` (only possible for an unused head) break out
> of that bucket. Then for every state number `i` from 0 through `th->used_states`
> inclusive, if `trie_states[i].is_final == 1` call `fsm_construct_set_final(newh, i)`.
> Set the initial state to 0 with `fsm_construct_set_initial(newh, 0)` and finish with
> `newnet = fsm_construct_done(newh)`.
> Cleanup: for each bucket free all chained overflow nodes (`head->next` onward; heads
> live inline in the array), then `sh_done(th->sh_hash)` (frees the interned symbol
> strings the arcs referenced during construction), then free `trie_states`, `trie_hash`,
> and the handle itself. Returns the constructed fsm; caller owns it.

> [spec:foma:def:trie.fsm-trie-end-word-fn]
> void fsm_trie_end_word(struct fsm_trie_handle *th)

> [spec:foma:sem:trie.fsm-trie-end-word-fn]
> Terminates the word being inserted: sets `trie_states[th->trie_cursor].is_final = 1`
> (the state reached by the preceding fsm_trie_symbol calls), then resets
> `th->trie_cursor = 0` so the next word starts from the root state. Calling it with the
> cursor already at 0 (e.g. for an empty word) marks the root state final.

> [spec:foma:def:trie.fsm-trie-init-fn]
> struct fsm_trie_handle *fsm_trie_init()

> [spec:foma:sem:trie.fsm-trie-init-fn]
> Allocates and returns a new trie handle with all state zeroed. `calloc(1, ...)` the
> `fsm_trie_handle` (so `used_states = 0`); `trie_hash` = calloc'd array of 1048573
> (THASH_TABLESIZE) `trie_hash` bucket-head structs (all fields NULL/0); `trie_states` =
> calloc'd array of 32768 (TRIE_STATESIZE) `trie_states` structs (all non-final);
> `statesize = 32768`; `trie_cursor = 0` (root state is state 0); `sh_hash = sh_init()`
> (per-trie string-interning table for symbols). Caller must finish with fsm_trie_done,
> which frees everything. No allocation-failure checking.

> [spec:foma:def:trie.fsm-trie-symbol-fn]
> void fsm_trie_symbol(struct fsm_trie_handle *th, char *insym, char *outsym)

> [spec:foma:sem:trie.fsm-trie-symbol-fn]
> Follows or creates the transition `(trie_cursor, insym:outsym)` and moves the cursor to
> its target. Lookup: `h = trie_hashf(th->trie_cursor, insym, outsym)`; if the bucket
> head's `insym` is non-NULL, walk the chain and on the first node whose `insym` and
> `outsym` both strcmp-equal the arguments AND whose `sourcestate == th->trie_cursor`,
> set `th->trie_cursor = node->targetstate` and return.
> Insert (no match): increment `th->used_states` — this is the new target state number.
> If the head's `insym` is NULL, fill the head in place; otherwise calloc a new
> `trie_hash` node and splice it in right after the head (`new->next = head->next;
> head->next = new`). Either way the entry gets `insym`/`outsym` interned via
> `sh_find_add_string(th->sh_hash, sym, 1)` (pointers into the handle's string table,
> not copies of the caller's buffers), `sourcestate = th->trie_cursor`,
> `targetstate = th->used_states`. Then set `th->trie_cursor = th->used_states`.
> Growth: if `used_states >= statesize`, set `statesize = next_power_of_two(statesize)`
> (doubles it, since statesize is always a power of two) and realloc `trie_states` to the
> new size (grown region uninitialized). Finally set
> `trie_states[used_states].is_final = 0` for the new state.

> [spec:foma:def:trie.trie-hashf-fn]
> unsigned int trie_hashf(unsigned int source, char *insym, char *outsym)

> [spec:foma:sem:trie.trie-hashf-fn]
> Hashes a transition (source state, input symbol, output symbol) to a bucket index.
> Start with `hash = 0` (unsigned 32-bit, wrapping arithmetic). For each byte of
> NUL-terminated `insym`: `hash = hash * 101 + byte`; then for each byte of `outsym`
> likewise; each `byte` is the C `char` value (signed on typical platforms, so bytes >=
> 0x80 add sign-extended negatives). Finally `hash = hash * 101 + source` and return
> `hash % 1048573` (1048573 = THASH_TABLESIZE). Pure function.


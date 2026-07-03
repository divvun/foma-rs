# foma/stringhash.c

> [spec:foma:def:stringhash.sh-add-string-fn]
> char *sh_add_string(struct sh_handle *sh, char *string, int value)

> [spec:foma:sem:stringhash.sh-add-string-fn]
> Unconditionally inserts `string` with `value` into the hash table (no duplicate check;
> callers wanting dedup use sh_find_add_string). Computes bucket index `sh_hashf(string)`
> and takes the in-array head slot `sh->hash + index`.
> If the head slot's `string` field is NULL (bucket empty): set it to `strdup(string)`,
> set its `value` field to `value`, and return the duplicated string pointer.
> Otherwise: malloc a new `sh_hashtable` node, set its `string` to `strdup(string)` and
> `value` to `value`, splice it in immediately after the head (`new->next = head->next;
> head->next = new`), and return the new node's string pointer.
> Ownership: the table owns the returned copy (freed by sh_done); the caller's input
> string is not retained. `sh->lastvalue` is not touched.

> [spec:foma:def:stringhash.sh-done-fn]
> void sh_done(struct sh_handle *sh)

> [spec:foma:sem:stringhash.sh-done-fn]
> Frees the entire string hash. For each of the 8191 bucket head slots (which live inline
> in the `sh->hash` array): free the head's `string` if non-NULL; then walk the overflow
> chain starting at `head->next`, and for each chained node (saving its `next` pointer
> first) free its `string` if non-NULL and free the node itself.
> Finally free the `sh->hash` array and the `sh` handle. All interned string pointers
> previously returned by sh_add_string/sh_find_string become dangling.

> [spec:foma:def:stringhash.sh-find-add-string-fn]
> char *sh_find_add_string(struct sh_handle *sh, char *string, int value)

> [spec:foma:sem:stringhash.sh-find-add-string-fn]
> Interning lookup-or-insert. Calls `sh_find_string(sh, string)`; if it returns non-NULL,
> returns that existing interned pointer (the `value` argument is ignored, and
> `sh->lastvalue` has been set to the stored entry's value as a side effect of the find).
> If the find returns NULL, calls `sh_add_string(sh, string, value)` and returns its
> result (a fresh table-owned copy; `sh->lastvalue` is not updated in this branch).

> [spec:foma:def:stringhash.sh-find-string-fn]
> char *sh_find_string(struct sh_handle *sh, char *string)

> [spec:foma:sem:stringhash.sh-find-string-fn]
> Looks up `string` in the table. Computes bucket index `sh_hashf(string)` and walks the
> chain starting at the in-array head slot, following `next` pointers.
> At each node: if the node's `string` field is NULL, return NULL immediately (an empty
> head slot means the bucket has never been used); if `strcmp(node->string, string) == 0`,
> set `sh->lastvalue = node->value` and return the node's stored string pointer (the
> interned copy owned by the table — not the caller's argument).
> If the chain is exhausted (next == NULL), return NULL. On a miss `sh->lastvalue` is
> left unchanged.

> [spec:foma:def:stringhash.sh-get-value-fn]
> int sh_get_value(struct sh_handle *sh)

> [spec:foma:sem:stringhash.sh-get-value-fn]
> Returns `sh->lastvalue`: the `value` field of the entry matched by the most recent
> successful `sh_find_string` on this handle (including finds made through
> `sh_find_add_string`). No other operation sets it; if no successful find has occurred
> the result is the field's initial (uninitialized, from malloc) content. Pure read,
> no state change.

> [spec:foma:def:stringhash.sh-hashf-fn]
> unsigned int sh_hashf(char *string)

> [spec:foma:sem:stringhash.sh-hashf-fn]
> Hash function over a NUL-terminated byte string. Start with `hash = 0` (unsigned
> 32-bit). For each byte until the NUL: `hash = hash * 101 + byte`, where `byte` is the
> C `char` value (signed on typical platforms, so bytes >= 0x80 contribute a
> sign-extended negative value) and all arithmetic wraps modulo 2^32.
> Return `hash % 8191` (8191 = STRING_HASH_SIZE, the bucket count). Empty string
> hashes to 0. Pure function, no state.

> [spec:foma:def:stringhash.sh-init-fn]
> struct sh_handle *sh_init()

> [spec:foma:sem:stringhash.sh-init-fn]
> Allocates and returns a new string-hash handle. mallocs a `sh_handle`, then sets its
> `hash` field to a zero-initialized (calloc) array of 8191 (STRING_HASH_SIZE)
> `sh_hashtable` structs — these are the bucket heads, stored inline in the array, each
> starting with `string = NULL`, `value = 0`, `next = NULL`. The `lastvalue` field is
> left uninitialized. Caller owns the handle and must release it with sh_done.
> No allocation-failure checking.


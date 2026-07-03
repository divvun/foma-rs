# foma/define.c

> [spec:foma:def:define.add-defined-fn]
> int add_defined(struct defined_networks *def, struct fsm *net, char *string)

> [spec:foma:sem:define.add-defined-fn]
> Inserts or replaces a named network in the registry — a singly linked list whose head node is a
> permanent dummy that doubles as the first usable slot, so the caller's head pointer never changes.
> If net is NULL, does nothing and returns 0. If strlen(string) > FSM_NAME_LEN (40), returns -1
> without storing or freeing net. Otherwise calls fsm_count(net) (refreshes state/arc/final counts),
> then linearly scans for a node with non-NULL name strcmp-equal to string: on a hit it
> fsm_destroy()s the old net, free()s the old name, stores net directly (ownership transferred, no
> copy) with name = strdup(string), and returns 1 (redefinition). On a miss: if the head's name is
> NULL (empty registry) the head node itself is filled; otherwise a new node is malloc'd and spliced
> in immediately after the head (new->next = head->next; head->next = new). The new node gets
> name = strdup(string), net = net; returns 0.

> [spec:foma:def:define.add-defined-function-fn]
> int add_defined_function(struct defined_functions *deff, char *name, char *regex, int numargs)

> [spec:foma:sem:define.add-defined-function-fn]
> Registers a regex "function" (a regex template later expanded by the parser) keyed by
> (name, numargs); names are stored as given (callers pass @-prefixed names, e.g. "@myfunc").
> Scans the list for a node with non-NULL name strcmp-equal to name AND numargs == numargs: on a
> hit it free()s the old regex, stores strdup(regex), prints "redefined %s@%i)\n" (literal text,
> including the unbalanced trailing ')') to stderr and fflush()es it when g_verbose is nonzero, and
> returns 1. On a miss: fills the dummy head node if its name is NULL, else malloc's a new node
> spliced in immediately after the head (new->next = head->next; head->next = new). Sets
> name = strdup(name), regex = strdup(regex), numargs = numargs; returns 0. Both string arguments
> are duplicated — the caller keeps ownership of what it passed in.

> [spec:foma:def:define.defined-functions-init-fn]
> struct defined_functions *defined_functions_init(void)

> [spec:foma:sem:define.defined-functions-init-fn]
> Allocates (calloc, so fully zeroed) a single struct defined_functions node and returns it. This
> node is a permanent dummy head: name/regex/next NULL, numargs 0. It is filled in-place by the
> first `[spec:foma:sem:define.add-defined-function-fn]` call, so the registry pointer held by
> callers stays valid for the life of the registry.

> [spec:foma:def:define.defined-networks-init-fn]
> struct defined_networks *defined_networks_init(void)

> [spec:foma:sem:define.defined-networks-init-fn]
> Allocates (calloc, so fully zeroed) a single struct defined_networks node and returns it. The
> node is a permanent dummy head (name/net/next all NULL) filled in-place on first definition, so
> the head pointer returned here never changes; the global registry g_defines is created this way.

> [spec:foma:def:define.find-defined-fn]
> struct fsm *find_defined(struct defined_networks *def, char *string)

> [spec:foma:sem:define.find-defined-fn]
> Linear scan of the defined-networks list: returns the stored struct fsm * of the first node whose
> name is non-NULL and strcmp-equal to string, or NULL if no such definition exists. The returned
> net is the registry's own copy, not a duplicate — callers must fsm_copy() it before mutating or
> consuming it.

> [spec:foma:def:define.find-defined-function-fn]
> char *find_defined_function(struct defined_functions *deff, char *name, int numargs)

> [spec:foma:sem:define.find-defined-function-fn]
> Linear scan of the defined-functions list: returns the stored regex string (not a copy) of the
> first node with non-NULL name strcmp-equal to name AND numargs exactly equal to numargs; NULL if
> absent. Because numargs participates in the key, the same function name may be overloaded by
> arity.

> [spec:foma:def:define.remove-defined-fn]
> int remove_defined(struct defined_networks *def, char *string)

> [spec:foma:sem:define.remove-defined-fn]
> Removes one definition (or all, if string is NULL); returns 0 on success, 1 if the named
> definition does not exist.
> Undefine-all (string == NULL): walks every node, calling fsm_destroy(net) when net is non-NULL
> and free(name) when name is non-NULL, then returns 0. BUG (document literal behavior): the nodes
> themselves are never freed and their name/net fields are not NULLed, so the list is left full of
> dangling pointers — any subsequent find/add/remove on the same registry reads freed memory.
> Single removal: scan for the node whose name strcmp-equals string, tracking the predecessor; if
> not found return 1. If the match is the head and has a successor: destroy the head's net, free
> its name, move the successor's name/net into the head, splice the successor out of the list and
> free it (the head address is preserved). If the match is the head and is the only node: destroy
> net, free name, and set name/net/next to NULL (back to the empty dummy state). Otherwise (non-
> head): destroy net, free name, set predecessor->next = node->next, free the node. Return 0.


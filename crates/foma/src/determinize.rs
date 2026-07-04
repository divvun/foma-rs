//! foma/determinize.c — literal (bug-for-bug) Wave-2 port per
//! docs/port/rust-conventions.md. Sem rules: docs/spec/port/foma/determinize.md
//! (per-file ids) plus the fomalib.h prototype ids for fsm_determinize /
//! fsm_epsilon_remove.
//!
//! Subset construction / epsilon removal engine. The C's pointer-chain
//! structures become index-based pools with the identical link discipline
//! (see minimize.rs for the established pattern):
//! - e_closure_memo: calloc'd head-node array + malloc'd chain nodes → one
//!   pool Vec (heads at indices 0..num_states, state number == index; chain
//!   nodes appended after); `target`/`next` pointers → Option<usize> pool
//!   indices; the DFS pushes pool indices on the ptr stack where the C
//!   pushes node pointers.
//! - nhash table: calloc'd bucket-head array → Vec<NhashList>, malloc'd
//!   collision nodes → owned Option<Box<NhashList>> chains with the same
//!   splice-after-head order.
//! - trans_array/trans_list: per-state interior pointers into the shared
//!   entry pool → base offsets (usize).
//!
//! File-static state → thread_local! per the conventions (non-reentrancy is
//! part of the contract, exactly as in C).

use std::cell::{Cell, RefCell};

use crate::constructions::fsm_count;
use crate::dynarray::{
    fsm_state_add_arc, fsm_state_close, fsm_state_end_state, fsm_state_init,
    fsm_state_set_current_state,
};
use crate::int_stack::{
    int_stack_clear, int_stack_isempty, int_stack_pop, int_stack_push, ptr_stack_isempty,
    ptr_stack_pop, ptr_stack_push,
};
use crate::mem::next_power_of_two;
use crate::sigma::sigma_max;
use crate::types::{Fsm, FsmState, EPSILON, UNKNOWN, YES};

/* C: #define SUBSET_EPSILON_REMOVE 1 / SUBSET_DETERMINIZE 2 /
SUBSET_TEST_STAR_FREE 3 */
pub const SUBSET_EPSILON_REMOVE: i32 = 1;
pub const SUBSET_DETERMINIZE: i32 = 2;
pub const SUBSET_TEST_STAR_FREE: i32 = 3;

/// load limit for nhash table size
pub const NHASH_LOAD_LIMIT: i32 = 2;

// [spec:foma:def:determinize.e-closure-memo]
/* target/next are pool indices into E_CLOSURE_MEMO (None ↔ NULL); head
nodes occupy indices 0..num_states (targets always reference head nodes, so
a target index is the target's state number); chain nodes are appended
after. Chain nodes' mark is malloc garbage in C and never read — 0 here. */
#[derive(Debug, Clone, Default)]
pub struct EClosureMemo {
    pub state: i32,
    pub mark: i32,
    pub target: Option<usize>,
    pub next: Option<usize>,
}

/* C: static unsigned int primes[26] = {...}; (never mutated) */
pub(crate) static PRIMES: [u32; 26] = [
    61, 127, 251, 509, 1021, 2039, 4093, 8191, 16381, 32749, 65521, 131071, 262139, 524287,
    1048573, 2097143, 4194301, 8388593, 16777213, 33554393, 67108859, 134217689, 268435399,
    536870909, 1073741789, 2147483647,
];

// [spec:foma:def:determinize.nhash-list]
/* size == 0 marks an empty bucket head (calloc); collision nodes hang off
`next` as an owned chain, spliced in directly after the head as in C. */
#[derive(Debug, Clone, Default)]
pub struct NhashList {
    pub setnum: i32,
    pub size: u32,
    pub set_offset: u32,
    pub next: Option<Box<NhashList>>,
}

// [spec:foma:def:determinize.t-memo]
#[derive(Debug, Clone, Default)]
pub struct TMemo {
    pub finalstart: u8,
    pub size: u32,
    pub set_offset: u32,
}

// [spec:foma:def:determinize.trans-list]
#[derive(Debug, Clone, Default)]
pub struct TransList {
    pub inout: i32,
    pub target: i32,
}

// [spec:foma:def:determinize.trans-array]
/* transitions is the C's struct trans_list * interior pointer into the
trans_list_determinize pool — here the base offset of this state's slice. */
#[derive(Debug, Clone, Default)]
pub struct TransArray {
    pub transitions: usize,
    pub size: u32,
    pub tail: u32,
}

thread_local! {
    // C: static int fsm_linecount, num_states, num_symbols, epsilon_symbol,
    //    *single_sigma_array, *double_sigma_array, limit, num_start_states, op;
    static FSM_LINECOUNT: Cell<i32> = const { Cell::new(0) };
    static NUM_STATES: Cell<i32> = const { Cell::new(0) };
    static NUM_SYMBOLS: Cell<i32> = const { Cell::new(0) };
    static EPSILON_SYMBOL: Cell<i32> = const { Cell::new(0) };
    static SINGLE_SIGMA_ARRAY: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static DOUBLE_SIGMA_ARRAY: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static LIMIT: Cell<i32> = const { Cell::new(0) };
    static NUM_START_STATES: Cell<i32> = const { Cell::new(0) };
    static OP: Cell<i32> = const { Cell::new(0) };

    // C: static _Bool *finals, deterministic, numss;
    static FINALS: RefCell<Vec<bool>> = const { RefCell::new(Vec::new()) };
    static DETERMINISTIC: Cell<bool> = const { Cell::new(false) };
    static NUMSS: Cell<bool> = const { Cell::new(false) };

    // C: static struct e_closure_memo *e_closure_memo; — head-node array
    // plus malloc'd chain nodes, all in one pool here (see module docs)
    static E_CLOSURE_MEMO: RefCell<Vec<EClosureMemo>> = const { RefCell::new(Vec::new()) };

    // C: int T_last_unmarked, T_limit; (non-static globals, but referenced
    // nowhere else in the tree)
    static T_LAST_UNMARKED: Cell<i32> = const { Cell::new(0) };
    static T_LIMIT: Cell<i32> = const { Cell::new(0) };

    // C: struct trans_list { ... } *trans_list_determinize; (non-static
    // global, but referenced nowhere else in the tree)
    static TRANS_LIST_DETERMINIZE: RefCell<Vec<TransList>> = const { RefCell::new(Vec::new()) };
    // C: struct trans_array { ... } *trans_array_determinize; (ditto)
    static TRANS_ARRAY_DETERMINIZE: RefCell<Vec<TransArray>> = const { RefCell::new(Vec::new()) };

    // C: static struct T_memo *T_ptr;
    static T_PTR: RefCell<Vec<TMemo>> = const { RefCell::new(Vec::new()) };

    // C: static int nhash_tablesize, nhash_load, current_setnum, *e_table,
    //    *marktable, *temp_move, mainloop, maxsigma, *set_table,
    //    set_table_size, star_free_mark;
    static NHASH_TABLESIZE: Cell<i32> = const { Cell::new(0) };
    static NHASH_LOAD: Cell<i32> = const { Cell::new(0) };
    static CURRENT_SETNUM: Cell<i32> = const { Cell::new(0) };
    static E_TABLE: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static MARKTABLE: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static TEMP_MOVE: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static MAINLOOP: Cell<i32> = const { Cell::new(0) };
    static MAXSIGMA: Cell<i32> = const { Cell::new(0) };
    static SET_TABLE: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static SET_TABLE_SIZE: Cell<i32> = const { Cell::new(0) };
    static STAR_FREE_MARK: Cell<i32> = const { Cell::new(0) };
    // C: static unsigned int set_table_offset;
    static SET_TABLE_OFFSET: Cell<u32> = const { Cell::new(0) };
    // C: static struct nhash_list *table;
    static TABLE: RefCell<Vec<NhashList>> = const { RefCell::new(Vec::new()) };
}

// [spec:foma:def:determinize.add-fsm-arc-fn]
// [spec:foma:sem:determinize.add-fsm-arc-fn]
/* C: extern int add_fsm_arc(struct fsm_state *fsm, int offset, int state_no,
int in, int out, int target, int final_state, int start_state); — an extern
declaration only; the definition lives in constructions.c (constructions.rs
here). determinize.c never calls it; this re-export mirrors the extern
declaration's visibility. */
pub use crate::constructions::add_fsm_arc;

// [spec:foma:def:determinize.fsm-epsilon-remove-fn]
// [spec:foma:sem:determinize.fsm-epsilon-remove-fn]
// [spec:foma:def:fomalib.fsm-epsilon-remove-fn]
// [spec:foma:sem:fomalib.fsm-epsilon-remove-fn]
pub fn fsm_epsilon_remove(net: Box<Fsm>) -> Box<Fsm> {
    fsm_subset(net, SUBSET_EPSILON_REMOVE)
}

// [spec:foma:def:determinize.fsm-determinize-fn]
// [spec:foma:sem:determinize.fsm-determinize-fn]
// [spec:foma:def:fomalib.fsm-determinize-fn]
// [spec:foma:sem:fomalib.fsm-determinize-fn]
pub fn fsm_determinize(net: Box<Fsm>) -> Box<Fsm> {
    fsm_subset(net, SUBSET_DETERMINIZE)
}

// [spec:foma:def:determinize.fsm-subset-fn]
// [spec:foma:sem:determinize.fsm-subset-fn]
#[allow(non_snake_case)]
pub(crate) fn fsm_subset(net: Box<Fsm>, operation: i32) -> Box<Fsm> {
    let mut net = net;
    let mut T: i32;
    let mut U: i32;

    if net.is_deterministic == YES && operation != SUBSET_TEST_STAR_FREE {
        return net;
    }
    /* Export this var */
    OP.set(operation);
    fsm_count(&mut net);
    NUM_STATES.set(net.statecount);
    DETERMINISTIC.set(true);
    init(&mut net);
    let num_states = NUM_STATES.get();
    nhash_init(if num_states < 12 { 6 } else { num_states / 2 });

    T = initial_e_closure(&net);

    int_stack_clear();

    /* numss is a C _Bool holding the truncated last-seen start state number,
    so numss == 0 really means "the single start state is state 0" */
    if DETERMINISTIC.get() && EPSILON_SYMBOL.get() == -1 && NUM_START_STATES.get() == 1 && !NUMSS.get()
    {
        net.is_deterministic = YES;
        net.is_epsilon_free = YES;
        let table = TABLE.with_borrow_mut(std::mem::take);
        nhash_free(table, NHASH_TABLESIZE.get());
        /* free(T_ptr); free(e_table); free(trans_list_determinize);
        free(trans_array_determinize); free(double_sigma_array);
        free(single_sigma_array); free(finals); free(temp_move);
        free(set_table); */
        T_PTR.with_borrow_mut(|v| *v = Vec::new());
        E_TABLE.with_borrow_mut(|v| *v = Vec::new());
        TRANS_LIST_DETERMINIZE.with_borrow_mut(|v| *v = Vec::new());
        TRANS_ARRAY_DETERMINIZE.with_borrow_mut(|v| *v = Vec::new());
        DOUBLE_SIGMA_ARRAY.with_borrow_mut(|v| *v = Vec::new());
        SINGLE_SIGMA_ARRAY.with_borrow_mut(|v| *v = Vec::new());
        FINALS.with_borrow_mut(|v| *v = Vec::new());
        TEMP_MOVE.with_borrow_mut(|v| *v = Vec::new());
        SET_TABLE.with_borrow_mut(|v| *v = Vec::new());
        return net;
    }

    if operation == SUBSET_EPSILON_REMOVE && EPSILON_SYMBOL.get() == -1 {
        net.is_epsilon_free = YES;
        let table = TABLE.with_borrow_mut(std::mem::take);
        nhash_free(table, NHASH_TABLESIZE.get());
        T_PTR.with_borrow_mut(|v| *v = Vec::new());
        E_TABLE.with_borrow_mut(|v| *v = Vec::new());
        TRANS_LIST_DETERMINIZE.with_borrow_mut(|v| *v = Vec::new());
        TRANS_ARRAY_DETERMINIZE.with_borrow_mut(|v| *v = Vec::new());
        DOUBLE_SIGMA_ARRAY.with_borrow_mut(|v| *v = Vec::new());
        SINGLE_SIGMA_ARRAY.with_borrow_mut(|v| *v = Vec::new());
        FINALS.with_borrow_mut(|v| *v = Vec::new());
        TEMP_MOVE.with_borrow_mut(|v| *v = Vec::new());
        SET_TABLE.with_borrow_mut(|v| *v = Vec::new());
        return net;
    }

    if operation == SUBSET_TEST_STAR_FREE {
        let sm = sigma_max(net.sigma.as_deref());
        fsm_state_init(sm + 1);
        STAR_FREE_MARK.set(0);
    } else {
        let sm = sigma_max(net.sigma.as_deref());
        fsm_state_init(sm);
        /* free(net->states) — the old line table is consumed here. (On the
        STAR_FREE branch above the C leaks it instead; here fsm_state_close
        drops it at the end.) */
        net.states = Vec::new();
    }

    /* init */

    loop {
        'stateloop: {
            let mut symbol_in: i32 = 0;
            let mut symbol_out: i32 = 0;

            let finalstart = T_PTR.with_borrow(|tp| tp[T as usize].finalstart);
            fsm_state_set_current_state(T, finalstart as i32, if T == 0 { 1 } else { 0 });

            /* Prepare set */
            let setsize = T_PTR.with_borrow(|tp| tp[T as usize].size) as i32;
            let mut theset = T_PTR.with_borrow(|tp| tp[T as usize].set_offset) as usize;
            let mut minsym: i32 = i32::MAX; /* INT_MAX */
            let mut has_trans = 0;
            for i in 0..setsize {
                let stateno = SET_TABLE.with_borrow(|st| st[theset + i as usize]);
                let (size0, tbase) = TRANS_ARRAY_DETERMINIZE.with_borrow_mut(|ta| {
                    let tptr = &mut ta[stateno as usize];
                    tptr.tail = 0;
                    (tptr.size, tptr.transitions)
                });
                if size0 == 0 {
                    continue;
                }
                let inout0 = TRANS_LIST_DETERMINIZE.with_borrow(|tl| tl[tbase].inout);
                if inout0 < minsym {
                    minsym = inout0;
                    has_trans = 1;
                }
            }
            if has_trans == 0 {
                /* close state */
                fsm_state_end_state();
                break 'stateloop; /* continue */
            }

            /* While set not empty */

            let mut next_minsym: i32 = i32::MAX;
            while minsym != i32::MAX {
                /* theset = set_table+(T_ptr+T)->set_offset — re-read each
                round (move_set may have realloc'd set_table in C) */
                theset = T_PTR.with_borrow(|tp| tp[T as usize].set_offset) as usize;

                let mut j: i32 = 0;
                for i in 0..setsize {
                    let stateno = SET_TABLE.with_borrow(|st| st[theset + i as usize]);
                    /* C: tail is a local int copy of tptr->tail; transitions
                    walks the pool from tptr->transitions + tail */
                    let (mut tail, tbase, tsize) = TRANS_ARRAY_DETERMINIZE.with_borrow(|ta| {
                        let tptr = &ta[stateno as usize];
                        (tptr.tail, tptr.transitions, tptr.size)
                    });

                    while tail < tsize {
                        let (inout, trgt) = TRANS_LIST_DETERMINIZE.with_borrow(|tl| {
                            let transitions = &tl[tbase + tail as usize];
                            (transitions.inout, transitions.target)
                        });
                        if inout != minsym {
                            break;
                        }
                        let marked = E_TABLE.with_borrow(|et| et[trgt as usize]);
                        if marked != MAINLOOP.get() {
                            E_TABLE.with_borrow_mut(|et| et[trgt as usize] = MAINLOOP.get());
                            TEMP_MOVE.with_borrow_mut(|tm| tm[j as usize] = trgt);
                            j += 1;

                            if operation == SUBSET_EPSILON_REMOVE {
                                MAINLOOP.set(MAINLOOP.get() + 1);
                                U = e_closure(j);
                                if U != -1 {
                                    single_symbol_to_symbol_pair(
                                        minsym,
                                        &mut symbol_in,
                                        &mut symbol_out,
                                    );
                                    let fs =
                                        T_PTR.with_borrow(|tp| tp[T as usize].finalstart);
                                    fsm_state_add_arc(
                                        T,
                                        symbol_in,
                                        symbol_out,
                                        U,
                                        fs as i32,
                                        if T == 0 { 1 } else { 0 },
                                    );
                                    j = 0;
                                }
                            }
                        }
                        tail += 1;
                    }

                    TRANS_ARRAY_DETERMINIZE.with_borrow_mut(|ta| ta[stateno as usize].tail = tail);

                    if tail == tsize {
                        continue;
                    }
                    /* Check next minsym */
                    let inout =
                        TRANS_LIST_DETERMINIZE.with_borrow(|tl| tl[tbase + tail as usize].inout);
                    if inout < next_minsym {
                        next_minsym = inout;
                    }
                }
                if operation == SUBSET_DETERMINIZE {
                    MAINLOOP.set(MAINLOOP.get() + 1);
                    U = e_closure(j);
                    if U != -1 {
                        single_symbol_to_symbol_pair(minsym, &mut symbol_in, &mut symbol_out);
                        let fs = T_PTR.with_borrow(|tp| tp[T as usize].finalstart);
                        fsm_state_add_arc(
                            T,
                            symbol_in,
                            symbol_out,
                            U,
                            fs as i32,
                            if T == 0 { 1 } else { 0 },
                        );
                    }
                }
                if operation == SUBSET_TEST_STAR_FREE {
                    MAINLOOP.set(MAINLOOP.get() + 1);
                    U = e_closure(j);
                    if U != -1 {
                        single_symbol_to_symbol_pair(minsym, &mut symbol_in, &mut symbol_out);
                        let fs = T_PTR.with_borrow(|tp| tp[T as usize].finalstart);
                        fsm_state_add_arc(
                            T,
                            symbol_in,
                            symbol_out,
                            U,
                            fs as i32,
                            if T == 0 { 1 } else { 0 },
                        );
                        if STAR_FREE_MARK.get() == 1 {
                            //fsm_state_add_arc(T, maxsigma, maxsigma, U, (T_ptr+T)->finalstart, T == 0 ? 1 : 0);
                            STAR_FREE_MARK.set(0);
                        }
                    }
                }
                minsym = next_minsym;
                next_minsym = i32::MAX;
            }
            /* end state */
            fsm_state_end_state();
        }
        T = next_unmarked();
        if T == -1 {
            break;
        }
    }

    /* wrapup() */
    let table = TABLE.with_borrow_mut(std::mem::take);
    nhash_free(table, NHASH_TABLESIZE.get());
    SET_TABLE.with_borrow_mut(|v| *v = Vec::new());
    T_PTR.with_borrow_mut(|v| *v = Vec::new());
    TEMP_MOVE.with_borrow_mut(|v| *v = Vec::new());
    E_TABLE.with_borrow_mut(|v| *v = Vec::new());
    TRANS_LIST_DETERMINIZE.with_borrow_mut(|v| *v = Vec::new());
    TRANS_ARRAY_DETERMINIZE.with_borrow_mut(|v| *v = Vec::new());

    if EPSILON_SYMBOL.get() != -1 {
        e_closure_free();
    }
    DOUBLE_SIGMA_ARRAY.with_borrow_mut(|v| *v = Vec::new());
    SINGLE_SIGMA_ARRAY.with_borrow_mut(|v| *v = Vec::new());
    FINALS.with_borrow_mut(|v| *v = Vec::new());
    fsm_state_close(&mut net);
    net
}

// [spec:foma:def:determinize.init-fn]
// [spec:foma:sem:determinize.init-fn]
pub(crate) fn init(net: &mut Fsm) {
    /* A temporary table for handling epsilon closure */
    /* to avoid doubles */

    E_TABLE.with_borrow_mut(|v| *v = vec![0; net.statecount as usize]);

    /* Counter for our access tables */

    MAINLOOP.set(1);

    /* Temporary table for storing sets and */
    /* passing to hash function */

    /* Table for listing current results of move & e-closure */
    /* (malloc — uninitialized in C; zero-filled here, write-before-read) */
    TEMP_MOVE.with_borrow_mut(|v| *v = vec![0; (net.statecount + 1) as usize]);

    /* We malloc this much memory to begin with for the new fsm */
    /* Then grow it by the double as needed */

    LIMIT.set(next_power_of_two(net.linecount));
    FSM_LINECOUNT.set(0);
    sigma_to_pairs(net);

    /* Optimistically malloc T_ptr array */
    /* We allocate memory for a number of pointers to a set of states */
    /* To handle fast lookup in array */
    /* Optimistically, we choose the initial size to be the number of */
    /* states in the non-deterministic fsm */

    T_LAST_UNMARKED.set(0);
    T_LIMIT.set(next_power_of_two(NUM_STATES.get()));

    /* T_ptr = calloc(T_limit,sizeof(struct T_memo)); */
    let t_limit = T_LIMIT.get();
    T_PTR.with_borrow_mut(|v| *v = vec![TMemo::default(); t_limit as usize]);

    /* Stores all sets consecutively in one table */
    /* T_ptr->set_offset and size                 */
    /* are used to retrieve the set               */

    SET_TABLE_SIZE.set(next_power_of_two(NUM_STATES.get()));
    /* set_table = malloc(...) — uninitialized in C; zero-filled here */
    let set_table_size = SET_TABLE_SIZE.get();
    SET_TABLE.with_borrow_mut(|v| *v = vec![0; set_table_size as usize]);
    SET_TABLE_OFFSET.set(0);

    init_trans_array(net);
}

// [spec:foma:def:determinize.trans-sort-cmp-fn]
// [spec:foma:sem:determinize.trans-sort-cmp-fn]
/* C: qsort comparator over const void * — typed references here */
pub(crate) fn trans_sort_cmp(a: &TransList, b: &TransList) -> i32 {
    a.inout - b.inout
}

// [spec:foma:def:determinize.init-trans-array-fn]
// [spec:foma:sem:determinize.init-trans-array-fn]
pub(crate) fn init_trans_array(net: &Fsm) {
    /* arrptr = trans_list_determinize = malloc(net->linecount * ...);
       trans_array_determinize = calloc(net->statecount, ...);
       (trans_list is uninitialized in C; Default-filled here) */
    TRANS_LIST_DETERMINIZE
        .with_borrow_mut(|v| *v = vec![TransList::default(); net.linecount as usize]);
    TRANS_ARRAY_DETERMINIZE
        .with_borrow_mut(|v| *v = vec![TransArray::default(); net.statecount as usize]);

    let fsm = &net.states;

    TRANS_LIST_DETERMINIZE.with_borrow_mut(|tl| {
        TRANS_ARRAY_DETERMINIZE.with_borrow_mut(|ta| {
            let mut laststate: i32 = -1;
            /* arrptr walks the shared entry pool — an index here */
            let mut arrptr: usize = 0;
            /* C: int size */
            let mut size: u32 = 0;

            let mut i = 0usize;
            while fsm[i].state_no != -1 {
                let state = fsm[i].state_no;
                if state != laststate {
                    if laststate != -1 {
                        ta[laststate as usize].size = size;
                    }
                    ta[state as usize].transitions = arrptr;
                    size = 0;
                }
                laststate = state;

                if fsm[i].target == -1 {
                    i += 1;
                    continue;
                }
                let inout = symbol_pair_to_single_symbol(fsm[i].r#in as i32, fsm[i].out as i32);
                if inout == EPSILON_SYMBOL.get() {
                    i += 1;
                    continue;
                }

                tl[arrptr].inout = inout;
                tl[arrptr].target = fsm[i].target;
                arrptr += 1;
                size += 1;
                i += 1;
            }

            if laststate != -1 {
                ta[laststate as usize].size = size;
            }

            for i in 0..net.statecount as usize {
                let arrptr = ta[i].transitions;
                let size = ta[i].size;
                if size > 1 {
                    /* qsort(arrptr, size, sizeof(struct trans_list),
                    trans_sort_cmp) — unstable sort; equal keys keep an
                    unspecified relative order */
                    tl[arrptr..arrptr + size as usize]
                        .sort_unstable_by(|a, b| trans_sort_cmp(a, b).cmp(&0));
                    let mut lastsym = -1;
                    /* Figure out if we're already deterministic */
                    for j in 0..size as usize {
                        if tl[arrptr + j].inout == lastsym {
                            DETERMINISTIC.set(false);
                        }
                        lastsym = tl[arrptr + j].inout;
                    }
                }
            }
        })
    });
}

// [spec:foma:def:determinize.e-closure-fn]
// [spec:foma:sem:determinize.e-closure-fn]
/* C: INLINE static int e_closure(int states) */
pub(crate) fn e_closure(states: i32) -> i32 {
    /* e_closure extends the list of states which are reachable */
    /* and appends these to e_table                             */

    if EPSILON_SYMBOL.get() == -1 {
        return TEMP_MOVE.with_borrow(|tm| set_lookup(tm, states));
    }

    if states == 0 {
        return -1;
    }

    MAINLOOP.set(MAINLOOP.get() - 1);

    let mut set_size = states;

    E_CLOSURE_MEMO.with_borrow_mut(|em| {
        MARKTABLE.with_borrow_mut(|marktable| {
            E_TABLE.with_borrow_mut(|e_table| {
                TEMP_MOVE.with_borrow_mut(|temp_move| {
                    for i in 0..states {
                        /* State number we want to do e-closure on */
                        /* ptr = e_closure_memo + *(temp_move+i) — a pool index */
                        let mut ptr = temp_move[i as usize] as usize;
                        if em[ptr].target.is_none() {
                            continue;
                        }
                        ptr_stack_push(ptr);

                        while ptr_stack_isempty() == 0 {
                            ptr = ptr_stack_pop();
                            /* Don't follow if already seen */
                            if marktable[em[ptr].state as usize] == MAINLOOP.get() {
                                continue;
                            }

                            em[ptr].mark = MAINLOOP.get();
                            marktable[em[ptr].state as usize] = MAINLOOP.get();
                            /* Add to tail of list */
                            if e_table[em[ptr].state as usize] != MAINLOOP.get() {
                                temp_move[set_size as usize] = em[ptr].state;
                                e_table[em[ptr].state as usize] = MAINLOOP.get();
                                set_size += 1;
                            }

                            if em[ptr].target.is_none() {
                                continue;
                            }
                            /* Traverse chain */

                            let mut p: Option<usize> = Some(ptr);
                            while let Some(pi) = p {
                                /* chain nodes always carry a target (head
                                targets were checked above) */
                                let tgt = em[pi].target.unwrap();
                                if em[tgt].mark != MAINLOOP.get() {
                                    /* Push */
                                    em[tgt].mark = MAINLOOP.get();
                                    ptr_stack_push(tgt);
                                }
                                p = em[pi].next;
                            }
                        }
                    }
                })
            })
        })
    });

    MAINLOOP.set(MAINLOOP.get() + 1);
    TEMP_MOVE.with_borrow(|tm| set_lookup(tm, set_size))
}

// [spec:foma:def:determinize.set-lookup-fn]
// [spec:foma:sem:determinize.set-lookup-fn]
/* C: INLINE static int set_lookup (int *lookup_table, int size) */
pub(crate) fn set_lookup(lookup_table: &[i32], size: i32) -> i32 {
    /* Look up a set and its corresponding state number */
    /* if it doesn't exist from before, assign a state number */

    nhash_find_insert(lookup_table, size)
}

// [spec:foma:def:determinize.add-t-ptr-fn]
// [spec:foma:sem:determinize.add-t-ptr-fn]
/* External linkage in C (not static) even though internal to the module */
#[allow(non_snake_case)]
pub fn add_T_ptr(setnum: i32, setsize: i32, theset: u32, fs: i32) {
    if setnum >= T_LIMIT.get() {
        T_LIMIT.set(T_LIMIT.get() * 2);
        let t_limit = T_LIMIT.get();
        T_PTR.with_borrow_mut(|tp| {
            /* realloc leaves the grown region uninitialized in C; only
            .size is cleared below (size == 0 is the "unused" sentinel).
            Default-filled here first. */
            tp.resize(t_limit as usize, TMemo::default());
            for i in setnum..t_limit {
                tp[i as usize].size = 0;
            }
        });
    }

    T_PTR.with_borrow_mut(|tp| {
        tp[setnum as usize].size = setsize as u32;
        tp[setnum as usize].set_offset = theset;
        /* int → unsigned char truncation */
        tp[setnum as usize].finalstart = fs as u8;
    });
    int_stack_push(setnum);
}

// [spec:foma:def:determinize.initial-e-closure-fn]
// [spec:foma:sem:determinize.initial-e-closure-fn]
pub(crate) fn initial_e_closure(net: &Fsm) -> i32 {
    /* finals = calloc(num_states, sizeof(_Bool)); */
    let num_states = NUM_STATES.get();
    FINALS.with_borrow_mut(|v| *v = vec![false; num_states as usize]);

    NUM_START_STATES.set(0);
    let fsm = &net.states;

    /* Create lookups for each state */
    let mut j: i32 = 0;
    FINALS.with_borrow_mut(|finals| {
        E_TABLE.with_borrow_mut(|e_table| {
            TEMP_MOVE.with_borrow_mut(|temp_move| {
                let mut i = 0usize;
                while fsm[i].state_no != -1 {
                    if fsm[i].final_state != 0 {
                        finals[fsm[i].state_no as usize] = true;
                    }
                    /* Add the start states as the initial set */
                    if (OP.get() == SUBSET_TEST_STAR_FREE) || fsm[i].start_state != 0 {
                        if e_table[fsm[i].state_no as usize] != MAINLOOP.get() {
                            NUM_START_STATES.set(NUM_START_STATES.get() + 1);
                            /* numss = (fsm+i)->state_no; — numss is a C _Bool,
                            so the assignment truncates to state_no != 0 */
                            NUMSS.set(fsm[i].state_no != 0);
                            e_table[fsm[i].state_no as usize] = MAINLOOP.get();
                            temp_move[j as usize] = fsm[i].state_no;
                            j += 1;
                        }
                    }
                    i += 1;
                }
            })
        })
    });
    MAINLOOP.set(MAINLOOP.get() + 1);
    /* Memoize e-closure(u) */
    if EPSILON_SYMBOL.get() != -1 {
        memoize_e_closure(fsm);
    }
    e_closure(j)
}

// [spec:foma:def:determinize.memoize-e-closure-fn]
// [spec:foma:sem:determinize.memoize-e-closure-fn]
pub(crate) fn memoize_e_closure(fsm: &[FsmState]) {
    let num_states = NUM_STATES.get();

    /* e_closure_memo = calloc(num_states,sizeof(struct e_closure_memo)); */
    E_CLOSURE_MEMO.with_borrow_mut(|v| *v = vec![EClosureMemo::default(); num_states as usize]);
    /* marktable = calloc(num_states,sizeof(int)); */
    MARKTABLE.with_borrow_mut(|v| *v = vec![0; num_states as usize]);
    /* Table for avoiding redundant epsilon arcs in closure */
    /* redcheck = malloc(num_states*sizeof(int)); — uninitialized; set to -1
    in the init loop below exactly as in C */
    let mut redcheck: Vec<i32> = vec![0; num_states as usize];

    E_CLOSURE_MEMO.with_borrow_mut(|em| {
        for i in 0..num_states as usize {
            em[i].state = i as i32;
            em[i].target = None;
            redcheck[i] = -1;
        }

        let mut laststate: i32 = -1;

        let mut i = 0usize;
        loop {
            let state = fsm[i].state_no;

            if state != laststate {
                if int_stack_isempty() == 0 {
                    DETERMINISTIC.set(false);
                    /* ptr = e_closure_memo+laststate; */
                    let mut ptr = laststate as usize;
                    /* ptr->target = e_closure_memo+int_stack_pop(); — target
                    indices are head-node indices (state numbers) */
                    em[ptr].target = Some(int_stack_pop() as usize);
                    while int_stack_isempty() == 0 {
                        /* ptr->next = malloc(sizeof(struct e_closure_memo));
                        → append a chain node to the pool (its mark is malloc
                        garbage in C and never read on chain nodes; 0 here) */
                        em.push(EClosureMemo {
                            state: laststate,
                            mark: 0,
                            target: Some(int_stack_pop() as usize),
                            next: None,
                        });
                        let ni = em.len() - 1;
                        em[ptr].next = Some(ni);
                        ptr = ni;
                    }
                }
            }
            if state == -1 {
                break;
            }
            if fsm[i].target == -1 {
                i += 1;
                continue;
            }
            /* Check if we have a redundant epsilon arc */
            if fsm[i].r#in as i32 == EPSILON && fsm[i].out as i32 == EPSILON {
                if redcheck[fsm[i].target as usize] != fsm[i].state_no {
                    if fsm[i].target != fsm[i].state_no {
                        int_stack_push(fsm[i].target);
                        redcheck[fsm[i].target as usize] = fsm[i].state_no;
                    }
                }
                laststate = state;
            }
            i += 1;
        }
    });
    /* free(redcheck) — dropped here */
}

// [spec:foma:def:determinize.next-unmarked-fn]
// [spec:foma:sem:determinize.next-unmarked-fn]
pub(crate) fn next_unmarked() -> i32 {
    if int_stack_isempty() != 0 {
        return -1;
    }
    int_stack_pop()

    /* Everything after the return in the C (a sequential T_last_unmarked
    scan terminating on T_limit or a zero-size T_ptr entry) is unreachable
    dead code from an earlier FIFO design — not ported per the sem rule. */
}

// [spec:foma:def:determinize.single-symbol-to-symbol-pair-fn]
// [spec:foma:sem:determinize.single-symbol-to-symbol-pair-fn]
pub(crate) fn single_symbol_to_symbol_pair(symbol: i32, symbol_in: &mut i32, symbol_out: &mut i32) {
    SINGLE_SIGMA_ARRAY.with_borrow(|s| {
        *symbol_in = s[(symbol * 2) as usize];
        *symbol_out = s[(symbol * 2 + 1) as usize];
    });
}

// [spec:foma:def:determinize.symbol-pair-to-single-symbol-fn]
// [spec:foma:sem:determinize.symbol-pair-to-single-symbol-fn]
pub(crate) fn symbol_pair_to_single_symbol(r#in: i32, out: i32) -> i32 {
    DOUBLE_SIGMA_ARRAY.with_borrow(|d| d[(MAXSIGMA.get() * r#in + out) as usize])
}

// [spec:foma:def:determinize.sigma-to-pairs-fn]
// [spec:foma:sem:determinize.sigma-to-pairs-fn]
pub(crate) fn sigma_to_pairs(net: &mut Fsm) {
    let mut next_x: i32 = 0;

    EPSILON_SYMBOL.set(-1);
    MAXSIGMA.set(sigma_max(net.sigma.as_deref()));
    MAXSIGMA.set(MAXSIGMA.get() + 1);
    let maxsigma = MAXSIGMA.get();

    /* single_sigma_array = malloc(2*maxsigma*maxsigma*sizeof(int));
       double_sigma_array = malloc(maxsigma*maxsigma*sizeof(int));
       — malloc'd (uninitialized) in C; zero-filled here (double is
       overwritten with -1 below, single only ever read where written) */
    SINGLE_SIGMA_ARRAY.with_borrow_mut(|v| *v = vec![0; (2 * maxsigma * maxsigma) as usize]);
    DOUBLE_SIGMA_ARRAY.with_borrow_mut(|v| *v = vec![0; (maxsigma * maxsigma) as usize]);

    DOUBLE_SIGMA_ARRAY.with_borrow_mut(|d| {
        for i in 0..maxsigma {
            for j in 0..maxsigma {
                d[(maxsigma * i + j) as usize] = -1;
            }
        }
    });

    /* f(x) -> y,z sigma pair */
    /* f(y,z) -> x simple entry */
    /* if exists f(n) <-> EPSILON, EPSILON, save n */
    /* symbol(x) x>=1 */

    /* Forward mapping: */
    /* *(double_sigma_array+maxsigma*in+out) */

    /* Backmapping: */
    /* *(single_sigma_array+(symbol*2) = in(symbol) */
    /* *(single_sigma_array+(symbol*2+1) = out(symbol) */

    /* Table for checking whether a state is final */

    let mut x: i32 = 0;
    net.arity = 1;
    SINGLE_SIGMA_ARRAY.with_borrow_mut(|s| {
        DOUBLE_SIGMA_ARRAY.with_borrow_mut(|d| {
            let mut i = 0usize;
            while net.states[i].state_no != -1 {
                let y = net.states[i].r#in as i32;
                let z = net.states[i].out as i32;
                if (y == -1) || (z == -1) {
                    i += 1;
                    continue;
                }
                if y != z || y == UNKNOWN || z == UNKNOWN {
                    net.arity = 2;
                }
                if d[(maxsigma * y + z) as usize] == -1 {
                    d[(maxsigma * y + z) as usize] = x;
                    s[next_x as usize] = y;
                    next_x += 1;
                    s[next_x as usize] = z;
                    next_x += 1;
                    if y == EPSILON && z == EPSILON {
                        EPSILON_SYMBOL.set(x);
                    }
                    x += 1;
                }
                i += 1;
            }
        })
    });
    NUM_SYMBOLS.set(x);
}

/* Functions for hashing n integers */
/* with permutations hashing to the same value */
/* necessary for subset construction */

// [spec:foma:def:determinize.nhash-find-insert-fn]
// [spec:foma:sem:determinize.nhash-find-insert-fn]
pub(crate) fn nhash_find_insert(set: &[i32], setsize: i32) -> i32 {
    /* C: unsigned int hashval — hashf's int return converted; values stay
    below nhash_tablesize, so i32 carries them losslessly */
    let mut hashval = hashf(set, setsize);
    let head_size = TABLE.with_borrow(|t| t[hashval as usize].size);
    if head_size == 0 {
        nhash_insert(hashval, set, setsize)
    } else {
        let found_setnum = TABLE.with_borrow(|t| {
            SET_TABLE.with_borrow(|set_table| {
                E_TABLE.with_borrow(|e_table| {
                    let mut tableptr: Option<&NhashList> = Some(&t[hashval as usize]);
                    while let Some(tp) = tableptr {
                        if tp.size as i32 != setsize {
                            tableptr = tp.next.as_deref();
                            continue;
                        }
                        /* Compare the list at hashval position */
                        /* to the current set by looking at etable */
                        /* entries */
                        let mut found = 1;
                        let currlist = tp.set_offset as usize;
                        for j in 0..setsize as usize {
                            if e_table[set_table[currlist + j] as usize] != MAINLOOP.get() - 1 {
                                found = 0;
                                break;
                            }
                        }
                        if OP.get() == SUBSET_TEST_STAR_FREE && found == 1 {
                            for j in 0..setsize as usize {
                                if set[j] != set_table[currlist + j] {
                                    /* Set mark */
                                    STAR_FREE_MARK.set(1);
                                }
                            }
                        }
                        if found == 1 {
                            return Some(tp.setnum);
                        }
                        tableptr = tp.next.as_deref();
                    }
                    None
                })
            })
        });
        if let Some(setnum) = found_setnum {
            return setnum;
        }

        /* Growth check only runs on this collision-miss path — inserting
        into an empty bucket never triggers a rebuild */
        if NHASH_LOAD.get() / NHASH_LOAD_LIMIT > NHASH_TABLESIZE.get() {
            nhash_rebuild_table();
            hashval = hashf(set, setsize);
        }
        nhash_insert(hashval, set, setsize)
    }
}

// [spec:foma:def:determinize.hashf-fn]
// [spec:foma:sem:determinize.hashf-fn]
/* C: INLINE static int hashf(int *set, int setsize) */
pub(crate) fn hashf(set: &[i32], setsize: i32) -> i32 {
    let mut hashval: u32;
    let mut sum: u32 = 0;
    hashval = 6703271;
    for i in 0..setsize {
        /* C: hashval = (unsigned int) (*(set+i) + 1103 * setsize) * hashval;
        — the int addition wraps, then the unsigned multiply wraps */
        hashval =
            (set[i as usize].wrapping_add(1103_i32.wrapping_mul(setsize)) as u32).wrapping_mul(hashval);
        /* C: sum += *(set+i) + i; — int add converted to unsigned */
        sum = sum.wrapping_add(set[i as usize].wrapping_add(i) as u32);
    }
    hashval = hashval.wrapping_add(sum.wrapping_mul(31));
    hashval = hashval % (NHASH_TABLESIZE.get() as u32);
    hashval as i32
}

// [spec:foma:def:determinize.move-set-fn]
// [spec:foma:sem:determinize.move-set-fn]
pub(crate) fn move_set(set: &[i32], setsize: i32) -> u32 {
    /* C compares set_table_offset + setsize >= set_table_size in unsigned
    arithmetic (note >=: growth also triggers on an exact fit) */
    if SET_TABLE_OFFSET.get().wrapping_add(setsize as u32) >= SET_TABLE_SIZE.get() as u32 {
        while SET_TABLE_OFFSET.get().wrapping_add(setsize as u32) >= SET_TABLE_SIZE.get() as u32 {
            SET_TABLE_SIZE.set(SET_TABLE_SIZE.get() * 2);
        }
        /* realloc: the grown region is uninitialized in C; zero-filled here
        (only written offsets are ever read) */
        let set_table_size = SET_TABLE_SIZE.get();
        SET_TABLE.with_borrow_mut(|st| st.resize(set_table_size as usize, 0));
    }
    /* memcpy(set_table+set_table_offset, set, setsize * sizeof(int)); */
    let old_offset = SET_TABLE_OFFSET.get();
    SET_TABLE.with_borrow_mut(|st| {
        let off = old_offset as usize;
        st[off..off + setsize as usize].copy_from_slice(&set[..setsize as usize]);
    });
    SET_TABLE_OFFSET.set(old_offset + setsize as u32);
    old_offset
}

// [spec:foma:def:determinize.nhash-insert-fn]
// [spec:foma:sem:determinize.nhash-insert-fn]
pub(crate) fn nhash_insert(hashval: i32, set: &[i32], setsize: i32) -> i32 {
    let mut fs = 0;

    CURRENT_SETNUM.set(CURRENT_SETNUM.get() + 1);
    let current_setnum = CURRENT_SETNUM.get();

    NHASH_LOAD.set(NHASH_LOAD.get() + 1);
    FINALS.with_borrow(|finals| {
        for i in 0..setsize {
            if finals[set[i as usize] as usize] {
                fs = 1;
            }
        }
    });
    let head_empty = TABLE.with_borrow(|t| t[hashval as usize].size == 0);
    if head_empty {
        let set_offset = move_set(set, setsize);
        TABLE.with_borrow_mut(|t| {
            let tableptr = &mut t[hashval as usize];
            tableptr.set_offset = set_offset;
            tableptr.size = setsize as u32;
            tableptr.setnum = current_setnum;
        });

        add_T_ptr(current_setnum, setsize, set_offset, fs);
        return current_setnum;
    }

    /* tableptr = malloc(...); spliced in as the second chain element.
    (C assigns set_offset = move_set(...) after the splice; move_set only
    touches the set_table statics, so computing it first is unobservable) */
    let set_offset = move_set(set, setsize);
    TABLE.with_borrow_mut(|t| {
        let head = &mut t[hashval as usize];
        let tableptr = Box::new(NhashList {
            setnum: current_setnum,
            size: setsize as u32,
            set_offset,
            next: head.next.take(),
        });
        head.next = Some(tableptr);
    });

    add_T_ptr(current_setnum, setsize, set_offset, fs);
    current_setnum
}

// [spec:foma:def:determinize.nhash-rebuild-table-fn]
// [spec:foma:sem:determinize.nhash-rebuild-table-fn]
pub(crate) fn nhash_rebuild_table() {
    let oldtable = TABLE.with_borrow_mut(std::mem::take);
    let oldsize = NHASH_TABLESIZE.get();

    NHASH_LOAD.set(0);
    /* C: for (i=0; primes[i] < nhash_tablesize; i++) {} — lands exactly on
    the current prime, then takes the following entry. If already at the
    last prime, primes[i+1] reads past the array in C — panics here
    (practically unreachable). */
    let mut i = 0usize;
    while PRIMES[i] < NHASH_TABLESIZE.get() as u32 {
        i += 1;
    }
    NHASH_TABLESIZE.set(PRIMES[i + 1] as i32);

    /* table = calloc(nhash_tablesize,sizeof(struct nhash_list)); */
    let nhash_tablesize = NHASH_TABLESIZE.get();
    TABLE.with_borrow_mut(|t| *t = vec![NhashList::default(); nhash_tablesize as usize]);
    for i in 0..oldsize as usize {
        if oldtable[i].size == 0 {
            continue;
        }
        let mut tableptr: Option<&NhashList> = Some(&oldtable[i]);
        while let Some(tp) = tableptr {
            /* rehash */
            let hashval =
                SET_TABLE.with_borrow(|st| hashf(&st[tp.set_offset as usize..], tp.size as i32));
            TABLE.with_borrow_mut(|t| {
                let ntableptr = &mut t[hashval as usize];
                if ntableptr.size == 0 {
                    /* quirk kept: nhash_load only counts occupied buckets
                    here, understating the load factor for later checks */
                    NHASH_LOAD.set(NHASH_LOAD.get() + 1);
                    ntableptr.size = tp.size;
                    ntableptr.set_offset = tp.set_offset;
                    ntableptr.setnum = tp.setnum;
                    ntableptr.next = None;
                } else {
                    let newptr = Box::new(NhashList {
                        setnum: tp.setnum,
                        size: tp.size,
                        set_offset: tp.set_offset,
                        next: ntableptr.next.take(),
                    });
                    ntableptr.next = Some(newptr);
                }
            });
            tableptr = tp.next.as_deref();
        }
    }
    nhash_free(oldtable, oldsize);
}

// [spec:foma:def:determinize.nhash-init-fn]
// [spec:foma:sem:determinize.nhash-init-fn]
pub(crate) fn nhash_init(initial_size: i32) {
    /* C: for (i=0; primes[i] < initial_size; i++) {} — unsigned comparison;
    minimum table size is primes[0] == 61 */
    let mut i = 0usize;
    while PRIMES[i] < initial_size as u32 {
        i += 1;
    }
    NHASH_LOAD.set(0);
    NHASH_TABLESIZE.set(PRIMES[i] as i32);
    /* table = calloc(nhash_tablesize , sizeof(struct nhash_list)); — zeroed
    so size == 0 marks an empty bucket */
    let nhash_tablesize = NHASH_TABLESIZE.get();
    TABLE.with_borrow_mut(|t| *t = vec![NhashList::default(); nhash_tablesize as usize]);
    CURRENT_SETNUM.set(-1);
}

// [spec:foma:def:determinize.e-closure-free-fn]
// [spec:foma:sem:determinize.e-closure-free-fn]
pub(crate) fn e_closure_free() {
    /* free(marktable); */
    MARKTABLE.with_borrow_mut(|v| *v = Vec::new());
    /* C walks each head node's ->next chain freeing the malloc'd chain
    nodes, then frees the head array; heads and chain nodes share the
    E_CLOSURE_MEMO pool here, so clearing it frees everything */
    E_CLOSURE_MEMO.with_borrow_mut(|v| *v = Vec::new());
}

// [spec:foma:def:determinize.nhash-free-fn]
// [spec:foma:sem:determinize.nhash-free-fn]
pub(crate) fn nhash_free(mut nptr: Vec<NhashList>, size: i32) {
    /* for each bucket, free every chained node reachable from ->next (the
    heads are elements of the array itself); iterative take()s mirror the
    node-by-node frees and avoid recursive Box drops on long chains */
    for i in 0..size as usize {
        let mut nptr2 = nptr[i].next.take();
        while let Some(mut node) = nptr2 {
            let nnext = node.next.take();
            drop(node); /* free(nptr2) */
            nptr2 = nnext;
        }
    }
    /* free(nptr) — the bucket array is dropped on return */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::{apply_clear, apply_down, apply_init};
    use crate::dynarray::{
        fsm_construct_add_arc, fsm_construct_done, fsm_construct_init, fsm_construct_set_final,
        fsm_construct_set_initial,
    };
    use crate::types::{Fsm, NO, UNK};

    fn accepts(net: &Fsm, word: &str) -> Option<String> {
        let mut h = apply_init(net);
        let r = apply_down(&mut h, Some(word));
        apply_clear(h);
        r
    }

    fn lines(net: &Fsm) -> Vec<(i32, i16, i16, i32, i8, i8)> {
        net.states
            .iter()
            .map(|s| (s.state_no, s.r#in, s.out, s.target, s.final_state, s.start_state))
            .collect()
    }

    /* NFA over {a}: 0 start, 0-a->0, 0-a->1, 1-a->2 (final). L = a^n, n >= 2. */
    fn build_a_ge2() -> Box<Fsm> {
        let mut hc = fsm_construct_init("d");
        fsm_construct_set_initial(&mut hc, 0);
        fsm_construct_add_arc(&mut hc, 0, 0, "a", "a");
        fsm_construct_add_arc(&mut hc, 0, 1, "a", "a");
        fsm_construct_add_arc(&mut hc, 1, 2, "a", "a");
        fsm_construct_set_final(&mut hc, 2);
        fsm_construct_done(hc)
    }

    // Full subset construction over the whole engine: fsm_determinize drives
    // fsm_subset, init, init_trans_array (+ trans_sort_cmp on the state with two
    // a-arcs), initial_e_closure, e_closure (no-epsilon branch), set_lookup and
    // the do-loop's next_unmarked agenda pops.
    // [spec:foma:sem:determinize.fsm-determinize-fn/test]
    // [spec:foma:sem:fomalib.fsm-determinize-fn/test]
    // [spec:foma:sem:determinize.fsm-subset-fn/test]
    // [spec:foma:sem:determinize.init-fn/test]
    // [spec:foma:sem:determinize.init-trans-array-fn/test]
    // [spec:foma:sem:determinize.initial-e-closure-fn/test]
    // [spec:foma:sem:determinize.e-closure-fn/test]
    // [spec:foma:sem:determinize.set-lookup-fn/test]
    #[test]
    fn determinize_subset_construction_shape() {
        let net = build_a_ge2();
        assert_ne!(net.is_deterministic, YES, "input NFA is nondeterministic");
        let d = fsm_determinize(net);
        /* subsets {0}, {0,1}, {0,1,2 final}: 3 states, 3 arcs, 1 final */
        assert_eq!(d.statecount, 3);
        assert_eq!(d.arccount, 3);
        assert_eq!(d.finalcount, 1);
        assert_eq!(d.is_deterministic, YES);
        assert_eq!(d.is_epsilon_free, YES);
        /* start state renumbered to 0, densely numbered result */
        assert_eq!(
            d.states.iter().filter(|s| s.state_no != -1 && s.start_state != 0).count(),
            1
        );
        assert_eq!(accepts(&d, ""), None);
        assert_eq!(accepts(&d, "a"), None);
        assert_eq!(accepts(&d, "aa"), Some("aa".to_string()));
        assert_eq!(accepts(&d, "aaaa"), Some("aaaa".to_string()));
    }

    // fsm_epsilon_remove drives fsm_subset with epsilon memoization: memoize is
    // exercised on the input's eps arc and the closure DFS runs the epsilon
    // branch of e_closure; e_closure_free tears the memo down in wrapup.
    // [spec:foma:sem:determinize.fsm-epsilon-remove-fn/test]
    // [spec:foma:sem:fomalib.fsm-epsilon-remove-fn/test]
    // [spec:foma:sem:determinize.e-closure-fn/test]
    #[test]
    fn epsilon_remove_eliminates_epsilon_arcs() {
        /* eps-NFA: 0 start -eps-> 1, 1 -a-> 1 (loop), 1 final. L = a*. */
        let mut hc = fsm_construct_init("e");
        fsm_construct_set_initial(&mut hc, 0);
        fsm_construct_add_arc(&mut hc, 0, 1, "@_EPSILON_SYMBOL_@", "@_EPSILON_SYMBOL_@");
        fsm_construct_add_arc(&mut hc, 1, 1, "a", "a");
        fsm_construct_set_final(&mut hc, 1);
        let net = fsm_construct_done(hc);
        assert_eq!(net.is_epsilon_free, NO);
        let er = fsm_epsilon_remove(net);
        assert_eq!(er.is_epsilon_free, YES);
        /* no (EPSILON:EPSILON) arc survives */
        assert!(!er.states.iter().any(|s| s.r#in == 0 && s.out == 0 && s.target != -1));
        /* state 0's epsilon closure reaches final state 1 -> language a* kept */
        assert_eq!(accepts(&er, ""), Some("".to_string()));
        assert_eq!(accepts(&er, "a"), Some("a".to_string()));
        assert_eq!(accepts(&er, "aaa"), Some("aaa".to_string()));
    }

    // fsm_subset's second fast path: EPSILON_REMOVE on a net with no epsilon arc
    // sets is_epsilon_free and returns the net unmodified (NOT determinized).
    // [spec:foma:sem:determinize.fsm-subset-fn/test]
    #[test]
    fn epsilon_remove_no_epsilon_fast_path() {
        /* nondeterministic (skips the top-level det early return) but eps-free */
        let mut hc = fsm_construct_init("f");
        fsm_construct_set_initial(&mut hc, 0);
        fsm_construct_add_arc(&mut hc, 0, 1, "a", "a");
        fsm_construct_add_arc(&mut hc, 0, 2, "a", "a"); /* two a-arcs -> nondet */
        fsm_construct_set_final(&mut hc, 1);
        fsm_construct_set_final(&mut hc, 2);
        let net = fsm_construct_done(hc);
        assert_ne!(net.is_deterministic, YES);
        let sc = net.statecount;
        let before = lines(&net);
        let er = fsm_epsilon_remove(net);
        assert_eq!(er.is_epsilon_free, YES);
        assert_eq!(er.statecount, sc);
        assert_ne!(er.is_deterministic, YES, "not determinized on the eps-free path");
        assert_eq!(lines(&er), before, "line table returned untouched");
    }

    // Internal already-deterministic shortcut: forced past the top-level
    // is_deterministic==YES early return, a structurally-deterministic single-
    // start-at-0 net takes the shortcut, which sets det/eps flags but does NOT
    // rebuild (is_pruned/is_minimized preserved, line table intact).
    // [spec:foma:sem:determinize.fsm-subset-fn/test]
    #[test]
    fn already_deterministic_shortcut_preserves_flags() {
        let mut hc = fsm_construct_init("A");
        fsm_construct_set_initial(&mut hc, 0);
        fsm_construct_set_final(&mut hc, 0);
        fsm_construct_add_arc(&mut hc, 0, 1, "a", "a");
        fsm_construct_set_final(&mut hc, 1);
        fsm_construct_add_arc(&mut hc, 1, 1, "a", "a");
        let mut net = fsm_construct_done(hc);
        net.is_deterministic = UNK; /* skip the top-level early return */
        net.is_pruned = YES;
        net.is_minimized = YES;
        let before = lines(&net);
        let d = fsm_determinize(net);
        assert_eq!(d.is_deterministic, YES);
        assert_eq!(d.is_epsilon_free, YES);
        assert_eq!(d.is_pruned, YES, "shortcut does not touch is_pruned");
        assert_eq!(d.is_minimized, YES, "shortcut does not touch is_minimized");
        assert_eq!(lines(&d), before, "line table not rebuilt");
    }

    // numss _Bool truncation quirk: a deterministic net whose single start state
    // is NOT state 0 has numss = (state_no != 0) = true, so the shortcut is
    // skipped and the full subset construction runs (rebuild via fsm_state_close
    // clears is_pruned/is_minimized and renumbers the start state to 0).
    // [spec:foma:sem:determinize.fsm-subset-fn/test]
    // [spec:foma:sem:determinize.initial-e-closure-fn/test]
    #[test]
    fn numss_bool_truncation_forces_full_path() {
        let mut hc = fsm_construct_init("B");
        fsm_construct_set_initial(&mut hc, 1);
        fsm_construct_add_arc(&mut hc, 1, 0, "a", "a");
        fsm_construct_set_final(&mut hc, 0);
        let mut net = fsm_construct_done(hc);
        net.is_deterministic = UNK;
        net.is_pruned = YES;
        net.is_minimized = YES;
        let d = fsm_determinize(net);
        /* full path taken -> the builder close resets these to UNK */
        assert_eq!(d.is_pruned, UNK);
        assert_eq!(d.is_minimized, UNK);
        let start_states: Vec<i32> = d
            .states
            .iter()
            .filter(|s| s.state_no != -1 && s.start_state != 0)
            .map(|s| s.state_no)
            .collect();
        assert_eq!(start_states, vec![0], "start state renumbered to 0");
        assert_eq!(accepts(&d, "a"), Some("a".to_string()));
        assert_eq!(accepts(&d, ""), None);
    }

    // hashf: the fixed 6703271 seed (observable on the empty set) and the
    // permutation invariance the subset hashing relies on.
    // [spec:foma:sem:determinize.hashf-fn/test]
    #[test]
    fn hashf_seed_and_permutation() {
        NHASH_TABLESIZE.set(61);
        assert_eq!(hashf(&[], 0), (6703271u32 % 61) as i32);
        /* large prime table: modulo does not mask the permutation equality */
        NHASH_TABLESIZE.set(2147483647);
        let base = hashf(&[7, 3, 19, 2], 4);
        assert_eq!(base, hashf(&[2, 19, 3, 7], 4));
        assert_eq!(base, hashf(&[19, 2, 7, 3], 4));
    }

    // nhash_init picks the smallest prime >= initial_size off the ladder
    // (minimum 61) and resets load / current_setnum.
    // [spec:foma:sem:determinize.nhash-init-fn/test]
    #[test]
    fn nhash_init_prime_ladder() {
        nhash_init(6);
        assert_eq!(NHASH_TABLESIZE.get(), 61);
        assert_eq!(CURRENT_SETNUM.get(), -1);
        assert_eq!(NHASH_LOAD.get(), 0);
        nhash_init(61);
        assert_eq!(NHASH_TABLESIZE.get(), 61);
        nhash_init(62);
        assert_eq!(NHASH_TABLESIZE.get(), 127);
        nhash_init(0);
        assert_eq!(NHASH_TABLESIZE.get(), 61);
        nhash_init(2000);
        assert_eq!(NHASH_TABLESIZE.get(), 2039);
    }

    // nhash_rebuild_table advances to the next prime and rehashes (empty here).
    // [spec:foma:sem:determinize.nhash-rebuild-table-fn/test]
    #[test]
    fn nhash_rebuild_advances_prime() {
        nhash_init(6); /* 61, empty */
        nhash_rebuild_table();
        assert_eq!(NHASH_TABLESIZE.get(), 127);
        assert_eq!(NHASH_LOAD.get(), 0);
    }

    // Round-trip through the subset canonicaliser: set_lookup -> nhash_find_insert
    // -> nhash_insert -> move_set + add_T_ptr. A first set is numbered 0 and its
    // members copied into set_table; a permutation of it canonicalises back to 0
    // (order-insensitive membership test via e_table); a distinct set gets 1.
    // add_T_ptr pushed both onto the agenda (next_unmarked pops LIFO).
    // [spec:foma:sem:determinize.set-lookup-fn/test]
    // [spec:foma:sem:determinize.nhash-find-insert-fn/test]
    // [spec:foma:sem:determinize.nhash-insert-fn/test]
    // [spec:foma:sem:determinize.move-set-fn/test]
    // [spec:foma:sem:determinize.add-t-ptr-fn/test]
    #[test]
    fn nhash_insert_find_roundtrip() {
        let n = 5usize;
        FINALS.with_borrow_mut(|v| *v = vec![false; n]);
        E_TABLE.with_borrow_mut(|v| *v = vec![0; n]);
        MAINLOOP.set(1);
        SET_TABLE_SIZE.set(64);
        SET_TABLE.with_borrow_mut(|v| *v = vec![0; 64]);
        SET_TABLE_OFFSET.set(0);
        T_LIMIT.set(8);
        T_PTR.with_borrow_mut(|v| *v = vec![TMemo::default(); 8]);
        OP.set(SUBSET_DETERMINIZE);
        crate::int_stack::int_stack_clear();
        nhash_init(6);

        /* first insert of {2,0,1} -> subset 0, members copied to set_table */
        assert_eq!(set_lookup(&[2, 0, 1], 3), 0);
        assert_eq!(SET_TABLE_OFFSET.get(), 3);
        SET_TABLE.with_borrow(|st| assert_eq!(&st[0..3], &[2, 0, 1]));
        assert_eq!(T_PTR.with_borrow(|t| (t[0].size, t[0].set_offset)), (3, 0));

        /* find a permutation: mark members e_table == mainloop-1, bump mainloop */
        E_TABLE.with_borrow_mut(|e| {
            e[0] = 1;
            e[1] = 1;
            e[2] = 1;
        });
        MAINLOOP.set(2);
        assert_eq!(set_lookup(&[0, 1, 2], 3), 0, "permutation canonicalises to 0");

        /* a distinct set gets the next number */
        assert_eq!(set_lookup(&[3, 4], 2), 1);
        assert_eq!(SET_TABLE_OFFSET.get(), 5);

        /* both subsets were pushed on the agenda by add_T_ptr (LIFO) */
        assert_eq!(next_unmarked(), 1);
        assert_eq!(next_unmarked(), 0);
        assert_eq!(next_unmarked(), -1);
    }

    // next_unmarked pops the agenda LIFO, -1 when empty.
    // [spec:foma:sem:determinize.next-unmarked-fn/test]
    #[test]
    fn next_unmarked_pops_lifo() {
        crate::int_stack::int_stack_clear();
        crate::int_stack::int_stack_push(3);
        crate::int_stack::int_stack_push(7);
        assert_eq!(next_unmarked(), 7);
        assert_eq!(next_unmarked(), 3);
        assert_eq!(next_unmarked(), -1);
    }

    // sigma_to_pairs builds the (in,out)<->composite bijection, flags a
    // transducer (arity 2), records epsilon_symbol for the (0,0) pair, and the
    // two mapping functions invert each other for every registered pair.
    // [spec:foma:sem:determinize.sigma-to-pairs-fn/test]
    // [spec:foma:sem:determinize.symbol-pair-to-single-symbol-fn/test]
    // [spec:foma:sem:determinize.single-symbol-to-symbol-pair-fn/test]
    #[test]
    fn sigma_to_pairs_and_symbol_mappings() {
        let mut hc = fsm_construct_init("s");
        fsm_construct_set_initial(&mut hc, 0);
        fsm_construct_add_arc(&mut hc, 0, 1, "a", "b"); /* a:b -> arity 2 */
        fsm_construct_add_arc(&mut hc, 1, 2, "a", "a"); /* a:a */
        fsm_construct_add_arc(&mut hc, 0, 2, "@_EPSILON_SYMBOL_@", "@_EPSILON_SYMBOL_@");
        fsm_construct_set_final(&mut hc, 2);
        let mut net = fsm_construct_done(hc);
        sigma_to_pairs(&mut net);
        assert_eq!(net.arity, 2);
        assert_ne!(EPSILON_SYMBOL.get(), -1);
        assert_eq!(EPSILON_SYMBOL.get(), symbol_pair_to_single_symbol(EPSILON, EPSILON));
        for st in net.states.iter() {
            let (i, o) = (st.r#in as i32, st.out as i32);
            if i < 0 || o < 0 {
                continue;
            }
            let c = symbol_pair_to_single_symbol(i, o);
            assert!(c >= 0 && c < NUM_SYMBOLS.get());
            let (mut si, mut so) = (0, 0);
            single_symbol_to_symbol_pair(c, &mut si, &mut so);
            assert_eq!((si, so), (i, o), "back-map inverts forward-map");
        }
    }

    // memoize_e_closure builds the per-state epsilon adjacency graph, skipping
    // self-loops and duplicate (source,target) pairs; fanout is a head ->target
    // plus a ->next chain (LIFO of int_stack pops).
    // [spec:foma:sem:determinize.memoize-e-closure-fn/test]
    #[test]
    fn memoize_e_closure_builds_epsilon_graph() {
        crate::int_stack::int_stack_clear();
        NUM_STATES.set(3);
        let e = EPSILON as i16;
        let fsm = vec![
            FsmState { state_no: 0, r#in: e, out: e, target: 1, final_state: 0, start_state: 1 },
            FsmState { state_no: 0, r#in: e, out: e, target: 2, final_state: 0, start_state: 1 },
            FsmState { state_no: 0, r#in: e, out: e, target: 1, final_state: 0, start_state: 1 }, /* dup */
            FsmState { state_no: 0, r#in: e, out: e, target: 0, final_state: 0, start_state: 1 }, /* self */
            FsmState { state_no: 1, r#in: -1, out: -1, target: -1, final_state: 0, start_state: 0 },
            FsmState { state_no: 2, r#in: -1, out: -1, target: -1, final_state: 1, start_state: 0 },
            FsmState { state_no: -1, r#in: -1, out: -1, target: -1, final_state: -1, start_state: -1 },
        ];
        memoize_e_closure(&fsm);
        E_CLOSURE_MEMO.with_borrow(|em| {
            /* head 0 -> successors {2,1} (LIFO), heads 1,2 have none */
            assert_eq!(em[0].state, 0);
            assert_eq!(em[0].target, Some(2));
            let chain = em[0].next.expect("fanout chain node");
            assert_eq!(em[chain].target, Some(1));
            assert_eq!(em[chain].next, None);
            assert_eq!(em[1].target, None);
            assert_eq!(em[2].target, None);
        });
    }

    // e_closure_free drops marktable and the memo pool.
    // [spec:foma:sem:determinize.e-closure-free-fn/test]
    #[test]
    fn e_closure_free_clears_memo() {
        MARKTABLE.with_borrow_mut(|v| *v = vec![1, 2, 3]);
        E_CLOSURE_MEMO.with_borrow_mut(|v| *v = vec![EClosureMemo::default(); 4]);
        e_closure_free();
        assert!(MARKTABLE.with_borrow(|v| v.is_empty()));
        assert!(E_CLOSURE_MEMO.with_borrow(|v| v.is_empty()));
    }

    // nhash_free walks each bucket's ->next chain without panicking.
    // [spec:foma:sem:determinize.nhash-free-fn/test]
    #[test]
    fn nhash_free_walks_chains() {
        let mut table = vec![NhashList::default(); 2];
        table[0].size = 1;
        table[0].next = Some(Box::new(NhashList {
            setnum: 1,
            size: 1,
            set_offset: 0,
            next: Some(Box::new(NhashList::default())),
        }));
        nhash_free(table, 2);
    }

    // trans_sort_cmp: ascending by composite symbol (a->inout - b->inout).
    // [spec:foma:sem:determinize.trans-sort-cmp-fn/test]
    #[test]
    fn trans_sort_cmp_orders_by_inout() {
        let a = TransList { inout: 5, target: 0 };
        let b = TransList { inout: 2, target: 9 };
        assert_eq!(trans_sort_cmp(&a, &b), 3);
        assert_eq!(trans_sort_cmp(&b, &a), -3);
        assert_eq!(trans_sort_cmp(&a, &a), 0);
    }

    // The extern add_fsm_arc re-export writes one flat line and returns offset+1.
    // [spec:foma:sem:determinize.add-fsm-arc-fn/test]
    #[test]
    fn add_fsm_arc_reexport_writes_line() {
        let mut fsm = vec![
            FsmState { state_no: 0, r#in: 0, out: 0, target: 0, final_state: 0, start_state: 0 };
            2
        ];
        let r = add_fsm_arc(&mut fsm, 0, 5, 1, 2, 3, 1, 1);
        assert_eq!(r, 1);
        assert_eq!(
            (fsm[0].state_no, fsm[0].r#in, fsm[0].out, fsm[0].target, fsm[0].final_state, fsm[0].start_state),
            (5, 1, 2, 3, 1, 1)
        );
    }
}

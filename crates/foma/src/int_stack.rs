//! foma/int_stack.c — literal Wave-2 (bug-for-bug) port per
//! docs/port/rust-conventions.md. Sem rules: docs/spec/port/foma/int_stack.md
//! (per-file ids) and the fomalibconf.h prototype ids.
//!
//! `MAX_STACK` / `MAX_PTR_STACK` (2097152) live in crate::types.

use std::cell::{Cell, RefCell};

use crate::types::{MAX_PTR_STACK, MAX_STACK};

thread_local! {
    // C: static int a[MAX_STACK]; static int top = -1;
    static A: RefCell<Vec<i32>> = RefCell::new(vec![0; MAX_STACK]);
    static TOP: Cell<i32> = const { Cell::new(-1) };

    // C: static void *ptr_stack[MAX_PTR_STACK]; static int ptr_stack_top = -1;
    // DEVIATION from C (opaque pointer stack: the C stores raw void*, but
    // every foma call site — structures.c, determinize.c, spelling.c —
    // pushes interior pointers into arrays that the port represents as
    // indices, so the stack holds usize index/handle tokens; callers
    // push/pop indices into their own tables and may do +1 arithmetic on
    // them exactly as the C does on the pointers)
    static PTR_STACK: RefCell<Vec<usize>> = RefCell::new(vec![0; MAX_PTR_STACK]);
    static PTR_STACK_TOP: Cell<i32> = const { Cell::new(-1) };
}

// [spec:foma:def:int-stack.ptr-stack-isempty-fn]
// [spec:foma:sem:int-stack.ptr-stack-isempty-fn]
// [spec:foma:def:fomalibconf.ptr-stack-isempty-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-isempty-fn]
pub fn ptr_stack_isempty() -> i32 {
    (PTR_STACK_TOP.with(|t| t.get()) == -1) as i32
}

// [spec:foma:def:int-stack.ptr-stack-clear-fn]
// [spec:foma:sem:int-stack.ptr-stack-clear-fn]
// [spec:foma:def:fomalibconf.ptr-stack-clear-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-clear-fn]
pub fn ptr_stack_clear() {
    PTR_STACK_TOP.with(|t| t.set(-1));
}

// [spec:foma:def:int-stack.ptr-stack-pop-fn]
// [spec:foma:sem:int-stack.ptr-stack-pop-fn]
// [spec:foma:def:fomalibconf.ptr-stack-pop-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-pop-fn]
pub fn ptr_stack_pop() -> usize {
    // C: return ptr_stack[ptr_stack_top--]; — no underflow check; popping an
    // empty stack reads ptr_stack[-1] (UB) and leaves ptr_stack_top at -2.
    // DEVIATION from C (OOB read on empty pop is UB in C; here the index
    // panics instead — callers must guard with ptr_stack_isempty, as in C)
    PTR_STACK_TOP.with(|t| {
        let top = t.get();
        let v = PTR_STACK.with(|s| s.borrow()[top as usize]);
        t.set(top - 1);
        v
    })
}

// [spec:foma:def:int-stack.ptr-stack-isfull-fn]
// [spec:foma:sem:int-stack.ptr-stack-isfull-fn]
// [spec:foma:def:fomalibconf.ptr-stack-isfull-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-isfull-fn]
pub fn ptr_stack_isfull() -> i32 {
    (PTR_STACK_TOP.with(|t| t.get()) == (MAX_PTR_STACK as i32 - 1)) as i32
}

// [spec:foma:def:int-stack.ptr-stack-push-fn]
// [spec:foma:sem:int-stack.ptr-stack-push-fn]
// [spec:foma:def:fomalibconf.ptr-stack-push-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-push-fn]
pub fn ptr_stack_push(ptr: usize) {
    if ptr_stack_isfull() != 0 {
        eprint!("Pointer stack full!\n");
        std::process::exit(1);
    }
    PTR_STACK_TOP.with(|t| {
        let top = t.get() + 1;
        t.set(top);
        PTR_STACK.with(|s| s.borrow_mut()[top as usize] = ptr);
    });
}

// [spec:foma:def:int-stack.int-stack-isempty-fn]
// [spec:foma:sem:int-stack.int-stack-isempty-fn]
// [spec:foma:def:fomalibconf.int-stack-isempty-fn]
// [spec:foma:sem:fomalibconf.int-stack-isempty-fn]
pub fn int_stack_isempty() -> i32 {
    (TOP.with(|t| t.get()) == -1) as i32
}

// [spec:foma:def:int-stack.int-stack-clear-fn]
// [spec:foma:sem:int-stack.int-stack-clear-fn]
// [spec:foma:def:fomalibconf.int-stack-clear-fn]
// [spec:foma:sem:fomalibconf.int-stack-clear-fn]
pub fn int_stack_clear() {
    TOP.with(|t| t.set(-1));
}

// [spec:foma:def:int-stack.int-stack-find-fn]
// [spec:foma:sem:int-stack.int-stack-find-fn]
// [spec:foma:def:fomalibconf.int-stack-find-fn]
// [spec:foma:sem:fomalibconf.int-stack-find-fn]
pub fn int_stack_find(entry: i32) -> i32 {
    if int_stack_isempty() != 0 {
        return 0;
    }
    let top = TOP.with(|t| t.get());
    A.with(|a| {
        let a = a.borrow();
        // C: for(i = 0; i <= top ; i++)
        let mut i: i32 = 0;
        while i <= top {
            if entry == a[i as usize] {
                return 1;
            }
            i += 1;
        }
        0
    })
}

// [spec:foma:def:int-stack.int-stack-size-fn]
// [spec:foma:sem:int-stack.int-stack-size-fn]
// [spec:foma:def:fomalibconf.int-stack-size-fn]
// [spec:foma:sem:fomalibconf.int-stack-size-fn]
pub fn int_stack_size() -> i32 {
    TOP.with(|t| t.get()) + 1
}

// [spec:foma:def:int-stack.int-stack-push-fn]
// [spec:foma:sem:int-stack.int-stack-push-fn]
// [spec:foma:def:fomalibconf.int-stack-push-fn]
// [spec:foma:sem:fomalibconf.int-stack-push-fn]
pub fn int_stack_push(c: i32) {
    if int_stack_isfull() != 0 {
        eprint!("Stack full!\n");
        std::process::exit(1);
    }
    TOP.with(|t| {
        let top = t.get() + 1;
        t.set(top);
        A.with(|a| a.borrow_mut()[top as usize] = c);
    });
}

// [spec:foma:def:int-stack.int-stack-pop-fn]
// [spec:foma:sem:int-stack.int-stack-pop-fn]
// [spec:foma:def:fomalibconf.int-stack-pop-fn]
// [spec:foma:sem:fomalibconf.int-stack-pop-fn]
pub fn int_stack_pop() -> i32 {
    // C: return a[top--]; — no underflow check; popping an empty stack reads
    // a[-1] (UB) and leaves top at -2.
    // DEVIATION from C (OOB read on empty pop is UB in C; here the index
    // panics instead — callers must guard with int_stack_isempty, as in C)
    TOP.with(|t| {
        let top = t.get();
        let v = A.with(|a| a.borrow()[top as usize]);
        t.set(top - 1);
        v
    })
}

// [spec:foma:def:int-stack.int-stack-isfull-fn]
// [spec:foma:sem:int-stack.int-stack-isfull-fn]
// [spec:foma:def:fomalibconf.int-stack-isfull-fn]
// [spec:foma:sem:fomalibconf.int-stack-isfull-fn]
pub fn int_stack_isfull() -> i32 {
    (TOP.with(|t| t.get()) == (MAX_STACK as i32 - 1)) as i32
}

// NOTE: fomalibconf.h also declares `int int_stack_status();` (rule id
// fomalibconf.int-stack-status-fn) but no definition exists anywhere in the
// C sources — it is a dead prototype and is deliberately NOT ported (no
// implementation is invented for it).

/* Dead prototype: declared in fomalibconf.h but never defined in any C
source. Calling it in C is a link error. DEVIATION from C (panics to
preserve the never-callable contract). */

// [spec:foma:def:fomalibconf.int-stack-status-fn]
// [spec:foma:sem:fomalibconf.int-stack-status-fn]
pub fn int_stack_status() -> i32 {
    panic!("int_stack_status: dead prototype in C foma (declared, never defined; link error)");
}

#[cfg(test)]
mod tests {
    use super::*;

    // [spec:foma:sem:int-stack.int-stack-isempty-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-isempty-fn/test]
    // [spec:foma:sem:int-stack.int-stack-size-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-size-fn/test]
    // [spec:foma:sem:int-stack.int-stack-push-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-push-fn/test]
    // [spec:foma:sem:int-stack.int-stack-pop-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-pop-fn/test]
    // [spec:foma:sem:int-stack.int-stack-clear-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-clear-fn/test]
    #[test]
    fn int_stack_push_pop_lifo_size_empty_clear() {
        // Fresh (thread-local) stack starts empty: top == -1.
        assert_eq!(int_stack_isempty(), 1);
        assert_eq!(int_stack_size(), 0);
        int_stack_push(10);
        int_stack_push(20);
        int_stack_push(30);
        assert_eq!(int_stack_isempty(), 0);
        assert_eq!(int_stack_size(), 3); // top + 1
        // LIFO pop with post-decrement of top.
        assert_eq!(int_stack_pop(), 30);
        assert_eq!(int_stack_pop(), 20);
        assert_eq!(int_stack_size(), 1);
        // clear resets top to -1 without touching storage.
        int_stack_clear();
        assert_eq!(int_stack_isempty(), 1);
        assert_eq!(int_stack_size(), 0);
    }

    // [spec:foma:sem:int-stack.int-stack-find-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-find-fn/test]
    #[test]
    fn int_stack_find_scans_bottom_through_top_inclusive() {
        // Empty stack short-circuits (via int_stack_isempty) to 0.
        assert_eq!(int_stack_find(5), 0);
        int_stack_push(5); // bottom (index 0)
        int_stack_push(7);
        int_stack_push(9); // top
        assert_eq!(int_stack_find(5), 1); // bottom found
        assert_eq!(int_stack_find(9), 1); // top found
        assert_eq!(int_stack_find(8), 0); // absent
    }

    // [spec:foma:sem:int-stack.int-stack-isfull-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-isfull-fn/test]
    #[test]
    fn int_stack_isfull_threshold_at_max_stack_minus_one() {
        // Non-exit paths only: drive `top` directly to the isfull boundary
        // (pushing MAX_STACK items would hit the exit(1) path).
        assert_eq!(int_stack_isfull(), 0); // empty
        TOP.with(|t| t.set(MAX_STACK as i32 - 2));
        assert_eq!(int_stack_isfull(), 0);
        TOP.with(|t| t.set(MAX_STACK as i32 - 1)); // == 2097151
        assert_eq!(int_stack_isfull(), 1);
        int_stack_clear();
    }

    // [spec:foma:sem:int-stack.int-stack-pop-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-pop-fn/test]
    #[test]
    #[should_panic]
    fn int_stack_pop_empty_panics_deviation() {
        // C reads a[-1] (OOB, UB); the port panics on the OOB index instead.
        int_stack_pop();
    }

    // [spec:foma:sem:int-stack.ptr-stack-isempty-fn/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-isempty-fn/test]
    // [spec:foma:sem:int-stack.ptr-stack-push-fn/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-push-fn/test]
    // [spec:foma:sem:int-stack.ptr-stack-pop-fn/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-pop-fn/test]
    // [spec:foma:sem:int-stack.ptr-stack-clear-fn/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-clear-fn/test]
    #[test]
    fn ptr_stack_push_pop_isempty_clear() {
        assert_eq!(ptr_stack_isempty(), 1);
        ptr_stack_push(42);
        ptr_stack_push(7);
        assert_eq!(ptr_stack_isempty(), 0);
        assert_eq!(ptr_stack_pop(), 7); // LIFO
        assert_eq!(ptr_stack_pop(), 42);
        assert_eq!(ptr_stack_isempty(), 1);
        ptr_stack_push(99);
        ptr_stack_clear(); // resets ptr_stack_top to -1
        assert_eq!(ptr_stack_isempty(), 1);
    }

    // [spec:foma:sem:int-stack.ptr-stack-isfull-fn/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-isfull-fn/test]
    #[test]
    fn ptr_stack_isfull_threshold_at_max_ptr_stack_minus_one() {
        assert_eq!(ptr_stack_isfull(), 0);
        PTR_STACK_TOP.with(|t| t.set(MAX_PTR_STACK as i32 - 2));
        assert_eq!(ptr_stack_isfull(), 0);
        PTR_STACK_TOP.with(|t| t.set(MAX_PTR_STACK as i32 - 1)); // == 2097151
        assert_eq!(ptr_stack_isfull(), 1);
        ptr_stack_clear();
    }

    // [spec:foma:sem:int-stack.ptr-stack-pop-fn/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-pop-fn/test]
    #[test]
    #[should_panic]
    fn ptr_stack_pop_empty_panics_deviation() {
        // C reads ptr_stack[-1] (OOB, UB); the port panics instead.
        ptr_stack_pop();
    }

    // Dead prototype: declared in fomalibconf.h, never defined in C (link
    // error if called); the port pins the never-callable contract via panic.
    // [spec:foma:sem:fomalibconf.int-stack-status-fn/test]
    #[test]
    #[should_panic]
    fn int_stack_status_dead_prototype_panics() {
        int_stack_status();
    }
}

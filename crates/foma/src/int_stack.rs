//! foma/int_stack.c — Wave-4 idiomatization.
//!
//! The C used two fixed `int[MAX_STACK]` / `void *[MAX_PTR_STACK]` static
//! arrays (2^21 slots each, ~16 MB of zeroed BSS) with a manual `top`
//! index and `exit(1)` on overflow. Here each stack is an owned `Vec`
//! wrapped in a small struct that grows on demand. The module keeps
//! thread-local instances so the free functions — called from
//! determinize/constructions/spelling/topsort/coaccessible/structures —
//! keep their exact C signatures and the callers need no changes.
//!
//! Growth is now unbounded: the `MAX_STACK` cap, the `isfull` boundary and
//! the `exit(1)`-on-overflow path are gone (see the `+1`-bumped `*-push-fn`
//! and `*-isfull-fn` sem rules). Popping an empty stack still panics — the
//! C read `a[-1]` (UB); callers guard with `is_empty`, exactly as in C.

use std::cell::RefCell;

use crate::error::FomaError;

/// A LIFO stack of `i32`, backed by a growable `Vec`.
#[derive(Debug, Default)]
pub struct IntStack {
    data: Vec<i32>,
}

impl IntStack {
    pub fn new() -> Self {
        IntStack { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn size(&self) -> i32 {
        self.data.len() as i32
    }

    pub fn find(&self, entry: i32) -> bool {
        self.data.contains(&entry)
    }

    pub fn push(&mut self, c: i32) {
        self.data.push(c);
    }

    pub fn pop(&mut self) -> i32 {
        self.data.pop().expect("int_stack_pop on empty stack")
    }
}

/// A LIFO stack of index/handle tokens — the port's stand-in for the C
/// `void *` pointer stack (every foma call site pushes interior indices,
/// not real pointers; see the determinize/spelling/structures callers).
#[derive(Debug, Default)]
pub struct PtrStack {
    data: Vec<usize>,
}

impl PtrStack {
    pub fn new() -> Self {
        PtrStack { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn push(&mut self, ptr: usize) {
        self.data.push(ptr);
    }

    pub fn pop(&mut self) -> usize {
        self.data.pop().expect("ptr_stack_pop on empty stack")
    }
}

thread_local! {
    // C: static int a[MAX_STACK]; static int top = -1;
    static INT_STACK: RefCell<IntStack> = RefCell::new(IntStack::new());
    // C: static void *ptr_stack[MAX_PTR_STACK]; static int ptr_stack_top = -1;
    static PTR_STACK: RefCell<PtrStack> = RefCell::new(PtrStack::new());
}

// [spec:foma:def:int-stack.ptr-stack-isempty-fn]
// [spec:foma:sem:int-stack.ptr-stack-isempty-fn]
// [spec:foma:def:fomalibconf.ptr-stack-isempty-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-isempty-fn]
pub fn ptr_stack_isempty() -> i32 {
    PTR_STACK.with(|s| s.borrow().is_empty()) as i32
}

// [spec:foma:def:int-stack.ptr-stack-clear-fn]
// [spec:foma:sem:int-stack.ptr-stack-clear-fn]
// [spec:foma:def:fomalibconf.ptr-stack-clear-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-clear-fn]
pub fn ptr_stack_clear() {
    PTR_STACK.with(|s| s.borrow_mut().clear());
}

// [spec:foma:def:int-stack.ptr-stack-pop-fn]
// [spec:foma:sem:int-stack.ptr-stack-pop-fn]
// [spec:foma:def:fomalibconf.ptr-stack-pop-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-pop-fn]
// C read ptr_stack[-1] on an empty pop (UB); popping empty panics here —
// callers guard with ptr_stack_isempty, as in C.
pub fn ptr_stack_pop() -> usize {
    PTR_STACK.with(|s| s.borrow_mut().pop())
}

// [spec:foma:def:int-stack.ptr-stack-isfull-fn]
// [spec:foma:sem:int-stack.ptr-stack-isfull-fn+1]
// [spec:foma:def:fomalibconf.ptr-stack-isfull-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-isfull-fn+1]
// Unbounded growth: the pointer stack is never full (was: top == MAX_PTR_STACK-1).
pub fn ptr_stack_isfull() -> i32 {
    0
}

// [spec:foma:def:int-stack.ptr-stack-push-fn]
// [spec:foma:sem:int-stack.ptr-stack-push-fn+1]
// [spec:foma:def:fomalibconf.ptr-stack-push-fn]
// [spec:foma:sem:fomalibconf.ptr-stack-push-fn+1]
// Infallible, unbounded push (was: exit(1) on a full 2^21-slot array).
pub fn ptr_stack_push(ptr: usize) {
    PTR_STACK.with(|s| s.borrow_mut().push(ptr));
}

// [spec:foma:def:int-stack.int-stack-isempty-fn]
// [spec:foma:sem:int-stack.int-stack-isempty-fn]
// [spec:foma:def:fomalibconf.int-stack-isempty-fn]
// [spec:foma:sem:fomalibconf.int-stack-isempty-fn]
pub fn int_stack_isempty() -> i32 {
    INT_STACK.with(|s| s.borrow().is_empty()) as i32
}

// [spec:foma:def:int-stack.int-stack-clear-fn]
// [spec:foma:sem:int-stack.int-stack-clear-fn]
// [spec:foma:def:fomalibconf.int-stack-clear-fn]
// [spec:foma:sem:fomalibconf.int-stack-clear-fn]
pub fn int_stack_clear() {
    INT_STACK.with(|s| s.borrow_mut().clear());
}

// [spec:foma:def:int-stack.int-stack-find-fn]
// [spec:foma:sem:int-stack.int-stack-find-fn]
// [spec:foma:def:fomalibconf.int-stack-find-fn]
// [spec:foma:sem:fomalibconf.int-stack-find-fn]
pub fn int_stack_find(entry: i32) -> i32 {
    INT_STACK.with(|s| s.borrow().find(entry)) as i32
}

// [spec:foma:def:int-stack.int-stack-size-fn]
// [spec:foma:sem:int-stack.int-stack-size-fn]
// [spec:foma:def:fomalibconf.int-stack-size-fn]
// [spec:foma:sem:fomalibconf.int-stack-size-fn]
pub fn int_stack_size() -> i32 {
    INT_STACK.with(|s| s.borrow().size())
}

// [spec:foma:def:int-stack.int-stack-push-fn]
// [spec:foma:sem:int-stack.int-stack-push-fn+1]
// [spec:foma:def:fomalibconf.int-stack-push-fn]
// [spec:foma:sem:fomalibconf.int-stack-push-fn+1]
// Infallible, unbounded push (was: exit(1) on a full 2^21-slot array).
pub fn int_stack_push(c: i32) {
    INT_STACK.with(|s| s.borrow_mut().push(c));
}

// [spec:foma:def:int-stack.int-stack-pop-fn]
// [spec:foma:sem:int-stack.int-stack-pop-fn]
// [spec:foma:def:fomalibconf.int-stack-pop-fn]
// [spec:foma:sem:fomalibconf.int-stack-pop-fn]
// C read a[-1] on an empty pop (UB); popping empty panics here — callers
// guard with int_stack_isempty, as in C.
pub fn int_stack_pop() -> i32 {
    INT_STACK.with(|s| s.borrow_mut().pop())
}

// [spec:foma:def:int-stack.int-stack-isfull-fn]
// [spec:foma:sem:int-stack.int-stack-isfull-fn+1]
// [spec:foma:def:fomalibconf.int-stack-isfull-fn]
// [spec:foma:sem:fomalibconf.int-stack-isfull-fn+1]
// Unbounded growth: the int stack is never full (was: top == MAX_STACK-1).
pub fn int_stack_isfull() -> i32 {
    0
}

// NOTE: fomalibconf.h also declares `int int_stack_status();` (rule id
// fomalibconf.int-stack-status-fn) but no definition exists anywhere in the
// C sources — it is a dead prototype (a link error if called in C).

// [spec:foma:def:fomalibconf.int-stack-status-fn]
// [spec:foma:sem:fomalibconf.int-stack-status-fn+1]
// Dead prototype: no C definition. Wave 4 surfaces the honest
// FomaError::Unimplemented (was: panic) instead of a link error.
pub fn int_stack_status() -> Result<i32, FomaError> {
    Err(FomaError::Unimplemented(
        "int_stack_status: declared in fomalibconf.h, never defined in C foma",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // [spec:foma:sem:int-stack.int-stack-isempty-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-isempty-fn/test]
    // [spec:foma:sem:int-stack.int-stack-size-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-size-fn/test]
    // [spec:foma:sem:int-stack.int-stack-push-fn+1/test]
    // [spec:foma:sem:fomalibconf.int-stack-push-fn+1/test]
    // [spec:foma:sem:int-stack.int-stack-pop-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-pop-fn/test]
    // [spec:foma:sem:int-stack.int-stack-clear-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-clear-fn/test]
    #[test]
    fn int_stack_push_pop_lifo_size_empty_clear() {
        // Fresh (thread-local) stack starts empty.
        assert_eq!(int_stack_isempty(), 1);
        assert_eq!(int_stack_size(), 0);
        int_stack_push(10);
        int_stack_push(20);
        int_stack_push(30);
        assert_eq!(int_stack_isempty(), 0);
        assert_eq!(int_stack_size(), 3);
        // LIFO pop.
        assert_eq!(int_stack_pop(), 30);
        assert_eq!(int_stack_pop(), 20);
        assert_eq!(int_stack_size(), 1);
        // clear resets to empty.
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
        int_stack_clear();
    }

    // [spec:foma:sem:int-stack.int-stack-isfull-fn+1/test]
    // [spec:foma:sem:fomalibconf.int-stack-isfull-fn+1/test]
    #[test]
    fn int_stack_isfull_always_false_with_unbounded_growth() {
        // Wave 4: the stack grows unbounded, so isfull is never true
        // (was: top == MAX_STACK - 1).
        assert_eq!(int_stack_isfull(), 0);
        int_stack_push(1);
        int_stack_push(2);
        assert_eq!(int_stack_isfull(), 0);
        int_stack_clear();
    }

    // [spec:foma:sem:int-stack.int-stack-pop-fn/test]
    // [spec:foma:sem:fomalibconf.int-stack-pop-fn/test]
    #[test]
    #[should_panic]
    fn int_stack_pop_empty_panics_deviation() {
        // C read a[-1] (OOB, UB); the port panics on the empty pop instead.
        int_stack_pop();
    }

    // [spec:foma:sem:int-stack.ptr-stack-isempty-fn/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-isempty-fn/test]
    // [spec:foma:sem:int-stack.ptr-stack-push-fn+1/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-push-fn+1/test]
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
        ptr_stack_clear(); // resets to empty
        assert_eq!(ptr_stack_isempty(), 1);
    }

    // [spec:foma:sem:int-stack.ptr-stack-isfull-fn+1/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-isfull-fn+1/test]
    #[test]
    fn ptr_stack_isfull_always_false_with_unbounded_growth() {
        assert_eq!(ptr_stack_isfull(), 0);
        ptr_stack_push(1);
        ptr_stack_push(2);
        assert_eq!(ptr_stack_isfull(), 0);
        ptr_stack_clear();
    }

    // [spec:foma:sem:int-stack.ptr-stack-pop-fn/test]
    // [spec:foma:sem:fomalibconf.ptr-stack-pop-fn/test]
    #[test]
    #[should_panic]
    fn ptr_stack_pop_empty_panics_deviation() {
        // C read ptr_stack[-1] (OOB, UB); the port panics instead.
        ptr_stack_pop();
    }

    // Dead prototype: no C definition (link error if called). Wave 4
    // returns FomaError::Unimplemented in place of the Wave-2 panic.
    // [spec:foma:sem:fomalibconf.int-stack-status-fn+1/test]
    #[test]
    fn int_stack_status_dead_prototype_is_unimplemented() {
        assert!(matches!(
            int_stack_status(),
            Err(FomaError::Unimplemented(_))
        ));
    }
}

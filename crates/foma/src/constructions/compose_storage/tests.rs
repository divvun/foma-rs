use super::*;
use crate::constructions::TriplethashTriplets;
use crate::structures::fsm_create;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_NONCE: AtomicU64 = AtomicU64::new(0);

struct TestParent(PathBuf);

impl TestParent {
    fn new() -> Self {
        let nonce = TEST_NONCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "foma-compose-storage-test-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&path).expect("create storage test parent");
        Self(path)
    }

    fn is_empty(&self) -> bool {
        std::fs::read_dir(&self.0)
            .expect("read storage test parent")
            .next()
            .is_none()
    }
}

impl Drop for TestParent {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn memory_plan_partitions_once() {
    assert_eq!(
        ComposeMemoryPlan::from_allowance(Some(100)),
        ComposeMemoryPlan::Bounded {
            allowance_bytes: 100,
            tracked_cap_bytes: 90,
            pair_interner_cap_bytes: 54,
            work_stack_cap_bytes: 9,
            output_cap_bytes: 27,
        }
    );
    assert_eq!(
        ComposeMemoryPlan::from_allowance(Some(0)),
        ComposeMemoryPlan::Bounded {
            allowance_bytes: 0,
            tracked_cap_bytes: 0,
            pair_interner_cap_bytes: 0,
            work_stack_cap_bytes: 0,
            output_cap_bytes: 0,
        }
    );
    assert_eq!(
        ComposeMemoryPlan::from_allowance(None),
        ComposeMemoryPlan::Unbounded
    );
}

#[test]
fn memory_plan_handles_maximum_allowance() {
    let ComposeMemoryPlan::Bounded {
        allowance_bytes,
        tracked_cap_bytes,
        pair_interner_cap_bytes,
        work_stack_cap_bytes,
        output_cap_bytes,
    } = ComposeMemoryPlan::from_allowance(Some(u64::MAX))
    else {
        panic!("maximum allowance must remain bounded");
    };
    assert_eq!(allowance_bytes, u64::MAX);
    assert_eq!(
        pair_interner_cap_bytes + work_stack_cap_bytes + output_cap_bytes,
        tracked_cap_bytes
    );
    assert_eq!(
        tracked_cap_bytes,
        u64::MAX / 10 * 9 + (u64::MAX % 10) * 9 / 10
    );
}

#[test]
fn state_codec_is_exact() {
    let state = FsmState {
        state_no: 123,
        r#in: -4,
        out: 31,
        target: 987,
        final_state: -1,
        start_state: 1,
    };
    assert_eq!(state_from_bytes(state_to_bytes(state)), state);
}

#[test]
fn zero_forces_every_store_to_scratch() {
    let parent = TestParent::new();
    let config = ComposeResourceConfig::bounded(0, &parent.0);
    let mut runtime = ComposeRuntime::new(&config, 3).expect("create bounded runtime");
    assert_eq!(runtime.intern_state(0, 0, 0).unwrap(), 0);
    runtime.begin_state(0, 1, 1);
    runtime.add_arc(0, 3, 3, 0, 1, 1).unwrap();
    runtime.end_state().unwrap();
    assert_eq!(runtime.spill_status(), (true, true, true));
    assert!(!parent.is_empty(), "live runtime owns scratch files");

    let product = runtime.finish().expect("finish spilled product");
    assert!(!parent.is_empty(), "spilled product retains scratch");
    drop(product);
    assert!(parent.is_empty(), "drop removes operation scratch");
}

#[test]
fn disk_interner_reuses_dense_ids() {
    let parent = TestParent::new();
    let config = ComposeResourceConfig::bounded(0, &parent.0);
    let mut runtime = ComposeRuntime::new(&config, 3).expect("create bounded runtime");
    for state in 0..400 {
        assert_eq!(
            runtime.intern_state(state, state * 7, state % 6).unwrap(),
            state
        );
    }
    for state in (0..400).rev() {
        assert_eq!(
            runtime.intern_state(state, state * 7, state % 6).unwrap(),
            state
        );
    }
    assert!(runtime.spill_status().0);
    drop(runtime);
    assert!(parent.is_empty());
}

#[test]
fn generous_cap_keeps_stores_resident() {
    let parent = TestParent::new();
    let config = ComposeResourceConfig::bounded(1024 * 1024, &parent.0);
    let mut runtime = ComposeRuntime::new(&config, 3).expect("create bounded runtime");
    assert_eq!(runtime.intern_state(0, 0, 0).unwrap(), 0);
    runtime.begin_state(0, 1, 1);
    runtime.add_arc(0, 3, 3, 0, 1, 1).unwrap();
    runtime.end_state().unwrap();
    assert_eq!(runtime.spill_status(), (false, false, false));
    drop(runtime);
    assert!(parent.is_empty());
}

fn work_item(state: i32) -> ComposeWorkItem {
    ComposeWorkItem {
        state,
        left: state * 2,
        right: state * 3,
        mode: state % 6,
    }
}

#[test]
fn work_spill_preserves_lifo_order() {
    let parent = TestParent::new();
    let mut stack = ComposeWorkStack::bounded(4 * 16, parent.0.clone());
    for state in 0..100 {
        stack.push(work_item(state)).unwrap();
    }
    assert!(stack.is_spilled(), "fifth item crosses the four-item cap");
    for state in (0..100).rev() {
        assert_eq!(stack.pop().unwrap(), Some(work_item(state)));
    }
    assert_eq!(stack.pop().unwrap(), None);
}

#[test]
fn pair_growth_crosses_exact_cap() {
    let parent = TestParent::new();
    let initial_bytes = 128 * std::mem::size_of::<TriplethashTriplets>() as u64;
    let mut interner = ComposeInterner::bounded(initial_bytes, parent.0.clone());
    for state in 0..64 {
        let inserted = interner.intern(state, state + 1, state % 3).unwrap();
        assert_eq!(inserted.state, state);
        assert!(inserted.is_new);
    }
    assert!(
        !interner.is_spilled(),
        "half-full initial table fits exactly"
    );

    let inserted = interner.intern(64, 65, 1).unwrap();
    assert_eq!(inserted.state, 64);
    assert!(inserted.is_new);
    assert!(interner.is_spilled(), "the required rehash crosses the cap");
    let duplicate = interner.intern(17, 18, 2).unwrap();
    assert_eq!(duplicate.state, 17);
    assert!(!duplicate.is_new);
}

#[test]
fn corrupt_spill_errors_and_cleans() {
    let parent = TestParent::new();
    let config = ComposeResourceConfig::bounded(0, &parent.0);
    let mut runtime = ComposeRuntime::new(&config, 3).unwrap();
    runtime.intern_state(0, 0, 0).unwrap();
    runtime.begin_state(0, 1, 1);
    runtime.end_state().unwrap();
    let product = runtime.finish().unwrap();
    let rows_path = match &product {
        ComposeProduct::Scratch { spilled, .. } => spilled.rows_path.clone(),
        ComposeProduct::Memory { .. } => panic!("zero cap must spill output rows"),
    };
    std::fs::OpenOptions::new()
        .write(true)
        .open(&rows_path)
        .unwrap()
        .set_len(1)
        .unwrap();

    let mut net = fsm_create("");
    let error = product
        .install_into(&mut net)
        .expect_err("partial product row must fail");
    assert!(
        error
            .to_string()
            .contains("complete composition product row")
    );
    assert!(parent.is_empty(), "error path removes operation scratch");
}

#[test]
fn resource_config_reports_policy() {
    let unbounded = ComposeResourceConfig::unbounded();
    assert_eq!(unbounded.memory_limit_bytes(), None);
    assert_eq!(unbounded.scratch_dir(), None);

    let bounded = ComposeResourceConfig::bounded(1234, "/tmp/foma-compose-policy");
    assert_eq!(bounded.memory_limit_bytes(), Some(1234));
    assert_eq!(
        bounded.scratch_dir(),
        Some(std::path::Path::new("/tmp/foma-compose-policy"))
    );
}

#[test]
fn scratch_names_are_operation_owned() {
    let parent = TestParent::new();
    let first = ComposeScratch::create(&parent.0).unwrap();
    let second = ComposeScratch::create(&parent.0).unwrap();
    assert_ne!(first.path(), second.path());
    assert!(
        first
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with(".foma-compose.")
    );
    assert!(first.path().to_string_lossy().ends_with(".scratch"));
    drop(first);
    assert_eq!(std::fs::read_dir(&parent.0).unwrap().count(), 1);
    drop(second);
    assert!(parent.is_empty());
}

#[test]
fn memory_plan_rounding_stays_conservative() {
    for allowance in 0..1000u64 {
        let ComposeMemoryPlan::Bounded {
            tracked_cap_bytes,
            pair_interner_cap_bytes,
            work_stack_cap_bytes,
            output_cap_bytes,
            ..
        } = ComposeMemoryPlan::from_allowance(Some(allowance))
        else {
            unreachable!();
        };
        assert_eq!(tracked_cap_bytes, allowance * 9 / 10);
        assert_eq!(
            pair_interner_cap_bytes + work_stack_cap_bytes + output_cap_bytes,
            tracked_cap_bytes
        );
        assert!(tracked_cap_bytes <= allowance);
    }
}

#[test]
fn disk_stack_reuses_popped_slots() {
    let parent = TestParent::new();
    let mut stack = ComposeWorkStack::bounded(0, parent.0.clone());
    stack.push(work_item(1)).unwrap();
    stack.push(work_item(2)).unwrap();
    assert_eq!(stack.pop().unwrap(), Some(work_item(2)));
    stack.push(work_item(3)).unwrap();
    assert_eq!(stack.pop().unwrap(), Some(work_item(3)));
    assert_eq!(stack.pop().unwrap(), Some(work_item(1)));
    assert_eq!(stack.pop().unwrap(), None);
}

#[test]
fn duplicate_pair_does_not_grow_frontier() {
    let parent = TestParent::new();
    let config = ComposeResourceConfig::bounded(0, &parent.0);
    let mut runtime = ComposeRuntime::new(&config, 3).unwrap();
    assert_eq!(runtime.intern_state(7, 8, 2).unwrap(), 0);
    assert_eq!(runtime.intern_state(7, 8, 2).unwrap(), 0);
    assert_eq!(
        runtime.pop_state().unwrap(),
        Some(work_item_from(0, 7, 8, 2))
    );
    assert_eq!(runtime.pop_state().unwrap(), None);
}

fn work_item_from(state: i32, left: i32, right: i32, mode: i32) -> ComposeWorkItem {
    ComposeWorkItem {
        state,
        left,
        right,
        mode,
    }
}

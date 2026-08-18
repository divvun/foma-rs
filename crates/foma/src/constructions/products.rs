//! Wave-4 split of constructions.c (see mod.rs). Cross-module and
//! external names come via `use super::*` (re-exported by mod.rs).
use super::*;
use crate::error::FomaError;
use smol_str::SmolStr;

/// Flag labels that a binary product operation exposes as identity self-loops
/// without adding transitions to either operand.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ComposeFlagOverlay {
    left_self_loops: Vec<SmolStr>,
    right_self_loops: Vec<SmolStr>,
    enforce_left_before_right: bool,
    flags_as_epsilon: bool,
}

impl ComposeFlagOverlay {
    /// Build a canonical overlay. The two sides must be disjoint so a virtual
    /// transition can never match another virtual transition.
    pub fn new(
        mut left_self_loops: Vec<SmolStr>,
        mut right_self_loops: Vec<SmolStr>,
        enforce_left_before_right: bool,
    ) -> Result<Self, FomaError> {
        left_self_loops.sort_unstable();
        left_self_loops.dedup();
        right_self_loops.sort_unstable();
        right_self_loops.dedup();

        if left_self_loops
            .iter()
            .any(|label| right_self_loops.binary_search(label).is_ok())
        {
            return Err(FomaError::MalformedInput(
                "flag-overlay label sets must be disjoint".to_string(),
            ));
        }

        Ok(Self {
            left_self_loops,
            right_self_loops,
            enforce_left_before_right,
            flags_as_epsilon: false,
        })
    }

    /// Treat real flag events and virtual loops as HFST's one-sided epsilon
    /// moves while retaining their labels for result restoration.
    // [spec:foma:req:constructions.compose-virtual-flags]
    pub fn with_flags_as_epsilon(mut self) -> Self {
        self.flags_as_epsilon = true;
        self
    }

    pub fn left_self_loops(&self) -> &[SmolStr] {
        &self.left_self_loops
    }

    pub fn right_self_loops(&self) -> &[SmolStr] {
        &self.right_self_loops
    }

    pub fn enforces_left_before_right(&self) -> bool {
        self.enforce_left_before_right
    }

    pub fn flags_are_epsilon(&self) -> bool {
        self.flags_as_epsilon
    }

    fn is_empty(&self) -> bool {
        self.left_self_loops.is_empty() && self.right_self_loops.is_empty()
    }
}

/// Operation-neutral name for the virtual identity-loop overlay.
pub type FlagOverlay = ComposeFlagOverlay;

const COMPOSE_EPSILON_MODE_COUNT: i32 = 3;

fn compose_product_mode(epsilon_mode: i32, saw_right_flag: bool) -> i32 {
    epsilon_mode + i32::from(saw_right_flag) * COMPOSE_EPSILON_MODE_COUNT
}

fn compose_epsilon_mode(product_mode: i32) -> i32 {
    product_mode % COMPOSE_EPSILON_MODE_COUNT
}

fn compose_saw_right_flag(product_mode: i32) -> bool {
    product_mode >= COMPOSE_EPSILON_MODE_COUNT
}

#[derive(Clone)]
struct ComposeOutEntry {
    symin: i16,
    symout: i16,
    target: i32,
    mainloop: i32,
}

#[derive(Clone)]
struct ComposeIndex {
    tail: usize,
}

struct NumericFlagOverlay {
    left_self_loops: Vec<bool>,
    right_self_loops: Vec<bool>,
    left_labels: Vec<i32>,
    right_labels: Vec<i32>,
    enforce_left_before_right: bool,
    flags_as_epsilon: bool,
}

impl NumericFlagOverlay {
    fn is_flag(&self, label: i32) -> bool {
        label >= 0
            && (self.left_self_loops[label as usize] || self.right_self_loops[label as usize])
    }

    fn next_intersection_order(&self, output: i32, saw_right: bool) -> Option<bool> {
        if !self.enforce_left_before_right {
            return Some(false);
        }
        if output >= 0 && self.right_self_loops[output as usize] {
            return (!saw_right).then_some(saw_right);
        }
        if output >= 0 && self.left_self_loops[output as usize] {
            return Some(true);
        }
        if output != EPSILON {
            return Some(false);
        }
        Some(saw_right)
    }
}

fn intern_intersection_state(
    states: &mut Triplethash,
    work: &mut IntStack,
    left: i32,
    right: i32,
    saw_right: bool,
) -> i32 {
    let order = i32::from(saw_right);
    match triplet_hash_find(states, left, right, order) {
        Some(state) => state,
        None => {
            work.push(order);
            work.push(right);
            work.push(left);
            triplet_hash_insert(states, left, right, order)
        }
    }
}

#[derive(Clone, Copy)]
struct ComposeStateInfo {
    left: i32,
    right: i32,
    epsilon_mode: i32,
    saw_right_flag: bool,
    output_state: i32,
    final_state: i32,
    start_state: i32,
}

fn index_compose_right_state(
    state: i32,
    transitions: &[FsmState],
    first_transition: usize,
    max_sigma: i32,
    mainloop: i32,
    index: &mut [ComposeIndex],
    entries: &mut [ComposeOutEntry],
) {
    let mut position = first_transition;
    while transitions[position].state_no == state {
        let input = if transitions[position].r#in as i32 == IDENTITY {
            UNKNOWN
        } else {
            transitions[position].r#in as i32
        };
        if input < 0 || transitions[position].target < 0 {
            position += 1;
            continue;
        }

        let mut entry = index[input as usize].tail;
        if entries[entry].mainloop != mainloop {
            entry = (input * (max_sigma + 2)) as usize;
        } else {
            entry += 1;
        }
        entries[entry] = ComposeOutEntry {
            symin: transitions[position].r#in,
            symout: transitions[position].out,
            target: transitions[position].target,
            mainloop,
        };
        index[input as usize].tail = entry;
        position += 1;
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_compose_symbol_matches(
    state: ComposeStateInfo,
    transitions: &[FsmState],
    first_transition: usize,
    max_sigma: i32,
    mainloop: i32,
    index: &[ComposeIndex],
    entries: &[ComposeOutEntry],
    overlay: &NumericFlagOverlay,
    runtime: &mut ComposeRuntime,
) -> Result<(), FomaError> {
    let mut position = first_transition;
    while transitions[position].state_no == state.left {
        let original_output = transitions[position].out as i32;
        if original_output < 0 {
            position += 1;
            continue;
        }

        let search_label = if original_output == IDENTITY {
            UNKNOWN
        } else {
            original_output
        };
        let mut entry = (search_label * (max_sigma + 2)) as usize;
        let tail = index[search_label as usize].tail + 1;
        while entry != tail && entries[entry].mainloop == mainloop {
            let mut input = transitions[position].r#in as i32;
            let mut left_output = original_output;
            let mut right_input = entries[entry].symin as i32;
            let mut output = entries[entry].symout as i32;

            if left_output == IDENTITY && right_input == UNKNOWN {
                input = UNKNOWN;
                left_output = UNKNOWN;
            } else if left_output == UNKNOWN && right_input == IDENTITY {
                right_input = UNKNOWN;
                output = UNKNOWN;
            }

            if right_input == left_output && (right_input != EPSILON || state.epsilon_mode == 0) {
                let next_saw_right = if overlay.enforce_left_before_right
                    && left_output != EPSILON
                    && !overlay.is_flag(left_output)
                {
                    false
                } else {
                    state.saw_right_flag
                };
                let next_mode = compose_product_mode(0, next_saw_right);
                let target = runtime.intern_state(
                    transitions[position].target,
                    entries[entry].target,
                    next_mode,
                )?;
                runtime.add_arc(
                    state.output_state,
                    input,
                    output,
                    target,
                    state.final_state,
                    state.start_state,
                )?;
            }
            entry += 1;
        }

        if !overlay.flags_as_epsilon
            && overlay.right_self_loops[original_output as usize]
            && !(overlay.enforce_left_before_right && state.saw_right_flag)
        {
            let next_mode = compose_product_mode(0, state.saw_right_flag);
            let target =
                runtime.intern_state(transitions[position].target, state.right, next_mode)?;
            runtime.add_arc(
                state.output_state,
                transitions[position].r#in as i32,
                original_output,
                target,
                state.final_state,
                state.start_state,
            )?;
        }
        position += 1;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_virtual_left_matches(
    state: ComposeStateInfo,
    transitions: &[FsmState],
    first_transition: usize,
    overlay: &NumericFlagOverlay,
    runtime: &mut ComposeRuntime,
) -> Result<(), FomaError> {
    let mut position = first_transition;
    while transitions[position].state_no == state.right {
        let input = transitions[position].r#in as i32;
        if !overlay.flags_as_epsilon && input >= 0 && overlay.left_self_loops[input as usize] {
            let target = runtime.intern_state(
                state.left,
                transitions[position].target,
                compose_product_mode(0, true),
            )?;
            runtime.add_arc(
                state.output_state,
                input,
                transitions[position].out as i32,
                target,
                state.final_state,
                state.start_state,
            )?;
        }
        position += 1;
    }
    Ok(())
}

fn next_left_epsilon_mode(tristate: bool, epsilon_mode: i32) -> Option<i32> {
    if !tristate && epsilon_mode == 0 {
        Some(0)
    } else if tristate && epsilon_mode != 2 {
        Some(1)
    } else {
        None
    }
}

fn next_right_epsilon_mode(tristate: bool, epsilon_mode: i32) -> Option<i32> {
    if !tristate {
        Some(1)
    } else if epsilon_mode != 1 {
        Some(2)
    } else {
        None
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_left_epsilon_moves(
    state: ComposeStateInfo,
    transitions: &[FsmState],
    first_transition: usize,
    tristate: bool,
    flag_is_epsilon: bool,
    is_flag: &[bool],
    overlay: &NumericFlagOverlay,
    runtime: &mut ComposeRuntime,
) -> Result<(), FomaError> {
    let mut position = first_transition;
    while transitions[position].state_no == state.left {
        let output = transitions[position].out as i32;
        let overlay_flag = overlay.flags_as_epsilon && overlay.is_flag(output);
        if output != EPSILON && !flag_is_epsilon && !overlay_flag {
            position += 1;
            continue;
        }
        let input = transitions[position].r#in as i32;
        if overlay_flag
            && let Some(epsilon_mode) = next_left_epsilon_mode(tristate, state.epsilon_mode)
            && let Some(saw_right) = overlay.next_intersection_order(output, state.saw_right_flag)
        {
            let target = runtime.intern_state(
                transitions[position].target,
                state.right,
                compose_product_mode(epsilon_mode, saw_right),
            )?;
            runtime.add_arc(
                state.output_state,
                input,
                EPSILON,
                target,
                state.final_state,
                state.start_state,
            )?;
        }
        if flag_is_epsilon && output >= 0 && state.epsilon_mode == 0 && is_flag[output as usize] {
            let target = runtime.intern_state(
                transitions[position].target,
                state.right,
                compose_product_mode(0, state.saw_right_flag),
            )?;
            runtime.add_arc(
                state.output_state,
                input,
                output,
                target,
                state.final_state,
                state.start_state,
            )?;
        }

        let next_epsilon_mode = next_left_epsilon_mode(tristate, state.epsilon_mode);
        if output == EPSILON
            && let Some(epsilon_mode) = next_epsilon_mode
        {
            let target = runtime.intern_state(
                transitions[position].target,
                state.right,
                compose_product_mode(epsilon_mode, state.saw_right_flag),
            )?;
            runtime.add_arc(
                state.output_state,
                input,
                EPSILON,
                target,
                state.final_state,
                state.start_state,
            )?;
        }
        position += 1;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_right_epsilon_moves(
    state: ComposeStateInfo,
    transitions: &[FsmState],
    first_transition: usize,
    tristate: bool,
    flag_is_epsilon: bool,
    is_flag: &[bool],
    overlay: &NumericFlagOverlay,
    runtime: &mut ComposeRuntime,
) -> Result<(), FomaError> {
    let mut position = first_transition;
    while transitions[position].state_no == state.right {
        let input = transitions[position].r#in as i32;
        let overlay_flag = overlay.flags_as_epsilon && overlay.is_flag(input);
        if input != EPSILON && !flag_is_epsilon && !overlay_flag {
            position += 1;
            continue;
        }
        let output = transitions[position].out as i32;
        if overlay_flag
            && let Some(epsilon_mode) = next_right_epsilon_mode(tristate, state.epsilon_mode)
        {
            let target = runtime.intern_state(
                state.left,
                transitions[position].target,
                compose_product_mode(epsilon_mode, state.saw_right_flag),
            )?;
            runtime.add_arc(
                state.output_state,
                EPSILON,
                output,
                target,
                state.final_state,
                state.start_state,
            )?;
        }
        if flag_is_epsilon && input >= 0 && is_flag[input as usize] {
            let target = runtime.intern_state(
                state.left,
                transitions[position].target,
                compose_product_mode(1, state.saw_right_flag),
            )?;
            runtime.add_arc(
                state.output_state,
                input,
                output,
                target,
                state.final_state,
                state.start_state,
            )?;
        }

        let next_epsilon_mode = next_right_epsilon_mode(tristate, state.epsilon_mode);
        if input == EPSILON
            && let Some(epsilon_mode) = next_epsilon_mode
        {
            let target = runtime.intern_state(
                state.left,
                transitions[position].target,
                compose_product_mode(epsilon_mode, state.saw_right_flag),
            )?;
            runtime.add_arc(
                state.output_state,
                EPSILON,
                output,
                target,
                state.final_state,
                state.start_state,
            )?;
        }
        position += 1;
    }
    Ok(())
}

fn emit_virtual_epsilon_moves(
    state: ComposeStateInfo,
    tristate: bool,
    overlay: &NumericFlagOverlay,
    runtime: &mut ComposeRuntime,
) -> Result<(), FomaError> {
    if !overlay.flags_as_epsilon {
        return Ok(());
    }
    if let Some(epsilon_mode) = next_left_epsilon_mode(tristate, state.epsilon_mode) {
        for label in &overlay.left_labels {
            if let Some(saw_right) = overlay.next_intersection_order(*label, state.saw_right_flag) {
                let target = runtime.intern_state(
                    state.left,
                    state.right,
                    compose_product_mode(epsilon_mode, saw_right),
                )?;
                runtime.add_arc(
                    state.output_state,
                    *label,
                    EPSILON,
                    target,
                    state.final_state,
                    state.start_state,
                )?;
            }
        }
    }
    if let Some(epsilon_mode) = next_right_epsilon_mode(tristate, state.epsilon_mode) {
        for label in &overlay.right_labels {
            let target = runtime.intern_state(
                state.left,
                state.right,
                compose_product_mode(epsilon_mode, state.saw_right_flag),
            )?;
            runtime.add_arc(
                state.output_state,
                EPSILON,
                *label,
                target,
                state.final_state,
                state.start_state,
            )?;
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_epsilon_pairs(
    state: ComposeStateInfo,
    left_transitions: &[FsmState],
    left_first: usize,
    right_transitions: &[FsmState],
    right_first: usize,
    overlay: &NumericFlagOverlay,
    runtime: &mut ComposeRuntime,
) -> Result<(), FomaError> {
    if !overlay.flags_as_epsilon || state.epsilon_mode != 0 {
        return Ok(());
    }

    let mut left_position = left_first;
    while left_transitions[left_position].state_no == state.left {
        let left_output = left_transitions[left_position].out as i32;
        if left_output == EPSILON || overlay.is_flag(left_output) {
            let saw_right = if left_output == EPSILON {
                Some(state.saw_right_flag)
            } else {
                overlay.next_intersection_order(left_output, state.saw_right_flag)
            };
            if let Some(saw_right) = saw_right {
                let mut right_position = right_first;
                while right_transitions[right_position].state_no == state.right {
                    let right_input = right_transitions[right_position].r#in as i32;
                    if (right_input == EPSILON || overlay.is_flag(right_input))
                        && !(left_output == EPSILON && right_input == EPSILON)
                    {
                        let target = runtime.intern_state(
                            left_transitions[left_position].target,
                            right_transitions[right_position].target,
                            compose_product_mode(0, saw_right),
                        )?;
                        runtime.add_arc(
                            state.output_state,
                            left_transitions[left_position].r#in as i32,
                            right_transitions[right_position].out as i32,
                            target,
                            state.final_state,
                            state.start_state,
                        )?;
                    }
                    right_position += 1;
                }
                for right_label in &overlay.right_labels {
                    let target = runtime.intern_state(
                        left_transitions[left_position].target,
                        state.right,
                        compose_product_mode(0, saw_right),
                    )?;
                    runtime.add_arc(
                        state.output_state,
                        left_transitions[left_position].r#in as i32,
                        *right_label,
                        target,
                        state.final_state,
                        state.start_state,
                    )?;
                }
            }
        }
        left_position += 1;
    }

    for left_label in &overlay.left_labels {
        if let Some(saw_right) = overlay.next_intersection_order(*left_label, state.saw_right_flag)
        {
            let mut right_position = right_first;
            while right_transitions[right_position].state_no == state.right {
                let right_input = right_transitions[right_position].r#in as i32;
                if right_input == EPSILON || overlay.is_flag(right_input) {
                    let target = runtime.intern_state(
                        state.left,
                        right_transitions[right_position].target,
                        compose_product_mode(0, saw_right),
                    )?;
                    runtime.add_arc(
                        state.output_state,
                        *left_label,
                        right_transitions[right_position].out as i32,
                        target,
                        state.final_state,
                        state.start_state,
                    )?;
                }
                right_position += 1;
            }
            for right_label in &overlay.right_labels {
                let target = runtime.intern_state(
                    state.left,
                    state.right,
                    compose_product_mode(0, saw_right),
                )?;
                runtime.add_arc(
                    state.output_state,
                    *left_label,
                    *right_label,
                    target,
                    state.final_state,
                    state.start_state,
                )?;
            }
        }
    }
    Ok(())
}

// [spec:foma:def:constructions.fsm-intersect-fn]
// [spec:foma:sem:constructions.fsm-intersect-fn]
// [spec:foma:def:fomalib.fsm-intersect-fn]
// [spec:foma:sem:fomalib.fsm-intersect-fn]
pub fn fsm_intersect(opts: &FomaOptions, net1: Fsm, net2: Fsm) -> Fsm {
    fsm_intersect_with_flag_overlay(opts, net1, net2, &FlagOverlay::default())
        .expect("ordinary intersection uses an empty, always-valid flag overlay")
}

/// Intersect owned operands while exposing configured flag labels as virtual
/// identity self-loops.
// [spec:foma:req:constructions.intersect-virtual-flags]
pub fn fsm_intersect_with_flag_overlay(
    opts: &FomaOptions,
    net1: Fsm,
    net2: Fsm,
    overlay: &FlagOverlay,
) -> Result<Fsm, FomaError> {
    let mut int_stack = IntStack::new();
    /* C: struct blookup {int mainloop; int target; } *array, *bptr; —
    function-local type */
    #[derive(Clone)]
    struct Blookup {
        mainloop: i32,
        target: i32,
    }

    let mut net1 = fsm_minimize(opts, net1);
    let mut net2 = fsm_minimize(opts, net2);

    if fsm_isempty(opts, &mut net1) || fsm_isempty(opts, &mut net2) {
        fsm_destroy(net1);
        fsm_destroy(net2);
        return Ok(fsm_empty_set());
    }

    // Minimization may remove alphabet-only symbols. Restore every overlay
    // label before merging sigmas so wildcard expansion cannot consume flags.
    if !overlay.is_empty() {
        for label in overlay
            .left_self_loops()
            .iter()
            .chain(overlay.right_self_loops())
        {
            if sigma_find(label, &net1.sigma).is_none() {
                sigma_add(label, &mut net1.sigma);
            }
            if sigma_find(label, &net2.sigma).is_none() {
                sigma_add(label, &mut net2.sigma);
            }
        }
        sigma_sort(&mut net1);
        sigma_sort(&mut net2);
    }

    fsm_merge_sigma(opts, &mut net1, &mut net2);

    let merged_sigma_size = (sigma_max(&net1.sigma) + 1) as usize;
    let mut numeric_overlay = NumericFlagOverlay {
        left_self_loops: vec![false; merged_sigma_size],
        right_self_loops: vec![false; merged_sigma_size],
        left_labels: Vec::with_capacity(overlay.left_self_loops().len()),
        right_labels: Vec::with_capacity(overlay.right_self_loops().len()),
        enforce_left_before_right: overlay.enforces_left_before_right(),
        flags_as_epsilon: overlay.flags_are_epsilon(),
    };
    for label in overlay.left_self_loops() {
        let number = sigma_find(label, &net1.sigma).ok_or_else(|| {
            FomaError::MalformedInput(format!(
                "left flag-overlay label {label:?} is absent from the merged sigma"
            ))
        })?;
        if number <= IDENTITY {
            return Err(FomaError::MalformedInput(format!(
                "special symbol {label:?} cannot be a virtual flag loop"
            )));
        }
        numeric_overlay.left_self_loops[number as usize] = true;
        numeric_overlay.left_labels.push(number);
    }
    for label in overlay.right_self_loops() {
        let number = sigma_find(label, &net1.sigma).ok_or_else(|| {
            FomaError::MalformedInput(format!(
                "right flag-overlay label {label:?} is absent from the merged sigma"
            ))
        })?;
        if number <= IDENTITY {
            return Err(FomaError::MalformedInput(format!(
                "special symbol {label:?} cannot be a virtual flag loop"
            )));
        }
        numeric_overlay.right_self_loops[number as usize] = true;
        numeric_overlay.right_labels.push(number);
    }

    fsm_update_flags(&mut net1, YES, NO, UNK, YES, UNK, UNK);

    let sigma2size = sigma_max(&net2.sigma) + 1;
    /* calloc — zeroed entries; mainloop stamps start at 1 below, so all
    entries begin stale */
    let mut array: Vec<Blookup> = vec![
        Blookup {
            mainloop: 0,
            target: 0,
        };
        (sigma2size * sigma2size) as usize
    ];
    let mut mainloop = 0;

    /* Intersect two networks by the running-in-parallel method */
    /* new state 0 = {0,0} */

    /* The third state component carries the two-sided flag-order bit. */
    int_stack.push(0);
    int_stack.push(0);
    int_stack.push(0);

    let mut th = triplet_hash_init();
    triplet_hash_insert(&mut th, 0, 0, 0);

    let mut builder = fsm_state_init(sigma_max(&net1.sigma));

    let point_a = init_state_pointers(&net1.states.rows());
    let point_b = init_state_pointers(&net2.states.rows());

    {
        let fsm1 = net1.states.rows();
        let fsm2 = net2.states.rows();
        while !int_stack.is_empty() {
            /* Get a pair of states to examine */

            let a = int_stack.pop();
            let b = int_stack.pop();
            let saw_right = int_stack.pop() != 0;

            let current_state = triplet_hash_find(&th, a, b, i32::from(saw_right))
                .expect("state pair popped off the work stack was inserted into the triplet hash");
            let current_start = if point_a[a as usize].start == 1 && point_b[b as usize].start == 1
            {
                1
            } else {
                0
            };
            let current_final =
                if point_a[a as usize].r#final == 1 && point_b[b as usize].r#final == 1 {
                    1
                } else {
                    0
                };

            fsm_state_set_current_state(&mut builder, current_state, current_final, current_start);

            /* Create a lookup index for machine b */
            /* array[in][out] holds the target for this state and the symbol pair in:out */
            /* Also, we keep track of whether an entry is fresh by the mainloop counter */
            /* so we don't mistakenly use an old entry and don't have to clear the table */
            /* between each state pair we encounter */

            mainloop += 1;
            let mut bi = point_b[b as usize].transitions;
            while fsm2[bi].state_no == b {
                if fsm2[bi].r#in < 0 {
                    bi += 1;
                    continue;
                }
                let bptr = ((fsm2[bi].r#in as i32) * sigma2size + fsm2[bi].out as i32) as usize;
                array[bptr].mainloop = mainloop;
                array[bptr].target = fsm2[bi].target;
                bi += 1;
            }

            /* The main loop where we run the machines in parallel */
            /* We look at each transition of a in this state, and consult the index of b */
            /* we just created */

            let mut ai = point_a[a as usize].transitions;
            while fsm1[ai].state_no == a {
                if fsm1[ai].r#in < 0 || fsm1[ai].out < 0 {
                    ai += 1;
                    continue;
                }
                let bptr = ((fsm1[ai].r#in as i32) * sigma2size + fsm1[ai].out as i32) as usize;

                let atarget = fsm1[ai].target;
                let (ain, aout) = (fsm1[ai].r#in as i32, fsm1[ai].out as i32);
                if array[bptr].mainloop == mainloop
                    && let Some(next_order) =
                        numeric_overlay.next_intersection_order(aout, saw_right)
                {
                    let target_number = intern_intersection_state(
                        &mut th,
                        &mut int_stack,
                        atarget,
                        array[bptr].target,
                        next_order,
                    );
                    fsm_state_add_arc(
                        &mut builder,
                        current_state,
                        ain,
                        aout,
                        target_number,
                        current_final,
                        current_start,
                    );
                }

                // A real left flag matches the virtual identity loop on the
                // right without advancing the right state.
                if ain == aout
                    && numeric_overlay.right_self_loops[aout as usize]
                    && let Some(next_order) =
                        numeric_overlay.next_intersection_order(aout, saw_right)
                {
                    let target_number =
                        intern_intersection_state(&mut th, &mut int_stack, atarget, b, next_order);
                    fsm_state_add_arc(
                        &mut builder,
                        current_state,
                        ain,
                        aout,
                        target_number,
                        current_final,
                        current_start,
                    );
                }

                ai += 1;
            }

            // A real right flag matches the virtual identity loop on the left
            // without advancing the left state.
            let mut bi = point_b[b as usize].transitions;
            while fsm2[bi].state_no == b {
                let (bin, bout) = (fsm2[bi].r#in as i32, fsm2[bi].out as i32);
                if bin >= 0
                    && bin == bout
                    && numeric_overlay.left_self_loops[bin as usize]
                    && let Some(next_order) =
                        numeric_overlay.next_intersection_order(bout, saw_right)
                {
                    let target_number = intern_intersection_state(
                        &mut th,
                        &mut int_stack,
                        a,
                        fsm2[bi].target,
                        next_order,
                    );
                    fsm_state_add_arc(
                        &mut builder,
                        current_state,
                        bin,
                        bout,
                        target_number,
                        current_final,
                        current_start,
                    );
                }
                bi += 1;
            }
            fsm_state_end_state(&mut builder);
        }
    }
    let mut new_net = fsm_create("");
    fsm_sigma_destroy(core::mem::take(&mut new_net.sigma));
    new_net.sigma = core::mem::take(&mut net1.sigma);
    fsm_destroy(net2);
    fsm_destroy(net1);
    fsm_state_close(&mut builder, &mut new_net);
    /* free(point_a); free(point_b); free(array) */
    drop(point_a);
    drop(point_b);
    drop(array);
    triplet_hash_free(Some(th));
    Ok(fsm_coaccessible(new_net))
}

// [spec:foma:def:constructions.fsm-compose-fn]
// [spec:foma:sem:constructions.fsm-compose-fn]
// [spec:foma:def:fomalib.fsm-compose-fn]
// [spec:foma:sem:fomalib.fsm-compose-fn]
pub fn fsm_compose(opts: &FomaOptions, net1: Fsm, net2: Fsm) -> Fsm {
    compose_with_flag_overlay(
        opts,
        net1,
        net2,
        &ComposeFlagOverlay::default(),
        &ComposeResourceConfig::unbounded(),
    )
    .expect("ordinary composition uses an empty, always-valid flag overlay")
}

/// Compose owned operands while exposing configured flag labels as virtual
/// identity self-loops.
// [spec:foma:req:constructions.compose-virtual-flags]
pub fn fsm_compose_with_flag_overlay(
    opts: &FomaOptions,
    net1: Fsm,
    net2: Fsm,
    overlay: &ComposeFlagOverlay,
) -> Result<Fsm, FomaError> {
    fsm_compose_with_config(
        opts,
        net1,
        net2,
        overlay,
        &ComposeResourceConfig::unbounded(),
    )
}

/// Compose owned operands with virtual flag loops and an optional exact
/// working-memory allowance.
// [spec:foma:req:constructions.compose-memory-budget]
pub fn fsm_compose_with_config(
    opts: &FomaOptions,
    net1: Fsm,
    net2: Fsm,
    overlay: &ComposeFlagOverlay,
    resources: &ComposeResourceConfig,
) -> Result<Fsm, FomaError> {
    if opts.flag_is_epsilon && !overlay.is_empty() {
        return Err(FomaError::MalformedInput(
            "virtual flag composition cannot be combined with flag-is-epsilon".to_string(),
        ));
    }
    compose_with_flag_overlay(opts, net1, net2, overlay, resources)
}

fn compose_with_flag_overlay(
    opts: &FomaOptions,
    net1: Fsm,
    net2: Fsm,
    overlay: &ComposeFlagOverlay,
    resources: &ComposeResourceConfig,
) -> Result<Fsm, FomaError> {
    /* The composition algorithm is the basic naive composition where we lazily      */
    /* take the cross-product of states P and Q and move to a new state with symbols */
    /* ain, bout if the symbols aout = bin.  Also, if aout = 0 state p goes to       */
    /* its target, while q stays.  Similarly, if bin = 0, q goes to its target       */
    /* while p stays.                                                                */

    /* We have two variants of the algorithm to avoid creating multiple paths:       */
    /* 1) Bistate composition.  In this variant, when we create a new state, we call it */
    /*    (p,q,mode) where mode = 0 or 1, depending on what kind of an arc we followed  */
    /*    to get there.  If we followed an x:y arc where x and y are both real symbols  */
    /*    we always go to mode 0, however, if we followed an 0:y arc, we go to mode 1.  */
    /*    from mode 1, we do not follow x:0 arcs.  Each (p,q,mode) is unique, and       */
    /*    from (p,q,X) we always consider the transitions from p and q.                 */
    /*    We never create arcs (x:0 0:y) yielding x:y.                                  */

    /* 2) Tristate composition. Here we always go to mode 0 with a x:y arc.             */
    /*    (x:0,0:y) yielding x:y is allowed, but only in mode 0                         */
    /*    (x:y y:z) is always allowed and results in target = mode 0                    */
    /*    0:y arcs lead to mode 2, and from there we stay in mode 2 with 0:y            */
    /*    in mode 2 we only consider 0:y and x:y arcs                                   */
    /*    x:0 arcs lead to mode 1, and from there we stay in mode 1 with x:0            */
    /*    in mode 1 we only consider x:0 and x:y arcs                                   */

    /* It seems unsettled which type of composition is better.  Tristate is similar to  */
    /* the filter transducer given in Mohri, Pereira and Riley (1996) and works well    */
    /* for cases such as [a:0 b:0 c:0 .o. 0:d 0:e 0:f], yielding the shortest path.     */
    /* However, for generic cases, bistate seems to yield smaller transducers.          */
    /* The global variable g_compose_tristate is set to OFF by default                  */

    let g_compose_tristate = opts.compose_tristate;
    let g_flag_is_epsilon = opts.flag_is_epsilon;

    let mut net1 = fsm_minimize(opts, net1);
    let mut net2 = fsm_minimize(opts, net2);

    if fsm_isempty(opts, &mut net1) || fsm_isempty(opts, &mut net2) {
        fsm_destroy(net1);
        fsm_destroy(net2);
        return Ok(fsm_empty_set());
    }

    /* If flag-is-epsilon is on, we need to add the flag symbols    */
    /* in both networks to each other's sigma so that UNKNOWN       */
    /* or IDENTITY symbols do not match these flags, since they are */
    /* supposed to have the behavior of EPSILON                     */
    /* And we need to do this before merging the sigmas, of course  */

    if g_flag_is_epsilon {
        let mut flags1 = 0;
        let mut flags2 = 0;
        let max2sigma = sigma_max(&net2.sigma);
        for idx in 0..net1.sigma.len() {
            let sym = net1.sigma[idx].symbol.clone();
            if flag_check(&sym) {
                flags1 = 1;
                if sigma_find(&sym, &net2.sigma).is_none() {
                    sigma_add(&sym, &mut net2.sigma);
                }
            }
        }

        for idx in 0..net2.sigma.len() {
            let s2num = net2.sigma[idx].number;
            let sym = net2.sigma[idx].symbol.clone();
            if flag_check(&sym) {
                if s2num <= max2sigma {
                    flags2 = 1;
                }
                if sigma_find(&sym, &net1.sigma).is_none() {
                    sigma_add(&sym, &mut net1.sigma);
                }
            }
        }
        sigma_sort(&mut net2);
        sigma_sort(&mut net1);
        if flags1 != 0 && flags2 != 0 {
            tracing::warn!(
                "flag-is-epsilon is ON and both networks contain flags in composition.  This may yield incorrect results.  Set flag-is-epsilon to OFF."
            );
        }
    }

    // Minimization may remove alphabet-only symbols. Restore every configured
    // overlay label to both sigmas before the merge: this both keeps the label
    // addressable and prevents UNKNOWN/IDENTITY expansion from treating a flag
    // as absent on the virtual-loop side.
    if !overlay.is_empty() {
        for label in overlay
            .left_self_loops()
            .iter()
            .chain(overlay.right_self_loops())
        {
            if sigma_find(label, &net1.sigma).is_none() {
                sigma_add(label, &mut net1.sigma);
            }
            if sigma_find(label, &net2.sigma).is_none() {
                sigma_add(label, &mut net2.sigma);
            }
        }
        sigma_sort(&mut net1);
        sigma_sort(&mut net2);
    }

    fsm_merge_sigma(opts, &mut net1, &mut net2);

    let merged_sigma_size = (sigma_max(&net1.sigma) + 1) as usize;
    let mut numeric_overlay = NumericFlagOverlay {
        left_self_loops: vec![false; merged_sigma_size],
        right_self_loops: vec![false; merged_sigma_size],
        left_labels: Vec::with_capacity(overlay.left_self_loops().len()),
        right_labels: Vec::with_capacity(overlay.right_self_loops().len()),
        enforce_left_before_right: overlay.enforces_left_before_right(),
        flags_as_epsilon: overlay.flags_are_epsilon(),
    };
    for label in overlay.left_self_loops() {
        let number = sigma_find(label, &net1.sigma).ok_or_else(|| {
            FomaError::MalformedInput(format!(
                "left flag-overlay label {label:?} is absent from the merged sigma"
            ))
        })?;
        if number <= IDENTITY {
            return Err(FomaError::MalformedInput(format!(
                "special symbol {label:?} cannot be a virtual flag loop"
            )));
        }
        numeric_overlay.left_self_loops[number as usize] = true;
        numeric_overlay.left_labels.push(number);
    }
    for label in overlay.right_self_loops() {
        let number = sigma_find(label, &net1.sigma).ok_or_else(|| {
            FomaError::MalformedInput(format!(
                "right flag-overlay label {label:?} is absent from the merged sigma"
            ))
        })?;
        if number <= IDENTITY {
            return Err(FomaError::MalformedInput(format!(
                "special symbol {label:?} cannot be a virtual flag loop"
            )));
        }
        numeric_overlay.right_self_loops[number as usize] = true;
        numeric_overlay.right_labels.push(number);
    }

    let mut is_flag: Vec<bool> = Vec::new();
    if g_flag_is_epsilon {
        /* Create lookup table for quickly checking if a symbol is a flag */
        /* C: malloc'd (uninitialized for numbers absent from the sigma);
        zero-initialized here */
        is_flag = vec![false; (sigma_max(&net1.sigma) + 1) as usize];
        for s1 in &net1.sigma {
            is_flag[s1.number as usize] = flag_check(&s1.symbol);
        }
    }

    fsm_update_flags(&mut net1, YES, NO, UNK, YES, UNK, UNK);

    let max2sigma = sigma_max(&net2.sigma);

    /* Create an index for looking up input symbols in machine b quickly */
    /* We store each machine_b->in symbol in outarray[symin][...] */
    /* the array index[symin] points to the tail of the current list in outarray */
    /* (we may have many entries for one input symbol as there may be many outputs */
    /* The field mainloop tells us whether the entry is current as we want to loop */
    /* UNKNOWN and IDENTITY are indexed as UNKNOWN because we need to find both */
    /* as they share some semantics */

    let mut index: Vec<ComposeIndex> = vec![ComposeIndex { tail: 0 }; (max2sigma + 1) as usize];
    let mut outarray: Vec<ComposeOutEntry> = vec![
        ComposeOutEntry {
            symin: 0,
            symout: 0,
            target: 0,
            mainloop: 0,
        };
        ((max2sigma + 2) * (max2sigma + 1)) as usize
    ];

    for i in 0..=max2sigma {
        index[i as usize].tail = ((max2sigma + 2) * i) as usize;
    }

    let mut runtime = ComposeRuntime::new(resources, sigma_max(&net1.sigma))?;
    let start_state = runtime.intern_state(0, 0, 0)?;
    debug_assert_eq!(start_state, 0);

    let point_a = init_state_pointers(&net1.states.rows());
    let point_b = init_state_pointers(&net2.states.rows());

    let mut mainloop = 0;

    let fsm1 = net1.states.rows();
    let fsm2 = net2.states.rows();
    while let Some(work) = runtime.pop_state()? {
        /* Get a pair of states to examine */

        let a = work.left;
        let b = work.right;
        let product_mode = work.mode;
        let mode = compose_epsilon_mode(product_mode);
        let saw_right_flag = compose_saw_right_flag(product_mode);

        let current_state = work.state;
        let current_start = if point_a[a as usize].start == 1
            && point_b[b as usize].start == 1
            && product_mode == 0
        {
            1
        } else {
            0
        };
        let current_final = if point_a[a as usize].r#final == 1 && point_b[b as usize].r#final == 1
        {
            1
        } else {
            0
        };

        runtime.begin_state(current_state, current_final, current_start);

        mainloop += 1;
        index_compose_right_state(
            b,
            &fsm2,
            point_b[b as usize].transitions,
            max2sigma,
            mainloop,
            &mut index,
            &mut outarray,
        );

        let state = ComposeStateInfo {
            left: a,
            right: b,
            epsilon_mode: mode,
            saw_right_flag,
            output_state: current_state,
            final_state: current_final,
            start_state: current_start,
        };
        emit_compose_symbol_matches(
            state,
            &fsm1,
            point_a[a as usize].transitions,
            max2sigma,
            mainloop,
            &index,
            &outarray,
            &numeric_overlay,
            &mut runtime,
        )?;
        emit_virtual_left_matches(
            state,
            &fsm2,
            point_b[b as usize].transitions,
            &numeric_overlay,
            &mut runtime,
        )?;
        emit_epsilon_pairs(
            state,
            &fsm1,
            point_a[a as usize].transitions,
            &fsm2,
            point_b[b as usize].transitions,
            &numeric_overlay,
            &mut runtime,
        )?;

        emit_left_epsilon_moves(
            state,
            &fsm1,
            point_a[a as usize].transitions,
            g_compose_tristate,
            g_flag_is_epsilon,
            &is_flag,
            &numeric_overlay,
            &mut runtime,
        )?;
        emit_right_epsilon_moves(
            state,
            &fsm2,
            point_b[b as usize].transitions,
            g_compose_tristate,
            g_flag_is_epsilon,
            &is_flag,
            &numeric_overlay,
            &mut runtime,
        )?;
        emit_virtual_epsilon_moves(state, g_compose_tristate, &numeric_overlay, &mut runtime)?;
        runtime.end_state()?;
    }
    let product = runtime.finish()?;
    drop(fsm1);
    drop(fsm2);

    /* free(net1->states) */
    net1.states = Vec::new().into();
    fsm_destroy(net2);
    /* free(point_a); free(point_b); free(index); free(outarray) */
    drop(point_a);
    drop(point_b);
    drop(index);
    drop(outarray);

    if g_flag_is_epsilon {
        /* free(is_flag) */
        drop(is_flag);
    }
    let externally_pruned = product.install_into(&mut net1)?;
    let net1 = if externally_pruned {
        net1
    } else {
        fsm_coaccessible(net1)
    };
    let net1 = fsm_topsort(net1);
    Ok(fsm_coaccessible(net1))
}

// [spec:foma:def:constructions.fsm-cross-product-fn]
// [spec:foma:sem:constructions.fsm-cross-product-fn]
// [spec:foma:def:fomalib.fsm-cross-product-fn]
// [spec:foma:sem:fomalib.fsm-cross-product-fn]
pub fn fsm_cross_product(opts: &FomaOptions, net1: Fsm, net2: Fsm) -> Fsm {
    let mut int_stack = IntStack::new();
    /* Perform a cross product by running two machines in parallel */
    /* The approach here allows a state to stay, creating a a:0 or 0:b transition */
    /* with the a/b-state waiting, and the arc going to {a,stay} or {stay,b} */
    /* the wait maneuver is only possible if the waiting state is final */

    /* For the rewrite rules compilation, a different cross-product is used:  */
    /* rewrite_cp() synchronizes A and B as long as possible to get a unique  */
    /* output match for each cross product.                                   */

    /* This behavior where we postpone zeroes on either side and perform */
    /* and equal length cross-product as long as possible and never intermix */
    /* ?:0 and 0:? arcs (i.e. we keep both machines synchronized as long as possible */
    /* can be done by [A .x. B] & ?:?* [?:0*|0:?*] at the cost of possibly */
    /* up to three times larger transducers. */
    /* This is very similar to the idea in "tristate composition" in fsm_compose() */

    /* This function is only used for explicit cross products */
    /* such as a:b or A.x.B, etc.  In rewrite rules, we use rewrite_cp() */

    let mut net1 = fsm_minimize(opts, net1);
    let mut net2 = fsm_minimize(opts, net2);

    fsm_merge_sigma(opts, &mut net1, &mut net2);

    fsm_count(&mut net1);
    fsm_count(&mut net2);

    /* new state 0 = {0,0} */

    /* STACK_2_PUSH(0,0) */
    int_stack.push(0);
    int_stack.push(0);

    let mut th = triplet_hash_init();
    triplet_hash_insert(&mut th, 0, 0, 0);

    let mut builder = fsm_state_init(sigma_max(&net1.sigma));

    let point_a = init_state_pointers(&net1.states.rows());
    let point_b = init_state_pointers(&net2.states.rows());

    let fsm1 = net1.states.rows();
    let fsm2 = net2.states.rows();
    while !int_stack.is_empty() {
        /* Get a pair of states to examine */

        let a = int_stack.pop();
        let b = int_stack.pop();

        /* printf("Treating pair: {%i,%i}\n",a,b); */

        let current_state = triplet_hash_find(&th, a, b, 0)
            .expect("state pair popped off the work stack was inserted into the triplet hash");
        let current_start = if point_a[a as usize].start == 1 && point_b[b as usize].start == 1 {
            1
        } else {
            0
        };
        let current_final = if point_a[a as usize].r#final == 1 && point_b[b as usize].r#final == 1
        {
            1
        } else {
            0
        };

        fsm_state_set_current_state(&mut builder, current_state, current_final, current_start);

        let mut ai = point_a[a as usize].transitions;
        while fsm1[ai].state_no == a {
            let mut bi = point_b[b as usize].transitions;
            while fsm2[bi].state_no == b {
                if fsm1[ai].target == -1 && fsm2[bi].target == -1 {
                    bi += 1;
                    continue;
                }
                if fsm1[ai].target == -1 && fsm1[ai].final_state == 0 {
                    bi += 1;
                    continue;
                }
                if fsm2[bi].target == -1 && fsm2[bi].final_state == 0 {
                    bi += 1;
                    continue;
                }
                /* Main check */
                if !(fsm1[ai].target == -1 || fsm2[bi].target == -1) {
                    let atarget = fsm1[ai].target;
                    let btarget = fsm2[bi].target;
                    let target_number = match triplet_hash_find(&th, atarget, btarget, 0) {
                        Some(n) => n,
                        None => {
                            /* STACK_2_PUSH(machine_b->target, machine_a->target) */
                            int_stack.push(btarget);
                            int_stack.push(atarget);
                            triplet_hash_insert(&mut th, atarget, btarget, 0)
                        }
                    };
                    let mut symbol1 = fsm1[ai].r#in as i32;
                    let mut symbol2 = fsm2[bi].r#in as i32;
                    if symbol1 == IDENTITY && symbol2 != IDENTITY {
                        symbol1 = UNKNOWN;
                    }
                    if symbol2 == IDENTITY && symbol1 != IDENTITY {
                        symbol2 = UNKNOWN;
                    }

                    fsm_state_add_arc(
                        &mut builder,
                        current_state,
                        symbol1,
                        symbol2,
                        target_number,
                        current_final,
                        current_start,
                    );
                    /* @:@ -> @:@ and also ?:? */
                    if fsm1[ai].r#in as i32 == IDENTITY && fsm2[bi].r#in as i32 == IDENTITY {
                        fsm_state_add_arc(
                            &mut builder,
                            current_state,
                            UNKNOWN,
                            UNKNOWN,
                            target_number,
                            current_final,
                            current_start,
                        );
                    }
                }
                if fsm1[ai].final_state == 1 && fsm2[bi].target != -1 {
                    /* Add 0:b i.e. stay in state A */
                    let astate = fsm1[ai].state_no;
                    let btarget = fsm2[bi].target;
                    let target_number = match triplet_hash_find(&th, astate, btarget, 0) {
                        Some(n) => n,
                        None => {
                            /* STACK_2_PUSH(machine_b->target, machine_a->state_no) */
                            int_stack.push(btarget);
                            int_stack.push(astate);
                            triplet_hash_insert(&mut th, astate, btarget, 0)
                        }
                    };
                    /* @:0 becomes ?:0 */
                    let symbol2 = if fsm2[bi].r#in as i32 == IDENTITY {
                        UNKNOWN
                    } else {
                        fsm2[bi].r#in as i32
                    };
                    fsm_state_add_arc(
                        &mut builder,
                        current_state,
                        EPSILON,
                        symbol2,
                        target_number,
                        current_final,
                        current_start,
                    );
                }

                if fsm2[bi].final_state == 1 && fsm1[ai].target != -1 {
                    /* Add a:0 i.e. stay in state B */
                    let atarget = fsm1[ai].target;
                    let bstate = fsm2[bi].state_no;
                    let target_number = match triplet_hash_find(&th, atarget, bstate, 0) {
                        Some(n) => n,
                        None => {
                            /* STACK_2_PUSH(machine_b->state_no, machine_a->target) */
                            int_stack.push(bstate);
                            int_stack.push(atarget);
                            triplet_hash_insert(&mut th, atarget, bstate, 0)
                        }
                    };
                    /* @:0 becomes ?:0 */
                    let symbol1 = if fsm1[ai].r#in as i32 == IDENTITY {
                        UNKNOWN
                    } else {
                        fsm1[ai].r#in as i32
                    };
                    fsm_state_add_arc(
                        &mut builder,
                        current_state,
                        symbol1,
                        EPSILON,
                        target_number,
                        current_final,
                        current_start,
                    );
                }
                bi += 1;
            }
            ai += 1;
        }
        /* Check arctrack */
        fsm_state_end_state(&mut builder);
    }
    drop(fsm1);
    drop(fsm2);

    /* free(net1->states) */
    net1.states = Vec::new().into();
    fsm_state_close(&mut builder, &mut net1);

    let mut epsilon = 0;
    let mut unknown = 0;
    {
        let fsm = net1.states.rows();
        let mut i = 0usize;
        while fsm[i].state_no != -1 {
            if fsm[i].r#in as i32 == EPSILON || fsm[i].out as i32 == EPSILON {
                epsilon = 1;
            }
            if fsm[i].r#in as i32 == UNKNOWN || fsm[i].out as i32 == UNKNOWN {
                unknown = 1;
            }
            i += 1;
        }
    }
    if epsilon == 1 && sigma_find_number(EPSILON, &net1.sigma).is_none() {
        sigma_add_special(EPSILON, &mut net1.sigma);
    }
    if unknown == 1 && sigma_find_number(UNKNOWN, &net1.sigma).is_none() {
        sigma_add_special(UNKNOWN, &mut net1.sigma);
    }
    /* free(point_a); free(point_b) */
    drop(point_a);
    drop(point_b);
    fsm_destroy(net2);
    triplet_hash_free(Some(th));
    fsm_coaccessible(net1)
}

use super::*;

// [spec:foma:req:constructions.intersect-virtual-flags/test]
#[test]
fn one_sided_overlays_match_eager_loops() {
    let opts = &FomaOptions::default();
    let left_flag = "LEFT_FLAG";
    let right_flag = "RIGHT_FLAG";

    let left = re(&format!(r#""{left_flag}" a"#));
    let right = re("a");
    let eager = fsm_intersect(
        opts,
        left.clone(),
        fsm_add_loop(right.clone(), &fsm_symbol(left_flag), 2),
    );
    let overlay = FlagOverlay::new(Vec::new(), vec![left_flag.into()], false)
        .expect("right-side loop overlay is valid");
    let actual = fsm_intersect_with_flag_overlay(opts, left, right, &overlay)
        .expect("virtual right loop intersects");
    assert_eq!(words(&actual), words(&eager));

    let left = re("a");
    let right = re(&format!(r#""{right_flag}" a"#));
    let eager = fsm_intersect(
        opts,
        fsm_add_loop(left.clone(), &fsm_symbol(right_flag), 2),
        right.clone(),
    );
    let overlay = FlagOverlay::new(vec![right_flag.into()], Vec::new(), false)
        .expect("left-side loop overlay is valid");
    let actual = fsm_intersect_with_flag_overlay(opts, left, right, &overlay)
        .expect("virtual left loop intersects");
    assert_eq!(words(&actual), words(&eager));
}

#[test]
fn two_sided_overlay_enforces_order() {
    let opts = &FomaOptions::default();
    let left_flag = "LEFT_FLAG_1";
    let right_flag = "RIGHT_FLAG_2";
    let overlay = FlagOverlay::new(vec![right_flag.into()], vec![left_flag.into()], true)
        .expect("renamed overlay sets are disjoint");
    let result = fsm_intersect_with_flag_overlay(
        opts,
        re(&format!(r#""{left_flag}" a"#)),
        re(&format!(r#""{right_flag}" a"#)),
        &overlay,
    )
    .expect("two-sided virtual intersection succeeds");

    assert_eq!(words(&result), vec![format!("{left_flag}{right_flag}a")]);
}

#[test]
fn epsilon_output_does_not_reset_order() {
    let opts = &FomaOptions::default();
    let left_flag = "LEFT_FLAG_1";
    let right_flag = "RIGHT_FLAG_2";
    let overlay = FlagOverlay::new(vec![right_flag.into()], vec![left_flag.into()], true)
        .expect("renamed overlay sets are disjoint");
    let mut result = fsm_intersect_with_flag_overlay(
        opts,
        re(&format!(r#"x:0 "{left_flag}" a"#)),
        re(&format!(r#""{right_flag}" x:0 a"#)),
        &overlay,
    )
    .expect("epsilon-order fixture intersects");

    assert!(
        fsm_isempty(opts, &mut result),
        "x:epsilon must not reset the right-before-left rejection state"
    );
}

#[test]
fn alphabet_only_overlay_avoids_wildcard_expansion() {
    let opts = &FomaOptions::default();
    let flag = "ALPHABET_ONLY_FLAG";
    let left = re(&format!(r#""{flag}" a"#));
    let mut right = re("? | a");
    sigma_add(flag, &mut right.sigma);
    sigma_sort(&mut right);

    let eager = fsm_intersect(
        opts,
        left.clone(),
        fsm_add_loop(right.clone(), &fsm_symbol(flag), 2),
    );
    let overlay = FlagOverlay::new(Vec::new(), vec![flag.into()], false)
        .expect("alphabet-only right overlay is valid");
    let actual = fsm_intersect_with_flag_overlay(opts, left, right, &overlay)
        .expect("alphabet-only virtual intersection succeeds");

    assert!(fsm_equivalent(opts, actual.clone(), eager.clone()));
    assert_eq!(words(&actual), words(&eager));
}

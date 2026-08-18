use super::*;
use smol_str::SmolStr;

// [spec:foma:req:constructions.compose-virtual-flags/test]
#[test]
fn virtual_right_flag_matches_eager_loop() {
    let opts = &FomaOptions::default();
    let flag = "@P.FEATURE.LEFT@";
    let left = re(r#"a:"@P.FEATURE.LEFT@" b:c"#);
    let right = re("c:d");
    let marker = fsm_symbol(flag);
    let eager = fsm_compose(opts, left.clone(), fsm_add_loop(right.clone(), &marker, 2));
    let overlay = ComposeFlagOverlay::new(Vec::new(), vec![flag.into()], false)
        .expect("one-sided overlay is valid");
    let actual = fsm_compose_with_flag_overlay(opts, left, right, &overlay)
        .expect("virtual composition succeeds");

    assert!(fsm_equivalent(opts, eager.clone(), actual.clone()));
    assert_eq!(down(&actual, "ab"), down(&eager, "ab"));
}

// [spec:foma:req:constructions.compose-virtual-flags/test]
#[test]
fn virtual_left_flag_matches_eager_loop() {
    let opts = &FomaOptions::default();
    let flag = "@P.FEATURE.RIGHT@";
    let left = fsm_empty_string();
    let right = re(r#""@P.FEATURE.RIGHT@":z"#);
    let marker = fsm_symbol(flag);
    let eager = fsm_compose(opts, fsm_add_loop(left.clone(), &marker, 2), right.clone());
    let overlay = ComposeFlagOverlay::new(vec![flag.into()], Vec::new(), false)
        .expect("one-sided overlay is valid");
    let actual = fsm_compose_with_flag_overlay(opts, left, right, &overlay)
        .expect("virtual composition succeeds");

    assert!(fsm_equivalent(opts, eager.clone(), actual.clone()));
    assert_eq!(down(&actual, flag), down(&eager, flag));
}

// [spec:foma:req:constructions.compose-virtual-flags/test]
#[test]
fn flag_order_and_reset() {
    let opts = &FomaOptions::default();
    let left_flag = "LEFT_FLAG_1";
    let right_flag = "RIGHT_FLAG_2";
    let overlay = ComposeFlagOverlay::new(vec![right_flag.into()], vec![left_flag.into()], true)
        .expect("renamed flag sets are disjoint");

    let ordered = fsm_compose_with_flag_overlay(
        opts,
        re(&format!(r#""{left_flag}""#)),
        re(&format!(r#""{right_flag}""#)),
        &overlay,
    )
    .expect("ordered flag composition succeeds");
    let forward_path = format!("{left_flag}{right_flag}");
    assert_eq!(down(&ordered, &forward_path), vec![forward_path]);
    assert!(down(&ordered, &format!("{right_flag}{left_flag}")).is_empty());

    let reset = fsm_compose_with_flag_overlay(
        opts,
        re(&format!(r#"x "{left_flag}""#)),
        re(&format!(r#""{right_flag}" x"#)),
        &overlay,
    )
    .expect("regular symbols reset the flag-order state");
    let reset_path = format!("{right_flag}x{left_flag}");
    assert_eq!(down(&reset, &reset_path), vec![reset_path]);
}

// [spec:foma:req:constructions.compose-virtual-flags/test]
#[test]
fn flag_overlay_validation_rejects_ambiguous_modes() {
    let flag = SmolStr::new("@P.FEATURE.VALUE@");
    assert!(ComposeFlagOverlay::new(vec![flag.clone()], vec![flag], true).is_err());

    let opts = FomaOptions {
        flag_is_epsilon: true,
        ..FomaOptions::default()
    };
    let overlay = ComposeFlagOverlay::new(Vec::new(), vec!["@P.F.V@".into()], false)
        .expect("one-sided overlay is valid");
    assert!(fsm_compose_with_flag_overlay(&opts, re("a"), re("a"), &overlay).is_err());
}

// [spec:foma:req:constructions.compose-virtual-flags/test]
#[test]
fn empty_overlay_and_wildcard_parity() {
    let opts = &FomaOptions::default();
    let left = re("a:b | c:d");
    let right = re("b:x | d:y");
    let ordinary = fsm_compose(opts, left.clone(), right.clone());
    let configured =
        fsm_compose_with_flag_overlay(opts, left, right, &ComposeFlagOverlay::default())
            .expect("empty overlay is always valid");
    assert_eq!(lines(&configured), lines(&ordinary));
    assert_eq!(sigma_pairs(&configured), sigma_pairs(&ordinary));

    let overlay = ComposeFlagOverlay::new(Vec::new(), vec!["VIRTUAL_FLAG".into()], false)
        .expect("one-sided overlay is valid");
    let mut wildcard = fsm_compose_with_flag_overlay(opts, re("a:?"), fsm_empty_string(), &overlay)
        .expect("wildcard fixture composes");
    assert!(
        fsm_isempty(opts, &mut wildcard),
        "UNKNOWN must not expand into or match a virtual flag loop"
    );
}

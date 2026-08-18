use super::*;
use smol_str::SmolStr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static SCRATCH_NONCE: AtomicU64 = AtomicU64::new(0);

struct ScratchParent(PathBuf);

impl ScratchParent {
    fn new() -> Self {
        let nonce = SCRATCH_NONCE.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("foma-compose-test-{}-{nonce}", std::process::id()));
        std::fs::create_dir(&path).expect("create compose test scratch parent");
        Self(path)
    }
}

impl Drop for ScratchParent {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

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

// [spec:foma:req:constructions.compose-memory-budget/test]
#[test]
fn bounded_spill_matches_unbounded() {
    let opts = &FomaOptions::default();
    let left = re("a:b | c:d | e:f");
    let right = re("b:x | d:y");
    let expected = fsm_compose(opts, left.clone(), right.clone());
    let scratch = ScratchParent::new();
    let resources = ComposeResourceConfig::bounded(0, &scratch.0);
    let actual = fsm_compose_with_config(
        opts,
        left,
        right,
        &ComposeFlagOverlay::default(),
        &resources,
    )
    .expect("zero-cap composition spills and succeeds");

    assert_eq!(lines(&actual), lines(&expected));
    assert_eq!(sigma_pairs(&actual), sigma_pairs(&expected));
    assert_eq!(actual.statecount, expected.statecount);
    assert_eq!(actual.arccount, expected.arccount);
    assert!(
        std::fs::read_dir(&scratch.0)
            .expect("read scratch parent")
            .next()
            .is_none(),
        "operation-owned scratch is removed after success"
    );
}

// [spec:foma:req:constructions.compose-memory-budget/test]
#[test]
fn invalid_scratch_returns_error() {
    let scratch = ScratchParent::new();
    let missing = scratch.0.join("missing-parent");
    let resources = ComposeResourceConfig::bounded(0, missing);
    let error = fsm_compose_with_config(
        &FomaOptions::default(),
        re("a:b"),
        re("b:c"),
        &ComposeFlagOverlay::default(),
        &resources,
    )
    .expect_err("missing scratch parent must fail");
    assert!(error.to_string().contains("scratch directory"));
}

// [spec:foma:req:constructions.compose-memory-budget/test]
// [spec:foma:req:constructions.compose-virtual-flags/test]
#[test]
fn bounded_overlay_matches_unbounded() {
    let opts = &FomaOptions::default();
    let left_flag = "LEFT_FLAG_1";
    let right_flag = "RIGHT_FLAG_2";
    let overlay = ComposeFlagOverlay::new(vec![right_flag.into()], vec![left_flag.into()], true)
        .expect("renamed flag sets are disjoint");
    let left = re(&format!(r#"x "{left_flag}" | "{left_flag}" y"#));
    let right = re(&format!(r#""{right_flag}" x | y "{right_flag}""#));
    let expected = fsm_compose_with_flag_overlay(opts, left.clone(), right.clone(), &overlay)
        .expect("unbounded overlay composition succeeds");
    let scratch = ScratchParent::new();
    let actual = fsm_compose_with_config(
        opts,
        left,
        right,
        &overlay,
        &ComposeResourceConfig::bounded(0, &scratch.0),
    )
    .expect("bounded overlay composition succeeds");

    assert_eq!(lines(&actual), lines(&expected));
    assert_eq!(sigma_pairs(&actual), sigma_pairs(&expected));
    assert_eq!(actual.statecount, expected.statecount);
    assert_eq!(actual.arccount, expected.arccount);
    assert!(std::fs::read_dir(&scratch.0).unwrap().next().is_none());
}

// [spec:foma:req:constructions.compose-memory-budget/test]
#[test]
fn bounded_cutoff_matrix_is_exact() {
    let cases = [
        ("a:b | c:d", "b:x | d:y"),
        ("a:0 b:c | d:e", "0:x c:y | e:z"),
        ("a:b | c:d | e:f", "b:x | d:y | q:r"),
        ("a:?", "a:x | b:y"),
    ];
    for tristate in [false, true] {
        let opts = FomaOptions {
            compose_tristate: tristate,
            ..FomaOptions::default()
        };
        for (left_regex, right_regex) in cases {
            let left = re(left_regex);
            let right = re(right_regex);
            let expected = fsm_compose(&opts, left.clone(), right.clone());
            for allowance in [0, 1, 200, 1024 * 1024] {
                let scratch = ScratchParent::new();
                let actual = fsm_compose_with_config(
                    &opts,
                    left.clone(),
                    right.clone(),
                    &ComposeFlagOverlay::default(),
                    &ComposeResourceConfig::bounded(allowance, &scratch.0),
                )
                .unwrap_or_else(|error| {
                    panic!(
                        "compose {left_regex:?} with {right_regex:?} at {allowance} bytes: {error}"
                    )
                });
                assert_eq!(lines(&actual), lines(&expected));
                assert_eq!(sigma_pairs(&actual), sigma_pairs(&expected));
                assert_eq!(actual.statecount, expected.statecount);
                assert_eq!(actual.arccount, expected.arccount);
                assert!(std::fs::read_dir(&scratch.0).unwrap().next().is_none());
            }
        }
    }
}

// [spec:foma:req:constructions.compose-memory-budget/test]
#[test]
fn bounded_empty_product_needs_no_scratch() {
    let scratch = ScratchParent::new();
    let missing_parent = scratch.0.join("unused");
    let actual = fsm_compose_with_config(
        &FomaOptions::default(),
        fsm_empty_set(),
        re("a"),
        &ComposeFlagOverlay::default(),
        &ComposeResourceConfig::bounded(0, &missing_parent),
    )
    .expect("empty operands short-circuit before scratch setup");
    let mut actual = actual;
    assert!(fsm_isempty(&FomaOptions::default(), &mut actual));
    assert!(!missing_parent.exists());
}

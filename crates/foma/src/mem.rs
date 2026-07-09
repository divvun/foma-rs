//! foma/mem.c — literal Wave-2 (bug-for-bug) port per
//! docs/port/rust-conventions.md. Sem rules: docs/spec/port/foma/mem.md.

// C: the `g_*` option globals defined at the top of mem.c and `extern`'d by
// the other translation units live in `crate::options::FomaOptions` now (one
// value per `Session`, threaded by reference into the library) — nothing here.

// [spec:foma:def:mem.xxstrndup-fn]
// [spec:foma:sem:mem.xxstrndup-fn]
// [spec:foma:def:fomalibconf.xxstrndup-fn]
// [spec:foma:sem:fomalibconf.xxstrndup-fn]
pub fn xxstrndup(s: &str, n: usize) -> String {
    // C: p = s; while (*p++ && n--); n = p - s - 1; — the copied length is
    // min(n, strlen(s)), where strlen stops at an interior NUL.
    let bytes = s.as_bytes();
    let strlen = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let end = strlen.min(n);
    // DEVIATION from C (a cut inside a UTF-8 codepoint would yield an invalid
    // byte string in C; String must be valid UTF-8, so lossy-decode — every
    // foma call site cuts at symbol boundaries, where this is byte-identical).
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

// [spec:foma:def:mem.next-power-of-two-fn]
// [spec:foma:sem:mem.next-power-of-two-fn]
// [spec:foma:def:fomalibconf.next-power-of-two-fn]
// [spec:foma:sem:fomalibconf.next-power-of-two-fn]
pub fn next_power_of_two(v: i32) -> i32 {
    let mut v = v;
    let mut i: i32 = 0;
    // C: for (i=0; v > 0; i++) v = v >> 1;
    while v > 0 {
        v = v >> 1;
        i += 1;
    }
    // C: 1 << i overflows int for i == 31 (UB in C); Rust yields i32::MIN.
    1 << i
}

// [spec:foma:def:mem.round-up-to-power-of-two-fn]
// [spec:foma:sem:mem.round-up-to-power-of-two-fn]
// [spec:foma:def:fomalibconf.round-up-to-power-of-two-fn]
// [spec:foma:sem:fomalibconf.round-up-to-power-of-two-fn]
pub fn round_up_to_power_of_two(v: u32) -> u32 {
    // C v--/v++ wrap on unsigned: v = 0 smears to 0xFFFFFFFF and returns 0.
    let mut v = v;
    v = v.wrapping_sub(1);
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v = v.wrapping_add(1);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    // [spec:foma:sem:mem.xxstrndup-fn/test]
    // [spec:foma:sem:fomalibconf.xxstrndup-fn/test]
    #[test]
    fn xxstrndup_copies_min_n_strlen_bytes() {
        // Effective length = min(n, strlen(s)).
        assert_eq!(xxstrndup("hello", 3), "hel"); // n < strlen → cut to n
        assert_eq!(xxstrndup("hello", 5), "hello"); // n == strlen
        assert_eq!(xxstrndup("hello", 10), "hello"); // n > strlen → whole string
        // n == 0 or empty s yields an (allocated) empty string.
        assert_eq!(xxstrndup("hello", 0), "");
        assert_eq!(xxstrndup("", 5), "");
        assert_eq!(xxstrndup("", 0), "");
    }

    // [spec:foma:sem:mem.next-power-of-two-fn/test]
    // [spec:foma:sem:fomalibconf.next-power-of-two-fn/test]
    #[test]
    fn next_power_of_two_smallest_strictly_greater_power() {
        assert_eq!(next_power_of_two(7), 8);
        assert_eq!(next_power_of_two(8), 16); // exact power is doubled
        assert_eq!(next_power_of_two(32), 64);
        assert_eq!(next_power_of_two(32768), 65536);
        assert_eq!(next_power_of_two(1), 2);
        // v <= 0: the loop runs zero times → 1 << 0 == 1.
        assert_eq!(next_power_of_two(0), 1);
        assert_eq!(next_power_of_two(-5), 1);
        // Documented overflow: highest bit at position 30 → i == 31, and
        // `1 << 31` (UB in C) yields i32::MIN in Rust.
        assert_eq!(next_power_of_two(1 << 30), i32::MIN);
    }

    // [spec:foma:sem:mem.round-up-to-power-of-two-fn/test]
    // [spec:foma:sem:fomalibconf.round-up-to-power-of-two-fn/test]
    #[test]
    fn round_up_to_power_of_two_ceils_exact_powers_unchanged() {
        assert_eq!(round_up_to_power_of_two(3), 4);
        assert_eq!(round_up_to_power_of_two(7), 8);
        assert_eq!(round_up_to_power_of_two(8), 8); // exact power unchanged
        assert_eq!(round_up_to_power_of_two(1), 1);
        assert_eq!(round_up_to_power_of_two(32768), 32768);
        // Wrapping-unsigned quirks: v == 0 → 0, any v > 2^31 → 0.
        assert_eq!(round_up_to_power_of_two(0), 0);
        assert_eq!(round_up_to_power_of_two(0x8000_0001), 0);
    }
}

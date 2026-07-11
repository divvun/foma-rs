# foma/mem.c

> [spec:foma:def:mem.next-power-of-two-fn]
> int next_power_of_two(int v)

> [spec:foma:sem:mem.next-power-of-two-fn]
> Returns the smallest power of two strictly greater than `v`, for `v > 0`. Algorithm:
> count `i` = the number of right-shifts (`v >>= 1`) needed until `v` becomes 0
> (i.e. bit-length of v: for highest set bit at position k, i = k+1), then return
> `1 << i` = 2^(k+1). Consequences: an exact power of two is doubled
> (next_power_of_two(32768) == 65536); v=7 yields 8. For `v <= 0` the loop runs zero
> times and the result is `1 << 0` = 1. Pure function; note 2^(k+1) can overflow `int`
> for k == 30 (undefined in C). Used as the growth policy for the trie state array.

> [spec:foma:def:mem.round-up-to-power-of-two-fn]
> unsigned int round_up_to_power_of_two(unsigned int v)

> [spec:foma:sem:mem.round-up-to-power-of-two-fn]
> Returns the smallest power of two greater than or equal to `v` (unsigned 32-bit; an
> exact power of two is returned unchanged, unlike next_power_of_two). Classic bit-smear:
> `v--`; then OR `v` with itself shifted right by 1, 2, 4, 8, and 16 (setting every bit
> below the highest set bit); then `v++` and return.
> Edge cases (wrapping unsigned arithmetic): v = 0 returns 0 (0-1 smears to 0xFFFFFFFF,
> +1 wraps to 0); v = 1 returns 1; any v > 2^31 also wraps to 0. Pure function.


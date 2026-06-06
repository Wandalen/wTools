# Test Surface: Invariant — strategy Parameter Has No Effect

## Scope

- **In Scope**: Tests asserting that all accepted `strategy` values for `optimize_match!` produce identical output, and that default matches explicit `strategy = "first_match"`.
- **Out of Scope**: Strategy selection thresholds (see `002_match_strategy_thresholds.md`); `optimize_split!` (strategy parameter not applicable to split).
- **Status**: Active.

## Cases

### IN-1: All three strategy values produce identical output

- **Given**: The same source string and pattern.
- **When**: Comparing `optimize_match!( src, pat, strategy = "first_match" )`, `optimize_match!( src, pat, strategy = "longest_match" )`, and `optimize_match!( src, pat, strategy = "all_matches" )`.
- **Then**: All three return the same `Option<usize>` value. The `strategy` parameter is discarded by the macro and has no effect on generated code.
- **Status**: ✅

### IN-2: Default (no strategy) matches explicit `strategy = "first_match"`

- **Given**: The same source string and pattern.
- **When**: Comparing `optimize_match!( src, pat )` with `optimize_match!( src, pat, strategy = "first_match" )`.
- **Then**: Both return identical `Option<usize>` values. Omitting `strategy` is equivalent to any strategy value.
- **Status**: ✅

### Tests

| Case | Test Function | File |
|------|--------------|------|
| IN-1 | `tc_all_strategy_values_equivalent` | `optimize_match_tests.rs` |
| IN-2 | `tc8_default_value_equivalence` | `optimize_match_tests.rs` |

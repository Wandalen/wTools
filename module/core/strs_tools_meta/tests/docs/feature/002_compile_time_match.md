# Test Surface: Feature — Compile-Time Match

## Scope

- **In Scope**: Behavioral test cases for `optimize_match!` — single pattern, no-match, multiple patterns, strategy parameter, debug mode, empty input.
- **Out of Scope**: API contract cases (see `../api/002_optimize_match_api.md`); strategy threshold enforcement (see `../invariant/002_match_strategy_thresholds.md`); strategy no-op invariant (see `../invariant/003_strategy_param_no_op.md`).
- **Status**: Active.

## Cases

### FT-1: Single pattern returns byte position of first occurrence

- **Given**: A source string containing at least one occurrence of the pattern.
- **When**: `optimize_match!( src, "pattern" )` expands.
- **Then**: Returns `Some(n)` where `n` is the byte position of the first match. Non-overlapping occurrences after the first are not returned.
- **Status**: ✅

### FT-2: No match returns `None`

- **Given**: A source string that does not contain the pattern.
- **When**: `optimize_match!( src, "pattern" )` expands.
- **Then**: Returns `None`.
- **Status**: ✅

### FT-3: Multiple patterns return position of earliest match

- **Given**: A source string; two or more patterns supplied as an array.
- **When**: `optimize_match!( src, [ "p1", "p2" ] )` expands.
- **Then**: Returns `Some(n)` where `n` is the byte position of the first match across all patterns. Returns `None` if no pattern is found.
- **Status**: ✅

### FT-4: Strategy parameter does not alter output

- **Given**: The same source and pattern.
- **When**: Comparing `optimize_match!` with `strategy = "first_match"`, `strategy = "longest_match"`, and `strategy = "all_matches"`.
- **Then**: All three strategy values produce identical return values. The `strategy` parameter is accepted without compile error and is a no-op (see `invariant/003_strategy_param_no_op.md`).
- **Status**: ✅

### FT-5: `debug` flag emits diagnostics without altering return value

- **Given**: Any valid `optimize_match!` invocation with the `debug` flag appended.
- **When**: The macro expands at compile time.
- **Then**: Compile-time diagnostics are emitted (e.g. strategy selection). The return value is identical to the same invocation without `debug`.
- **Status**: ✅

### FT-6: Empty source string returns `None`

- **Given**: An empty source string `""`.
- **When**: `optimize_match!( "", "pattern" )` expands.
- **Then**: Returns `None`.
- **Status**: ✅

### Tests

| Case | Test Function | File |
|------|--------------|------|
| FT-1 | `tc1_single_pattern`, `corner_match_at_start` | `optimize_match_tests.rs`, `corner_cases_test.rs` |
| FT-2 | `tc9_no_match`, `corner_match_no_match` | `optimize_match_tests.rs`, `corner_cases_test.rs` |
| FT-3 | `tc2_multiple_small_patterns`, `corner_match_overlapping_patterns` | `optimize_match_tests.rs`, `corner_cases_test.rs` |
| FT-4 | `tc3_first_match_strategy`, `tc4_longest_match_strategy`, `tc5_all_matches_strategy`, `tc7_explicit_parameters`, `tc8_default_value_equivalence` | `optimize_match_tests.rs` |
| FT-5 | `tc6_debug_mode` | `optimize_match_tests.rs` |
| FT-6 | `tc10_empty_input`, `corner_match_empty_input` | `optimize_match_tests.rs`, `corner_cases_test.rs` |

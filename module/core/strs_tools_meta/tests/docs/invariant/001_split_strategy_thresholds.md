# Test Surface: Invariant — Split Strategy Selection Thresholds

## Scope

- **In Scope**: Threshold boundary tests for `optimize_split!` strategy selection — verifying that exactly 8 short delimiters selects `MultipleCharDelimiters` and 9 selects `ComplexPattern`.
- **Out of Scope**: Strategy output correctness (covered by feature tests); match strategy thresholds (see `002_match_strategy_thresholds.md`).
- **Status**: Active.

## Cases

### IN-1: Exactly 8 single-char delimiters selects `MultipleCharDelimiters`

- **Given**: Exactly 8 single-character delimiters (the threshold boundary).
- **When**: `optimize_split!` expands with `debug` flag revealing the chosen strategy.
- **Then**: Strategy `MultipleCharDelimiters` is selected. The split result is correct (segments are produced at each delimiter).
- **Status**: ✅

### IN-2: 9 single-char delimiters selects `ComplexPattern`

- **Given**: Exactly 9 single-character delimiters (one over the threshold boundary).
- **When**: `optimize_split!` expands with `debug` flag revealing the chosen strategy.
- **Then**: Strategy `ComplexPattern` is selected. The split result is correct despite the different code path.
- **Status**: ✅

### Tests

| Case | Test Function | File |
|------|--------------|------|
| IN-1 | `corner_many_delimiters_threshold` | `corner_cases_test.rs` |
| IN-2 | `corner_many_delimiters_over_threshold` | `corner_cases_test.rs` |

# Test Surface: Invariant — Match Strategy Selection Thresholds

## Scope

- **In Scope**: Threshold boundary tests for `optimize_match!` strategy selection — verifying that exactly 16 short patterns selects `TrieBasedMatch` and 17 selects `SequentialMatch`.
- **Out of Scope**: Strategy output correctness (covered by feature tests); split strategy thresholds (see `001_split_strategy_thresholds.md`).
- **Status**: Active.

## Cases

### IN-1: Exactly 16 patterns with `len ≤ 8` selects `TrieBasedMatch`

- **Given**: Exactly 16 patterns each no longer than 8 characters (the threshold boundary).
- **When**: `optimize_match!` expands with `debug` flag revealing the chosen strategy.
- **Then**: Strategy `TrieBasedMatch` is selected. The match result is correct (returns byte position of first match or `None`).
- **Status**: ✅

### IN-2: 17 patterns with `len ≤ 8` selects `SequentialMatch`

- **Given**: Exactly 17 patterns each no longer than 8 characters (one over the threshold boundary).
- **When**: `optimize_match!` expands with `debug` flag revealing the chosen strategy.
- **Then**: Strategy `SequentialMatch` is selected. The match result is correct (identical to `TrieBasedMatch` output for same input, since both currently generate identical code).
- **Status**: ✅

### Tests

| Case | Test Function | File |
|------|--------------|------|
| IN-1 | `corner_match_many_patterns_threshold` | `corner_cases_test.rs` |
| IN-2 | `corner_match_many_patterns_over_threshold` | `corner_cases_test.rs` |

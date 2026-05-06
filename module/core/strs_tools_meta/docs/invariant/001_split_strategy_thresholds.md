# Invariant: Split Strategy Selection Thresholds

## Scope

- **In Scope**: Threshold values and conditions that govern `optimize_split!` strategy selection at compile time.
- **Out of Scope**: Generated code correctness; runtime performance benchmarks.
- **Boundary**: Decision logic only; parameter semantics are in `api/001_optimize_split_api.md`.
- **Status**: Active.

## Statement

`optimize_split!` selects a split strategy at compile time based on delimiter count and maximum delimiter length using fixed thresholds:

| Condition | Strategy | Generated code path |
|-----------|----------|---------------------|
| 1 delimiter AND `len == 1` | `SingleCharDelimiter` | `str::split(char)` |
| `count ≤ 8` AND all `len ≤ 4` | `MultipleCharDelimiters` | Chained `find()` loop |
| Otherwise | `ComplexPattern` | Regex-based split |

These thresholds are evaluated at macro expansion time and cannot be overridden at runtime.

## Enforcement

Verified by threshold boundary tests in `tests/corner_cases_test.rs`:
- `corner_many_delimiters_threshold` — exactly 8 single-char delimiters → `MultipleCharDelimiters`
- `corner_many_delimiters_over_threshold` — 9 single-char delimiters → `ComplexPattern`

## Violation Consequences

A threshold change not reflected in tests causes silent strategy misclassification. Generated output is functionally correct (same split results) but the optimization path is wrong, degrading performance without any error or warning.

### Sources

| File | Notes |
|------|-------|
| `spec.md` (git `c13cf485~1`) | Original spec contained no threshold documentation. Thresholds derived from `src/lib.rs:analyze_split_pattern`. |
| `../../src/lib.rs` | Authoritative threshold implementation — `analyze_split_pattern()` |

### Cross-References

| Type | File | Notes |
|------|------|-------|
| source | `../../src/lib.rs` | `analyze_split_pattern`, `SplitOptimization` enum |
| test | `../../tests/corner_cases_test.rs` | `corner_many_delimiters_threshold`, `corner_many_delimiters_over_threshold` |
| doc | `../feature/001_compile_time_split.md` | Feature design context |
| doc | `../api/001_optimize_split_api.md` | Public interface |

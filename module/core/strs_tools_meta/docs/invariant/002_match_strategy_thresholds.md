# Invariant: Match Strategy Selection Thresholds

## Scope

- **In Scope**: Threshold values and conditions governing `optimize_match!` strategy selection at compile time.
- **Out of Scope**: Trie data structure content (currently unused); `strategy` parameter semantics (see `003_strategy_param_no_op.md`).
- **Boundary**: Decision logic only; does not cover the `strategy` parameter no-op behavior.
- **Status**: Active.

## Statement

`optimize_match!` selects a match strategy at compile time based on pattern count and maximum pattern length:

| Condition | Strategy | Generated code path |
|-----------|----------|---------------------|
| 1 pattern | `SinglePattern` | Single `str::find()` call |
| `count ≤ 16` AND all `len ≤ 8` | `TrieBasedMatch` | Linear `find()` loop |
| Otherwise | `SequentialMatch` | Linear `find()` loop |

`TrieBasedMatch` and `SequentialMatch` currently produce identical generated code. The distinction is reserved for a future implementation of true compile-time trie construction.

## Enforcement

Verified by threshold boundary tests in `tests/corner_cases_test.rs`:
- `corner_match_many_patterns_threshold` — exactly 16 short patterns → `TrieBasedMatch`
- `corner_match_many_patterns_over_threshold` — 17 short patterns → `SequentialMatch`

## Violation Consequences

Threshold misclassification is silent and functionally transparent until true trie generation is implemented. After implementation, threshold errors would cause wrong strategy application with incorrect output.

### Sources

| File | Notes |
|------|-------|
| `spec.md` (git `c13cf485~1`) | Original spec contained no threshold documentation. Thresholds derived from `src/lib.rs:analyze_match_pattern`. |
| `../../src/lib.rs` | Authoritative threshold implementation — `analyze_match_pattern()` |

### Cross-References

| Type | File | Notes |
|------|------|-------|
| source | `../../src/lib.rs` | `analyze_match_pattern`, `MatchOptimization` enum |
| test | `../../tests/corner_cases_test.rs` | `corner_match_many_patterns_threshold`, `corner_match_many_patterns_over_threshold` |
| doc | `../feature/002_compile_time_match.md` | Feature design context |
| doc | `../api/002_optimize_match_api.md` | Public interface |
| doc | `003_strategy_param_no_op.md` | Related: strategy parameter no-op |

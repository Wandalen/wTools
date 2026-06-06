# Invariant: optimize_match! strategy Parameter Has No Effect

## Scope

- **In Scope**: The `strategy` parameter of `optimize_match!` and its (absent) effect on generated code.
- **Out of Scope**: Strategy selection thresholds (see `002_match_strategy_thresholds.md`); API syntax (see `api/002_optimize_match_api.md`).
- **Boundary**: Documents a current implementation limitation, not intended permanent design.
- **Status**: Active.

## Statement

The `strategy` parameter accepted by `optimize_match!` has no effect on strategy selection or generated code. Strategy selection is determined solely by pattern count and length thresholds (see `002_match_strategy_thresholds.md`). The values `"first_match"`, `"longest_match"`, and `"all_matches"` all produce identical generated code.

The parameter is parsed, accepted without error, and discarded. The source represents this as `_strategy` (leading underscore indicating intentional non-use).

## Enforcement

No automated test currently asserts that all strategy values produce identical output (a coverage gap). The closest existing test is `optimize_match_tests.rs:tc8_default_value_equivalence`, which compares default vs. `strategy = "first_match"` but does not compare across strategy values.

To fully enforce this invariant, add a test asserting:
```rust
let a = optimize_match!( "test", "t", strategy = "first_match" );
let b = optimize_match!( "test", "t", strategy = "longest_match" );
let c = optimize_match!( "test", "t", strategy = "all_matches" );
assert_eq!( a, b );
assert_eq!( b, c );
```

## Violation Consequences

If a future change makes `strategy` effective, this invariant must be retired and all callers relying on `strategy` being a no-op must be audited. Callers passing `strategy = "longest_match"` expecting no-op behavior would silently receive changed results.

### Sources

| File | Relationship |
|------|-------------|
| `spec.md` (git `c13cf485~1`) | Original spec made no mention of `strategy` being a no-op — gap in original documentation. |
| [`../../tests/manual/readme.md`](../../tests/manual/readme.md) | §Known Behavior item 3: "Strategy parameter simplified" — informal predecessor of this invariant. |
| [`../../src/lib.rs`](../../src/lib.rs) | `analyze_match_pattern`, `_strategy` parameter |

### Tests

| File | Relationship |
|------|-------------|
| [`../../tests/optimize_match_tests.rs`](../../tests/optimize_match_tests.rs) | `tc8_default_value_equivalence` (partial coverage) |

### Features

| File | Relationship |
|------|-------------|
| [`../feature/002_compile_time_match.md`](../feature/002_compile_time_match.md) | Feature design |

### APIs

| File | Relationship |
|------|-------------|
| [`../api/002_optimize_match_api.md`](../api/002_optimize_match_api.md) | API interface |

### Invariants

| File | Relationship |
|------|-------------|
| [`002_match_strategy_thresholds.md`](002_match_strategy_thresholds.md) | Related: threshold selection |

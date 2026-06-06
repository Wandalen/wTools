# tests/docs/invariant

Invariant test surface specs for `optimize_split!` and `optimize_match!` macros.

## Scope

- **In Scope**: Enforcement tests for compile-time invariants — strategy selection thresholds and strategy parameter no-op behavior.
- **Out of Scope**: Feature behavior (see `../feature/`); API contracts (see `../api/`).
- **Status**: Active.

## Responsibility Table

| File | Responsibility | Status |
|------|----------------|--------|
| `001_split_strategy_thresholds.md` | IN cases for split strategy selection thresholds | ✅ |
| `002_match_strategy_thresholds.md` | IN cases for match strategy selection thresholds | ✅ |
| `003_strategy_param_no_op.md` | IN cases for strategy parameter no-op invariant | ✅ |

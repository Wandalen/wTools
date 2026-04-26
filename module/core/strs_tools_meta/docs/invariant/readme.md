# Invariant Doc Entity

## Scope

- **In Scope**: Compile-time invariants for optimization strategy selection; behavioral contracts that must hold across all macro expansions.
- **Out of Scope**: Runtime performance guarantees; generated code correctness proofs.
- **Boundary**: Invariants express constraints on internal macro strategy selection; API-level contracts are in `api/`.
- **Status**: Active.

### Overview Table

| # | File | Responsibility |
|---|------|----------------|
| 1 | `001_split_strategy_thresholds.md` | Split optimization strategy selection thresholds |
| 2 | `002_match_strategy_thresholds.md` | Match optimization strategy selection thresholds |
| 3 | `003_strategy_param_no_op.md` | `optimize_match!` strategy parameter has no effect |

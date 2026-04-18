# Invariant: Column Fold Invariants

### Scope

- **Purpose**: Define the behavioral guarantees that column folding must maintain across all rendering scenarios.
- **Responsibility**: Documents three invariants: header non-folding, CSV/TSV bypass, and fold point determinism.
- **In Scope**: Header row guarantee, data format bypass, fold determinism given identical input and config.
- **Out of Scope**: Budget allocation algorithm (see `algorithm/004_budget_allocation.md`), fold rendering details (see `algorithm/005_column_fold_detection.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/table.rs` | `determine_fold_point`, `should_auto_fold`, header rendering guard |
| test | `tests/auto_fold_test.rs` | T08, T09, T19 |
| doc | `../feature/005_auto_fit.md` | Auto-fit Strategy 1 description |
| task | `../../task/completed/020_column_folding_with_auto_fold.md` | Implementation task (completed) |

### Invariant 1 — Header Row Never Folds

The header row always renders all column names inline, regardless of terminal width or fold configuration. Only data rows produce continuation lines.

**Rationale**: Folded headers would make the table unreadable — the user needs to see all column names to understand which columns are primary and which are folded.

**Enforcement**: The fold rendering path applies only to data rows; the header rendering path is unconditional.

**Test**: T19 in `auto_fold_test.rs` — header row renders all columns in any fold scenario.

### Invariant 2 — CSV/TSV Never Fold

When the column separator is `,` (CSV) or `\t` (TSV), column folding is disabled regardless of `auto_fold` setting and terminal width.

**Rationale**: CSV and TSV are machine-readable data interchange formats. Folding columns to continuation lines would produce malformed data that parsers cannot interpret.

**Enforcement**: `auto_fold` check includes a CSV/TSV bypass (same guard as `auto_wrap`).

**Test**: T08 and T09 in `auto_fold_test.rs` — CSV and TSV presets never produce continuation lines.

### Invariant 3 — Fold Point Determinism

Given identical input data and `TableConfig`, the fold point (the column index where folding begins) is deterministic — always the same column index for the same input.

**Rationale**: Non-deterministic fold points would make table output unpredictable, breaking snapshot tests and visual expectations.

**Enforcement**: The fold point algorithm is a pure function of column widths and terminal width — no randomness, no runtime-dependent ordering.

**Test**: T11 in `auto_fold_test.rs` — multiple rows with same data produce consistent fold point.

### Violation Consequences

| Invariant | Consequence of Violation |
|-----------|------------------------|
| Header non-folding | Table becomes unreadable; column labels disappear |
| CSV/TSV bypass | Machine-readable data becomes unparseable; downstream pipeline failure |
| Fold determinism | Snapshot tests become flaky; CI failures on identical code |

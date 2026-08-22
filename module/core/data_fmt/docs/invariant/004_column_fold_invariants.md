# Invariant: Column Fold Invariants

### Scope

- **Purpose**: Define the behavioral guarantees that column folding must maintain across all rendering scenarios.
- **Responsibility**: Documents three invariants: header row never renders empty, CSV/TSV bypass, and fold point determinism.
- **In Scope**: Header row guarantee, data format bypass, fold determinism given identical input and config.
- **Out of Scope**: Budget allocation algorithm (see `algorithm/004_budget_allocation.md`), fold rendering details (see `algorithm/005_column_fold_detection.md`).

### Features

| File | Relationship |
|------|-------------|
| [005_auto_fit.md](../feature/005_auto_fit.md) | Auto-fit Strategy 1 description |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/table/auto_fit.rs`](../../src/formatters/table/auto_fit.rs) | `determine_fold_point`, `should_auto_fold` |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | header row sliced by `fold_point` (`primary_headers`), same as data rows |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/auto_fold_test.rs`](../../tests/auto_fold_test.rs) | T08, T09, T19 |

### Invariant Statement

#### Invariant 1 — Header Row Never Renders Empty

The header row is sliced by the same `fold_point` as data rows: it renders only the primary columns (`0..fold_point`) inline, and folded columns' names do not appear in the header line — they surface only as per-row labels in continuation lines. The guarantee is a floor, not full display: `determine_fold_point` always returns at least 1, so the header row can never render as empty (zero column names), even when the first column alone exceeds the terminal width. Only data rows produce continuation lines — the header row itself never does.

**Rationale**: A header row with zero columns would leave the table with no readable structure at all. Full inline display of every column name regardless of width is not guaranteed by this invariant — that is what the fold continuation lines exist to convey instead.

**Test**: T19 in `auto_fold_test.rs` verifies the primary (non-folded) columns render inline in the header; T24 (`fold_point_zero_preserves_first_column_in_header`) verifies the never-empty floor when even the first column exceeds the terminal width.

#### Invariant 2 — CSV/TSV Never Fold

When the column separator is `,` (CSV) or `\t` (TSV), column folding is disabled regardless of `auto_fold` setting and terminal width.

**Rationale**: CSV and TSV are machine-readable data interchange formats. Folding columns to continuation lines would produce malformed data that parsers cannot interpret.

**Test**: T08 and T09 in `auto_fold_test.rs` — CSV and TSV presets never produce continuation lines.

#### Invariant 3 — Fold Point Determinism

Given identical input data and `TableConfig`, the fold point (the column index where folding begins) is deterministic — always the same column index for the same input.

**Rationale**: Non-deterministic fold points would make table output unpredictable, breaking snapshot tests and visual expectations.

**Test**: T11 in `auto_fold_test.rs` — multiple rows with same data produce consistent fold point.

### Enforcement Mechanism

Invariant 1 is enforced by `determine_fold_point`'s `i.max(1)` clamp (Fix BUG-007), which guarantees `fold_point >= 1` whenever at least one column exists, so `primary_headers = &headers[..fold_point]` is never empty; the header rendering path is sliced by `fold_point` exactly like data rows and simply never calls the continuation-line renderer for itself. Invariant 2 is enforced by the `should_auto_fold` guard, which checks the column separator before enabling fold logic — the same guard as `auto_wrap`. Invariant 3 is enforced by `determine_fold_point`, a pure function of column widths and terminal width with no randomness or runtime-dependent ordering.

### Violation Consequences

| Invariant | Consequence of Violation |
|-----------|------------------------|
| Header non-empty floor | Header row could render with zero columns; table loses all readable column labels |
| CSV/TSV bypass | Machine-readable data becomes unparseable; downstream pipeline failure |
| Fold determinism | Snapshot tests become flaky; CI failures on identical code |

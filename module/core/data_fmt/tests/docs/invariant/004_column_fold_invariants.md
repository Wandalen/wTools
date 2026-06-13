# Invariant: Column Fold Invariants

### Scope

- **Purpose**: Drive test coverage for the column fold invariants.
- **Responsibility**: Documents test cases for column fold invariants (determinism, consistency, header protection, format bypass) in `docs/invariant/004_column_fold_invariants.md`.
- **In Scope**: Header never folds, CSV/TSV format bypass, fold-point determinism across repeated calls, consistent fold point across identical rows.
- **Out of Scope**: Fold detection algorithm details (see `algorithm/005`); auto-fit feature behavior (see `feature/005`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| IN-1 | header row never folds regardless of data row folding | ✅ |
| IN-2 | CSV format never folds regardless of auto_fold setting | ✅ |
| IN-3 | TSV format never folds regardless of auto_fold setting | ✅ |
| IN-4 | fold point deterministic given identical input data and config | ✅ |
| IN-5 | multiple rows with identical data fold at the same column index | ✅ |

---

### IN-1: header row never folds regardless of data row folding

- **Given:** A table configured so that data rows fold (total width exceeds
  terminal); `auto_fold=true`; at least one data row produces continuation lines.
- **When:** Rendered and the header row output line is inspected.
- **Then:** The header row contains all column names on a single line; no column
  name appears in a continuation line; no header column is missing from the primary
  table row.
- **Note:** Covered by T19 (`header_row_never_folds`) in `tests/auto_fold_test.rs`.

---

### IN-2: CSV format never folds regardless of auto_fold setting

- **Given:** A table using `TableConfig::csv()` whose content would exceed the
  terminal width; `auto_fold=true` is set explicitly.
- **When:** Rendered.
- **Then:** No continuation lines are present in the output; every row occupies
  exactly one output line; the output is well-formed comma-separated values.
- **Note:** Covered by T08 in `tests/auto_fold_test.rs`.

---

### IN-3: TSV format never folds regardless of auto_fold setting

- **Given:** A table using `TableConfig::tsv()` whose content would exceed the
  terminal width; `auto_fold=true` is set explicitly.
- **When:** Rendered.
- **Then:** No continuation lines are present in the output; every row occupies
  exactly one output line; the output is well-formed tab-separated values.
- **Note:** Covered by T09 in `tests/auto_fold_test.rs`.

---

### IN-4: fold point deterministic given identical input data and config

- **Given:** A `TableFormatter` configured with fold enabled (`terminal_width` set,
  all columns `ColumnFlex::Fixed`).
- **When:** `format()` is called twice with identical table arguments.
- **Then:** Both calls produce byte-equal strings; no internal mutable state
  accumulates between calls; fold point, continuation labels, and column widths
  are identical across both outputs.
- **Note:** Covered by `fold_output_is_idempotent_on_repeated_calls` (T25) in
  `tests/auto_fold_test.rs`.

---

### IN-5: multiple rows with identical data fold at the same column index

- **Given:** A `TableFormatter` with fold configured (multi-column table,
  `terminal_width` set, all columns `ColumnFlex::Fixed`); a table containing 3 or
  more rows each with identical cell content.
- **When:** Rendered with `auto_fold=true`.
- **Then:** Every data row folds at the same column index as every other data row;
  no row folds earlier or later than its siblings; continuation line column labels
  are consistent across all rows; no per-row mutable state persists between rows.
- **Note:** Covered by T11 (`multiple_rows_same_data_consistent_fold_point`) in
  `tests/auto_fold_test.rs`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/invariant/004_column_fold_invariants.md`](../../../docs/invariant/004_column_fold_invariants.md) | Source invariant spec — header protection, format bypass, determinism |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/auto_fold_test.rs`](../../auto_fold_test.rs) | Invariant test implementation (T08, T09, T11, T19) |

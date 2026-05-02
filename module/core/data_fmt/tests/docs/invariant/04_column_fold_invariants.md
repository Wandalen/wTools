# Invariant Spec: Column Fold Invariants

## Source
`docs/invariant/004_column_fold_invariants.md`

## Test Implementation
`tests/auto_fold_test.rs` (T08, T09, T11, T19)

## Case Index

| ID | Name | Status |
|----|------|--------|
| IC-1 | header row never folds regardless of data row folding | ✅ |
| IC-2 | CSV format never folds regardless of auto_fold setting | ✅ |
| IC-3 | TSV format never folds regardless of auto_fold setting | ✅ |
| IC-4 | fold output is idempotent across repeated format() calls | ✅ |

---

### IC-1: header row never folds regardless of data row folding

**Given:** A table configured so that data rows fold (total width exceeds
terminal); `auto_fold=true`; at least one data row produces continuation lines.
**When:** Rendered and the header row output line is inspected.
**Then:** The header row contains all column names on a single line; no column
name appears in a continuation line; no header column is missing from the primary
table row.
**Note:** Covered by T19 (`header_row_never_folds`) in `tests/auto_fold_test.rs`.

---

### IC-2: CSV format never folds regardless of auto_fold setting

**Given:** A table using `TableConfig::csv()` whose content would exceed the
terminal width; `auto_fold=true` is set explicitly.
**When:** Rendered.
**Then:** No continuation lines are present in the output; every row occupies
exactly one output line; the output is well-formed comma-separated values.
**Note:** Covered by T08 in `tests/auto_fold_test.rs`.

---

### IC-3: TSV format never folds regardless of auto_fold setting

**Given:** A table using `TableConfig::tsv()` whose content would exceed the
terminal width; `auto_fold=true` is set explicitly.
**When:** Rendered.
**Then:** No continuation lines are present in the output; every row occupies
exactly one output line; the output is well-formed tab-separated values.
**Note:** Covered by T09 in `tests/auto_fold_test.rs`.

---

### IC-4: fold output is idempotent across repeated format() calls

**Given:** A `TableFormatter` configured with `fold_config()` (6-column table,
`terminal_width=40`, all `ColumnFlex::Fixed`); the same `TreeNode` data passed
to both calls.
**When:** `formatter.format(&tree)` is called twice in succession with identical
arguments.
**Then:** Both return values are byte-equal strings; no mutable state in
`TableFormatter` affects the output between calls; fold point, continuation
labels, and column widths are all deterministic.

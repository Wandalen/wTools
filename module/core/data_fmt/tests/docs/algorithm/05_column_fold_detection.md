# Algorithm Spec: Column Fold Detection

## Source
`docs/algorithm/005_column_fold_detection.md`

## Test Implementation
`tests/auto_fold_test.rs` (T01–T02, T08–T09, T24)

## Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | all columns fit — no fold | ✅ |
| AC-2 | rightmost overflowing column triggers fold | ✅ |
| AC-3 | fold_point=0 clamped to 1 — first column always stays primary | ✅ |
| AC-4 | CSV preset bypasses fold detection | ✅ |
| AC-5 | TSV preset bypasses fold detection | ✅ |

---

### AC-1: all columns fit — no fold

- **Given:** A table with 3 columns whose total natural width fits within the
  configured terminal width.
- **When:** Rendered with `auto_fold=true`.
- **Then:** No continuation lines are emitted; every data row occupies exactly one
  physical line; `determine_fold_point` returns `None`.

---

### AC-2: rightmost overflowing column triggers fold

- **Given:** A 6-column table whose total width exceeds `terminal_width=40`; the
  4th column (`Path`) is the first to cause overflow.
- **When:** Rendered with `auto_fold=true` and all columns `ColumnFlex::Fixed`.
- **Then:** Columns 0–2 appear in the primary table row; columns 3–5 appear in
  labeled continuation lines below each data row; the fold point equals column
  index 3.

---

### AC-3: fold_point=0 clamped to 1 — first column always stays primary

- **Given:** A table with column 0 width of 18 characters (`"VeryLongColumnName"`);
  `terminal_width=3` (narrower than the first column).
- **When:** Rendered with `auto_fold=true` and `ColumnFlex::Fixed` on all columns.
- **Then:** `determine_fold_point` returns 1 (not 0) due to `.max(1)` clamp; column
  0 appears in the primary header; all other columns appear in continuation lines;
  no panic and no empty header row.
- **Note:** Covered by `bug_reproducer` issue-fold-point-zero.

---

### AC-4: CSV preset bypasses fold detection

- **Given:** A CSV table whose total width exceeds the terminal width; `auto_fold`
  is `true`.
- **When:** Rendered with `TableConfig::csv()`.
- **Then:** No continuation lines are produced; the output is well-formed CSV; fold
  detection is skipped entirely for CSV presets.

---

### AC-5: TSV preset bypasses fold detection

- **Given:** A TSV table whose total width exceeds the terminal width; `auto_fold`
  is `true`.
- **When:** Rendered with `TableConfig::tsv()`.
- **Then:** No continuation lines are produced; the output is well-formed TSV; fold
  detection is skipped entirely for TSV presets.

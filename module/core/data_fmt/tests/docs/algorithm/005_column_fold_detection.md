# Algorithm: Column Fold Detection

### Scope

- **Purpose**: Drive test coverage for the column fold detection algorithm.
- **Responsibility**: Documents test cases for the column fold detection algorithm in `docs/algorithm/005_column_fold_detection.md`.
- **In Scope**: Fold point calculation, fold_point=0 clamp, CSV/TSV bypass, FoldStyle::Bare, FoldStyle::Stacked, per-row computation, single overflow column, all-columns-overflow edge case.
- **Out of Scope**: Budget allocation (see `algorithm/004`); fold invariants (see `invariant/004`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | all columns fit — no fold | ✅ |
| AC-2 | rightmost overflowing column triggers fold | ✅ |
| AC-3 | fold_point=0 clamped to 1 — first column always stays primary | ✅ |
| AC-4 | CSV preset bypasses fold detection | ✅ |
| AC-5 | TSV preset bypasses fold detection | ✅ |
| AC-6 | FoldStyle::Bare joins all overflow values on single continuation line | ✅ |
| AC-7 | fold point computed per-row; short rows have no continuation lines | ✅ |
| AC-8 | single overflow column produces exactly one continuation line | ✅ |
| AC-9 | FoldStyle::Stacked emits one labeled continuation line per overflow column | ✅ |
| AC-10 | all columns overflow — first column remains in primary, rest in continuation | ✅ |
| AC-11 | very narrow terminal — fold at column 1 even with many remaining columns | ✅ |

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
- **Note:** Covered by `bug_reproducer` BUG-007.

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

---

### AC-6: FoldStyle::Bare joins all overflow values on single continuation line

- **Given:** A table configured with `FoldStyle::Bare` instead of the default labeled
  style; one data row has 3 overflow columns with values `"val_d"`, `"val_e"`, `"val_f"`.
- **When:** Rendered with `auto_fold=true`.
- **Then:** All overflow column values appear on a single continuation line (not one
  labeled line per column); no column labels are emitted for the overflow section;
  the bare continuation line contains all overflow values joined by the configured
  separator.

---

### AC-7: fold point computed per-row; short rows have no continuation lines

- **Given:** A table where some rows have long cell values causing overflow and other
  rows have short values fitting within the terminal width; `auto_fold=true`.
- **When:** Rendered.
- **Then:** Long rows produce continuation lines; short rows do not; the fold point
  is evaluated independently for each row based on that row's actual cell widths;
  no continuation lines are emitted for rows whose total rendered width fits within
  the terminal.

---

### AC-8: single overflow column produces exactly one continuation line

- **Given:** A 4-column table where columns 0–2 fit within `terminal_width` and
  column 3 causes overflow; all cells have one line of content; `auto_fold=true`.
- **When:** Rendered.
- **Then:** Each data row produces exactly one continuation line containing the label
  and value of column 3; no extra blank continuation lines are emitted; the primary
  table row contains exactly columns 0–2.

---

### AC-9: FoldStyle::Stacked emits one labeled continuation line per overflow column

- **Given:** A table configured with `FoldStyle::Stacked`; one data row has 2
  overflow columns (e.g. `"ColC"` and `"ColD"`).
- **When:** Rendered with `auto_fold=true`.
- **Then:** Exactly two continuation lines are emitted — one per overflow column;
  each continuation line contains the column label and its value inline on the same
  line (not on separate lines); `"ColC"` and `"ColD"` each appear as a label on
  their respective continuation line; column order is preserved.
- **Note:** `FoldStyle::Stacked` separates overflow columns onto individual lines
  (one line per column) rather than joining all overflow on a single line
  (`FoldStyle::Bare`) or emitting label+value on consecutive separate lines.

---

### AC-10: all columns overflow — first column remains in primary, rest in continuation

- **Given:** A table where even column 0 alone exceeds `terminal_width`; `auto_fold=true`;
  multiple data columns.
- **When:** Rendered.
- **Then:** Column 0 appears in the primary row (enforced by `.max(1)` clamp);
  all remaining columns appear in continuation lines; no panic occurs; header row
  shows only column 0.

---

### AC-11: very narrow terminal — fold at column 1 even with many remaining columns

- **Given:** A 5-column table; `terminal_width` set to a value narrower than any
  single column; `auto_fold=true`; all columns `ColumnFlex::Fixed`.
- **When:** Rendered.
- **Then:** Fold point is 1 (clamped); column 0 is in the primary row; columns 1–4
  are all in continuation lines; the number of continuation lines per row equals 4
  (or 1 for Bare style); no output truncation or panic.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/005_column_fold_detection.md`](../../../docs/algorithm/005_column_fold_detection.md) | Source algorithm spec — fold point calculation, FoldStyle variants |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/auto_fold_test.rs`](../../auto_fold_test.rs) | Algorithm test implementation (T01–T02, T08–T09, T24) |

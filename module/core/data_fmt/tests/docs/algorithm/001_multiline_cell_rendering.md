# Algorithm: Multiline Cell Rendering

### Scope

- **Purpose**: Drive test coverage for the multiline cell rendering algorithm.
- **Responsibility**: Documents test cases for the multiline cell rendering algorithm in `docs/algorithm/001_multiline_cell_rendering.md`.
- **In Scope**: Cell height calculation, sub-line splitting on `\n`, padding to row height, border emission per sub-line, ANSI preservation across sub-lines, CSV/TSV escape behavior, sub-row detail ordering, whitespace-only sub-lines.
- **Out of Scope**: Word-wrap budget allocation (see `algorithm/004`); column fold detection (see `algorithm/005`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | single-line cell unchanged | ✅ |
| AC-2 | embedded newline splits into sub-lines | ✅ |
| AC-3 | row height equals max cell height | ✅ |
| AC-4 | short cells padded to row height | ✅ |
| AC-5 | border pipes emitted on every sub-line | ✅ |
| AC-6 | ANSI codes preserved across sub-lines | ✅ |
| AC-7 | CSV/TSV format escapes newlines instead of rendering sub-lines | ✅ |
| AC-8 | sub-row detail lines appear after all multiline sub-lines | ✅ |
| AC-9 | truncation marker applied to last sub-line when cell truncated | ✅ |
| AC-10 | three or more embedded newlines produce correct sub-line count | ✅ |
| AC-11 | whitespace-only sub-lines are preserved as non-empty padding lines | ✅ |

---

### AC-1: single-line cell unchanged

- **Given:** A table with one row where all cells contain plain text with no `\n` characters.
- **When:** The row is rendered with `TableFormatter`.
- **Then:** Each cell produces exactly one physical output line; output matches the
  non-multiline rendering path byte-for-byte; no extra blank lines are emitted.
- **Note:** Covered by `single_line_cells_unchanged` in `tests/multiline_cells.rs`
  (default config output == plain config output; data lines = 1).

---

### AC-2: embedded newline splits into sub-lines

- **Given:** A table with one row; one cell contains `"first\nsecond"`.
- **When:** The row is rendered with `TableFormatter`.
- **Then:** The cell produces two physical sub-lines containing `"first"` and
  `"second"` respectively; adjacent single-line cells are padded to the same height;
  column separators appear on both sub-lines.

---

### AC-3: row height equals max cell height

- **Given:** A three-column row where the cells have 1, 3, and 2 logical lines
  respectively (second cell = `"a\nb\nc"`).
- **When:** The row is rendered.
- **Then:** Exactly 3 physical output lines are emitted for that row (height = max
  of 1, 3, 2 = 3); no more, no fewer.

---

### AC-4: short cells padded to row height

- **Given:** A row where cell A has 1 line (`"x"`) and cell B has 3 lines
  (`"a\nb\nc"`).
- **When:** The row is rendered.
- **Then:** Cell A produces 2 additional blank padding lines so that all 3 physical
  output lines maintain consistent column width and visual alignment.

---

### AC-5: border pipes emitted on every sub-line

- **Given:** A row with a multiline cell rendered using a bordered style
  (e.g. `TableConfig::markdown()` or `TableConfig::unicode_box()`).
- **When:** All physical sub-lines of the row are inspected.
- **Then:** Every sub-line contains the left and right column separator characters;
  no sub-line is missing its borders.

---

### AC-6: ANSI codes preserved across sub-lines

- **Given:** A cell containing `"\x1b[31mRed\x1b[0m\nNormal"` (ANSI red on first
  line, plain text on second line).
- **When:** The cell is rendered in a table.
- **Then:** Sub-line 0 contains the ANSI red escape sequence and its reset code;
  sub-line 1 contains `"Normal"` without any injected escape sequences; no codes
  are stripped or corrupted.

---

### AC-7: CSV/TSV format escapes newlines instead of rendering sub-lines

- **Given:** A table using `TableConfig::csv()` where one cell contains
  `"first\nsecond"`.
- **When:** The row is rendered with `TableFormatter`.
- **Then:** The CSV output does not split the cell into multiple physical lines;
  the newline is represented as a literal `\n` escape sequence within the field;
  the row occupies exactly one output line. (Source: Trigger Condition —
  `docs/algorithm/001_multiline_cell_rendering.md`: "not CSV/TSV (data formats
  escape `\n` instead)".)

---

### AC-8: sub-row detail lines appear after all multiline sub-lines

- **Given:** A table row where a cell contains `"line1\nline2"` and the row
  also has a sub-row detail annotation.
- **When:** The row is rendered with `TableFormatter`.
- **Then:** Both cell sub-lines (`"line1"` and `"line2"`) appear first; the
  sub-row detail line appears below both sub-lines; the detail is not interleaved
  between sub-lines. (Source: Interaction table — `docs/algorithm/001_multiline_cell_rendering.md`.)

---

### AC-9: truncation marker applied to last sub-line when cell truncated

- **Given:** A cell containing `"line1\nline2\nline3"` in a column with
  `max_column_width` set to a value that triggers truncation.
- **When:** The cell is rendered with `TableFormatter`.
- **Then:** The truncation marker (`"..."` by default) appears on the last kept
  sub-line; no additional sub-lines are emitted beyond the truncation point; earlier
  sub-lines are rendered without the marker.

---

### AC-10: three or more embedded newlines produce correct sub-line count

- **Given:** A cell containing `"a\nb\nc\nd"` (three embedded `\n` characters,
  four logical lines).
- **When:** The row is rendered with `TableFormatter`.
- **Then:** Exactly 4 physical sub-lines are produced for that cell; the row
  height equals 4; no sub-line is dropped or duplicated.

---

### AC-11: whitespace-only sub-lines are preserved as non-empty padding lines

- **Given:** A cell containing `"first\n   \nsecond"` where the middle segment is
  three spaces (whitespace only, not an empty string).
- **When:** The row is rendered with `TableFormatter`.
- **Then:** The whitespace-only middle segment is preserved as-is (three-space string);
  it is NOT collapsed to an empty line or stripped; the column separator appears on
  that sub-line; adjacent cells produce their correct padding line for that sub-line.
- **Note:** Distinguishes the whitespace-only case from the empty-string case
  (`"first\n\nsecond"`) and from padding lines produced by the height-equalization
  algorithm.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/001_multiline_cell_rendering.md`](../../../docs/algorithm/001_multiline_cell_rendering.md) | Source algorithm spec — trigger condition, interaction table, algorithm steps |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/multiline_cells.rs`](../../multiline_cells.rs) | Algorithm test implementation |

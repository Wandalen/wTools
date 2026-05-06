# Algorithm Spec: Multiline Cell Rendering

## Source
`docs/algorithm/001_multiline_cell_rendering.md`

## Test Implementation
`tests/multiline_cells.rs`

## Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | single-line cell unchanged | ✅ |
| AC-2 | embedded newline splits into sub-lines | ✅ |
| AC-3 | row height equals max cell height | ✅ |
| AC-4 | short cells padded to row height | ✅ |
| AC-5 | border pipes emitted on every sub-line | ✅ |
| AC-6 | ANSI codes preserved across sub-lines | ✅ |

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

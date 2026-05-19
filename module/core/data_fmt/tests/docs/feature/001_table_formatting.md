# Feature: Table Formatting

### Scope

- **Purpose**: Drive test coverage for the table formatting feature.
- **Responsibility**: Documents test cases for the nine-preset table formatting feature in `docs/feature/001_table_formatting.md`.
- **In Scope**: Nine preset constructors, multiline cell activation, column truncation with marker, ANSI header coloring, alternating row colors, sub-row detail lines, min_column_width floor behavior.
- **Out of Scope**: Auto-fit (see `feature/005`); color themes (see `feature/004`); word wrapping algorithm (see `algorithm/002`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FT-1 | nine preset constructors produce distinct valid output | ✅ |
| FT-2 | multiline cell rendering activates on embedded newline | ✅ |
| FT-3 | column truncation limits cell width and appends marker | ✅ |
| FT-4 | ANSI header coloring wraps header row with color+reset | ✅ |
| FT-5 | alternating row colors apply correct codes to each row | ✅ |
| FT-6 | sub-row detail lines appear after all row content lines | ✅ |
| FT-7 | min_column_width raises column width to configured floor | ✅ |

---

### FT-1: nine preset constructors produce distinct valid output

- **Given:** The nine `TableConfig` preset constructors: `plain()`, `minimal()`, `bordered()`, `markdown()`, `grid()`, `unicode_box()`, `csv()`, `tsv()`, `compact()`.
- **When:** Each is used to render the same two-row table with three columns.
- **Then:** Each produces non-empty output; no two presets produce identical output; each output contains the expected header and data values.

---

### FT-2: multiline cell rendering activates on embedded newline

- **Given:** A table row where one cell contains `"line1\nline2"`.
- **When:** The row is rendered with `TableFormatter`.
- **Then:** The output splits into two physical sub-lines for that cell; adjacent single-line cells are padded to match the row height; see [`algorithm/001`](../algorithm/001_multiline_cell_rendering.md) for full algorithm spec.

---

### FT-3: column truncation limits cell width and appends marker

- **Given:** A `TableConfig` with `max_column_width(20)` and default truncation marker `"..."`.
- **When:** A cell containing a 30-character string is rendered.
- **Then:** The rendered cell is 20 characters wide; the last three characters of the cell are `"..."`, and the preceding 17 characters are the start of the original string.

---

### FT-4: ANSI header coloring wraps header row with color+reset

- **Given:** A `TableConfig` with `colorize_header(true)` and a header color ANSI code.
- **When:** The table is rendered.
- **Then:** The header row output contains the ANSI color code before cell content and `\x1b[0m` after cell content on the same physical line; data rows do not contain the header color code.

---

### FT-5: alternating row colors apply correct codes to each row

- **Given:** A `TableConfig` with `alternating_rows(true)` and two alternating color codes.
- **When:** A table with three data rows is rendered.
- **Then:** Row 0 uses color 0, row 1 uses color 1, row 2 uses color 0; each colored line ends with `\x1b[0m` before its trailing newline; no color bleeds across row boundaries.

---

### FT-6: sub-row detail lines appear after all row content lines

- **Given:** A table row added via `add_row_with_detail` with a non-empty detail annotation.
- **When:** The row is rendered.
- **Then:** All cell content lines appear first; the detail line appears below all cell content; the detail line is indented by `sub_row_indent` (default 2 spaces); the detail does not affect column width calculation.

---

### FT-7: min_column_width raises column width to configured floor

- **Given:** A table where a column's natural content width is 3 characters;
  `TableConfig` configured with `min_column_width(10)`.
- **When:** The table is rendered.
- **Then:** The column is at least 10 characters wide in the output; the cell value
  is padded with spaces to the minimum width; column separators appear at the
  correct position respecting the floor.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/feature/001_table_formatting.md`](../../../docs/feature/001_table_formatting.md) | Source feature spec — nine presets, truncation, colors, sub-rows |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../table_styles_presets.rs) | Preset rendering test cases |
| [`tests/table_rendering_borders.rs`](../../table_rendering_borders.rs) | Border and multiline rendering tests |
| [`tests/sub_row_test.rs`](../../sub_row_test.rs) | Sub-row detail test cases |
| [`tests/column_truncation.rs`](../../column_truncation.rs) | Column truncation test cases |

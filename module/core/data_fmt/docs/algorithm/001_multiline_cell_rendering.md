# Algorithm: Multiline Cell Rendering

### Scope

- **Purpose**: Render table rows where multiline cells produce multiple physical output lines per logical row with correct alignment and borders.
- **Responsibility**: Documents the two-pass multiline cell rendering algorithm and its interaction with other table features.
- **In Scope**: Cell splitting, row height measurement, sub-line rendering with padding and borders.
- **Out of Scope**: Word wrapping within cells (see `algorithm/002_word_wrapping.md`).

### Sources

| File | Relationship |
|------|--------------|
| `src/formatters/table/mod.rs` | TableFormatter multiline cell rendering |

### Tests

| File | Relationship |
|------|--------------|
| `tests/multiline_cells.rs` | Multiline cell rendering test suite |

### Abstract

A two-pass algorithm for rendering table rows that contain newline characters. Pass 1 measures the maximum line count across all cells in the row to determine row height. Pass 2 emits one physical output line per sub-line index, padding shorter cells with empty space to maintain column alignment and border continuity across all sub-lines.

### Source Location

`src/formatters/table/mod.rs` — `format_multiline_row()` (lines 510-613), called from `format_row()` (lines 391-403).

### Trigger Condition

Activated when any cell in the row contains a literal `\n` character **and** the format is not CSV/TSV (data formats escape `\n` instead).

### Algorithm

Two-pass approach: measure all cell heights first, then render line-by-line.

#### Pass 1 — Measure

Split every cell on `\n` into sub-lines. The row height is the maximum line count across all cells.

1. Split every cell on `\n` to produce a list of sub-lines per column.
2. Compute row height as the maximum sub-line count across all columns (minimum 1).

#### Pass 2 — Render

Iterate from `line_idx = 0` to `row_height - 1`. For each sub-line index, emit one physical output line containing every column's content at that sub-line index. Cells with fewer lines than `row_height` produce empty padding at missing indices.

#### Key Properties

- **Row height is per-row** — different rows can have different heights.
- **Column widths** are computed from the maximum single-line display width inside each cell, not from raw string length.
- **Truncation** is applied per sub-line independently, not per cell.
- **Border pipes** (for AsciiGrid, Markdown, Unicode styles) are emitted on every sub-line, maintaining visual box continuity.
- **ANSI codes** are preserved; alignment uses `visual_len()` which excludes escape sequences from width.
- **Single-line cells** produce identical output to the non-multiline path.

### Complexity

- Time: O(cells * max_line_count) — two passes, both linear in output size.
- Space: O(cells) for the split cell vectors; no copies of cell content (uses `&str` slices).

### Interaction with Other Features

| Feature | Interaction |
|---------|-------------|
| Truncation (`max_column_width`) | Applied per sub-line after split |
| ANSI coloring (header/alternating rows) | Color wrapping applied to each physical output line in the caller |
| Sub-row detail lines | Detail emitted after all sub-lines of the row |
| CSV/TSV | Multiline rendering skipped; `\n` escaped as literal `\n` |

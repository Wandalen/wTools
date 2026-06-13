# BUG-011: Column Width Used Total Length of Multiline Cell

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/formatters/table/mod.rs` — `calculate_column_widths`

## Root Cause

`unicode_visual_len(cell)` on a multiline string counts `'\n'` characters and all sub-lines
cumulatively, returning the sum of all line lengths. Column width must be the maximum
single-line width (display width of the widest sub-line), not the total string width.
The previous code overestimated column width for cells with many short lines.

## Fix Location

`src/formatters/table/mod.rs` — `calculate_column_widths` function.
`Fix(BUG-011)`: uses `max single-line width` via per-line iteration and `max()` fold
instead of `unicode_visual_len` on the whole cell string.

## Pitfall

For multiline cells, always compute `max(line_widths)`, never the sum or total string width.
This applies in both header and data row passes.

## Test Reference

Covered by `tests/multiline_cells.rs` — column width invariant for multiline content.

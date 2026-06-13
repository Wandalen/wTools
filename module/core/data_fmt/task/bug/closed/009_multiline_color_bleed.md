# BUG-009: Row Color Bled Across Sub-Lines in Multiline Rows

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/formatters/table/mod.rs` — `format_colored_rows`, `format_header_with_color`

## Root Cause

When applying row background color to a formatted row buffer, the code wrapped the entire
buffer with a single `color + row_buf + RESET`. Any `'\n'` characters inside `row_buf`
appeared within the color sequence without an intervening RESET, causing terminal
background-color bleed across line boundaries.

## Fix Location

Two locations in `src/formatters/table/mod.rs`:
- `format_colored_rows`: `Fix(BUG-009)` — iterates `row_buf.lines()` and emits
  `color + line + reset` per output line.
- `format_header_with_color` (doc `# Fix(BUG-009)`): same per-line iteration pattern.

## Pitfall

Never use a single color/RESET wrap on output that may contain intermediate newlines.
Always iterate `.lines()` and emit `color + line + RESET` per output line.

## Test Reference

Covered by `tests/table_rendering_colors.rs` — header and row coloring tests.

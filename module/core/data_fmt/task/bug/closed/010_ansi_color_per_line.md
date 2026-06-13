# BUG-010: Sub-Row ANSI Color Bled Across Newline Boundaries

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/formatters/table/mod.rs` — `format_row_colored`

## Root Cause

Calling `ct.render()` on a multi-line colored cell produces `color + "line_a\nline_b" + RESET`.
The `'\n'` appears inside the color sequence; any terminal rendering of the next line
sees it as still inside the previous color context, causing background-color bleed.

## Fix Location

`src/formatters/table/mod.rs` — `format_row_colored` method (doc `# Fix(BUG-010)`).
Also inline call site at the sub-row detail rendering block: `Fix(BUG-010)`.
Iterates `.text.lines()` and emits `color + line + RESET` per output line.

## Pitfall

Never call `.render()` and then `.lines()` on the result. `.render()` produces a string
with RESET at the very end; splitting it by lines leaves intermediate lines unguarded.
Always iterate `.text.lines()` and emit `color + line + RESET` per output line independently.

## Test Reference

Covered by `tests/decorated_cells_test.rs` — per-line ANSI reset invariant tests.

# BUG-013: Multiline Detail Indent Missing on Continuation Lines

- **Status:** Closed (Fixed)
- **Root Cause:** `push_str(indent) + push_str(detail)` emitted the indent only
  before the first `\n`-terminated segment; subsequent lines of a multi-line
  detail string started at column 0 with no indent prefix.
- **Fix Location:** Sub-row rendering path in `src/formatters/expanded.rs` (or
  the equivalent location in the sub-row emit loop); fix uses `.lines()` iteration
  to prefix every line individually.
- **Pitfall:** Always use `.lines()` iteration when emitting user-provided strings
  that may contain newlines — the same pattern required for colored row rendering.
  A single `push_str(multiline_str)` can never apply per-line prefixes correctly.
- **Test Reference:** `tests/sub_row_test.rs` — `t19_multiline_detail_all_lines_indented`

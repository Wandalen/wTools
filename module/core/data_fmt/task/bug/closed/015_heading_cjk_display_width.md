# BUG-015: Heading CJK display width mismatch

- **Status**: Fixed
- **Severity**: Medium
- **Component**: `src/formatters/table/row_rendering.rs`

### Root Cause

`render_caption_if_present` used `.chars().count()` to measure heading content width. CJK characters occupy 2 display columns but count as 1 char, so the trailing rule was too long and the heading line exceeded the actual table body width.

### Fix Applied

Replaced `.chars().count()` with `crate::ansi_str::unicode_visual_len(&content)` for content width measurement, matching the display-column semantics used by the table body renderer.

### Pitfall

Any width arithmetic that must match terminal rendering must use `unicode_visual_len` (display columns), never `.chars().count()` (scalar count) or `.len()` (byte count).

### Test

`tests/table_caption_test.rs::heading_cjk_title_display_width_matches_table_body` (bug_reproducer)

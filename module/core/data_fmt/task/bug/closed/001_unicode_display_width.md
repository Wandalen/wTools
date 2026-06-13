# BUG-001: Unicode Display Width in Column Alignment

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/ansi_str.rs` — `truncate_single_line`

## Root Cause

Used `char_count()` (Unicode scalar count) instead of display width for column alignment
and truncation. CJK characters and emoji have display width 2, so the column was
consistently under-padded by the number of wide characters in the cell.

## Fix Location

`src/ansi_str.rs` — `truncate_single_line` function (doc comment `# Fix(BUG-001)`).
Uses `unicode_width` crate for terminal display-width calculations.

## Pitfall

Always use `unicode_width` (display width) for terminal layout, never Unicode scalar
count (`char_count`). Character count ≠ display width for CJK, emoji, and combining marks.

## Test Reference

`tests/unicode_display_width_alignment.rs` — 7× `bug_reproducer(BUG-001)`

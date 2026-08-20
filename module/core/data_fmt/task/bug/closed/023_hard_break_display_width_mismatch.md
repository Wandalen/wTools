# BUG-023: Hard-Break Display Width Mismatch

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/wrap.rs` — `hard_break_str`, `push_overlong_word`

## Root Cause

Both hard-break slicing sites computed the byte offset for a line boundary via
`remaining.char_indices().nth(avail)` — walking `avail` CHARACTERS to find a byte
offset, when `avail` is actually a DISPLAY-WIDTH budget (terminal columns). For
CJK/emoji content (`unicode_width` = 2 per char), this let `avail` *characters*
(up to `2 * avail` display columns) onto one line — double the configured width.
Same root cause as BUG-001, regressed at a second, independent call site.

## Fix Location

`src/wrap.rs`:
- `hard_break_str` — `Fix(BUG-023)` slices via `unicode_visual_byte_offset`
- `push_overlong_word` — `Fix(BUG-023)` slices via `unicode_visual_byte_offset`

`src/ansi_str.rs`:
- New `unicode_visual_byte_offset` helper (`Fix(BUG-023)` doc comment) — walks
  display width via `unicode_width`, ANSI-skip rules identical to `unicode_visual_len`.

## Pitfall

Character count ≠ display width for CJK, emoji, combining marks. Any byte-offset
computation driven by a width budget must walk display width per `unicode_width`,
never `char_indices().nth(n)` — this is the second call site in this crate to
regress on the exact invariant already fixed once in `truncate_single_line` (BUG-001).

## Test Reference

`tests/word_wrap.rs` — 2× `bug_reproducer(BUG-023)`:
- `hard_break_str_respects_visual_width_not_char_count`
- `push_overlong_word_respects_visual_width_not_char_count`

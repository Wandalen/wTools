# BUG-002: Word Wrap Triple Fix (tab-width, chunk trim, avail-per-line)

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/wrap.rs` — `expand_tabs`, `hard_break_str`, `push_overlong_word`

## Root Cause

Three related bugs in the word-wrap pipeline:

- **004a (tab-width=0):** Early return for `tab_width==0` kept literal `\t` characters
  instead of replacing them with 0 spaces (i.e. deleting them).
- **004b (chunk trim):** Hard-break sliced a pre-joined string without trimming the
  leading inter-word space at the start of each continuation chunk.
- **004c (avail-per-line):** `avail` (available width per line) was computed once from
  the first line; continuation lines with `subsequent_indent` exceeded `width` because
  indent differs across lines.

## Fix Location

`src/wrap.rs`:
- `expand_tabs` — `Fix(BUG-002)` removes early return
- `hard_break_str` — `Fix(BUG-002)` trims leading space after each slice
- `push_overlong_word` / word-break loop — `Fix(BUG-002)` computes `avail` per line inside loop

## Pitfall

Word-wrap geometry (available width, indent) must be recomputed per output line, not once
at the start. Any "compute once, loop many" shortcut breaks when initial and subsequent
indents differ.

## Test Reference

`tests/word_wrap.rs` — 3× `bug_reproducer(BUG-002)`:
- `expand_tabs_bug_zero_width_keeps_tab`
- `hard_break_bug_continuation_line_leading_space`
- `push_overlong_word_bug_subsequent_indent_overflow`

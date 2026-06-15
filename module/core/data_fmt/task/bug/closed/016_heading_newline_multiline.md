# BUG-016: Heading newline produces multi-line output

- **Status**: Fixed
- **Severity**: Medium
- **Component**: `src/config/table_caption.rs`

### Root Cause

`Heading::content_str()` emitted title and field strings verbatim. Embedded `\n` characters in title or fields broke the heading across multiple terminal lines, violating invariant IN-3 (heading always occupies exactly one output line).

### Fix Applied

`content_str()` delegates to `sanitize_line_breaks()` which replaces all line-break sequences (`\r\n` as pair first, then bare `\r` and `\n`) with a single space. Applied to both title and each field before concatenation.

### Pitfall

Any user-supplied string passed to heading content must be sanitized before width arithmetic — `\n` is invisible to `unicode_visual_len` but produces visible line breaks in terminal output.

### Test

- `tests/table_caption_test.rs::heading_newline_in_title_produces_single_line` (bug_reproducer)
- `tests/table_caption_test.rs::heading_newline_in_field_produces_single_line` (bug_reproducer)
- `tests/table_caption_test.rs::heading_crlf_and_cr_sanitized` (bug_reproducer)

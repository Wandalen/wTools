# BUG-018: TextFormatter KeyValue Trailing Blank Line

- **Status:** Closed (Fixed)
- **Root Cause:** `format_key_value()` unconditionally pushed `'\n'` after every record,
  producing a trailing blank line (`"k: v\n\n"` instead of `"k: v\n"` for single record).
- **Fix Location:** `src/formatters/text.rs` — `format_key_value()` changed from
  "terminator after each" to "separator between" pattern.
- **Pitfall:** Loop-and-append patterns should use separator-between logic, not
  terminator-after-each, to avoid trailing separators.
- **Test Reference:** `tests/corner_case_bug_reproducer_test.rs` —
  `bug_018_keyvalue_trailing_blank_line` tagged `bug_reproducer(BUG-018)`.

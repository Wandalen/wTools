# BUG-019: HtmlFormatter Empty Custom Class Emits class=""

- **Status:** Closed (Fixed)
- **Root Cause:** `HtmlVariant::Custom("")` matched the general `Custom(class)` arm,
  emitting `class=""` in the `<table>` tag — semantically incorrect HTML.
- **Fix Location:** `src/formatters/html.rs` — added guard pattern
  `Custom(class) if class.is_empty() => None` before the general `Custom(class)` arm.
- **Pitfall:** Always check for empty custom strings before emitting HTML attributes.
- **Test Reference:** `tests/corner_case_bug_reproducer_test.rs` —
  `bug_019_html_empty_custom_class` tagged `bug_reproducer(BUG-019)`.

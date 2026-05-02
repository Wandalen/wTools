# Invariant Spec: Auto-Wrap Backward Compatibility

## Source
`docs/invariant/003_auto_wrap_backward_compat.md`

## Test Implementation
`tests/auto_wrap_test.rs` (`auto_wrap_false_is_byte_identical`)

## Case Index

| ID | Name | Status |
|----|------|--------|
| IC-1 | auto_wrap=false produces byte-identical output to pre-wrap rendering | ✅ |
| IC-2 | toggling auto_wrap on non-wrapping content is a no-op | ✅ |

---

### IC-1: auto_wrap=false produces byte-identical output to pre-wrap rendering

**Given:** A table with cell content; one `TableFormatter` uses
`TableConfig::plain().auto_wrap(false).terminal_width(Some(40))`; a second uses
`TableConfig::plain()` with no `auto_wrap` or `terminal_width` override.
**When:** Both formatters render the same table data.
**Then:** The output of the `auto_wrap=false` formatter is byte-identical to the
output of the baseline formatter; disabling `auto_wrap` is a true opt-out with
no side effects.
**Note:** Covered by `auto_wrap_false_is_byte_identical` in `tests/auto_wrap_test.rs`.

---

### IC-2: toggling auto_wrap on non-wrapping content is a no-op

**Given:** A table whose row width is narrower than the terminal width even with
`auto_wrap=true` active; the same table rendered twice — once with `auto_wrap=true`
and once with `auto_wrap=false`.
**When:** Both renders complete.
**Then:** Both outputs are byte-identical; content that would not wrap regardless
is unaffected by the `auto_wrap` flag value.
**Note:** Covered by `auto_wrap_natural_fit_no_wrapping` (T01) in
`tests/auto_wrap_test.rs` (`terminal_width(Some(120))` with `auto_wrap=true` vs
`auto_wrap=false`; asserts `output_wrap == output_no_wrap`).

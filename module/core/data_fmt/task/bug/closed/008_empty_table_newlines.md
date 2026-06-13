# BUG-008: Zero-Column Table Emitted Bare Newlines

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/formatters/table/mod.rs` — `format_internal`

## Root Cause

`format_single_line_row` unconditionally appends `'\n'` for zero-column slices, producing
bare newlines. For a table with zero columns, `format_internal` called it twice (once for
the header row, once for the separator), yielding `"\n\n"` instead of `""`.

## Fix Location

`src/formatters/table/mod.rs` — `format_internal` entry point.
`Fix(BUG-008)`: early return `String::new()` when `headers.is_empty()`.

## Pitfall

Guarding on `rows.is_empty()` would be too aggressive — a headers-only table (no data rows)
should still render the header + separator as a useful "empty state" display. Only
zero-column tables (no headers) should produce empty output.

## Test Reference

Covered by `tests/formatters.rs` / `tests/data.rs` — IC-3 invariant: no columns → empty string.

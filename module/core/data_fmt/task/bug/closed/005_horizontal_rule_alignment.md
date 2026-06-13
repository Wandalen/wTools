# BUG-005: Header Separator Padding Wider Than Data Rows

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/formatters/table/rendering.rs` — `format_header_separator`

## Root Cause

The `Unicode` and `Markdown` header separator branches computed padding inline using
`width + 2`, which added padding around every column junction. This produced separators
that were `2*(N-1)` characters wider than the corresponding data rows, causing horizontal
misalignment in tables with multiple columns.

## Fix Location

`src/formatters/table/rendering.rs` — `HeaderSeparatorVariant::Unicode` and `Markdown` branches.
`Fix(BUG-005)`: both branches now delegate to `format_unicode_horizontal_rule` /
`format_ascii_horizontal_rule`, which add outer padding only at the two outer edges.

## Pitfall

Never replicate the padding logic inline in separator branches. Always delegate to the
`format_*_horizontal_rule` helpers to keep all horizontal-rule-producing paths in sync.

## Test Reference

`tests/table_config_validation_test.rs` — `test_unicode_box_all_lines_same_display_width`
(`// test_kind: bug_reproducer(BUG-005)`); and `test_markdown_all_lines_same_display_width`
for the Markdown branch.

# BUG-003: unicode_box Separator Mismatch

- **Status:** ✅ Closed (Fixed)
- **Affects:** External caller (`gi_infra::formatters::style::cli_table`) + `src/config.rs` API hardening

## Root Cause

`cli_table()` constructed `TableConfig` via struct literal, setting
`header_separator_variant: HeaderSeparatorVariant::Unicode` but relying on
`..TableConfig::default()` for `column_separator`, which defaults to `Spaces(2)`.
The Unicode header separator emits `┼` between columns in the separator row, but data
rows used spaces — producing misaligned, visually broken output.

## Fix Location

Two-part fix:
1. External caller replaced struct literal with `TableConfig::unicode_box()` preset.
2. `TableConfig` fields made private (v0.10.0) — struct literal initialization outside
   `src/config.rs` is a compile error; callers must use presets or the builder chain.

## Pitfall

Unicode separator components are interdependent — `header_separator_variant: Unicode`
requires `column_separator: Character('│')`. Always use `TableConfig::unicode_box()`
rather than manually pairing Unicode header separator with a non-Unicode column separator.

## Test Reference

`tests/table_styles_presets.rs` — `bug_reproducer(BUG-003)`:
`bug_reproducer_issue_011_unicode_box_column_separator_mismatch`

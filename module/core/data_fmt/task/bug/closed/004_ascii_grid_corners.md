# BUG-004: AsciiGrid Header Separator Used Wrong Corner Character

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/formatters/table/rendering.rs` — `format_header_separator`

## Root Cause

`'|'` was hardcoded as the corner/junction character in the `AsciiGrid` header separator
branch, but the border rule uses `'+'` at corners and junctions. This produced a visual
inconsistency between the header separator row and the top/bottom border rules.

## Fix Location

`src/formatters/table/rendering.rs` — `HeaderSeparatorVariant::AsciiGrid` branch.
`Fix(BUG-004)`: corners changed from `'|'` to `'+'`.

## Pitfall

Only the corner and junction characters change here; data-row pipe chars (`|`) stay as-is.
The `AsciiGrid` separator junction must match the border rule corners.

## Test Reference

Covered by `tests/table_styles_presets.rs` — AsciiGrid preset output correctness tests.

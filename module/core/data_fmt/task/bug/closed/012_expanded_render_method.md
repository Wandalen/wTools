# BUG-012: ExpandedFormatter Bypassed ANSI Color via Raw Data Access

- **Status:** ✅ Closed (Fixed)
- **Affects:** `src/formatters/expanded.rs` — key-value rendering loop

## Root Cause

The expanded formatter accessed cell content via `.data.as_ref().map_or("", ..)`, reading
the raw text string directly and bypassing `DecoratedText::render()`. For colored cells,
this emitted plain text without any ANSI color or RESET sequences, making `ExpandedFormatter`
ignore color configuration entirely.

## Fix Location

`src/formatters/expanded.rs` — key-value render loop.
`Fix(BUG-012)`: `cell.render()` replaces raw `.data` access.

## Pitfall

Never access `.data` directly for rendering. `DecoratedText::render()` is the only
correct output path — it handles both plain (`None` color) and colored cells uniformly.
Raw `.data` access is only valid for measurement (display width), never for output.

## Test Reference

Covered by `tests/formatters.rs` — `ExpandedFormatter` color-cell rendering tests.

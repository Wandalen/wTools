# Fix inner padding not applied between columns in table formatter

## Execution State

- **Executor Type:** any
- **Actor:** dev
- **Status:** ✅ (Completed)
- **Claimed At:** 2026-04-21

## Goal

Apply `cell_inner_padding` on both sides of every cell, not just the first and last, so that all padded presets (grid, bordered, markdown, unicode_box) render symmetric spacing around every column separator.

## Issue

`format_row()` in `formatters/table.rs` only adds `inner_padding` before the first cell and after the last cell. Cells in between get no padding around the column separator.

### grid

```
+--------+-------+
| PROFILE|STATUS |
+--------+-------+
| test   |       |
+--------+-------+
| test5  |active |
+--------+-------+
```

### bordered

```
| PROFILE|STATUS |
+--------+-------+
| test   |       |
| test5  |active |
```

### markdown

```
| PROFILE|STATUS |
|--------|-------|
| test   |       |
| test5  |active |
```

### unicode

```
┌────────┬───────┐
│ PROFILE│STATUS │
├────────┼───────┤
│ test   │       │
│ test5  │active │
└────────┴───────┘
```

### plain (not affected, inner_padding=0)

```
PROFILE  STATUS
-------  ------
test           
test5    active
```

Lines 284-287 and 305-309 in `format_row()` gate inner padding on `idx == 0` / `idx == last`. Same pattern in `format_header_separator()` for AsciiGrid (lines 382-395).

Fix: apply inner padding before and after every cell, not just at edges. Affects `bordered()`, `grid()`, `markdown()`, `unicode_box()` -- all presets with `inner_padding > 0`.

## Bug Reproducer

`tests/table_styles_presets.rs` — `test_inner_padding_applied_between_all_cells`

This test renders all four padded presets and asserts that no content token (e.g. `Alice`, `NAME`) is directly adjacent to a column separator character. Before the fix it would fail with:

```
grid: missing padding around separator in line: "| Alice|  30 |  NYC  |"
```

## Root Cause

`format_single_line_row` and `format_multiline_row` gated padding on `idx == 0` (leading) and `idx == last` (trailing), leaving no padding around inner separators. Additionally, the column separator was emitted *before* the trailing padding, so even moving the guard would have placed padding on the wrong side of the separator.

The same edge-only guard was present in `format_ascii_horizontal_rule`, `format_unicode_horizontal_rule`, and the inline `AsciiGrid`/`Dash` branches of `format_header_separator`, causing separator lines to be narrower than their corresponding data rows.

## Fix

Two-part change in `src/formatters/table/mod.rs`:

1. Remove `idx == 0` / `idx == last` guards — emit padding before and after every cell.
2. Swap emission order — push trailing padding **before** the column separator, not after.

Same correction applied to both horizontal rule helpers and the inline separator branches.

# Algorithm: Heading Rendering

### Scope

- **Purpose**: Compute and emit a titled rule line that matches the rendered table width above the table output.
- **Responsibility**: Documents the caption line assembly, lead/trail rule computation, and table width integration.
- **In Scope**: Content string construction, lead prefix, trailing rule width calculation, multi-byte character handling, table width computation.
- **Out of Scope**: Terminal width resolution (see `feature/005_auto_fit.md § Terminal Width Detection`), table body rendering (see `algorithm/001_multiline_cell_rendering.md`).

### Features

| File | Relationship |
|------|-------------|
| [007_table_caption.md](../feature/007_table_caption.md) | Feature that this algorithm implements |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/table/row_rendering.rs`](../../src/formatters/table/row_rendering.rs) | `render_caption_if_present()` — caption line assembly |
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Call site in `format_internal()` — provides `primary_widths` to caption renderer |
| [`src/config/table_caption.rs`](../../src/config/table_caption.rs) | `Heading` struct and field accessor |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../tests/table_caption_test.rs) | Caption rendering tests |

### Abstract

A four-step algorithm that assembles a titled rule line from a title, zero or more caption fields, a fixed lead prefix, and a trailing rule that fills to the rendered table width. The field separator and rule characters are multi-byte in UTF-8, so width is measured by display column count (not byte length or character count — CJK characters occupy 2 display columns each). Line breaks in title and fields are sanitized to spaces before assembly. The trailing rule is clamped to zero when caption content already meets or exceeds the rendered table width.

### Algorithm

1. **Build content string**: sanitize line breaks in title and each field (replace `\r\n`, `\r`, `\n` with space), then concatenate the title followed by `" {field_separator} {field_value}"` for each field. The field separator is a fixed middle dot character (U+00B7).
2. **Build lead prefix**: repeat the rule character (U+2500 BOX DRAWINGS LIGHT HORIZONTAL) × lead_width (fixed at 3), then append one space, producing `"─── "`.
3. **Compute trailing rule width**: `trail_width = table_width − lead_width − 1 − content_display_width − 1`, where `table_width` is the rendered display width of the table computed by `compute_total_row_width(primary_widths)` (accounts for column widths, separators, per-column padding, and border pipes). The subtractions account for the lead chars, the space after the lead, the content display column count, and one trailing space. Clamp to 0 if negative. Use display column count (`unicode_visual_len`), not byte length or character count — CJK characters are 1 char but 2 display columns; both the field separator (U+00B7) and rule character (U+2500) are multi-byte in UTF-8.
4. **Emit**: lead + content + " " + rule_char × trail_width + newline.

The `table_width` is passed in from `format_internal()` where the column widths (`primary_widths`) have already been computed by the auto-fit pipeline. The `terminal_width` setting continues to influence the auto-fit column budget but does not affect caption line width.

### Key Properties

- **Multi-byte safety**: the field separator and rule character are each two or more bytes in UTF-8. Width is measured in display columns (via `unicode_visual_len`), not bytes or character count — CJK characters occupy 2 display columns.
- **Clamp at zero**: when caption content alone equals or exceeds table width, trail_width becomes 0 — the trailing rule is omitted; content is never truncated.
- **Render position**: caption line is emitted before the table top border (or before the header row when no top border exists for the selected style).
- **Style-agnostic**: the algorithm is identical across all 9 table style presets.
- **Independence**: caption rendering is entirely independent of auto-wrap and auto-fold; the caption line is not subject to column folding or cell wrapping.

### Complexity

- Time: O(n) where n is the total character count of title and caption fields — linear scan to build the content string.
- Space: O(1) beyond the output string — no intermediate collections.

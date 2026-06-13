# Algorithm: Caption Rendering

### Scope

- **Purpose**: Compute and emit a titled rule line that fills the resolved terminal width above the table output.
- **Responsibility**: Documents the caption line assembly, lead/trail rule computation, and terminal width integration.
- **In Scope**: Content string construction, lead prefix, trailing rule width calculation, multi-byte character handling, terminal width chain.
- **Out of Scope**: Terminal width resolution (see `feature/005_auto_fit.md § Terminal Width Detection`), table body rendering (see `algorithm/001_multiline_cell_rendering.md`).

### Features

| File | Relationship |
|------|-------------|
| [007_table_caption.md](../feature/007_table_caption.md) | Feature that this algorithm implements |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | Caption rendering inside `format_internal()` |
| [`src/config.rs`](../../src/config.rs) | `TableCaption` struct and field accessor |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_caption_test.rs`](../../tests/table_caption_test.rs) | Caption rendering tests |

### Abstract

A four-step algorithm that assembles a titled rule line from a title, zero or more caption fields, a fixed lead prefix, and a trailing rule that fills the remaining terminal width. The field separator and rule characters are multi-byte in UTF-8, so width is measured by character count rather than byte length. The trailing rule is clamped to zero when caption content already meets or exceeds the terminal width.

### Algorithm

1. **Build content string**: concatenate the title, then for each caption field append " {field_separator} {field_value}". The field separator is a fixed middle dot character (U+00B7).
2. **Build lead prefix**: repeat the rule character (U+2500 BOX DRAWINGS LIGHT HORIZONTAL) × lead_width (fixed at 3), then append one space, producing `"─── "`.
3. **Compute trailing rule width**: `trail_width = terminal_width − lead_width − 1 − content_char_count − 1`, where the subtractions account for the lead chars, the space after the lead, the content character count, and one trailing space. Clamp to 0 if negative. Use character count, not byte length — both the field separator (U+00B7) and rule character (U+2500) are multi-byte in UTF-8.
4. **Emit**: lead + content + " " + rule_char × trail_width + newline.

Terminal width is resolved via the same four-tier chain as auto-fit: explicit override → `$COLUMNS` environment variable → terminal size detection library → hardcoded fallback (120).

### Key Properties

- **Multi-byte safety**: the field separator and rule character are each two or more bytes in UTF-8. Width is measured in Unicode scalar values (character count), not bytes.
- **Clamp at zero**: when caption content alone equals or exceeds terminal width, trail_width becomes 0 — the trailing rule is omitted; content is never truncated.
- **Render position**: caption line is emitted before the table top border (or before the header row when no top border exists for the selected style).
- **Style-agnostic**: the algorithm is identical across all 9 table style presets.
- **Independence**: caption rendering is entirely independent of auto-wrap and auto-fold; the caption line is not subject to column folding or cell wrapping.

### Complexity

- Time: O(n) where n is the total character count of title and caption fields — linear scan to build the content string.
- Space: O(1) beyond the output string — no intermediate collections.

# Feature: Table Formatting

### Scope

- **Purpose**: Configure and render tabular data with 9 style presets, multiline cell support, column truncation, ANSI-aware coloring, border variant rendering, and sub-row detail lines.
- **Responsibility**: Document table formatting capabilities and configuration options.
- **In Scope**: Style presets, multiline cells, truncation, coloring, borders, sub-rows, auto-fit overview, and column width calculation.
- **Out of Scope**: Algorithm details (see `../algorithm/`), API signatures (see `../api/`).

### Algorithms

| File | Relationship |
|------|-------------|
| [001_multiline_cell_rendering.md](../algorithm/001_multiline_cell_rendering.md) | Multiline cell algorithm |

### Features

| File | Relationship |
|------|-------------|
| [005_auto_fit.md](005_auto_fit.md) | Terminal-aware auto-wrapping and column folding |
| [007_table_caption.md](007_table_caption.md) | Caption line rendered above the table |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | TableFormatter implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_rendering_borders.rs`](../../tests/table_rendering_borders.rs) | Border and style rendering tests |
| [`tests/table_styles_presets.rs`](../../tests/table_styles_presets.rs) | Preset configuration tests |

### Design

#### Style Presets

`TableConfig` provides 9 preset constructors. Each returns a fully configured instance ready for use.

**Recommended default:** `plain()` is the preferred variant for general-purpose CLI output. It works on all terminals, CI pipelines, and Windows without Unicode support. `TableConfig::default()` returns `plain()`. Use bordered or unicode styles only when a specific visual requirement justifies the narrower compatibility.

| Preset | Description | Usage |
|--------|-------------|-------|
| `plain()` | Space-separated columns with dash header separator — **recommended default** | `TableConfig::plain()` |
| `minimal()` | Space-separated columns, no header separator | `TableConfig::minimal()` |
| `bordered()` | Traditional pipe-bordered table (PostgreSQL style) | `TableConfig::bordered()` |
| `markdown()` | GitHub-flavored Markdown table format | `TableConfig::markdown()` |
| `grid()` | Full ASCII grid with `+` intersections | `TableConfig::grid()` |
| `unicode_box()` | Unicode box-drawing characters — requires Unicode terminal; avoid for general CLI tools | `TableConfig::unicode_box()` |
| `csv()` | Comma-separated values | `TableConfig::csv()` |
| `tsv()` | Tab-separated values | `TableConfig::tsv()` |
| `compact()` | Minimal spacing for maximum density | `TableConfig::compact()` |

#### Multiline Cells

When any cell contains `\n`, the formatter activates multiline rendering automatically using a two-pass algorithm. See [`001_multiline_cell_rendering.md`](../algorithm/001_multiline_cell_rendering.md) for full algorithm specification.

#### Column Truncation

Controlled by two `TableConfig` fields:

- `max_column_width` — cells wider than this value are truncated. Disabled by default.
- `truncation_marker` — appended to truncated content. Defaults to `"..."`.

For example, a 20-character limit with a `"..."` marker shortens `"Very long content that exceeds twenty characters"` to `"Very long conten..."` (20 chars total, marker included).

Truncation is ANSI-aware: `visual_len()` excludes escape codes from the width count and `strs_tools::ansi::truncate()` preserves color codes in the truncated output. Truncation applies to both header and data cells during `format_row()`.

#### Multiline + Truncation Interaction

When both features are active, truncation is applied per-line after splitting on `\n`. Each sub-line is independently truncated to `max_column_width`, producing a clean visual result.

#### Min Column Width Floor

`min_column_width` widens every column to at least the given display-character count. Width starts from the content-driven maximum, is then capped at `max_column_width` (if set), and is finally raised to `min_column_width` if the result falls below it.

If `min_column_width > max_column_width`, columns settle at `min_column_width` (floor wins).

#### Override Bypass

When `TableConfig::column_widths` is set explicitly, it replaces calculated widths entirely. Both `min_column_width` and `max_column_width` are ignored. This is intended for callers that need exact control.

#### ANSI Header and Row Coloring

Two independent color features controlled via `TableConfig` builder methods:

**Header coloring** -- `colorize_header( true )` + `header_color( code )` wraps the header row in the given ANSI code. Each output line is wrapped individually: `color + content + \x1b[0m + \n`.

**Alternating row colors** -- `alternating_rows( true )` + `row_colors( color1, color2 )` alternates between two ANSI codes for data rows.

Every colored line ends with `\x1b[0m` before the trailing `\n` to prevent terminal background-color bleed. For multiline cells, each sub-line is wrapped with its own color/RESET pair.

#### Border Variant Rendering

`BorderVariant` controls the overall border style:

| Variant | Rendering |
|---------|-----------|
| `None` | No borders, space-separated columns |
| `Ascii` | Pipe borders: `\| + -` |
| `AsciiGrid` | Full grid with top/bottom borders and `+` corners: `+---+---+` |
| `Unicode` | Box-drawing with top/bottom borders: corner/junction characters |
| `Markdown` | Markdown-style: `\| col \| col \|` with `\|---\|` separator |

`AsciiGrid` and `Unicode` render top and bottom borders in addition to the header separator. Inter-row separators are drawn for grid style.

`HeaderSeparatorVariant` and `ColumnSeparator` provide independent control over the separator line below the header and the delimiter between columns, respectively.

#### Sub-Row Detail Lines

Optional annotation lines that appear below a data row, outside the cell grid. Each detail may carry an ANSI color code for per-row coloring without affecting column formatting. Detail lines are added via `RowBuilder::add_row_with_detail`, which accepts an optional detail annotation alongside the row data. Plain string references convert to decorated text automatically, with no ANSI color attached and zero runtime overhead.

Rendering behavior:

- Detail lines are emitted AFTER all row content lines (including multiline cells) and BEFORE any inter-row separator.
- Each detail line is prefixed with `sub_row_indent` (default: 2 spaces). Configure via `TableConfig::sub_row_indent( indent )`.
- Multi-line details (containing `\n`) are split on newlines; every resulting line receives the indent prefix and its own color/reset pair independently (no ANSI bleed across line boundaries).
- Detail lines do NOT participate in column width calculation — they are metadata, not cell values.
- Detail lines are NOT colored by alternating row colors — only by the detail's own color field.
- For bordered styles (`AsciiGrid`, `Unicode`), detail lines appear outside the cell grid (no border pipes).
- Absent and empty-text details are suppressed -- no blank line emitted.
- Rows added via `add_row()` or `add_row_mut()` implicitly have no annotation (no output change).

#### Auto-Fit (Terminal-Aware Rendering)

When `auto_wrap` and `auto_fold` are both enabled (the default), `TableFormatter` automatically fits output within terminal width using two cooperating strategies: cell wrapping (grows row height) and column folding (moves overflow columns to continuation lines). Zero configuration required.

See `005_auto_fit.md` for full behavioral specification, configuration fields, progressive degradation rules, and interaction with other features.

#### Column Width Calculation Summary

The full column width pipeline:

1. **Content-driven max** -- for each column, take the maximum display width across header and all data cells. Multiline cells use the maximum single-line display width across all sub-lines.
2. **Max cap** -- if `max_column_width` is set, cap each column at that value.
3. **Min floor** -- if `min_column_width > 0`, raise any column below it.
4. **Override bypass** -- if `column_widths` is set explicitly, skip steps 1-3 entirely.

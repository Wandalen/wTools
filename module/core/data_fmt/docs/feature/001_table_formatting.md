# Feature: Table Formatting

### Scope

- **Purpose**: Configure and render tabular data with 9 style presets, multiline cell support, column truncation, ANSI-aware coloring, border variant rendering, and sub-row detail lines.
- **Responsibility**: Document table formatting capabilities and configuration options.
- **In Scope**: Style presets, multiline cells, truncation, coloring, borders, sub-rows, auto-fit overview, and column width calculation.
- **Out of Scope**: Algorithm details (see `../algorithm/`), API signatures (see `../api/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/table/mod.rs` | TableFormatter implementation |
| test | `tests/table_rendering_borders.rs` | Border and style rendering tests |
| test | `tests/table_styles_presets.rs` | Preset configuration tests |
| doc | `../algorithm/001_multiline_cell_rendering.md` | Multiline cell algorithm |
| doc | `005_auto_fit.md` | Terminal-aware auto-wrapping and column folding |

### Design

#### Style Presets

`TableConfig` provides 9 preset constructors. Each returns a fully configured instance ready for use.

| Preset | Description | Usage |
|--------|-------------|-------|
| `plain()` | Space-separated columns with dash header separator | `TableConfig::plain()` |
| `minimal()` | Space-separated columns, no header separator | `TableConfig::minimal()` |
| `bordered()` | Traditional pipe borders (default style) | `TableConfig::bordered()` |
| `markdown()` | GitHub-flavored Markdown table format | `TableConfig::markdown()` |
| `grid()` | Full ASCII grid with `+` intersections | `TableConfig::grid()` |
| `unicode_box()` | Unicode box-drawing characters | `TableConfig::unicode_box()` |
| `csv()` | Comma-separated values | `TableConfig::csv()` |
| `tsv()` | Tab-separated values | `TableConfig::tsv()` |
| `compact()` | Minimal spacing for maximum density | `TableConfig::compact()` |

#### Multiline Cells

When any cell contains `\n`, the formatter activates multiline rendering automatically using a two-pass algorithm.

**Pass 1 -- measure**: Split every cell on `\n` and record the maximum line count across cells in the row. That count becomes the row height.

**Pass 2 -- render**: Iterate from line 0 to row height. For each sub-line index, emit one physical output line containing the corresponding sub-line of every cell (or blank padding for cells with fewer lines). Column separators and borders are applied to each physical line.

Behavior details:

- Row height is per-row -- different rows can have different heights.
- Column widths are computed from the maximum single-line width inside each cell via `.lines().map( visual_len ).max()`, not from the raw string length.
- Shorter cells are padded with empty strings to match the row height.
- ANSI codes are preserved and alignment uses `visual_len()` (color codes excluded from width).
- Single-line cells work identically to pre-multiline behavior.
- CSV/TSV formats disable multiline rendering and keep `\n` as literal characters.

#### Column Truncation

Controlled by two `TableConfig` fields:

- `max_column_width : Option< usize >` -- cells wider than this value are truncated. Disabled by default (`None`).
- `truncation_marker : String` -- appended to truncated content. Defaults to `"..."`.

```rust
let config = TableConfig::plain()
  .max_column_width( Some( 20 ) )
  .truncation_marker( "...".to_string() );
// "Very long content that exceeds twenty characters"
// becomes "Very long conten..." (20 chars including marker)
```

Truncation is ANSI-aware: `visual_len()` excludes escape codes from the width count and `strs_tools::ansi::truncate()` preserves color codes in the truncated output. Truncation applies to both header and data cells during `format_row()`.

#### Multiline + Truncation Interaction

When both features are active, truncation is applied per-line after splitting on `\n`. Each sub-line is independently truncated to `max_column_width`, producing a clean visual result.

#### Min Column Width Floor

`min_column_width : usize` widens every column to at least the given display-character count. Applied after the content-driven max and after the `max_column_width` cap:

```
width = max( header_width, max( row_widths ) )   // content-driven
width = min( width, max_column_width )            // cap (if set)
width = max( width, min_column_width )            // floor (if > 0)
```

If `min_column_width > max_column_width`, columns settle at `min_column_width` (floor wins).

#### Override Bypass

When `TableConfig::column_widths` is set explicitly, it replaces calculated widths entirely. Both `min_column_width` and `max_column_width` are ignored. This is intended for callers that need exact control.

#### ANSI Header and Row Coloring

Two independent color features controlled via `TableConfig` builder methods:

**Header coloring** -- `colorize_header( true )` + `header_color( code )` wraps the header row in the given ANSI code. Each output line is wrapped individually: `color + content + \x1b[0m + \n`.

**Alternating row colors** -- `alternating_rows( true )` + `row_colors( color1, color2 )` alternates between two ANSI codes for data rows.

```rust
let config = TableConfig::plain()
  .colorize_header( true )
  .header_color( "\x1b[1;36m".to_string() )
  .alternating_rows( true )
  .row_colors(
    "\x1b[0m".to_string(),
    "\x1b[48;5;236m".to_string(),
  );
```

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

Optional annotation lines that appear below a data row, outside the cell grid. Each detail is typed as `Option<DecoratedText>`, enabling per-row ANSI color without affecting column formatting.

```rust
use data_fmt::{ RowBuilder, DecoratedText };

let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
  .add_row_with_detail(
    vec![ "Alice".into(), "30".into() ],
    Some( DecoratedText::from( "Senior engineer" ).with_color( "\x1b[33m" ) ),
  )
  .add_row_with_detail( vec![ "Bob".into(), "25".into() ], Some( "plain note".into() ) )
  .build_view();
```

`"plain note".into()` calls `DecoratedText::from(&str)` — transparent, zero-overhead, no color attached. All existing `.into()` call sites remain source-compatible.

Rendering behavior:

- Detail lines are emitted AFTER all row content lines (including multiline cells) and BEFORE any inter-row separator.
- Each detail line is prefixed with `sub_row_indent` (default: 2 spaces). Configure via `TableConfig::sub_row_indent( indent )`.
- Multi-line details (containing `\n`) are split on newlines; every resulting line receives the indent prefix and its own color/reset pair independently (no ANSI bleed across line boundaries).
- Detail lines do NOT participate in column width calculation — they are metadata, not cell values.
- Detail lines are NOT colored by alternating row colors — only by the `DecoratedText.color` field.
- For bordered styles (`AsciiGrid`, `Unicode`), detail lines appear outside the cell grid (no border pipes).
- `None` and empty-text details are suppressed -- no blank line emitted.
- Rows added via `add_row()` or `add_row_mut()` implicitly have `None` detail (no output change).

#### Auto-Fit (Terminal-Aware Rendering)

When `auto_wrap` and `auto_fold` are both enabled (the default), `TableFormatter` automatically fits output within terminal width using two cooperating strategies: cell wrapping (grows row height) and column folding (moves overflow columns to continuation lines). Zero configuration required.

See `005_auto_fit.md` for full behavioral specification, configuration fields, progressive degradation rules, and interaction with other features.

#### Column Width Calculation Summary

The full column width pipeline:

1. **Content-driven max** -- for each column, take the maximum display width across header and all data cells. Multiline cells use `.lines().map( visual_len ).max()`.
2. **Max cap** -- if `max_column_width` is set, cap each column at that value.
3. **Min floor** -- if `min_column_width > 0`, raise any column below it.
4. **Override bypass** -- if `column_widths` is set explicitly, skip steps 1-3 entirely.

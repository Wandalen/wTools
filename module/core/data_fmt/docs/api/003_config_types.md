# API: Config Types

### Scope

- **Purpose**: Document the public API surface for configuration and style types.
- **Responsibility**: Define enums and config structs that control formatter output appearance.
- **In Scope**: Config struct fields, preset constructors, builder setters, width calculation order.
- **Out of Scope**: Behavioral invariants (see `../invariant/`), construction patterns (see `../builder/`).

### Sources

| File | Relationship |
|------|-------------|
| [`src/config/table_config.rs`](../../src/config/table_config.rs) | `TableConfig` struct, presets, and all builder setters |
| [`src/config/table_enums.rs`](../../src/config/table_enums.rs) | `BorderVariant`, `HeaderSeparatorVariant`, `ColumnSeparator`, `ColumnFlex`, `FoldStyle` |
| [`src/config/table_caption.rs`](../../src/config/table_caption.rs) | `Heading` struct and builder |
| [`src/config/expanded_config.rs`](../../src/config/expanded_config.rs) | `ExpandedConfig` struct and builder setters |
| [`src/config/tree_config.rs`](../../src/config/tree_config.rs) | `TreeConfig` struct and builder setters |
| [`src/config/mod.rs`](../../src/config/mod.rs) | Public re-exports for all config types |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_config_corner_cases.rs`](../../tests/table_config_corner_cases.rs) | Config edge case tests |

### Abstract

Three config structs, one heading builder type, and six supporting enums form the configuration API. `TableConfig` governs all `TableFormatter` rendering parameters: borders, separators, column sizing, coloring, auto-fit, and optional heading. `Heading` is a builder type for the titled-rule line rendered above a table. `ExpandedConfig` controls `ExpandedFormatter` key-value presentation. `TreeConfig` controls `TreeFormatter` structure and indentation. The six enum types — `BorderVariant`, `HeaderSeparatorVariant`, `ColumnSeparator`, `PaddingSide`, `ColumnFlex`, and `FoldStyle` — are embedded in the config structs or passed as builder arguments. All consuming builder setters across all four types follow the `with_` prefix convention (`with_{name}(mut self, …) -> Self`).

### Operations

#### BorderVariant

Controls outer border rendering for `TableConfig`. Five variants: `None` (no borders, space-separated), `Ascii` (pipe borders using `|` and `-`), `AsciiGrid` (full grid with `+` at intersections), `Unicode` (box-drawing characters), `Markdown` (GitHub-flavored markdown table format with `|`).

#### HeaderSeparatorVariant

Controls the separator line drawn below the header row. Five variants: `None`, `Dash` (plain dashes), `AsciiGrid` (dash-plus separator), `Unicode` (box-drawing junction), `Markdown` (pipe-dash separator).

#### ColumnSeparator

Controls the delimiter between columns. Three variants: `Spaces` (N space characters between columns, count specified), `Character` (single character such as `|`, `,`, or `\t`), `String` (arbitrary multi-character separator).

#### PaddingSide

Controls alignment padding placement in `ExpandedFormatter` key-value output. `BeforeSeparator` pads keys to align separators vertically; `AfterSeparator` pads values after the separator character.

#### ColumnFlex

Per-column classification for the auto-fit budget allocation algorithm. `Fixed` columns retain their natural content width and are never wrapped or folded. `Flex` columns shrink to the allocated budget and their content wraps when needed. When `TableConfig::column_flex` is empty (the default), columns are auto-classified: max cell display width ≤ 12 = `Fixed`, otherwise `Flex`.

#### FoldStyle

Controls how overflow columns are formatted in continuation lines. `Labeled` (default) emits `"ColName: value"` pairs. `Bare` joins all overflow values on one line. `Stacked` emits one labeled line per overflow column.

#### Heading

Builder type for the titled-rule line rendered above a table. Two fields: `title` (primary label text) and `fields` (zero or more metadata strings appended with the field separator). Construct with `Heading::new(title: impl Into<String>)` and chain zero or more `.with_field(f: impl Into<String>)` calls to append heading fields. The resulting value is attached to `TableConfig` via `.with_heading(Heading::new("..."))`. The heading renders as `─── Title · Field1 · Field2 ─────` filling the rendered table width. Three formatting constants are publicly exported: `CAPTION_FIELD_SEP` (`·`, U+00B7), `CAPTION_RULE_CHAR` (`─`, U+2500), and `CAPTION_LEAD_WIDTH` (`3`). When no heading is set (the default `None`), table output is byte-identical to the pre-heading baseline.

#### TableConfig

All fields are private; accessed via preset constructors and builder setters. Nine preset constructors: `plain()`, `minimal()`, `bordered()`, `markdown()`, `grid()`, `unicode_box()`, `csv()`, `tsv()`, `compact()`. All return fully configured instances. 24 consuming builder setters follow the `with_` prefix convention: `with_column_widths`, `with_align_right`, `with_border_variant`, `with_header_separator_variant`, `with_column_separator`, `with_outer_padding`, `with_inner_padding`, `with_colorize_header`, `with_header_color`, `with_alternating_rows`, `with_row_colors`, `with_color_reset`, `with_min_column_width`, `with_max_column_width`, `with_truncation_marker`, `with_sub_row_indent`, `with_terminal_width`, `with_auto_wrap`, `with_column_flex`, `with_auto_fold`, `with_fold_style`, `with_fold_indent`, `with_border_color`, and `with_heading`. All setters are `#[ must_use ]` and return `Self`.

**Width calculation order** (when auto-fit fields are combined): (1) content-driven max per column; (2) cap at `max_column_width` if set; (3) raise to `min_column_width` floor if non-zero; (4) `column_widths` override replaces all calculated widths; (5) auto-fit budget shrinks flex columns to terminal budget; (6) auto-fold moves remaining overflow columns to continuation lines.

#### ExpandedConfig

Controls `ExpandedFormatter` output. Fields: `record_separator`, `key_value_separator`, `show_record_numbers`, `colorize_keys`, `key_color`, `padding_side`, `indent_prefix`. Two preset constructors: `new()` / `postgres_style()` (aligned keys, pipe separator) and `property_style()` (colon separator, after-separator padding). Seven builder setters follow the `with_` prefix convention: `with_record_separator`, `with_key_value_separator`, `with_show_record_numbers`, `with_colorize_keys`, `with_key_color`, `with_padding_side`, `with_indent_prefix`.

#### TreeConfig

Controls `TreeFormatter` output. Seven fields: `show_branches` (draw branch connector symbols), `show_root` (render root node), `indent_size` (spaces per depth level, default 4), `max_depth` (depth cutoff), `column_separator` (string between aligned columns), `min_column_width` (minimum per-column display width), `branch_color` (optional ANSI color string for branch connector symbols). Constructor: `new()`. Seven builder setters follow the `with_` prefix convention: `with_show_branches`, `with_show_root`, `with_indent_size`, `with_max_depth`, `with_column_separator`, `with_min_column_width`, `with_branch_color`.

### Error Handling

Config construction does not return errors. Preset constructors always succeed. Builder setters perform no validation at call time — invalid combinations (e.g., `min_column_width` > `max_column_width`) are resolved at render time with defined behavior (floor wins over cap). Terminal width of `0` is clamped to `1` at render time to prevent division-by-zero in budget allocation.

### Compatibility Guarantees

All preset constructors are stable and produce output byte-identical across minor versions. New builder setters are additive — callers that chain only the setters they need are unaffected by new fields. `TableConfig::csv()` and `TableConfig::tsv()` automatically disable auto-fit features regardless of manual settings; this coupling is stable. `ExpandedConfig::postgres_style()` and `property_style()` maintain their named formatting characteristics across versions.

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
| [`src/config/table_heading.rs`](../../src/config/table_heading.rs) | `Heading` struct and builder |
| [`src/config/expanded_config.rs`](../../src/config/expanded_config.rs) | `ExpandedConfig` struct and builder setters |
| [`src/config/tree_config.rs`](../../src/config/tree_config.rs) | `TreeConfig` struct and builder setters |
| [`src/config/mod.rs`](../../src/config/mod.rs) | Public re-exports for all config types |
| [`src/wrap.rs`](../../src/wrap.rs) | `WrapConfig`, `WrapFormatter`, `BreakStrategy`, `Overflow` |
| [`src/formatters/text.rs`](../../src/formatters/text.rs) | `TextFormatter`'s `heading` / `footer` fields and `with_heading()` / `with_footer()` builder setters (no separate config type) |
| [`src/formatters/yaml.rs`](../../src/formatters/yaml.rs) | `YamlFormatter`'s `heading` / `footer` fields and `with_heading()` / `with_footer()` builder setters (no separate config type; comment-wrapped rendering) |
| [`src/formatters/toml_fmt.rs`](../../src/formatters/toml_fmt.rs) | `TomlFormatter`'s `heading` / `footer` fields and `with_heading()` / `with_footer()` builder setters (no separate config type; comment-wrapped rendering) |
| [`src/formatters/sql.rs`](../../src/formatters/sql.rs) | `SqlFormatter`'s `heading` / `footer` fields and `with_heading()` / `with_footer()` builder setters (comment-wrapped rendering) |
| [`src/formatters/html.rs`](../../src/formatters/html.rs) | `HtmlFormatter`'s `heading` / `footer` fields and `with_heading()` / `with_footer()` builder setters (no separate config type; comment-delimited rendering with both prefix and suffix) |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_config_corner_cases.rs`](../../tests/table_config_corner_cases.rs) | Config edge case tests |

### Abstract

Four config structs, one heading builder type, and eight supporting enums form the configuration API. `TableConfig` governs all `TableFormatter` rendering parameters: borders, separators, column sizing, coloring, auto-fit, and an optional heading and/or footer. `Heading` is a builder type for the titled-rule line rendered above a formatter's output (heading, via `with_heading()`) or below it (footer, via `with_footer()`) — the same type serves both positions, across every adopting formatter. `ExpandedConfig` controls `ExpandedFormatter` key-value presentation, and an optional heading and/or footer (same `Heading` type and mechanism as `TableConfig`). `TreeConfig` controls `TreeFormatter` structure, indentation, and an optional heading and/or footer (same `Heading` type and mechanism as `TableConfig`). `TextFormatter` also carries an optional heading and/or footer (same `Heading` type and mechanism), but has no separate config type at all — the `heading`/`footer` fields live directly on the formatter struct, per an explicit YAGNI decision against inventing a `TextConfig` type that nothing else needs. `YamlFormatter`, `TomlFormatter`, `SqlFormatter`, and `HtmlFormatter` follow the identical no-separate-config-type, direct-`pub`-fields pattern, but render the titled rule *comment-wrapped* (`"# "` for Yaml/Toml, `"-- "` for Sql, `"<!-- "` + `" -->"` for Html) via `render_commented_rule_if_present()` rather than the bare `render_rule_if_present()` every other adopter uses — `HtmlFormatter` is the only one of the four that passes a non-empty comment suffix, since `<!-- -->` needs an explicit closing delimiter — see `#### YamlFormatter` / `#### TomlFormatter` / `#### SqlFormatter` / `#### HtmlFormatter` below. `WrapConfig` controls `WrapFormatter`'s word-wrap behavior — a standalone utility formatter outside the `Format`-trait family documented in `../api/004_formatters.md`. The eight enum types — `BorderVariant`, `HeaderSeparatorVariant`, `ColumnSeparator`, `PaddingSide`, `ColumnFlex`, `FoldStyle`, `BreakStrategy`, and `Overflow` — are embedded in the config structs or passed as builder arguments. `TableConfig`, `ExpandedConfig`, `TreeConfig`, and `Heading` builder setters follow the `with_` prefix convention (`with_{name}(mut self, …) -> Self`); `WrapConfig` is the one exception — its setters use bare field names with no prefix (`width(mut self, …) -> Self`, not `with_width`).

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

Builder type for a titled-rule line rendered above a formatter's output (heading) or below it (footer) — position is determined solely by which config builder attaches it, not by the type itself. Two fields: `title` (primary label text) and `fields` (zero or more metadata strings appended with the field separator). Construct with `Heading::new(title: impl Into<String>)` and chain zero or more `.with_field(f: impl Into<String>)` calls to append heading fields. The resulting value is attached via `.with_heading(Heading::new("..."))` (renders before the output) or `.with_footer(Heading::new("..."))` (renders after the output) — both may be set independently on the same config. `TableConfig` renders it before/after the top/bottom border (or the header/last row when no border exists for the selected style); `TreeConfig` renders it before/after the tree body; `ExpandedConfig` renders it before/after the vertical key-value record body; `TextFormatter` renders it before/after the rendered text body (fields live directly on the formatter — see `#### TextFormatter` below). Each renders as `─── Title · Field1 · Field2 ─────` filling the calling formatter's rendered width. Three formatting constants are publicly exported: `HEADING_FIELD_SEP` (`·`, U+00B7), `HEADING_RULE_CHAR` (`─`, U+2500), and `HEADING_LEAD_WIDTH` (`3`) — shared by both positions and every adopting formatter. When neither heading nor footer is set (both default `None`), output is byte-identical to the pre-heading baseline.

#### TableConfig

All fields are private; accessed via preset constructors and builder setters. Nine preset constructors: `plain()`, `minimal()`, `bordered()`, `markdown()`, `grid()`, `unicode_box()`, `csv()`, `tsv()`, `compact()`. All return fully configured instances. 25 consuming builder setters follow the `with_` prefix convention: `with_column_widths`, `with_align_right`, `with_border_variant`, `with_header_separator_variant`, `with_column_separator`, `with_outer_padding`, `with_inner_padding`, `with_colorize_header`, `with_header_color`, `with_alternating_rows`, `with_row_colors`, `with_color_reset`, `with_min_column_width`, `with_max_column_width`, `with_truncation_marker`, `with_sub_row_indent`, `with_terminal_width`, `with_auto_wrap`, `with_column_flex`, `with_auto_fold`, `with_fold_style`, `with_fold_indent`, `with_border_color`, `with_heading`, and `with_footer`. All setters are `#[ must_use ]` and return `Self`.

**Width calculation order** (when auto-fit fields are combined): (1) content-driven max per column; (2) cap at `max_column_width` if set; (3) raise to `min_column_width` floor if non-zero; (4) `column_widths` override replaces all calculated widths; (5) auto-fit budget shrinks flex columns to terminal budget; (6) auto-fold moves remaining overflow columns to continuation lines.

#### ExpandedConfig

Controls `ExpandedFormatter` output. All fields are `pub` (unlike `TableConfig`/`TreeConfig`'s private-field-plus-accessor convention — see `WrapConfig` below for the one other all-`pub` config): `record_separator`, `key_value_separator`, `show_record_numbers`, `colorize_keys`, `key_color`, `padding_side`, `indent_prefix`, `heading` (optional `Heading` rendered above the output, `None` by default), `footer` (optional `Heading` rendered below the output, `None` by default). Two preset constructors: `new()` / `postgres_style()` (aligned keys, pipe separator) and `property_style()` (colon separator, after-separator padding). Nine builder setters follow the `with_` prefix convention: `with_record_separator`, `with_key_value_separator`, `with_show_record_numbers`, `with_colorize_keys`, `with_key_color`, `with_padding_side`, `with_indent_prefix`, `with_heading`, `with_footer`.

#### TreeConfig

Controls `TreeFormatter` output. Nine fields: `show_branches` (draw branch connector symbols), `show_root` (render root node), `indent_size` (spaces per depth level, default 4), `max_depth` (depth cutoff), `column_separator` (string between aligned columns), `min_column_width` (minimum per-column display width), `branch_color` (optional ANSI color string for branch connector symbols), `heading` (optional `Heading` rendered above the tree, `None` by default), `footer` (optional `Heading` rendered below the tree, `None` by default). Constructor: `new()`. Nine builder setters follow the `with_` prefix convention: `with_show_branches`, `with_show_root`, `with_indent_size`, `with_max_depth`, `with_column_separator`, `with_min_column_width`, `with_branch_color`, `with_heading`, `with_footer`.

#### TextFormatter

Not a config struct — `TextFormatter` carries its output parameters as `pub` fields directly on the formatter, matching `ExpandedConfig`'s all-`pub` convention rather than `TableConfig`'s private-field-plus-accessor convention (`TreeConfig` is a mixed case — see `#### TreeConfig` above: most fields `pub`, only `heading`/`footer` private behind accessors). Three pre-existing fields (`variant`: `TextVariant`, `indent`, `separator`) plus two added for this feature: `heading` (optional `Heading` rendered above the output, `None` by default), `footer` (optional `Heading` rendered below the output, `None` by default). Preset constructors: `new( variant : TextVariant )`, `bullets()`, `numbered()`, `key_value()`, `sections()`, `compact()`, `cli_help()`. Builder setters relevant here: `with_heading`, `with_footer` (both `#[ must_use ]`, returning `Self`, alongside the pre-existing `with_indent`/`with_separator`). Both fields are read once, at the single funnel point where `Format::format()` calls `wrap_with_heading_footer()` on the already-rendered body before returning — this covers every `TextVariant` uniformly, including `CliHelp`'s own internal empty-rows early return.

#### YamlFormatter

Not a config struct — same all-`pub`-fields-on-formatter convention as `TextFormatter`. Two fields, both added for this feature: `heading` (optional `Heading` rendered above the output as a `#`-commented line, `None` by default), `footer` (optional `Heading` rendered below the output as a `#`-commented line, `None` by default) — no pre-existing fields, since YAML serialization needs no configuration beyond `serde_yaml_ng::to_string()`. Constructor: `new()` (also backs the `Default` impl). Builder setters: `with_heading`, `with_footer` (both `#[ must_use ]`, returning `Self`). Both fields are read once, at the single funnel point where `Format::format()` calls `wrap_with_heading_footer()` on the already-serialized YAML body before returning. Unlike `TableConfig`/`TreeConfig`/`ExpandedConfig`/`TextFormatter`, which render the rule bare via `render_rule_if_present()`, `YamlFormatter` renders it through `render_commented_rule_if_present()` with a `"# "` prefix, so the titled rule stays valid YAML (a `#`-prefixed comment line) rather than corrupting the parse.

#### TomlFormatter

Identical shape to `YamlFormatter` — same two fields (`heading`, `footer`), same `new()` constructor, same `with_heading`/`with_footer` builders, same single funnel point in `Format::format()`, same `"# "` comment prefix via `render_commented_rule_if_present()` (a `#`-prefixed line is a valid TOML comment, same as YAML). The only difference from `YamlFormatter` is the serialization call itself (`toml::to_string()` against an internal `TomlWrapper` struct, needed because TOML requires an array-of-tables wrapper rather than a bare top-level array).

#### SqlFormatter

Five fields: three pre-existing (`table_name`: `String`; `variant`: `SqlVariant`; `empty_as_null`: `bool`, default `false`) plus two added for this feature: `heading` (optional `Heading` rendered above the output as a `--`-commented line, `None` by default), `footer` (optional `Heading` rendered below the output as a `--`-commented line, `None` by default). Two constructors: `new( table_name : impl Into< String > )` (`Ansi` variant) and `with_variant( table_name : impl Into< String >, variant : SqlVariant )` — both initialize `heading`/`footer` to `None`. Builder setters relevant here: `with_heading`, `with_footer` (alongside the pre-existing `with_empty_as_null`, all `#[ must_use ]`, returning `Self`). Renders the rule through `render_commented_rule_if_present()` with a `"-- "` prefix and an empty `""` suffix, so the titled rule stays valid SQL. `Format::format()` calls `wrap_with_heading_footer()` from both of its return points — the BUG-020 empty-rows early return and the final populated-rows return — so heading/footer apply uniformly regardless of that branch. The populated-rows body is one of two exceptions among all eight adopting formatters (the other is `HtmlFormatter`, below) whose body ends with no trailing newline — here a bare `;` (every other line-oriented formatter's body always ends in `\n`) — so `wrap_with_heading_footer()` inserts a separating `\n` before the footer whenever `footer.is_some() && !body.is_empty() && !body.ends_with('\n')` — the `!body.is_empty()` guard specifically avoids introducing a spurious blank line on the empty-rows branch.

#### HtmlFormatter

Five fields: three pre-existing (`variant`: `HtmlVariant`; `include_wrapper`: `bool`, default `false`; `table_id`: `Option<String>`, default `None`) plus two added for this feature: `heading` (optional `Heading` rendered above the output as an `<!-- -->`-commented line, `None` by default), `footer` (optional `Heading` rendered below the output as an `<!-- -->`-commented line, `None` by default). Three constructors: `new()` (`Minimal` variant), `with_variant( variant : HtmlVariant )`, and `with_table_class( class : impl Into< String > )` (`Custom` variant) — all three initialize `heading`/`footer` to `None`. Builder setters relevant here: `with_heading`, `with_footer` (alongside the pre-existing `with_table_id`, `with_include_wrapper`, all `#[ must_use ]`, returning `Self`). Renders the rule through `render_commented_rule_if_present()` with a `"<!-- "` prefix AND a `" -->"` suffix — the only adopting formatter with a non-empty suffix, since HTML's `<!-- -->` is a delimited comment rather than a line comment: an unclosed `<!--` would silently swallow everything up to the next `-->` in the document, including the `<table>` markup itself. `Format::format()` calls `wrap_with_heading_footer()` from its single return point, wrapping the *entire* rendered output — including the optional `<!DOCTYPE>`/`<html>`/`<body>` prelude when `include_wrapper` is `true` — so the heading is always the first line of output and the footer always the last, regardless of the wrapper setting. Like `SqlFormatter`'s populated-rows body, HTML output ends with a bare closing tag (`</table>` or `</html>`) and no trailing newline, so the identical separating-`\n`-before-footer guard applies.

#### BreakStrategy

Controls how lines are broken when content exceeds `WrapConfig::width`. Three variants: `Word` (break at the last word boundary before the limit; overlong tokens are handled by `break_long_words`), `Hard` (break at exactly `width` chars, ignoring word boundaries), `WordThenHard` (default — word-boundary first, hard-break only when a single token exceeds available width).

#### Overflow

Controls output behavior when `WrapConfig::max_lines` is exceeded. Two variants: `Truncate` (drop excess lines silently), `Ellipsis( String )` (append the given suffix to the last kept line, truncating content so line length stays ≤ `width`).

#### WrapConfig

All 9 fields are `pub`, same as `ExpandedConfig` — but unlike `TableConfig`, whose fields are all private (`TreeConfig` is a mixed case: most fields `pub`, only `heading`/`footer` private — see `#### TreeConfig` above): `width` (`usize`, default `80`), `initial_indent` / `subsequent_indent` (`String`, default `""`), `break_strategy` (`BreakStrategy`, default `WordThenHard`), `break_long_words` (`bool`, default `true`), `preserve_newlines` (`bool`, default `true`), `max_lines` (`Option< usize >`, default `None`), `overflow` (`Overflow`, default `Truncate`), `tab_width` (`usize`, default `4`). Constructor: `new()` (also backs the `Default` impl). Ten consuming setter methods — **no `with_` prefix**, unlike the other three config types: `width`, `initial_indent`, `subsequent_indent`, `indent` (sets both indent fields to the same value in one call), `break_strategy`, `break_long_words`, `preserve_newlines`, `max_lines`, `overflow`, `tab_width`. All `#[ must_use ]`, returning `Self`.

#### WrapFormatter

Not part of the `Format`-trait family (see `../api/004_formatters.md`) — a standalone utility with its own return types rather than `Result< String, FormatError >`. Constructed via `new()` (default `WrapConfig`) or `with_config( config : WrapConfig )`. Two methods: `wrap( &self, text : &str ) -> Vec< String >` (one entry per output line, each with its indent already applied; empty input returns an empty `Vec`), `wrap_joined( &self, text : &str ) -> String` (equivalent to `self.wrap( text ).join( "\n" )`).

### Error Handling

Config construction does not return errors. Preset constructors always succeed. Builder setters perform no validation at call time — invalid combinations (e.g., `min_column_width` > `max_column_width`) are resolved at render time with defined behavior (floor wins over cap). Terminal width of `0` is clamped to `1` at render time to prevent division-by-zero in budget allocation. `WrapFormatter::wrap` / `wrap_joined` never return errors either — every `(text, config)` combination, including a zero `width`, produces a defined `Vec< String >` / `String` result.

### Compatibility Guarantees

All preset constructors are stable and produce output byte-identical across minor versions. New builder setters are additive — callers that chain only the setters they need are unaffected by new fields. `TableConfig::csv()` and `TableConfig::tsv()` automatically disable auto-fit features regardless of manual settings; this coupling is stable. `ExpandedConfig::postgres_style()` and `property_style()` maintain their named formatting characteristics across versions. `WrapConfig::new()`'s defaults (width `80`, `WordThenHard` strategy, `break_long_words`/`preserve_newlines` both `true`, `tab_width` `4`) are stable.

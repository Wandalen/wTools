# API: Formatters

### Scope

- **Purpose**: Document the public API surface for all formatter types.
- **Responsibility**: Define formatter struct methods, trait implementations, feature flags, and helpers.
- **In Scope**: Formatter constructors, format methods, feature flag gating, ANSI/Unicode helpers.
- **Out of Scope**: Behavioral invariants (see `../invariant/`), construction patterns (see `../builder/`).

### Traits

| File | Relationship |
|------|-------------|
| [001_format.md](../trait/001_format.md) | Format trait contract |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/mod.rs`](../../src/formatters/mod.rs) | Formatter module organization |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/formatters.rs`](../../tests/formatters.rs) | Formatter integration tests |

### Abstract

Nine of ten formatters implement the unified `Format` trait and accept `&TableView`. `TreeFormatter` uses direct method dispatch without the trait. Three visual formatters (`TableFormatter`, `ExpandedFormatter`, `TreeFormatter`) also accept `&TreeNode< T >` via `format_tree` methods. Seven additional formatters (`LogfmtFormatter`, `HtmlFormatter`, `SqlFormatter`, `JsonFormatter`, `YamlFormatter`, `TomlFormatter`, `TextFormatter`) are gated behind feature flags. Two ANSI/Unicode helper functions (`visual_len`, `pad_to_width`) support width-aware rendering.

### Operations

#### Format Trait

The unified interface implemented by all formatters. Takes `&self` and `&TableView`, returns `Result< String, FormatError >`. Callers build a `TableView` once via `RowBuilder::build_view()` and pass it to any formatter through this trait.

#### FormatError

Error type returned by `Format::format`. Three variants: `Serialization( String )` (only available with the `serde_support` feature; emitted by JSON/YAML/TOML formatters on serialization failure), `InvalidData( String )` (input data is structurally invalid for the requested format), `UnsupportedOperation( String )` (operation not supported by the given formatter configuration). Implements `Display` and `std::error::Error`.

#### TableFormatter

Horizontal tabular display with configurable borders, column sizing, and coloring. Constructed via `TableFormatter::new()` (default config) or `TableFormatter::with_config( config : TableConfig )`. Key methods: `format( &self, tree : &TreeNode< String > ) -> String` (table-shaped tree via `TableShapedView`), `format_tree< T : Display >( &self, tree : &TreeNode< T > ) -> String` (hierarchical tree auto-flattened to path/name/depth/data columns), `write_to< W : Write >( &self, tree, writer )` (streaming output). Implements `Format` for the canonical `TableView` path.

#### ExpandedFormatter

Vertical record display rendering one record per row as labeled key-value pairs. Constructed via `ExpandedFormatter::new()` or `ExpandedFormatter::with_config( config : ExpandedConfig )`. Implements `Format` (since v0.3.0). Methods: `Format::format( &self, data : &TableView ) -> Result< String, FormatError >`, `format_tree< T : Display >( &self, tree : &TreeNode< T > ) -> String`, `write_to< W : Write >( &self, view, writer )`.

#### TreeFormatter

Hierarchical tree display with box-drawing characters. Constructed via `TreeFormatter::new()`, `TreeFormatter::with_config( config : TreeConfig )`, or `TreeFormatter::with_symbols( symbols : TreeSymbols )`. Three format methods: `format< T, F >( &self, tree : &TreeNode< T >, render_item : F ) -> String` (custom render closure), `format_aligned( &self, tree : &TreeNode< ColumnData > ) -> String` (column-aligned output), `format_with_aggregation< T, V, A, F, D, C >( &self, tree, grand_total, ... ) -> String` (subtree totals and percentages). Streaming output via `write_to< T, F, W >( &self, tree, writer, render_item )`.

#### TableShapedFormatter Trait (Removed)

Removed in v0.3.0. Was implemented by `TableFormatter` and `ExpandedFormatter`. Use `Format` with `RowBuilder::build_view()` instead.

#### Additional Formatters

Seven feature-gated formatters, all implementing `Format`:

| Formatter | Feature Flag | External Deps | Output |
|-----------|-------------|---------------|--------|
| `LogfmtFormatter` | `format_logfmt` | None | `key=value` structured log pairs |
| `HtmlFormatter` | `format_html` | None | HTML `<table>` with CSS variants |
| `SqlFormatter` | `format_sql` | None | SQL `INSERT` statements |
| `JsonFormatter` | `format_json` | serde, serde_json | JSON array of row objects |
| `YamlFormatter` | `format_yaml` | serde, serde_yaml | YAML sequence of mappings |
| `TomlFormatter` | `format_toml` | serde, toml | TOML array of tables |
| `TextFormatter` | `format_text` | None | Plain text (6 styles) |

#### ANSI/Unicode Helpers

`visual_len( text : &str ) -> usize` — counts visible Unicode codepoints, excluding ANSI escape sequences. Uses character count (not display width); ANSI sequences contribute zero. `pad_to_width( text : &str, target_width : usize, align_right : bool ) -> String` — pads a string to a target display width using East Asian Width for terminal column alignment (CJK and emoji count as 2 display columns). Returns text unchanged when display width already meets or exceeds target.

### Error Handling

`Format::format` returns `Result< String, FormatError >`. `TableFormatter`, `ExpandedFormatter`, and `TreeFormatter` return `Ok` for all valid inputs; they do not emit `FormatError` in practice. `JsonFormatter`, `YamlFormatter`, and `TomlFormatter` may return `FormatError::Serialization` when serialization fails. `FormatError::Serialization` is only present when the `serde_support` feature is enabled; without it, the error type has two variants.

### Feature Flags

| Feature | Enables |
|---------|---------|
| `format_table` | `TableFormatter` |
| `format_expanded` | `ExpandedFormatter` |
| `format_tree` | `TreeFormatter` |
| `format_logfmt` | `LogfmtFormatter` |
| `format_html` | `HtmlFormatter` |
| `format_sql` | `SqlFormatter` |
| `format_json` | `JsonFormatter` + `serde_support` |
| `format_yaml` | `YamlFormatter` + `serde_support` |
| `format_toml` | `TomlFormatter` + `serde_support` |
| `format_text` | `TextFormatter` |
| `format_meta_visual` | `format_table` + `format_expanded` + `format_tree` + `format_logfmt` |
| `format_meta_web` | `format_html` + `format_sql` |
| `format_meta_data` | `format_json` + `format_yaml` + `format_toml` |
| `all_formats` | All formatters + `themes` |
| `enabled` | Core deps + default visual formatters |
| `full` | `enabled` + `all_formats` + `terminal_size` |

### Compatibility Guarantees

The `Format` trait signature (`format( &self, data : &TableView ) -> Result< String, FormatError >`) is stable. `TableShapedFormatter` was removed in v0.3.0; use `Format` with `TableView` instead. Formatter constructors `new()` and `with_config()` are stable. `visual_len` and `pad_to_width` are stable utility functions. Feature flag names are stable — adding a formatter never changes an existing flag's behavior.

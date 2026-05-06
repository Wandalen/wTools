# Architecture

## Purpose

data_fmt is a generic multi-format data visualization library with a unified format interface. It provides reusable formatters for displaying data in multiple formats (Table, Expanded, Tree, Logfmt, JSON, YAML, TOML, Text) with seamless conversion between representations, enabling the same data to appear as horizontal tables, vertical records, or hierarchical trees.

## Design Principles

1. **Single Data Structure** -- `TreeNode< T >` for all data (hierarchical and tabular)
2. **Unified Format Interface** -- same API for all formatters via Format trait
3. **Canonical Data Format** -- `TableView` struct for format-agnostic code
4. **Granular Features** -- zero-cost abstractions with optional formatters
5. **Mutual Replaceability** -- any data can be displayed in any format
6. **Minimal Dependencies** -- core has zero dependencies, formatters are optional
7. **Generic** -- works with any data type via `TreeNode< T >`
8. **ANSI-Aware** -- proper alignment with color codes
9. **Flexible Output** -- String return and `io::Write` support
10. **Helper Traits** -- ergonomic builders and traits for table-shaped trees
11. **Modular Architecture** -- separated concerns across 16 source modules

## Three-Layer Architecture

### Layer 1: Data (TreeNode)

`TreeNode< T >` is the single data structure serving both hierarchical and tabular use cases:

```rust
pub struct TreeNode< T >
{
  pub name : String,
  pub data : Option< T >,
  pub children : Vec< TreeNode< T > >,
}
```

Hierarchical trees use `data = None` for directories and `data = Some(T)` for files. Table-shaped trees encode rows as children of root, with each row's children named after columns.

### Layer 2: Builders and Traits

Ergonomic construction and generic extraction for table-shaped trees:

- `RowBuilder` -- fluent and mutable APIs for building table-shaped trees
- `TableShapedView` trait -- generic extraction of headers and rows from any `TreeNode< T >` where `T : Display`
- `TableView` -- canonical interchange struct holding `headers` and `rows` for format-agnostic code

### Layer 3: Formatters

Format-specific renderers that consume `TreeNode< T >` or `TableView`:

- `TableFormatter` -- horizontal tabular display (9 style presets)
- `ExpandedFormatter` -- vertical record display (PostgreSQL and property styles)
- `TreeFormatter` -- hierarchical box-drawing display (with aligned column mode)
- `LogfmtFormatter` -- structured logging key=value pairs
- `HtmlFormatter` -- semantic HTML tables with CSS themes
- `SqlFormatter` -- SQL INSERT statement generation (4 dialects)
- `JsonFormatter`, `YamlFormatter`, `TomlFormatter` -- serialization formats
- `TextFormatter` -- plain text output (6 styles)

All formatters implement the `Format` trait for a unified API.

## Module File Structure

```
src/
  lib.rs                     # Re-exports public API
  data.rs                    # TreeNode, TableView struct, TableShapedView trait
  builder.rs                 # TreeBuilder (hierarchical)
  table_tree.rs              # RowBuilder (table-shaped)
  config.rs                  # TreeConfig, TableConfig, ExpandedConfig
  conversions.rs             # Tree<->Table conversions, FlattenConfig
  ansi_str.rs                # visual_len, pad_to_width, truncate_cell
  wrap.rs                    # WrapConfig, WrapFormatter, BreakStrategy, Overflow
  themes.rs                  # ColorTheme predefined and custom themes
  formatters/
    mod.rs                   # TableShapedFormatter trait (deprecated), Format trait re-export
    format_trait.rs          # Format trait, FormatError
    tree.rs                  # TreeFormatter with format() and format_aligned()
    table/                   # TableFormatter (split into directory)
    expanded.rs              # ExpandedFormatter
    logfmt.rs                # LogfmtFormatter
    html.rs                  # HtmlFormatter
    sql.rs                   # SqlFormatter
    json.rs                  # JsonFormatter
    yaml.rs                  # YamlFormatter
    toml_fmt.rs              # TomlFormatter
    text.rs                  # TextFormatter
```

## Formatter Design

`TableFormatter` and `ExpandedFormatter` use the `TableShapedView` trait to extract headers and rows from any `TreeNode< T >` where `T : Display`. This decouples formatting logic from tree internals -- formatters work with flat vectors of strings.

`TreeFormatter` renders `TreeNode< T >` directly using method-level generics rather than relying on `TableShapedView`. Its `format()` and `format_aligned()` methods accept `&TreeNode< T >` where `T : Display`, producing box-drawing output with configurable symbols and indentation.

The `TableShapedFormatter` trait is deprecated (since 0.1.0). The `Format` trait is the canonical interface for all formatters. See `docs/trait/002_table_shaped_formatter.md` for migration guidance.

All formatters support both `format()` (returns `String`) and `write_to()` (writes to any `io::Write`).

## Configuration Builder Pattern

All config structs expose fluent builder APIs for constructing formatter options:

```rust
let config = TreeConfig::new()
  .show_branches( false )
  .max_depth( Some( 3 ) );
```

Config structs: `TreeConfig`, `TableConfig`, `ExpandedConfig`. Each formatter accepts its corresponding config at construction time.

## Concept Quick Reference

| Concept | Kind | Role |
|---------|------|------|
| `TableView` | struct | Tabular input type — canonical flat table (headers + rows + optional row details) |
| `TreeNode< T >` | struct | Hierarchical input type — recursive named-node tree with typed leaf payloads |
| `RowBuilder` | struct | Builds `TableView`; validates row lengths at insertion |
| `TreeBuilder< T >` | struct | Builds `TreeNode< T >` from path-based insertions |
| `Format` | trait | Unified rendering contract: `fn format(&self, data: &TableView)` |
| `FormatError` | enum | Error type returned by `Format::format()` — `Serialization`, `InvalidData`, `UnsupportedOperation` |
| `TableFormatter` | struct | Implements `Format`; holds `TableConfig`; 9 visual table styles |
| `ExpandedFormatter` | struct | Does NOT implement `Format` (uses deprecated `TableShapedFormatter`); holds `ExpandedConfig` |
| `TreeFormatter` | struct | Does NOT implement `Format`; direct-dispatch generic methods; holds `TreeConfig` |
| `LogfmtFormatter` | struct | Implements `Format`; no config state |
| `HtmlFormatter` | struct | Implements `Format`; holds `HtmlVariant` |
| `SqlFormatter` | struct | Implements `Format`; holds `SqlVariant` |
| `JsonFormatter` | struct | Implements `Format`; no config state; requires `serde_support` |
| `YamlFormatter` | struct | Implements `Format`; no config state; requires `serde_support` |
| `TomlFormatter` | struct | Implements `Format`; no config state; requires `serde_support` |
| `TextFormatter` | struct | Implements `Format`; holds `TextVariant` |
| `TableConfig` | struct | Formatter parameters for `TableFormatter` — style, wrap, width, padding, etc. |
| `ExpandedConfig` | struct | Formatter parameters for `ExpandedFormatter` |
| `TreeConfig` | struct | Formatter parameters for `TreeFormatter` — branch symbols, depth, indentation |
| `TableMetadata` | struct | Column names and per-column `DataType` semantic labels |
| `ColumnData` | struct | Multi-column leaf payload for aligned tree formatting |
| `DecoratedText` | struct | ANSI-aware string cell (from `color_tools`) — carries text + optional color |
| `WrapFormatter` | struct | Word-wrap pre-processor; not a `Format` implementor; wraps cell content before table formatting |
| `TableShapedView` | trait | Deprecated: extracts headers/rows from `TreeNode< T where T: Display >`; use `RowBuilder::build_view()` instead |

**Pipeline summary**: `RowBuilder` → `TableView` → `Format::format()` → `String`

# Feature: Unified Format Interface

### Scope

- **Purpose**: Provide a single `Format` trait and canonical `TableView` data type so callers can write format-agnostic code while each formatter lives behind its own feature flag for zero-cost abstractions.
- **Responsibility**: Document the unified format interface design, formatter registry, and feature flag configuration.
- **In Scope**: Format trait, TableView interchange, formatter registry, feature bundles, and usage patterns.
- **Out of Scope**: Algorithm details (see `../algorithm/`), API signatures (see `../api/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/format_trait.rs` | Format trait definition |
| test | `tests/unified_format_trait.rs` | Format trait tests |
| doc | `../trait/001_format.md` | Format trait contract |

### Design

#### Design Goals

1. **Unified interface** -- same API for all formatters (table, json, yaml, toml, text, etc.).
2. **Canonical data format** -- `TableView` struct as common interchange format between data producers and formatters.
3. **Granular features** -- each formatter behind an optional feature flag; unused formatters compile to zero code and zero dependencies.
4. **Zero-cost abstractions** -- no runtime overhead from the trait dispatch; unused formats add nothing to the binary.

#### Core Types

#### TableView

The canonical data format consumed by all formatters.

```rust
pub struct TableView
{
  pub metadata : TableMetadata,
  pub rows : Vec< Vec< String > >,
}
```

#### Format Trait

```rust
pub trait Format
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >;
}
```

Every formatter implements `Format`. Callers build a `TableView` once and pass it to any formatter.

#### Formatter Registry

| Formatter | Feature Flag | Dependencies | Use Case |
|-----------|-------------|--------------|----------|
| `TableFormatter` | `format_table` | None | Visual table output |
| `ExpandedFormatter` | `format_expanded` | None | Vertical records |
| `TreeFormatter` | `format_tree` | None | Hierarchical display |
| `LogfmtFormatter` | `format_logfmt` | None | Structured logging |
| `HtmlFormatter` | `format_html` | None | Web tables (HTML) |
| `SqlFormatter` | `format_sql` | None | SQL INSERT statements |
| `JsonFormatter` | `format_json` | serde, serde_json | Data interchange, APIs |
| `YamlFormatter` | `format_yaml` | serde, serde_yaml | Configuration files |
| `TomlFormatter` | `format_toml` | serde, toml | Rust config files |
| `TextFormatter` | `format_text` | None | Human-readable lists |

#### Feature Bundles

| Bundle | Includes |
|--------|----------|
| `format_meta_visual` | `format_table` + `format_expanded` + `format_tree` + `format_logfmt` |
| `format_meta_web` | `format_html` + `format_sql` |
| `format_meta_data` | `format_json` + `format_yaml` + `format_toml` |
| `all_formats` | `format_meta_visual` + `format_meta_web` + `format_meta_data` + `format_text` + `themes` |

#### Feature Configuration

```toml
[features]
default       = []
full          = [ "enabled", "all_formats", "terminal_size" ]
terminal_size = [ "dep:terminal_size" ]
enabled = [
  "dep:error_tools", "error_tools/enabled",
  "dep:strs_tools", "strs_tools/ansi",
  "dep:color_tools", "color_tools/enabled",
  "dep:unicode-width",
  "table_plain", "expanded_postgres", "tree_hierarchical", "format_logfmt",
]
serde_support = [ "dep:serde", "color_tools/serde_support" ]

# Individual formatter meta-features (aggregate granular variant flags)
format_table = [ "format_table_visual", "format_table_export" ]
format_expanded = [ "expanded_postgres", "expanded_property" ]
format_tree = [ "tree_hierarchical", "tree_aligned", "tree_aggregated" ]
format_html = [ "format_html_basic", "format_html_frameworks" ]
format_sql = [ "sql_ansi", "sql_postgres", "sql_mysql", "sql_sqlite" ]
format_json = [ "serde_support", "dep:serde_json" ]
format_yaml = [ "serde_support", "dep:serde_yaml" ]
format_toml = [ "serde_support", "dep:toml" ]
format_text = []
format_logfmt = []
themes = []

# Convenience bundles
format_meta_visual = [ "format_table", "format_expanded", "format_tree", "format_logfmt" ]
format_meta_web    = [ "format_html", "format_sql" ]
format_meta_data   = [ "format_json", "format_yaml", "format_toml" ]
all_formats = [ "format_meta_visual", "format_meta_web", "format_meta_data", "format_text", "themes" ]
```

#### Cargo.toml Usage

```toml
# Standard workspace integration (enabled = core deps + default visual formatters)
data_fmt = { version = "0.1.0", features = [ "enabled" ] }

# Add JSON support
data_fmt = { version = "0.1.0", features = [ "enabled", "format_json" ] }

# All formatters
data_fmt = { version = "0.1.0", features = [ "full" ] }

# Minimal: only the plain table variant (no core deps)
data_fmt = { version = "0.1.0", features = [ "table_plain" ] }
```

#### Usage Pattern

Build data once with `build_view()`, then format with any formatter through the `Format` trait.

```rust
use data_fmt::{ RowBuilder, Format };

// Build data once
let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
  .add_row( vec![ "Alice".into(), "30".into() ] )
  .build_view();

// Format as JSON
#[ cfg( feature = "format_json" ) ]
{
  use data_fmt::JsonFormatter;
  let json = JsonFormatter::new();
  let output = Format::format( &json, &view )?;
}

// Format as table
#[ cfg( feature = "format_table" ) ]
{
  use data_fmt::TableFormatter;
  let table = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &table, &view )?;
}
```

#### Migration from TreeNode

The existing `RowBuilder` API is unchanged. The new `build_view()` method is additive.

- `RowBuilder::build()` -- still returns `TreeNode< String >` (unchanged).
- `RowBuilder::build_view()` -- returns `TableView` (new).
- `TableView::to_tree_node()` -- converts back to `TreeNode` for backward compatibility.

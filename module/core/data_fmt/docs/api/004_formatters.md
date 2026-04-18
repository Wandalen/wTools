# API: Formatters

### Scope

- **Purpose**: Document the public API surface for all formatter types.
- **Responsibility**: Define formatter struct methods, trait implementations, feature flags, and helpers.
- **In Scope**: Formatter constructors, format methods, feature flag gating, ANSI/Unicode helpers.
- **Out of Scope**: Behavioral invariants (see `../invariant/`), construction patterns (see `../builder/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/mod.rs` | Formatter module organization |
| test | `tests/formatters.rs` | Formatter integration tests |
| doc | `../trait/001_format.md` | Format trait contract |

### Format Trait

Unified interface implemented by all formatters. Accepts `&TableView` (built via `RowBuilder::build_view()`).

```rust
pub trait Format
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >;
}
```

### FormatError

```rust
#[ derive( Debug ) ]
pub enum FormatError
{
  #[ cfg( feature = "serde_support" ) ]
  Serialization( String ),
  InvalidData( String ),
  UnsupportedOperation( String ),
}

impl std::fmt::Display for FormatError { ... }
impl std::error::Error for FormatError {}
```

### TableShapedFormatter Trait

Polymorphism trait for visual formatters that consume `TreeNode< String >`.

```rust
pub trait TableShapedFormatter
{
  fn format( &self, tree : &TreeNode< String > ) -> String;
}
```

Enables trait object dispatch:

```rust
let formatters : Vec< Box< dyn TableShapedFormatter > > = vec![
  Box::new( TableFormatter::new() ),
  Box::new( ExpandedFormatter::new() ),
];
```

### TableFormatter

Horizontal tabular display with configurable borders, alignment, and column limits.

```rust
impl TableFormatter
{
  pub fn new() -> Self;
  pub const fn with_config( config : TableConfig ) -> Self;

  /// Format table-shaped tree (headers/rows via TableShapedView)
  pub fn format( &self, tree : &TreeNode< String > ) -> String;
  /// Format hierarchical tree (auto-flattens to path/name/depth/data)
  pub fn format_tree< T : Display >( &self, tree : &TreeNode< T > ) -> String;
  /// Write directly to io::Write
  pub fn write_to< W : std::io::Write >( &self, tree : &TreeNode< String >, writer : &mut W ) -> std::io::Result< () >;
}

impl TableShapedFormatter for TableFormatter { ... }
impl Format for TableFormatter { ... }
```

### ExpandedFormatter

Vertical record display (one record per row, key-value pairs).

```rust
impl ExpandedFormatter
{
  pub fn new() -> Self;
  pub const fn with_config( config : ExpandedConfig ) -> Self;

  pub fn format( &self, tree : &TreeNode< String > ) -> String;
  pub fn format_tree< T : Display >( &self, tree : &TreeNode< T > ) -> String;
  pub fn write_to< W : std::io::Write >( &self, tree : &TreeNode< String >, writer : &mut W ) -> std::io::Result< () >;
}

impl TableShapedFormatter for ExpandedFormatter { ... }
impl Format for ExpandedFormatter { ... }
```

### TreeFormatter

Hierarchical tree display with box-drawing characters. Supports custom renderers, column alignment, and aggregation.

```rust
impl TreeFormatter
{
  pub fn new() -> Self;
  pub fn with_config( config : TreeConfig ) -> Self;
  pub fn with_symbols( symbols : TreeSymbols ) -> Self;

  /// Format with custom render closure
  pub fn format< T, F >( &self, tree : &TreeNode< T >, render_item : F ) -> String
  where
    F : Fn( &T ) -> String;

  /// Format with column-aligned data
  pub fn format_aligned( &self, tree : &TreeNode< ColumnData > ) -> String;

  /// Format with aggregated directory totals and percentages
  pub fn format_with_aggregation< T, V, A, F, D, C >(
    &self,
    tree : &TreeNode< T >,
    grand_total : V,
    aggregate_fn : A,
    convert_to_f64 : C,
    render_file : F,
    render_directory : D,
  ) -> String
  where
    V : Copy + std::ops::Add< Output = V > + Default + std::iter::Sum,
    A : Fn( &T ) -> V,
    C : Fn( V ) -> f64,
    F : Fn( &T, V, f64 ) -> String,
    D : Fn( &str, V, f64 ) -> String;

  /// Write directly to io::Write
  pub fn write_to< T, F, W >(
    &self,
    tree : &TreeNode< T >,
    writer : &mut W,
    render_item : F,
  ) -> std::io::Result< () >
  where
    F : Fn( &T ) -> String,
    W : std::io::Write;
}
```

### Additional Formatters

These formatters implement the `Format` trait and are gated behind feature flags.

| Formatter | Feature Flag | Dependencies | Purpose |
|-----------|-------------|--------------|---------|
| `LogfmtFormatter` | `format_logfmt` | None | Structured logging (`key=value` pairs) |
| `HtmlFormatter` | `format_html` | None | HTML `<table>` output with CSS variants |
| `SqlFormatter` | `format_sql` | None | SQL INSERT statements |
| `JsonFormatter` | `format_json` | serde, serde_json | JSON array of row objects |
| `YamlFormatter` | `format_yaml` | serde, serde_yaml | YAML sequence of mappings |
| `TomlFormatter` | `format_toml` | serde, toml | TOML array of tables |
| `TextFormatter` | `format_text` | None | Plain text (6 styles: bullets, numbered, sections, key-value, compact, cli-help) |

All implement:

```rust
impl Format for XxxFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >;
}
```

### Feature Flags

#### Individual Formatters

| Feature | Formatter | External Dependencies |
|---------|-----------|----------------------|
| `format_table` | TableFormatter | None |
| `format_expanded` | ExpandedFormatter | None |
| `format_tree` | TreeFormatter | None |
| `format_logfmt` | LogfmtFormatter | None |
| `format_html` | HtmlFormatter | None |
| `format_sql` | SqlFormatter | None |
| `format_text` | TextFormatter | None |
| `format_json` | JsonFormatter | serde, serde_json |
| `format_yaml` | YamlFormatter | serde, serde_yaml |
| `format_toml` | TomlFormatter | serde, toml |

#### Bundles

| Bundle | Includes |
|--------|----------|
| `visual_formats` (default) | `format_table` + `format_expanded` + `format_tree` + `format_logfmt` |
| `web_formats` | `format_html` + `format_sql` |
| `data_formats` | `format_json` + `format_yaml` + `format_toml` |
| `all_formats` | `visual_formats` + `web_formats` + `data_formats` + `format_text` + `themes` |

#### Other Features

| Feature | Purpose |
|---------|---------|
| `themes` | Predefined color schemes for visual formatters |
| `serde_support` | Enables serde derives on data structures (required by data formatters) |
| `integration` | Legacy no-op (all tests run unconditionally) |

### ANSI / Unicode Helpers

```rust
/// Visual character count ignoring ANSI escape sequences (codepoint count, not display width)
pub fn visual_len( text : &str ) -> usize;

/// Pad string to target display width (display-width-aware via unicode-width crate)
pub fn pad_to_width( text : &str, target_width : usize, align_right : bool ) -> String;
```

`visual_len` counts Unicode codepoints. `pad_to_width` uses East Asian Width for terminal column alignment (CJK/emoji = 2 columns).

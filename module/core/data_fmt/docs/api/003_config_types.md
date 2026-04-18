# API: Config Types

### Scope

- **Purpose**: Document the public API surface for configuration and style types.
- **Responsibility**: Define enums and config structs that control formatter output appearance.
- **In Scope**: Config struct fields, preset constructors, builder setters, width calculation order.
- **Out of Scope**: Behavioral invariants (see `../invariant/`), construction patterns (see `../builder/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/config.rs` | Configuration type definitions |
| test | `tests/table_config_corner_cases.rs` | Config edge case tests |

### BorderVariant

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum BorderVariant
{
  None,       // No borders, space-separated
  Ascii,      // Pipe borders: | + -
  AsciiGrid,  // Full ASCII grid: +---+
  Unicode,    // Unicode box drawing
  Markdown,   // Markdown table: | col |
}
```

### HeaderSeparatorVariant

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum HeaderSeparatorVariant
{
  None,       // No separator line
  Dash,       // Dash-only: -----
  AsciiGrid,  // ASCII grid: +-----+
  Unicode,    // Unicode: ├─────┤
  Markdown,   // Markdown: |-----|
}
```

### ColumnSeparator

```rust
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum ColumnSeparator
{
  Spaces( usize ),    // N spaces between columns
  Character( char ),  // Single character (|, comma, tab)
  String( String ),   // Custom string separator
}
```

### PaddingSide

Controls alignment padding placement in expanded format key-value pairs.

```rust
pub enum PaddingSide
{
  /// Pad keys before separator: "Name   | Value"
  BeforeSeparator,
  /// Pad values after separator: "Name: Value"
  AfterSeparator,
}
```

### ColumnFlex

Per-column classification for auto-fit budget allocation.

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ColumnFlex
{
  Fixed,  // keeps natural width; never wrapped or folded
  Flex,   // shrinks to budget; content wraps if needed
}
```

When `TableConfig::column_flex` is empty (default), columns are auto-classified: max cell width ≤ 12 display chars = `Fixed`, otherwise `Flex`.

### FoldStyle

Controls the format of continuation lines when columns are folded.

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum FoldStyle
{
  Bare,      // values only, no labels
  Labeled,   // "ColName: value" pairs (default)
  Stacked,   // each folded column on its own line with label
}
```

### TableConfig

Formatter parameters for table output. All fields are private; use preset constructors or builder setters.

#### Fields (private)

| Field | Type | Purpose |
|-------|------|---------|
| `column_widths` | `Vec< usize >` | Manual column width overrides |
| `align_right` | `Vec< bool >` | Per-column right-alignment flags |
| `border_variant` | `BorderVariant` | Border rendering style |
| `header_separator_variant` | `HeaderSeparatorVariant` | Header separator style |
| `column_separator` | `ColumnSeparator` | Inter-column separator |
| `outer_padding` | `bool` | Padding outside border edges |
| `inner_padding` | `usize` | Spaces inside cell boundaries |
| `colorize_header` | `bool` | Apply ANSI color to header row |
| `header_color` | `String` | ANSI color code for header |
| `alternating_rows` | `bool` | Alternate row background colors |
| `row_color1` | `String` | Even row ANSI color |
| `row_color2` | `String` | Odd row ANSI color |
| `min_column_width` | `usize` | Minimum display width per column |
| `max_column_width` | `Option< usize >` | Maximum display width (truncates beyond) |
| `truncation_marker` | `String` | Appended to truncated cells (default: "...") |
| `sub_row_indent` | `String` | Prefix for sub-row detail lines (default: "  ") |
| `terminal_width` | `Option< usize >` | Target width for auto-fit (None = auto-detect via `terminal_size` feature; fallback: 120) |
| `auto_wrap` | `bool` | Auto-wrap flex cells at budget width (default: true) |
| `auto_fold` | `bool` | Auto-fold overflow columns to continuation lines (default: true) |
| `column_flex` | `Vec< ColumnFlex >` | Per-column flex classification (empty = auto-classify) |
| `fold_style` | `FoldStyle` | Continuation line format for folded columns (default: Labeled) |
| `fold_indent` | `String` | Indent prefix for folded continuation lines (default: "    ") |

#### Preset Constructors

```rust
impl TableConfig
{
  pub fn new() -> Self;          // Default (bordered)
  pub fn plain() -> Self;        // Space-separated, dash separator
  pub fn minimal() -> Self;      // Space-separated, no separator
  pub fn bordered() -> Self;     // Pipe borders, dash+plus separator
  pub fn markdown() -> Self;     // Markdown table syntax
  pub fn grid() -> Self;         // Full ASCII grid (+---+)
  pub fn unicode_box() -> Self;  // Unicode box drawing
  pub fn csv() -> Self;          // Comma-separated
  pub fn tsv() -> Self;          // Tab-separated
  pub fn compact() -> Self;      // Minimal spacing
}
```

#### Builder Methods

All return `Self`, all marked `#[ must_use ]`.

```rust
impl TableConfig
{
  pub fn column_widths( self, widths : Vec< usize > ) -> Self;
  pub fn align_right( self, align : Vec< bool > ) -> Self;
  pub fn border_variant( self, variant : BorderVariant ) -> Self;
  pub fn header_separator_variant( self, variant : HeaderSeparatorVariant ) -> Self;
  pub fn column_separator( self, sep : ColumnSeparator ) -> Self;
  pub fn outer_padding( self, enabled : bool ) -> Self;
  pub fn inner_padding( self, spaces : usize ) -> Self;
  pub fn colorize_header( self, enabled : bool ) -> Self;
  pub fn header_color( self, color : String ) -> Self;
  pub fn alternating_rows( self, enabled : bool ) -> Self;
  pub fn row_colors( self, color1 : String, color2 : String ) -> Self;
  pub fn min_column_width( self, width : usize ) -> Self;
  pub fn max_column_width( self, width : Option< usize > ) -> Self;
  pub fn truncation_marker( self, marker : String ) -> Self;
  pub fn sub_row_indent( self, indent : String ) -> Self;
  pub fn terminal_width( self, width : Option< usize > ) -> Self;
  pub fn auto_wrap( self, enabled : bool ) -> Self;
  pub fn column_flex( self, flex : Vec< ColumnFlex > ) -> Self;
  pub fn auto_fold( self, enabled : bool ) -> Self;
  pub fn fold_style( self, style : FoldStyle ) -> Self;
  pub fn fold_indent( self, indent : String ) -> Self;
}
```

#### Width Calculation Order

1. Content-driven max: `max( header_width, max( row_widths ) )`
2. Cap: `min( width, max_column_width )` if set
3. Floor: `max( width, min_column_width )` if non-zero
4. Override: `column_widths` replaces all calculated widths (skips cap/floor)
5. Auto-fit budget (when `auto_wrap` is true): flex columns shrink to terminal budget; cells auto-wrap
6. Auto-fold (when `auto_fold` is true and total still exceeds terminal): overflow columns fold to continuation lines

See `../feature/auto_fit.md` for full auto-fit pipeline.

### ExpandedConfig

Formatter parameters for expanded (vertical record) output.

```rust
#[ derive( Debug, Clone ) ]
pub struct ExpandedConfig
{
  pub record_separator : String,
  pub key_value_separator : String,
  pub show_record_numbers : bool,
  pub colorize_keys : bool,
  pub key_color : String,
  pub padding_side : PaddingSide,
  pub indent_prefix : String,
}
```

#### Preset Constructors

```rust
impl ExpandedConfig
{
  pub fn new() -> Self;              // Default (postgres style)
  pub fn postgres_style() -> Self;   // Aligned keys, pipe separator
  pub fn property_style() -> Self;   // Colon separator, after-separator padding
}
```

#### Builder Methods

All return `Self`, all marked `#[ must_use ]`.

```rust
impl ExpandedConfig
{
  pub fn record_separator( self, separator : String ) -> Self;
  pub fn key_value_separator( self, separator : String ) -> Self;
  pub fn show_record_numbers( self, show : bool ) -> Self;
  pub fn colorize_keys( self, enable : bool ) -> Self;
  pub fn key_color( self, color : String ) -> Self;
  pub fn padding_side( self, side : PaddingSide ) -> Self;
  pub fn indent_prefix( self, prefix : String ) -> Self;
}
```

### TreeConfig

Formatter parameters for hierarchical tree output.

```rust
#[ derive( Debug, Clone ) ]
pub struct TreeConfig
{
  pub show_branches : bool,
  pub show_root : bool,
  pub indent_size : usize,
  pub max_depth : Option< usize >,
  pub column_separator : String,
  pub min_column_width : usize,
}
```

#### Constructor and Builders

```rust
impl TreeConfig
{
  pub fn new() -> Self;

  #[ must_use ]
  pub fn show_branches( self, show : bool ) -> Self;
  #[ must_use ]
  pub fn show_root( self, show : bool ) -> Self;
  #[ must_use ]
  pub fn indent_size( self, size : usize ) -> Self;
  #[ must_use ]
  pub fn max_depth( self, depth : Option< usize > ) -> Self;
  #[ must_use ]
  pub fn column_separator( self, separator : String ) -> Self;
  #[ must_use ]
  pub fn min_column_width( self, width : usize ) -> Self;
}
```

# API: Data Types

### Scope

- **Purpose**: Document the public API surface for core data representation types.
- **Responsibility**: Define structs and enums used to hold hierarchical and tabular data.
- **In Scope**: Type definitions, field semantics, construction methods, and trait impls on data types.
- **Out of Scope**: Behavioral invariants (see `../invariant/`), construction patterns (see `../builder/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/data.rs` | Core data type definitions |
| test | `tests/data.rs` | Data type tests |
| doc | `../input_type/001_table_view.md` | TableView type details |
| doc | `../input_type/002_tree_node.md` | TreeNode type details |

### TreeNode< T >

Universal hierarchical data structure. All formatters consume this type.

```rust
#[ derive( Debug, Clone ) ]
pub struct TreeNode< T >
{
  pub name : String,
  pub data : Option< T >,
  pub children : Vec< TreeNode< T > >,
}

impl< T > TreeNode< T >
{
  pub const fn new( name : String, data : Option< T > ) -> Self;
}
```

- **Invariant**: Directory nodes have `data = None`, leaf nodes have `data = Some( T )`.
- **Generic**: Works with any `T`. Use `T : Display` for conversion utilities.

### TableView

Canonical data format for the unified `Format` trait. All formatters accept `&TableView`.

```rust
#[ derive( Debug, Clone ) ]
pub struct TableView
{
  pub metadata : TableMetadata,
  pub rows : Vec< Vec< String > >,
  pub row_details : Vec< Option< DecoratedText > >,
}

impl TableView
{
  pub fn new( metadata : TableMetadata, rows : Vec< Vec< String > > ) -> Self;
  pub fn with_details(
    metadata : TableMetadata,
    rows : Vec< Vec< String > >,
    row_details : Vec< Option< DecoratedText > >,
  ) -> Self;
  pub fn to_tree_node( &self ) -> TreeNode< Vec< String > >;
}
```

- `new()` defaults `row_details` to `vec![]` (backward compatible).
- `with_details()` accepts an explicit `row_details` vector parallel to `rows`.
- Build via `RowBuilder::build_view()`. Convert back to `TreeNode` with `to_tree_node()` for backward compatibility with visual formatters.

### TableMetadata

Semantic column metadata attached to `TableView`.

```rust
#[ derive( Debug, Clone ) ]
pub struct TableMetadata
{
  pub column_names : Vec< String >,
  pub column_types : Vec< DataType >,
}

impl TableMetadata
{
  /// Create with column names; all types default to DataType::String
  pub fn new( column_names : Vec< String > ) -> Self;
  /// Create with explicit per-column types
  pub fn with_types( column_names : Vec< String >, column_types : Vec< DataType > ) -> Self;
}
```

### DataType

Column type classification for type-aware formatting.

```rust
#[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
pub enum DataType
{
  #[ default ]
  String,
  Integer,
  Boolean,
  Path,
}
```

### ColumnData

Multi-column data payload for `TreeNode< ColumnData >`, used with `TreeFormatter::format_aligned`.

```rust
#[ derive( Debug, Clone ) ]
pub struct ColumnData
{
  pub columns : Vec< String >,
}

impl ColumnData
{
  pub fn new( columns : Vec< String > ) -> Self;
  pub fn from_pairs( pairs : Vec< ( &str, &str ) > ) -> Self;
  pub fn len( &self ) -> usize;
  pub fn is_empty( &self ) -> bool;
}

impl std::fmt::Display for ColumnData { /* joins columns with "  " */ }
```

### TreeSymbols

Box-drawing characters for tree rendering.

```rust
#[ derive( Debug, Clone ) ]
pub struct TreeSymbols
{
  pub branch : &'static str,
  pub last_branch : &'static str,
  pub vertical : &'static str,
  pub space : &'static str,
}
```

Default symbols: `branch = "├── "`, `last_branch = "└── "`, `vertical = "│   "`, `space = "    "`.

### TableShapedView Trait

Trait for extracting tabular data from tree structures.

```rust
pub trait TableShapedView
{
  fn extract_headers( &self ) -> Option< Vec< String > >;
  fn is_table_shaped( &self ) -> bool;
  fn to_rows( &self ) -> Vec< Vec< String > >;
}

impl< T : std::fmt::Display > TableShapedView for TreeNode< T >
{
  fn extract_headers( &self ) -> Option< Vec< String > >;
  fn is_table_shaped( &self ) -> bool;
  fn to_rows( &self ) -> Vec< Vec< String > >;
}
```

Converts `T` to `String` via `Display` trait. Used internally by `TableFormatter` and `ExpandedFormatter` to extract headers and rows from a `TreeNode`.

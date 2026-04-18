# API: Builders

### Scope

- **Purpose**: Document the public API surface for builder types.
- **Responsibility**: Define builder struct signatures, method chains, and terminal operations.
- **In Scope**: RowBuilder and TreeBuilder public methods, FlattenConfig, flatten functions.
- **Out of Scope**: Behavioral invariants (see `../invariant/`), construction patterns (see `../builder/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/builder.rs` | Builder implementation |
| test | `tests/builder.rs` | Builder API tests |
| doc | `../builder/001_row_builder.md` | RowBuilder construction patterns |
| doc | `../builder/002_tree_builder.md` | TreeBuilder construction patterns |

### RowBuilder

Fluent builder for constructing table-shaped trees from header + row data.

```rust
pub struct RowBuilder
{
  root : TreeNode< String >,
  headers : Vec< String >,
  row_count : usize,
  rows : Vec< Vec< String > >,
  row_details : Vec< Option< String > >,
}

impl RowBuilder
{
  pub fn new( headers : Vec< String > ) -> Self;

  // Fluent API (consuming self, chainable)
  #[ must_use ]
  pub fn add_row( self, row : Vec< String > ) -> Self;
  #[ must_use ]
  pub fn add_row_with_name( self, row_name : String, row : Vec< String > ) -> Self;
  #[ must_use ]
  pub fn add_row_with_detail( self, row : Vec< String >, detail : Option< String > ) -> Self;

  // Mutable API (for programmatic / loop-based construction)
  pub fn add_row_mut( &mut self, row : Vec< String > );
  pub fn add_row_with_name_mut( &mut self, row_name : String, row : Vec< String > );
  pub fn add_row_with_detail_mut( &mut self, row : Vec< String >, detail : Option< String > );

  // Terminal operations
  pub fn build( self ) -> TreeNode< String >;
  pub fn build_view( self ) -> TableView;
}
```

- **Invariant**: All rows must have length equal to `headers.len()`.
- **Invariant**: `rows` and `row_details` vectors are always parallel (same length).
- `build()` returns a `TreeNode< String >` for use with visual formatters.
- `build_view()` returns a `TableView` for use with the unified `Format` trait.

### TreeBuilder< T >

Path-based tree construction. Intermediate directory nodes are created automatically.

```rust
pub struct TreeBuilder< T >
{
  root : TreeNode< T >,
}

impl< T > TreeBuilder< T >
{
  pub fn new( root_name : impl Into< String > ) -> Self;
  pub fn insert( self, path : &[ &str ], data : T ) -> Self;
  pub fn build( self ) -> TreeNode< T >;
}

impl< T : Clone > TreeBuilder< T >
{
  pub fn from_items< I, P >( root_name : impl Into< String >, items : I ) -> Self
  where
    I : IntoIterator< Item = ( P, T ) >,
    P : AsRef< [ String ] >;
}
```

- `insert` splits the path into components, traverses/creates intermediate nodes, and places `data` at the leaf.
- `from_items` batch-constructs from an iterator of `( path, data )` pairs.

### FlattenConfig

Configuration for `flatten_to_table_tree` column selection and naming.

```rust
pub struct FlattenConfig
{
  pub include_path : bool,
  pub include_name : bool,
  pub include_depth : bool,
  pub include_data : bool,
  pub column_names : Option< ( String, String, String, String ) >,
}

impl FlattenConfig
{
  pub fn new() -> Self;

  #[ must_use ]
  pub fn include_path( self, include : bool ) -> Self;
  #[ must_use ]
  pub fn include_name( self, include : bool ) -> Self;
  #[ must_use ]
  pub fn include_depth( self, include : bool ) -> Self;
  #[ must_use ]
  pub fn include_data( self, include : bool ) -> Self;
  #[ must_use ]
  pub fn column_names( self, path : String, name : String, depth : String, data : String ) -> Self;
}
```

Defaults: all four columns included, names = `"path"`, `"name"`, `"depth"`, `"data"`.

### Flatten Functions

```rust
/// Flatten hierarchical tree to table-shaped tree with default columns
pub fn flatten_to_table_tree< T : Display >( tree : &TreeNode< T > ) -> TreeNode< String >;

/// Flatten with custom column selection and naming
pub fn flatten_to_table_tree_with_config< T : Display >(
  tree : &TreeNode< T >,
  config : &FlattenConfig,
) -> TreeNode< String >;
```

DFS traversal produces one row per node with columns: path, name, depth, data (configurable via `FlattenConfig`).

# API: Builders

### Scope

- **Purpose**: Document the public API surface for builder types.
- **Responsibility**: Define builder struct signatures, method chains, and terminal operations.
- **In Scope**: RowBuilder and TreeBuilder public methods, FlattenConfig, flatten functions.
- **Out of Scope**: Behavioral invariants (see `../invariant/`), construction patterns (see `../builder/`).

### Builders

| File | Relationship |
|------|-------------|
| [001_row_builder.md](../builder/001_row_builder.md) | RowBuilder construction patterns |
| [002_tree_builder.md](../builder/002_tree_builder.md) | TreeBuilder construction patterns |

### Sources

| File | Relationship |
|------|-------------|
| [`src/builder.rs`](../../src/builder.rs) | Builder implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/builder.rs`](../../tests/builder.rs) | Builder API tests |

### Abstract

Three builder types and two conversion functions form the construction API. `RowBuilder` assembles tabular data row by row from headers and string values, producing `TableView` for the `Format` trait path. `TreeBuilder< T >` assembles hierarchical trees from path-based insertions, creating intermediate directory nodes automatically. `FlattenConfig` and the `flatten_to_table_tree` functions convert a hierarchical `TreeNode< T >` into a flat table-shaped tree, extracting path, name, depth, and data as columns.

### Operations

#### RowBuilder

Fluent and mutable builder for constructing table-shaped data from headers and rows. Initialized with `RowBuilder::new( headers : Vec< String > )`.

**Fluent API** (consuming self, chainable): `add_row( row )`, `add_row_with_name( name, row )`, `add_row_with_detail( row, detail )`. All are `#[ must_use ]`.

**Mutable API** (for loop-based construction): `add_row_mut( &mut self, row )`, `add_row_with_name_mut( &mut self, name, row )`, `add_row_with_detail_mut( &mut self, row, detail )`.

**Terminal operation**: `build_view( self ) -> TableView` (use with `Format` trait).

#### TreeBuilder< T >

Path-based tree constructor. Initialized with `TreeBuilder::new( root_name )`. `insert( self, path : &[ &str ], data : T ) -> Self` traverses or creates intermediate directory nodes and places `data` at the leaf. `from_items< I, P >( root_name, items : I ) -> Self` batch-constructs from an iterator of `( path, data )` pairs (requires `T : Clone`). Terminal operation: `build( self ) -> TreeNode< T >`.

#### FlattenConfig

Configuration for which columns to include when flattening a hierarchical tree to a table-shaped tree. Boolean fields `include_path`, `include_name`, `include_depth`, `include_data` control column inclusion (all default to `true`). Optional `column_names` tuple overrides the default column name strings `"path"`, `"name"`, `"depth"`, `"data"`. All builder setters are `#[ must_use ]` and return `Self`.

#### Flatten Functions

`flatten_to_table_tree< T : Display >( tree : &TreeNode< T > ) -> TableView` produces a table view with four default columns. `flatten_to_table_tree_with_config< T : Display >( tree, config : &FlattenConfig ) -> TableView` applies a `FlattenConfig` to control column inclusion and naming. Both perform a DFS traversal emitting one row per node.

### Error Handling

Builder construction does not return `Result`. `RowBuilder` panics if a row's length does not equal `headers.len()`. `TreeBuilder::insert` with an empty path slice inserts directly at the root. Flatten functions propagate no errors; `Display` conversion is infallible.

### Compatibility Guarantees

`RowBuilder::build_view()` is stable. `RowBuilder::build()` was removed in v0.3.0; callers must use `build_view()` with the `Format` trait. The fluent and mutable APIs are additive — callers using `add_row` continue working after new `add_row_with_detail` methods were introduced. `FlattenConfig` defaults all fields to `true` for maximum column inclusion; new optional columns added in future versions will also default to `true`.

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

### Abstract

The core data layer consists of five types and one extraction trait. `TreeNode< T >` is the universal hierarchical container for both tree-structured and tabular data. `TableView` is the canonical flat representation consumed by all `Format`-trait formatters. `TableMetadata` carries column names and type classifications alongside a `TableView`. `ColumnData` wraps multi-column values for use with aligned tree rendering. `TreeSymbols` customizes box-drawing characters for tree output. The `TableShapedView` trait provides generic extraction of headers and rows from any `TreeNode< T : Display >`.

### Operations

#### TreeNode< T >

A generic tree node with three public fields: `name : String` (node label), `data : Option< T >` (leaf payload; `None` for directory nodes, `Some( T )` for data-bearing leaf nodes), and `children : Vec< TreeNode< T > >` (child nodes). Constructed via `TreeNode::new( name, data )`. Generic over any `T`; use `T : Display` to enable `TableShapedView` extraction and `TableView` conversion.

#### TableView

The canonical interchange format for the `Format` trait. Holds `metadata : TableMetadata`, `rows : Vec< Vec< String > >`, and `row_details : Vec< Option< DecoratedText > >`. The primary construction path is `RowBuilder::build_view()`. Two direct constructors exist for advanced use: `TableView::new( metadata, rows )` (defaults `row_details` to empty) and `TableView::with_details( metadata, rows, row_details )` (explicit parallel vector). Converts back to `TreeNode< Vec< String > >` via `TableView::to_tree_node()` for visual formatters that predate the `Format` trait.

#### TableMetadata

Semantic column metadata carried alongside a `TableView`. Holds `column_names : Vec< String >` and `column_types : Vec< DataType >`. Two constructors: `TableMetadata::new( column_names )` (all types default to `DataType::String`) and `TableMetadata::with_types( column_names, column_types )` (explicit per-column types).

#### DataType

Enum classifying column value types for type-aware formatters. Variants: `String` (default), `Integer`, `Boolean`, `Path`. Implements `Default` (returns `String`). Used by formatters that render column values differently based on their semantic type.

#### ColumnData

Multi-column payload for use with `TreeNode< ColumnData >` and `TreeFormatter::format_aligned`. Wraps `columns : Vec< String >`. Constructed via `ColumnData::new( columns )` or `ColumnData::from_pairs( &[( &str, &str )] )`. Provides `len()` and `is_empty()` to report column count. `Display` implementation joins columns with two spaces.

#### TreeSymbols

Customizable box-drawing characters for tree output, passed to `TreeFormatter::with_symbols()`. Four `&'static str` fields: `branch` (branch connector, default `"├── "`), `last_branch` (last-child connector, default `"└── "`), `vertical` (continuation line, default `"│   "`), and `space` (blank continuation, default `"    "`).

#### TableShapedView Trait

Implemented for `TreeNode< T >` where `T : Display`. Three methods: `is_table_shaped()` (checks whether the tree encodes row/column structure), `extract_headers()` (returns `Option< Vec< String > >` of column names if table-shaped), and `to_rows()` (converts leaf `T` values to `String` via `Display` for row extraction). Used internally by `TableFormatter` and `ExpandedFormatter`.

### Error Handling

Data type construction does not return errors. `RowBuilder` enforces at construction time that each row has the same length as the header vector. `TableView::with_details` requires `row_details.len() == rows.len()`. No `Result`-returning constructors exist in this surface.

### Compatibility Guarantees

`TreeNode< T >`, `TableView`, and `TableMetadata` are stable public types. `TableView::new()` defaults `row_details` to an empty vector for backward compatibility with callers predating `DecoratedText` support. `TableShapedView` is implemented for any `TreeNode< T : Display >` — callers that add `Display` to custom `T` types gain trait coverage automatically. `ColumnData` and `TreeSymbols` are stable. `DataType` supports additive new variants without breaking existing match arms when used with `_` fallback patterns.

# Invariant: Data Model

### Scope

- **Purpose**: Define the structural contracts and validation rules that `TreeNode< T >`, `RowBuilder`, and `TableView` maintain across all operations.
- **Responsibility**: Documents data model invariants for tree nodes, row builders, and table views.
- **In Scope**: TreeNode design invariants, RowBuilder contracts, TableView interchange format, edge case contracts.
- **Out of Scope**: Rendering algorithms (see `algorithm/` docs) and ANSI/Unicode handling (see `invariant/002_ansi_unicode.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/data.rs` | Core data type definitions |
| source | `src/builder.rs` | Builder contract enforcement |
| test | `tests/data.rs` | Data type invariant tests |
| test | `tests/builder.rs` | Builder contract tests |

### TreeNode Design Invariants

#### Directory vs File Nodes

- **Directory nodes**: `data = None`, may have children
- **File (leaf) nodes**: `data = Some(T)`, typically no children

#### Hierarchical Trees

- Unlimited nesting depth
- Any node may have zero or more children
- Generic over `T` with minimal trait bounds

#### Table-Shaped Trees

Table-shaped trees encode tabular data as a specific tree structure:

```
root
  1 (row name)
    sid: "3"
    sname: "Alice"
    gap: "5"
  2 (row name)
    sid: "6"
    sname: "Joe"
    gap: "1"
```

Structural rules:

- Root has row nodes as direct children
- Each row node has column-named children holding cell values
- **Table validation invariant**: all row nodes must have identical child structure (same column names in the same order)

### RowBuilder Invariants

Every row added via `add_row()`, `add_row_mut()`, or `add_row_with_detail()` must have length equal to `headers.len()`. The builder enforces this at construction time so that downstream formatters never encounter ragged rows.

**Parallel vectors invariant**: `rows` and `row_details` are always the same length. Every `add_row_internal()` call pushes to both vectors simultaneously. Rows added without explicit detail get `None`.

```rust
pub struct RowBuilder
{
  root : TreeNode< String >,
  headers : Vec< String >,
  row_count : usize,
  rows : Vec< Vec< String > >,
  row_details : Vec< Option< DecoratedText > >,
}
```

Construction patterns:

- **Fluent**: `RowBuilder::new( headers ).add_row( r1 ).add_row_with_detail( r2, detail ).build_view()`
- **Mutable**: call `builder.add_row_mut( row )` / `builder.add_row_with_detail_mut( row, detail )` in a loop, then `builder.build_view()`
- **Manual**: use `TreeBuilder` for custom row naming

### TableView as Canonical Interchange Format

`TableView` is the format-agnostic data structure that all formatters can consume. It holds extracted headers and rows as plain `Vec< String >` values, decoupled from `TreeNode` internals.

`TableShapedView` trait provides generic extraction:

```rust
pub trait TableShapedView
{
  fn extract_headers( &self ) -> Option< Vec< String > >;
  fn is_table_shaped( &self ) -> bool;
  fn to_rows( &self ) -> Vec< Vec< String > >;
}
```

Implemented for `TreeNode< T >` where `T : Display`. The `to_rows()` method converts `T` values to `String` via the `Display` trait.

### Edge Case Contracts

- **EC-1**: Empty tables return empty string in all formats
- **EC-2**: Empty trees return empty string when formatted
- **EC-3**: Single-row tables display correctly in all formats
- **EC-4**: Generic `TableShapedView` works with any `T : Display`

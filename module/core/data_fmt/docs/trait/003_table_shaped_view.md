# Trait: TableShapedView

### Scope

- **Purpose**: Document the TableShapedView interface contract, implementors, and coverage.
- **Responsibility**: Define the input-side trait for extracting tabular data from tree structures.
- **In Scope**: Trait definition, method table, blanket impl, pipeline role, relationship to TableView.
- **Out of Scope**: Formatter implementation (see `../feature/`), variant output (see `../variant/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/table_tree.rs` | TableShapedView implementation |
| test | `tests/formatters.rs` | View extraction tests |

### Signature

`TableShapedView` defines three methods. `extract_headers( &self ) -> Option<Vec<String>>` returns the column names from the first row's children, or `None` if the tree is not table-shaped. `is_table_shaped( &self ) -> bool` checks that all rows have identical column structure. `to_rows( &self ) -> Vec<Vec<String>>` returns cell values as a flat matrix — one inner vec per row, one element per column.

### Implementors

| Type | Bound | Provided By |
|------|-------|-------------|
| `TreeNode<T>` | `T : Display` | Blanket impl in `src/table_tree.rs` |

There is exactly one implementation: a blanket impl for all `TreeNode<T>` where `T` implements `Display`. Cell values are produced via `T::to_string()`.

### Coverage Gaps

No known gaps for the current use case. `TableView` makes this trait largely redundant for new code — it stores headers and rows directly without tree encoding. `TableShapedView` exists for backward compatibility with the `TreeNode<String>` path used by `TableShapedFormatter`.

### Methods

| Method | Returns | Purpose |
|--------|---------|---------|
| `extract_headers()` | `Option<Vec<String>>` | Column names from the first row's children |
| `is_table_shaped()` | `bool` | All rows have identical column structure |
| `to_rows()` | `Vec<Vec<String>>` | Cell values, one inner vec per row |

### Role in the Pipeline

```text
RowBuilder::build() → TreeNode<String>
                           │
                    TableShapedView
                     ├── extract_headers()
                     ├── to_rows()
                     └── is_table_shaped()
                           │
                    TableShapedFormatter::format()
                           │
                      String output
```

Used internally by `TableFormatter` and `ExpandedFormatter` to decompose a tree-encoded table into headers and rows before rendering.

### Relationship to TableView

`TableView` makes this trait largely redundant for new code — it stores headers and rows directly without tree encoding. `TableShapedView` exists for backward compatibility with the `TreeNode<String>` path.

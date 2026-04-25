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

`TableShapedView` defines three methods: `extract_headers` returns column names from the first row's children (absent if the tree is not table-shaped); `is_table_shaped` checks that all rows have identical column structure; `to_rows` returns cell values as a flat matrix, one row per entry.

### Implementors

| Implementor | Provided By |
|-------------|-------------|
| `TreeNode` (display-capable data) | Blanket impl in `src/table_tree.rs` |

There is exactly one implementation: a blanket impl for all tree nodes whose data type supports display formatting. Cell values are converted to strings at extraction time.

### Coverage Gaps

No known gaps for the current use case. `TableView` makes this trait largely redundant for new code — it stores headers and rows directly without tree encoding. `TableShapedView` exists for backward compatibility with the table-encoded tree path used by `TableShapedFormatter`.

### Methods

| Method | Purpose |
|--------|---------|
| `extract_headers()` | Returns column names from the first row's children, or nothing if not table-shaped |
| `is_table_shaped()` | Returns whether all rows have identical column structure |
| `to_rows()` | Returns cell values as a matrix, one row per inner list |

### Role in the Pipeline

The tree produced by `RowBuilder::build()` implements `TableShapedView`. `TableFormatter` and `ExpandedFormatter` call `extract_headers()`, `to_rows()`, and `is_table_shaped()` internally to decompose the tree-encoded table into headers and rows before rendering.

### Relationship to TableView

`TableView` makes this trait largely redundant for new code — it stores headers and rows directly without tree encoding. `TableShapedView` exists for backward compatibility with the table-encoded tree path.

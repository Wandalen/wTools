# Trait: TableShapedView

### Scope

- **Purpose**: Document the TableShapedView interface contract, implementors, and coverage.
- **Responsibility**: Define the input-side trait for extracting tabular data from tree structures.
- **In Scope**: Trait definition, method table, blanket impl, pipeline role, relationship to TableView.
- **Out of Scope**: Formatter implementation (see `../feature/`), variant output (see `../variant/`).

### Sources

| File | Relationship |
|------|-------------|
| [`src/data.rs`](../../src/data.rs) | TableShapedView trait definition and blanket implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/formatters.rs`](../../tests/formatters.rs) | View extraction tests |

### Signature

`TableShapedView` defines three methods: `extract_headers` returns column names from the first row's children (absent if the tree is not table-shaped); `is_table_shaped` checks that all rows have identical column structure; `to_rows` returns cell values as a flat matrix, one row per entry.

### Implementors

| Implementor | Provided By |
|-------------|-------------|
| `TreeNode` (display-capable data) | Blanket impl in `src/data.rs` |

There is exactly one implementation: a blanket impl for all tree nodes whose data type supports display formatting. Cell values are converted to strings at extraction time.

### Coverage Gaps

No known gaps for the current use case. `TableView` makes this trait largely redundant for new code — it stores headers and rows directly without tree encoding. `TableShapedView` exists for backward compatibility with the table-encoded tree path formerly used by `TableShapedFormatter`, which was removed in v0.3.0 (see `trait/002_table_shaped_formatter.md`).

### Methods

| Method | Purpose |
|--------|---------|
| `extract_headers()` | Returns column names from the first row's children, or nothing if not table-shaped |
| `is_table_shaped()` | Returns whether all rows have identical column structure |
| `to_rows()` | Returns cell values as a matrix, one row per inner list |

### Role in the Pipeline

Neither `TableFormatter` nor `ExpandedFormatter` calls `extract_headers()`, `to_rows()`, or `is_table_shaped()` internally — both consume `TableView` directly via the `Format` trait, built through `RowBuilder::build_view()` (which accumulates rows directly and does not construct a `TreeNode` at all). `TableShapedView` is implemented via a blanket impl over any `TreeNode<T>` whose data type implements `Display`, and remains available to callers that already hold a table-shaped `TreeNode` and want direct header/row extraction without going through `RowBuilder`.

### Relationship to TableView

`TableView` makes this trait largely redundant for new code — it stores headers and rows directly without tree encoding. `TableShapedView` exists for backward compatibility with the table-encoded tree path.

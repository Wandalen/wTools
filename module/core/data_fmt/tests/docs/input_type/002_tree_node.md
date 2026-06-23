# Input Type: TreeNode

### Scope

- **Purpose**: Drive test coverage for the `TreeNode` generic struct.
- **Responsibility**: Documents test cases for the TreeNode type in `docs/input_type/002_tree_node.md`.
- **In Scope**: Struct fields (name, data, children), generic hierarchical specialization, multi-column ColumnData specialization, TableShapedView trait extraction, legacy tabular removal.
- **Out of Scope**: Conceptual data shape (see `../input_model/`), tree formatter output (see `../formatter/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| IV-1 | generic hierarchical specialization stores typed leaf data | ⏳ |
| IV-2 | multi-column specialization uses ColumnData | ⏳ |
| IV-3 | TableShapedView extracts headers and rows from tree | ⏳ |
| IV-4 | legacy tabular specialization removed in v0.3.0 | ⏳ |

---

### IV-1: generic hierarchical specialization stores typed leaf data

- **Given:** A `TreeNode<i64>` with root `"project"`, child `"src"`, and leaf `"main.rs"` with data `Some(150)`.
- **When:** Traversing the tree structure.
- **Then:** `root.name` is `"project"`; `root.data` is `None` (directory node); leaf node `"main.rs"` has `data == Some(150)`; the type parameter `i64` constrains all data payloads uniformly.

---

### IV-2: multi-column specialization uses ColumnData

- **Given:** A `TreeNode<ColumnData>` where leaf data contains multiple column values per node.
- **When:** Passed to `TreeFormatter::format_aligned()`.
- **Then:** Columns are aligned across all leaf nodes; each `ColumnData` value contributes one column in the aligned output; directory nodes do not carry column data.

---

### IV-3: TableShapedView extracts headers and rows from tree

- **Given:** A `TreeNode<String>` with table-shaped structure (root → 2 row children → column-named leaf children with cell data).
- **When:** `TableShapedView` trait methods are called to extract headers and rows.
- **Then:** Headers match the column names from the first row's children; rows are flat string vectors with one entry per column; row count matches the number of row children; `Clone` and `Debug` traits are implemented.

---

### IV-4: legacy tabular specialization removed in v0.3.0

- **Given:** Code attempting to use `RowBuilder::build()` (returns `TreeNode<String>` for table-encoded trees).
- **When:** Compiled against data_fmt v0.3.0+.
- **Then:** `RowBuilder::build()` is not available; `build_view()` is the replacement returning `TableView`; `TableShapedFormatter` trait is removed; callers must use `Format` trait instead.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/input_type/002_tree_node.md`](../../../docs/input_type/002_tree_node.md) | Source input type doc — TreeNode struct, specializations, trait implementations |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../data.rs) | Data model test implementation |

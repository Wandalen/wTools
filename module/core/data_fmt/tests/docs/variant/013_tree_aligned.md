# Variant: Tree Aligned

### Scope

- **Purpose**: Drive test coverage for the aligned multi-column tree output variant.
- **Responsibility**: Documents test cases for the aligned variant in `docs/variant/013_tree_aligned.md`.
- **In Scope**: Multi-column alignment across leaf nodes, ColumnData consumption, space-based column separation.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | columns aligned across all leaf nodes | ✅ |
| VT-2 | space-based column separation | ✅ |
| VT-3 | directory nodes show no column data | ✅ |
| VT-4 | single-leaf tree produces aligned output | ✅ |

---

### VT-1: columns aligned across all leaf nodes

- **Given:** A `TreeNode<ColumnData>` with leaves carrying 2-column data of varying widths.
- **When:** Formatted with `TreeFormatter::format_aligned()`.
- **Then:** Each column starts at the same horizontal position across all leaf nodes; shorter values are padded to match the widest value in their column.

---

### VT-2: space-based column separation

- **Given:** A `TreeNode<ColumnData>` with multiple columns per leaf.
- **When:** Formatted with `TreeFormatter::format_aligned()`.
- **Then:** Columns are separated by spaces (not pipes or tabs); the output maintains tree connector characters alongside aligned data.

---

### VT-3: directory nodes show no column data

- **Given:** A `TreeNode<ColumnData>` with directory node `"src"` containing leaf nodes with column data.
- **When:** Formatted with `TreeFormatter::format_aligned()`.
- **Then:** The `"src"` directory line shows only the node name; column data appears only on leaf lines; directory lines do not disrupt column alignment.

---

### VT-4: single-leaf tree produces aligned output

- **Given:** A `TreeNode<ColumnData>` with one root and one leaf carrying 2 columns.
- **When:** Formatted with `TreeFormatter::format_aligned()`.
- **Then:** Output shows root name on first line and leaf with aligned columns on second line; formatting is correct even with only one data row.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/013_tree_aligned.md`](../../../docs/variant/013_tree_aligned.md) | Source variant doc — aligned tree attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/aligned_tree_basic.rs`](../../aligned_tree_basic.rs) | Tree alignment and rendering tests |

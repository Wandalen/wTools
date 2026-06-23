# Variant: Tree Hierarchical

### Scope

- **Purpose**: Drive test coverage for the standard hierarchical tree output variant.
- **Responsibility**: Documents test cases for the hierarchical variant in `docs/variant/012_tree_hierarchical.md`.
- **In Scope**: Unicode box-drawing connectors, hierarchical indentation, leaf data display, empty tree handling.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output uses Unicode box-drawing connectors | ⏳ |
| VT-2 | hierarchical indentation increases with depth | ⏳ |
| VT-3 | leaf nodes display data after arrow | ⏳ |
| VT-4 | single-node tree produces root-only output | ⏳ |

---

### VT-1: output uses Unicode box-drawing connectors

- **Given:** A `TreeNode` with root `"Root"` and children `["Alice", "Bob"]`.
- **When:** Formatted with `TreeFormatter::format()`.
- **Then:** Output contains `├──` for non-last children and `└──` for last children; `│` connects sibling groups vertically.

---

### VT-2: hierarchical indentation increases with depth

- **Given:** A `TreeNode` with root → child → grandchild (3 levels deep).
- **When:** Formatted with `TreeFormatter::format()`.
- **Then:** Each level is indented further from the left margin; grandchild lines have more leading whitespace/connector characters than child lines.

---

### VT-3: leaf nodes display data after arrow

- **Given:** A `TreeNode<i64>` with leaf node `"main.rs"` having data `Some(150)`.
- **When:** Formatted with `TreeFormatter::format()`.
- **Then:** The leaf line includes the data value after the node name (e.g., `main.rs → 150` or similar separator); directory nodes show only their name.

---

### VT-4: single-node tree produces root-only output

- **Given:** A `TreeNode` with only a root node `"Root"` and no children.
- **When:** Formatted with `TreeFormatter::format()`.
- **Then:** Output is the root name only; no connector characters (├──, └──, │) appear; output is minimal.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/012_tree_hierarchical.md`](../../../docs/variant/012_tree_hierarchical.md) | Source variant doc — hierarchical tree attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/aligned_tree_basic.rs`](../../aligned_tree_basic.rs) | Tree alignment and rendering tests |

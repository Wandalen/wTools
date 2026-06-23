# Builder: TreeBuilder

### Scope

- **Purpose**: Drive test coverage for the TreeBuilder construction helper.
- **Responsibility**: Documents test cases for the TreeBuilder API in `docs/builder/002_tree_builder.md`.
- **In Scope**: Basic tree building, nested children via multi-segment paths, path-based intermediate node creation, batch construction via `from_items`, empty path handling, single-element paths, sibling ordering.
- **Out of Scope**: TreeFormatter rendering (see `../algorithm/`); TreeNode internals (see `../input_type/002_tree_node.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| BL-9 | basic single-leaf tree | ⏳ |
| BL-10 | nested children via multi-segment path | ⏳ |
| BL-11 | intermediate nodes created automatically | ⏳ |
| BL-12 | batch construction via from_items | ⏳ |
| BL-13 | empty path components are filtered out | ⏳ |
| BL-14 | single-element path creates direct child | ⏳ |
| BL-15 | multiple siblings under same parent | ⏳ |
| BL-16 | batch equivalence with manual inserts | ⏳ |

---

### BL-9: basic single-leaf tree

- **Given:** A `TreeBuilder` created with root name `"project"`.
- **When:** One leaf is inserted at path `&["readme.md"]` with data `42`; then `build()` is called.
- **Then:** The root node has name `"project"` and `data` is `None`; it has exactly 1 child; the child has name `"readme.md"` and `data` is `Some(42)`; the child has no children.

---

### BL-10: nested children via multi-segment path

- **Given:** A `TreeBuilder` created with root name `"root"`.
- **When:** A leaf is inserted at path `&["src", "main.rs"]` with data `100`; then `build()` is called.
- **Then:** The root has 1 child named `"src"` with `data` `None`; `"src"` has 1 child named `"main.rs"` with `data` `Some(100)`; the tree depth is 3 (root -> src -> main.rs).

---

### BL-11: intermediate nodes created automatically

- **Given:** A `TreeBuilder` created with root name `"root"`.
- **When:** A leaf is inserted at path `&["a", "b", "c", "leaf.txt"]` with data `1`; then `build()` is called.
- **Then:** Nodes `"a"`, `"b"`, and `"c"` all exist as intermediate directory nodes with `data` `None`; only `"leaf.txt"` has `data` `Some(1)`; each intermediate has exactly 1 child; the tree depth is 5.

---

### BL-12: batch construction via from_items

- **Given:** A slice of 3 items, each containing a `/`-separated path string and a numeric size value.
- **When:** `TreeBuilder::from_items` is called with a path extractor that splits on `'/'` and a data extractor that returns the item clone.
- **Then:** The returned `TreeNode` has root name `"root"` (default for `from_items`); all 3 items appear as leaf nodes at their correct paths; intermediate directory nodes are present where paths share common prefixes.

---

### BL-13: empty path components are filtered out

- **Given:** A `TreeBuilder` created with root name `"root"`.
- **When:** A leaf is inserted at path `&["src", "", "main.rs"]` with data `10`; then `build()` is called.
- **Then:** The empty component `""` is silently filtered; the resulting tree has root -> `"src"` -> `"main.rs"` with no empty-named intermediate node; `"main.rs"` has `data` `Some(10)`.

---

### BL-14: single-element path creates direct child

- **Given:** A `TreeBuilder` created with root name `"root"`.
- **When:** A leaf is inserted at path `&["file.txt"]` with data `5`; then `build()` is called.
- **Then:** The root has exactly 1 child; that child has name `"file.txt"`, `data` `Some(5)`, and no children; no intermediate nodes are created.

---

### BL-15: multiple siblings under same parent

- **Given:** A `TreeBuilder` created with root name `"root"`.
- **When:** Three leaves are inserted: `&["src", "a.rs"]` with data `1`, `&["src", "b.rs"]` with data `2`, `&["src", "c.rs"]` with data `3`; then `build()` is called.
- **Then:** The root has 1 child `"src"`; `"src"` has exactly 3 children (`"a.rs"`, `"b.rs"`, `"c.rs"`); children appear in insertion order; each leaf carries its respective data value; `"src"` itself has `data` `None`.

---

### BL-16: batch equivalence with manual inserts

- **Given:** A slice of 2 items with paths `"src/main.rs"` (data `100`) and `"src/lib.rs"` (data `200`).
- **When:** Tree A is built using `TreeBuilder::from_items` with path/data extractors; tree B is built using `TreeBuilder::new("root").insert(&["src","main.rs"], 100).insert(&["src","lib.rs"], 200).build()`.
- **Then:** Both trees have identical structure: same root name, same intermediate nodes, same leaf nodes with same data values; the two construction methods produce equivalent results.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/builder/002_tree_builder.md`](../../../docs/builder/002_tree_builder.md) | Source builder spec — path semantics, batch construction, invariants |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/builder.rs`](../../builder.rs) | Builder test implementation |

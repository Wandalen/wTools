# Input Model: Hierarchical

### Scope

- **Purpose**: Drive test coverage for the hierarchical input model shape.
- **Responsibility**: Documents test cases for the hierarchical data model in `docs/input_model/002_hierarchical.md`.
- **In Scope**: Single root invariant, leaf vs directory node distinction, node name constraints, three formatter specializations.
- **Out of Scope**: Rust type details (see `../input_type/`), builder API (see `../builder/`), tree formatter output (see `../formatter/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| IM-1 | every tree has exactly one root node | ⏳ |
| IM-2 | leaf nodes carry data, directory nodes have None | ⏳ |
| IM-3 | node names are plain strings not paths | ⏳ |
| IM-4 | three specializations consume hierarchical data | ⏳ |

---

### IM-1: every tree has exactly one root node

- **Given:** A tree constructed via `TreeBuilder` with paths `"src/main.rs"` and `"tests/test.rs"`.
- **When:** `build()` is called.
- **Then:** The result is a single `TreeNode` root; `root.children` contains the top-level directories; there is no sibling at the root level.

---

### IM-2: leaf nodes carry data, directory nodes have None

- **Given:** A tree with path `"src/main.rs"` having data value `150` and intermediate directory `"src/"`.
- **When:** Traversing the built tree.
- **Then:** The `"main.rs"` leaf node has `data == Some(150)`; the `"src"` directory node has `data == None`; the distinction between leaf and directory is solely the presence/absence of data.

---

### IM-3: node names are plain strings not paths

- **Given:** A `TreeBuilder` with path `"src/lib.rs"` inserted.
- **When:** `build()` is called and the tree is traversed.
- **Then:** The intermediate node name is `"src"` (not `"src/"`); the leaf node name is `"lib.rs"` (not `"src/lib.rs"`); hierarchy is expressed through nesting, not path separators in names.

---

### IM-4: three specializations consume hierarchical data

- **Given:** Three trees: a generic tree with typed leaf data, a multi-column tree with `ColumnData` leaves, and an aggregating tree with computed directory totals.
- **When:** Each is passed to its corresponding `TreeFormatter` method: `format()`, `format_aligned()`, `format_with_aggregation()`.
- **Then:** Each method produces well-formed output; the generic tree shows box-drawing connectors with leaf values; the multi-column tree aligns columns across leaves; the aggregating tree includes computed totals for directory nodes.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/input_model/002_hierarchical.md`](../../../docs/input_model/002_hierarchical.md) | Source input model doc — hierarchical data shape, invariants, specializations |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../data.rs) | Data model test implementation |

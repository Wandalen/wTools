# Algorithm Spec: Tree Column Alignment

## Source
`docs/algorithm/003_tree_column_alignment.md`

## Test Implementation
`tests/aligned_tree_basic.rs`, `tests/aligned_tree_configuration.rs`, `tests/aligned_tree_edge_cases.rs`

## Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | single-depth tree aligns column widths | ✅ |
| AC-2 | multi-depth tree uses correct connectors | ✅ |
| AC-3 | column widths normalized across all sibling nodes | ✅ |
| AC-4 | empty tree returns empty string | ✅ |
| AC-5 | unicode node names use visual width for alignment | ✅ |

---

### AC-1: single-depth tree aligns column widths

- **Given:** A flat tree with three leaf nodes, each having two data columns;
  all nodes are at the same depth.
- **When:** Rendered with `AlignedTreeFormatter` (or equivalent `AlignedTree` variant).
- **Then:** All data column values are aligned at a consistent horizontal position
  across all rows; the column separator appears at the same character offset on
  every line.

---

### AC-2: multi-depth tree uses correct connectors

- **Given:** A tree with a root node, two children, each with two grandchildren.
- **When:** Rendered.
- **Then:** Middle children (not last at their level) have `├─` prefix; the last
  child at each level has `└─` prefix; continuation lines below non-last children
  have `│` vertical bar; the last node at any level has no continuation bar below it.
- **Note:** Covered structurally by `test_aligned_tree_crate_list_simulation` in
  `tests/aligned_tree_edge_cases.rs` (root → api_ollama + unikit → 2 grandchildren);
  connector presence verified by `test_aligned_tree_single_child_two_columns`.

---

### AC-3: column widths normalized across all sibling nodes

- **Given:** A tree level where some siblings have shorter and some have longer
  values in the same data column.
- **When:** Rendered.
- **Then:** Every node at that level uses the maximum column width observed across
  all siblings; shorter values are padded with spaces to maintain alignment.

---

### AC-4: empty tree returns empty string

- **Given:** A `TreeBuilder` with no nodes inserted (only a root with no children).
- **When:** Rendered with `AlignedTreeFormatter`.
- **Then:** Output is an empty string or contains only the root node name; no panic
  occurs; no stray separator characters are emitted.

---

### AC-5: unicode node names use visual width for alignment

- **Given:** A tree level where one node name contains CJK characters (display
  width 2 each, e.g. `"文件"` = 4 visual columns) and a sibling has ASCII name.
- **When:** Rendered.
- **Then:** Column alignment is computed using visual display width (not byte
  length); both nodes' data columns start at the same visual horizontal offset.
- **Note:** Covered by `test_aligned_tree_unicode_columns` in
  `tests/aligned_tree_edge_cases.rs` (`"日本語"` + `"english"` as siblings;
  exercises the visual-width alignment code path).

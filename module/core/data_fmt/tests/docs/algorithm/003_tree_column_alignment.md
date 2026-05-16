# Algorithm: Tree Column Alignment

### Scope

- **Purpose**: Drive test coverage for the tree column alignment algorithm.
- **Responsibility**: Documents test cases for the two-phase tree column alignment algorithm in `docs/algorithm/003_tree_column_alignment.md`.
- **In Scope**: Column width normalization across siblings, connector character correctness, max_depth pruning, show_root behavior, min_column_width floor, unicode visual width, empty tree edge case.
- **Out of Scope**: Tree node data model invariants (see `invariant/001`); tree builder API (covered in builder docs).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | single-depth tree aligns column widths | ✅ |
| AC-2 | multi-depth tree uses correct connectors | ✅ |
| AC-3 | column widths normalized across all sibling nodes | ✅ |
| AC-4 | empty tree returns empty string | ✅ |
| AC-5 | unicode node names use visual width for alignment | ✅ |
| AC-6 | max_depth excludes deeper nodes from measurement and rendering | ✅ |
| AC-7 | show_root=false renders children without root-level tree prefix | ✅ |
| AC-8 | min_column_width raises column widths below the configured floor | ✅ |
| AC-9 | show_branches=false omits tree connector characters | ✅ |
| AC-10 | custom column_separator appears between data columns | ✅ |
| AC-11 | max_depth=0 produces empty output (no nodes rendered) | ✅ |
| AC-12 | show_root=false with max_depth limits output to shallow children | ✅ |

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
- **Then:** No panic occurs; no stray column separator characters are emitted;
  output is either empty or contains at most the root node name on a single line
  with no column-padding artifacts. (EC-2: `docs/invariant/001_data_model.md`
  — "Empty trees return empty string when formatted.")
- **Note:** Cross-reference: `invariant/001_data_model.md` IN-7 documents this
  as a data model invariant.

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

---

### AC-6: max_depth excludes deeper nodes from measurement and rendering

- **Given:** A three-level tree (root → children → grandchildren) configured
  with `max_depth(1)`.
- **When:** Rendered with `AlignedTreeFormatter`.
- **Then:** Only direct children of the root appear in the output; grandchildren
  are absent; column widths are computed using only the depth-1 nodes (grandchildren
  do not widen columns); no panic occurs. (Source: `docs/algorithm/003_tree_column_alignment.md`
  — "max_depth respected in both phases".)

---

### AC-7: show_root=false renders children without root-level tree prefix

- **Given:** A tree with a root node and two children; each child has two data
  columns; configured with `show_root(false)`.
- **When:** Rendered.
- **Then:** The root node line does not appear anywhere in the output; the root's
  direct children appear as the topmost rendered entries; their column data is
  still aligned horizontally; no output line references the absent root.

---

### AC-8: min_column_width raises column widths below the configured floor

- **Given:** A tree where all column-1 values are 3 characters wide; configured
  with `min_column_width(10)`.
- **When:** Rendered.
- **Then:** The column-1 separator appears at a position consistent with a width
  of at least 10 characters (Phase 1 DFS raises each column width to the minimum
  floor); shorter values are padded to the minimum width. (Source:
  `docs/algorithm/003_tree_column_alignment.md` — "min_column_width applied after DFS".)

---

### AC-9: show_branches=false omits tree connector characters

- **Given:** A multi-level tree configured with `show_branches(false)`.
- **When:** Rendered with `AlignedTreeFormatter`.
- **Then:** No `├─`, `└─`, or `│` characters appear in the output; node names
  and data columns still appear in hierarchical order; column alignment is
  preserved without the tree-drawing prefix characters.

---

### AC-10: custom column_separator appears between data columns

- **Given:** A tree with two data columns per node; configured with a custom
  `column_separator(" | ")` (pipe with surrounding spaces).
- **When:** Rendered.
- **Then:** The custom separator string `" | "` appears between each pair of
  adjacent data columns on every rendered line; the default separator is not
  used; alignment accounts for the separator's visual width.

---

### AC-11: max_depth=0 produces empty output (no nodes rendered)

- **Given:** A tree with a root node and children; configured with `max_depth(0)`.
- **When:** Rendered with `AlignedTreeFormatter`.
- **Then:** Output is an empty string; no nodes — not even the root — appear
  in the output; no panic occurs.

---

### AC-12: show_root=false with max_depth limits output to shallow children

- **Given:** A three-level tree configured with both `show_root(false)` and
  `max_depth(1)`.
- **When:** Rendered.
- **Then:** Root is hidden; only depth-1 nodes (direct children of hidden root)
  appear; depth-2 nodes (grandchildren) are excluded by max_depth; the combined
  effect of both options is applied correctly without panic or empty output when
  depth-1 nodes exist.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/003_tree_column_alignment.md`](../../../docs/algorithm/003_tree_column_alignment.md) | Source algorithm spec — two-phase DFS, connector rules, config parameters |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/aligned_tree_basic.rs`](../../aligned_tree_basic.rs) | Basic alignment test cases |
| [`tests/aligned_tree_configuration.rs`](../../aligned_tree_configuration.rs) | Configuration parameter test cases |
| [`tests/aligned_tree_edge_cases.rs`](../../aligned_tree_edge_cases.rs) | Edge case and unicode test cases |

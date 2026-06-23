# Formatter: TreeFormatter

### Scope

- **Purpose**: Drive test coverage for the TreeFormatter output contract.
- **Responsibility**: Documents test cases for the `TreeFormatter` struct described in `docs/formatter/003_tree_formatter.md`.
- **In Scope**: Hierarchical render via custom closure, aligned render via ColumnData, format_with_aggregation render, empty tree handling, box-drawing symbol emission, indentation depth.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), hierarchical input model internals (see `tests/docs/input_model/002`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-9 | hierarchical render produces tree with box-drawing connectors | ⏳ |
| FM-10 | aligned render produces column-aligned output | ⏳ |
| FM-11 | ColumnData leaves render with alignment padding | ⏳ |
| FM-12 | empty tree produces minimal or empty output | ⏳ |
| FM-13 | nested children increase indentation depth | ⏳ |

---

### FM-9: hierarchical render produces tree with box-drawing connectors

- **Given:** A tree with root "project" containing two children "src" and "tests", where "src" has one child "main.rs".
- **When:** `TreeFormatter::format(tree, render_fn)` is called with a closure that renders each node's label.
- **Then:** The output contains box-drawing connector characters (e.g., lines from the Unicode Box Drawing block or ASCII equivalents like `├──`, `└──`); child nodes are indented relative to their parent; the root appears at indentation level 0.

---

### FM-10: aligned render produces column-aligned output

- **Given:** A tree where leaf nodes carry `ColumnData` with two columns of varying widths (e.g., `["file.rs", "1024 bytes"]` and `["lib.rs", "512 bytes"]`).
- **When:** `TreeFormatter::format_aligned(tree)` is called.
- **Then:** The leaf columns are right-aligned or padded so that corresponding columns share the same horizontal start position; non-leaf (branch) nodes render their label without column alignment; the output preserves tree connector symbols.

---

### FM-11: ColumnData leaves render with alignment padding

- **Given:** A tree with one root and two leaves: leaf A with columns `["short", "1"]` and leaf B with columns `["very_long_name", "2"]`.
- **When:** `TreeFormatter::format_aligned(tree)` is called.
- **Then:** The shorter column value in leaf A is padded with spaces so that the second column aligns vertically with the second column of leaf B; no column data bleeds into adjacent columns.

---

### FM-12: empty tree produces minimal or empty output

- **Given:** A tree with a single root node and no children.
- **When:** `TreeFormatter::format(tree, render_fn)` is called.
- **Then:** The output contains exactly the root node's label (from the render closure); no connector characters appear; no trailing blank lines are emitted beyond a single trailing newline.

---

### FM-13: nested children increase indentation depth

- **Given:** A tree with depth 3: root "a" has child "b" which has child "c".
- **When:** `TreeFormatter::format(tree, render_fn)` is called.
- **Then:** Node "b" appears indented one level from "a"; node "c" appears indented two levels from "a"; each indentation level adds the same fixed number of whitespace characters (consistent indent width).

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/003_tree_formatter.md`](../../../docs/formatter/003_tree_formatter.md) | Source formatter doc — no-trait interface, input types, method-level variants |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/tree_tests.rs`](../../tree_tests.rs) | TreeFormatter test implementation |

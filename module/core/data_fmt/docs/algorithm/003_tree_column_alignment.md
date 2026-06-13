# Algorithm: Tree Column Alignment

### Scope

- **Purpose**: Align multi-column data across tree nodes at different depths, accounting for variable-length tree prefix symbols so columns line up vertically.
- **Responsibility**: Documents the two-phase DFS width aggregation and recursive aligned rendering algorithm.
- **In Scope**: Prefix length formula, column width measurement, aligned rendering, depth limiting.
- **Out of Scope**: Tree node construction and data model (see `invariant/001_data_model.md`).

### Sources

| File | Relationship |
|------|--------------|
| [`src/formatters/tree.rs`](../../src/formatters/tree.rs) | TreeFormatter column alignment |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/aligned_tree_basic.rs`](../../tests/aligned_tree_basic.rs) | Tree alignment test suite |

### Invariants

| File | Relationship |
|------|-------------|
| [002_ansi_unicode.md](../invariant/002_ansi_unicode.md) | ANSI-aware alignment invariants respected by this algorithm |

### Abstract

A two-phase algorithm for rendering tree nodes with multi-column data so that columns align vertically across all tree depths. Phase 1 traverses the tree depth-first, measuring the maximum effective width per column and accounting for variable-length tree prefix symbols at each depth. Phase 2 re-traverses the tree, rendering each node using the measured widths to produce aligned column output.

### Problem

Tree nodes at different depths have prefixes of different lengths:

```
├── shallow_node    col2    col3      (depth 1: prefix = "├── ")
│   ├── deep_node   col2    col3      (depth 2: prefix = "│   ├── ")
│   │   └── deeper  col2    col3      (depth 3: prefix = "│   │   └── ")
```

Column 0 width must account for the longest `prefix + content` combination across all depths. Subsequent columns align independently of tree structure.

### Algorithm

#### Phase 1 — Measure: DFS Width Aggregation

Traverse the entire tree depth-first. For each node with column data, compute the effective width of each column. Column 0 includes the tree prefix length; columns 1+ use raw `visual_len`.

```
traverse_for_widths(node, max_widths, depth):
  if max_depth set and depth > max_depth: return

  if node has column data:
    for each column i:
      if i == 0:
        width = visual_len(col) + calculate_prefix_len(depth)
      else:
        width = visual_len(col)
      max_widths[i] = max(max_widths[i], width)

  for each child:
    traverse_for_widths(child, max_widths, depth + 1)
```

#### Prefix Length Formula

```
calculate_prefix_len(depth):
  if depth == 0: return 0
  branch_connector_len = visual_len("├──") + 1    // +1 for trailing space
  return (depth - 1) * indent_size + branch_connector_len
```

Each tree level contributes `indent_size` characters (default: 4) of indentation. The branch connector (`├──` or `└──`) plus one trailing space occupies a fixed width at the node's own depth level.

#### Phase 2 — Render: Recursive Aligned Output

Traverse the tree again, rendering each node with padding computed from Phase 1 widths.

```
format_aligned_node(node, column_widths, prefix, is_last, depth):
  if max_depth set and depth > max_depth: return

  output += prefix
  connector = "└── " if is_last else "├── "
  output += connector

  if node has column data:
    for each column i:
      if i == 0:
        output += col_value
        current_len = visual_len(prefix) + visual_len(connector) + 1 + visual_len(col)
        pad = column_widths[0] - current_len
        output += " " * pad
      else:
        output += column_separator
        output += pad_to_width(col, column_widths[i])
  else:
    output += node.name

  output += '\n'

  // Recurse to children
  new_prefix = prefix + ("    " if is_last else "│   ")
  for each child:
    format_aligned_node(child, column_widths, new_prefix, ...)
```

#### Key Properties

- **Column 0 absorbs tree structure** — prefix length varies by depth, but Phase 1 finds the global maximum so all column-1 separators align vertically.
- **`visual_len` throughout** — ANSI escape sequences in column content do not affect alignment.
- **`min_column_width` applied after DFS** — in `calculate_column_widths`, each width is raised to the floor after the full traversal.
- **`max_depth` respected in both phases** — nodes beyond `max_depth` are excluded from measurement and rendering.
- **Branch symbol selection** — `is_last` determines `└──` vs `├──`; continuation lines use `│` vs blank.

### Complexity

- Time: O(n * c) where n is the number of tree nodes and c is the number of columns — two full traversals.
- Space: O(c + d) where c is column count (widths vector) and d is max depth (recursion stack).

### Interaction with Other Features

| Feature | Interaction |
|---------|-------------|
| `show_root` | When false, children rendered at depth 0 (no prefix) |
| `show_branches` | When false, children not recursed for rendering (Phase 2 only) |
| `min_column_width` | Applied as floor after Phase 1 DFS completes |
| `max_depth` | Limits both measurement and rendering traversal |
| `column_separator` | Used between columns 1+ in Phase 2 |

# Algorithm: Tree Aggregation

### Scope

- **Purpose**: Compute rolled-up subtree totals and percentages for directory nodes in a tree, without a pre-existing aggregate-caching field on `TreeNode`.
- **Responsibility**: Documents the recursive per-node aggregate recomputation, percentage derivation, and how this rendering path diverges from `format_aligned`'s two-phase design.
- **In Scope**: `calculate_aggregate` recursive summation, `format_node_with_aggregation` traversal and rendering, percentage formula, `max_depth`/`show_root`/`show_branches` interaction (or lack thereof), zero-`grand_total` behavior.
- **Out of Scope**: Column-width alignment (see `003_tree_column_alignment.md`), tree node construction (see `../builder/002_tree_builder.md`).

### Sources

| File | Relationship |
|------|--------------|
| [`src/formatters/tree/aggregated.rs`](../../src/formatters/tree/aggregated.rs) | `format_with_aggregation`, `calculate_aggregate`, `format_node_with_aggregation` |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/variant_014_tree_aggregated_test.rs`](../../tests/variant_014_tree_aggregated_test.rs) | VT-1..VT-4: directory totals, leaf preservation, recursive aggregation, single-leaf |

### Variants

| File | Relationship |
|------|--------------|
| [014_tree_aggregated.md](../variant/014_tree_aggregated.md) | Output variant produced by this algorithm |

### Abstract

A single-pass depth-first traversal renders each node while a nested, per-directory recursive helper (`calculate_aggregate`) recomputes that directory's subtree total from scratch at the moment it is rendered. Unlike `format_aligned`'s two-phase design (§ `003_tree_column_alignment.md`), there is no separate measurement pass and no memoization: an ancestor directory's total re-walks every descendant subtree that has already been summed once for each of its own nested directory lines. Percentages are derived by dividing a node's aggregate through a caller-supplied `grand_total`, with no zero-guard.

### Problem

`TreeNode< T >` carries no aggregate-caching field — only `data : Option< T >` and `children : Vec< TreeNode< T > >`. To show a directory line like `src/ (450 lines, 66.7%)`, the formatter needs the sum of every leaf value in that directory's subtree, computed on demand from whatever `aggregate_fn` the caller supplies (e.g. `| f | f.lines`). The same subtree is visited once to render each descendant directory's own total, and again — from the top — whenever an ancestor needs its own total.

### Algorithm

**Entry point** — `format_with_aggregation(tree, grand_total, aggregate_fn, convert_to_f64, render_file, render_directory)`:

```
output = ""
format_node_with_aggregation(tree, output, prefix="", is_last=true, depth=0,
                              grand_total, is_root=true,
                              aggregate_fn, convert_to_f64, render_file, render_directory)
return output
```

**Aggregate summation** — `calculate_aggregate(node, aggregate_fn) -> V` (recursive, no memoization):

```
calculate_aggregate(node, aggregate_fn):
  direct = aggregate_fn(node.data) if node.data is Some else V::default()
  children_total = sum(calculate_aggregate(child, aggregate_fn) for child in node.children)
  return direct + children_total
```

**Traversal and rendering** — `format_node_with_aggregation(node, output, prefix, is_last, depth, grand_total, is_root, ...)`:

```
format_node_with_aggregation(node, output, prefix, is_last, depth, grand_total, is_root, ...):
  if max_depth set and depth >= max_depth: return   // note: >=, cuts off one level earlier than format_aligned's `>`

  if node.data is None and not is_root:              // directory node, never the root itself
    node_total = calculate_aggregate(node, aggregate_fn)   // re-walks the ENTIRE subtree again
    percentage = convert_to_f64(node_total) / convert_to_f64(grand_total) * 100.0   // no zero-guard
    output += "{node.name}/ {render_directory(node.name, node_total, percentage)}\n"

  child_prefix =
    ""                if is_root
    prefix + "    "   if is_last
    prefix + "│   "   otherwise

  if node.data is Some(file_data):                    // leaf node
    value = aggregate_fn(file_data)
    percentage = convert_to_f64(value) / convert_to_f64(grand_total) * 100.0
    symbol = "└──" if is_last else "├──"
    output += "{prefix}{symbol} {node.name} {render_file(file_data, value, percentage)}\n"

  for (idx, child) in node.children.enumerate():
    is_last_child = idx == node.children.len() - 1
    format_node_with_aggregation(child, output, child_prefix, is_last_child, depth + 1,
                                  grand_total, is_root=false, ...)   // unconditional recursion
```

#### Key Properties

- **Root total is never computed**: the top-level call always passes `is_root = true`, so the `node.data.is_none() && !is_root` guard short-circuits on `!is_root` before `calculate_aggregate` is ever invoked for the root — its directory line is never rendered and no aggregate is computed for it at all (not computed and then discarded).
- **No memoization — repeated subtree walks**: `calculate_aggregate` is called once per directory node encountered during the traversal, and each call re-walks that node's *entire* subtree from scratch. A directory's total is computed once for its own line, then re-summed again as part of every ancestor directory's own `calculate_aggregate` call — the deeper the nesting, the more times the same leaves are revisited. Contrast with `format_aligned`'s Phase 1 (§ `003_tree_column_alignment.md`), which measures every node exactly once.
- **`show_root` has no effect**: `format_aligned` reads `self.config.show_root` to decide whether to print the root's name line; `format_with_aggregation` never reads this flag — root suppression is unconditional (see previous bullet), independent of the config value.
- **`show_branches` has no effect**: `format_aligned` gates child recursion on `self.config.show_branches`; `format_with_aggregation`'s child loop has no such gate — children are always recursed into regardless of this flag.
- **`max_depth` cutoff is one level stricter than `format_aligned`**: `format_aligned` excludes nodes where `depth > max_depth` (a node at `max_depth` is still rendered); `format_with_aggregation` excludes nodes where `depth >= max_depth` (a node at `max_depth` is already cut off). The same `max_depth` value therefore renders one fewer level of depth under aggregation than under alignment.
- **Zero-`grand_total` is unguarded**: both percentage calculations divide by `convert_to_f64(grand_total)` with no check for zero. If the caller-supplied `grand_total` converts to `0.0`, every percentage is IEEE-754 `0.0/0.0 = NaN` (or `inf` for a nonzero numerator), propagated verbatim into `render_file`/`render_directory`'s `pct` parameter with no library-level clamp or substitution.

### Complexity

- Time: bounded by the sum of subtree sizes over every directory node — `O(Σ_d |subtree(d)|)`. For a balanced/bushy tree this is `O(n log n)`; for a deeply nested, mostly-linear directory chain it degrades to `O(n²)`. This is strictly worse than `format_aligned`'s clean `O(n * c)` two-pass measurement (§ `003_tree_column_alignment.md`'s Complexity), which never revisits a node.
- Space: `O(d)` recursion stack depth per `format_node_with_aggregation` call, plus a nested `O(d)` stack for each `calculate_aggregate` invocation it triggers.

### Interaction with Other Features

| Feature | Interaction |
|---------|--------------|
| `show_root` | No effect — root directory line is always suppressed regardless of this flag |
| `show_branches` | No effect — children are always recursed into regardless of this flag |
| `max_depth` | Cuts off at `depth >= max_depth`, one level earlier than `format_aligned`'s `depth > max_depth` |
| `min_column_width` / `column_separator` | Not applicable — aggregated output has no column layout |

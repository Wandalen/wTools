# Formatter: TreeFormatter

### Scope

- **Purpose**: Render hierarchical tree data as box-drawing ASCII/Unicode output with configurable symbols and optional column alignment.
- **Responsibility**: Document the `TreeFormatter` struct — its no-trait direct-method interface, input types, and the 3 method-dispatched variants.
- **In Scope**: No-trait interface, generic input type specializations, method-level variant selection, feature flags.
- **Out of Scope**: Variant output details (see `../variant/012_tree_hierarchical.md` through `014_tree_aggregated.md`), operation signatures (see `../api/004_formatters.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/tree.rs` | `TreeFormatter` implementation |
| source | `src/config.rs` | `TreeConfig`, `TreeSymbols` |
| doc | `../api/004_formatters.md` | Operation signatures |
| doc | `../input_model/002_hierarchical.md` | Hierarchical input model |
| doc | `../input_type/002_tree_node.md` | `TreeNode` type specializations |
| doc | `../variant/012_tree_hierarchical.md` | Variant: hierarchical |
| doc | `../variant/013_tree_aligned.md` | Variant: aligned |
| doc | `../variant/014_tree_aggregated.md` | Variant: aggregated |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ❌ Not implemented | Generic render closure cannot fit `Format` signature |
| `TableShapedFormatter` | ❌ Not implemented | Operates on hierarchical tree data, not the table-encoded tree format |

`TreeFormatter` implements no shared trait. Variant selection is done by calling the appropriate method directly. It cannot be used polymorphically through any interface.

### Input

| Input Type | Path | Via |
|------------|------|-----|
| Generic tree (display-capable data) | Hierarchical — custom render | `::format(tree, render_fn)` |
| Multi-column tree (ColumnData leaves) | Hierarchical — multi-column aligned | `::format_aligned(tree)` |
| Aggregating tree + aggregation spec | Hierarchical — subtree totals | `::format_with_aggregation(tree, …)` |

`TreeFormatter` is the exclusive consumer of the hierarchical input model. No other formatter accepts hierarchical tree data.

### Variants

Selection mechanism: the called method name determines the variant — there is no config enum or preset.

| Variant | Method | Feature Flag |
|---------|--------|--------------|
| hierarchical | `TreeFormatter::format(tree, render_fn)` | `tree_hierarchical` |
| aligned | `TreeFormatter::format_aligned(tree)` | `tree_aligned` |
| aggregated | `TreeFormatter::format_with_aggregation(tree, …)` | `tree_aggregated` |

Each feature flag is independent. Enabling `tree_aligned` alone does not compile `format_with_aggregation`.

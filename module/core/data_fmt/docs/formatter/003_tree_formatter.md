# Formatter: TreeFormatter

### Scope

- **Purpose**: Render hierarchical tree data as box-drawing ASCII/Unicode output with configurable symbols and optional column alignment.
- **Responsibility**: Document the `TreeFormatter` struct — its no-trait direct-method interface, input types, and the 3 method-dispatched variants.
- **In Scope**: No-trait interface, generic input type specializations, method-level variant selection, feature flags.
- **Out of Scope**: Variant output details (see `../variant/012_tree_hierarchical.md` through `014_tree_aggregated.md`), operation signatures (see `../api/004_formatters.md`).

### APIs

| File | Relationship |
|------|-------------|
| [004_formatters.md](../api/004_formatters.md) | Operation signatures |

### InputModels

| File | Relationship |
|------|-------------|
| [002_hierarchical.md](../input_model/002_hierarchical.md) | Hierarchical input model |

### InputTypes

| File | Relationship |
|------|-------------|
| [002_tree_node.md](../input_type/002_tree_node.md) | `TreeNode` type specializations |

### Variants

| File | Relationship |
|------|-------------|
| [012_tree_hierarchical.md](../variant/012_tree_hierarchical.md) | Variant: hierarchical |
| [013_tree_aligned.md](../variant/013_tree_aligned.md) | Variant: aligned |
| [014_tree_aggregated.md](../variant/014_tree_aggregated.md) | Variant: aggregated |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/tree/mod.rs`](../../src/formatters/tree/mod.rs) | `TreeFormatter` implementation |
| [`src/config/tree_config.rs`](../../src/config/tree_config.rs) | `TreeConfig`, `TreeSymbols` |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ❌ Not implemented | Generic render closure cannot fit `Format` signature |

`TreeFormatter` implements no shared trait. Variant selection is done by calling the appropriate method directly. It cannot be used polymorphically through any interface.

### Input

| Input Type | Path | Via |
|------------|------|-----|
| Generic tree (display-capable data) | Hierarchical — custom render | `::format(tree, render_fn)` |
| Multi-column tree (ColumnData leaves) | Hierarchical — multi-column aligned | `::format_aligned(tree)` |
| Aggregating tree + aggregation spec | Hierarchical — subtree totals | `::format_with_aggregation(tree, …)` |

`TreeFormatter` is the only formatter that renders hierarchical tree data natively (box-drawing, aligned, or aggregated output). `TableFormatter` and `ExpandedFormatter` also accept a `TreeNode` — via their own `format_tree(tree)` convenience method — but only after flattening it to a `TableView` through `conversions::flatten_to_table_tree`; they do not render hierarchy directly.

### Variants

Selection mechanism: the called method name determines the variant — there is no config enum or preset.

| Variant | Method | Feature Flag |
|---------|--------|--------------|
| hierarchical | `TreeFormatter::format(tree, render_fn)` | `tree_hierarchical` |
| aligned | `TreeFormatter::format_aligned(tree)` | `tree_aligned` |
| aggregated | `TreeFormatter::format_with_aggregation(tree, …)` | `tree_aggregated` |

The three flags are OR'd together at the crate's export boundary (`src/lib.rs`) to gate `TreeFormatter` as a whole — none of them individually cfg-gates a specific method, so enabling `tree_aligned` alone still compiles `format`, `format_aligned`, and `format_with_aggregation` together.

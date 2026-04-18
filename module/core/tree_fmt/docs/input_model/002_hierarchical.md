# Input Model: Hierarchical

### Scope

- **Purpose**: Define the hierarchical data shape: a rooted tree of named nodes where each node carries optional typed data and zero or more children.
- **Responsibility**: Document the conceptual structure and invariants of hierarchical data.
- **In Scope**: Node structure, leaf data, invariants, specializations, and builder entry points.
- **Out of Scope**: Rust type details (see `../input_type/`), construction APIs (see `../builder/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/data.rs` | TreeNode definition |
| test | `tests/data.rs` | Data model tests |
| doc | `../input_type/002_tree_node.md` | Rust type documentation |

### Structure

```text
root
├── src/
│   ├── main.rs   → 150 lines
│   └── lib.rs    → 300 lines
└── tests/
    └── test.rs   → 50 lines
```

- **Nodes** — named entities in a parent-child hierarchy.
- **Leaf data** — typed payload on terminal nodes (`data = Some(T)`).
- **Directory nodes** — interior nodes without payload (`data = None`).

### Invariants

- Every tree has exactly one root node.
- A node's name is a plain `String`, not a path — hierarchy is expressed through nesting, not separators.
- Leaf data type `T` is uniform across the entire tree.

### Specializations

| Type Parameter | Purpose | Formatter Method |
|---------------|---------|------------------|
| `TreeNode<T>` | Generic tree with typed leaves | `TreeFormatter::format()` |
| `TreeNode<ColumnData>` | Multi-column aligned tree | `TreeFormatter::format_aligned()` |
| `TreeNode<T>` + aggregation | Tree with computed directory totals | `TreeFormatter::format_with_aggregation()` |

### Builder

`TreeBuilder<T>` constructs hierarchical data from flat path-based insertions:

```rust
let tree = TreeBuilder::new( "root" )
  .insert( &[ "src", "main.rs" ], 150 )
  .insert( &[ "src", "lib.rs" ], 300 )
  .build();
```

### Accepted By

Only `TreeFormatter` — via direct methods, not via a trait.

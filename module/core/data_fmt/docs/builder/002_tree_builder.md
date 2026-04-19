# Builder: TreeBuilder

### Scope

- **Purpose**: Document TreeBuilder construction helper API and usage patterns.
- **Responsibility**: Describe path-based tree construction with automatic intermediate node creation.
- **In Scope**: Builder methods, path semantics, batch construction, downstream formatter compatibility.
- **Out of Scope**: Input type internals (see `../input_type/`), public API signatures (see `../api/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/builder.rs` | TreeBuilder implementation |
| test | `tests/builder.rs` | Builder tests |
| doc | `../api/002_builders.md` | Public API surface |
| doc | `../input_type/002_tree_node.md` | Output type documentation |

### Construction API

| Method | Bound | Output |
|--------|-------|--------|
| `new( root_name )` | — | Builder with named root |
| `insert( path, data )` | — | Insert leaf at path, creating intermediate nodes |
| `build()` | — | `TreeNode<T>` |
| `from_items( items, extract_path, extract_data )` | `T: Clone` | `TreeNode<T>` directly |

### Path Semantics

Each `insert()` call takes a `&[&str]` path. Intermediate nodes are created automatically as directory nodes (`data = None`). The final path component becomes a leaf with `data = Some(T)`.

```rust
let tree = TreeBuilder::new( "root" )
  .insert( &[ "src", "main.rs" ], 150 )
  .insert( &[ "src", "lib.rs" ], 300 )
  .insert( &[ "tests", "test.rs" ], 50 )
  .build();
```

Produces:

```text
root (None)
├── src (None)
│   ├── main.rs (Some(150))
│   └── lib.rs (Some(300))
└── tests (None)
    └── test.rs (Some(50))
```

### Batch Construction

`from_items()` builds an entire tree from a slice with user-supplied path and data extractors:

```rust
let tree = TreeBuilder::from_items( &files, | f | {
  f.path.split( '/' ).map( String::from ).collect()
}, | f | f.clone() );
```

### Input Model

Hierarchical — see `input_model/hierarchical.md`.

### Downstream

| TreeFormatter Method | Type Parameter |
|---------------------|---------------|
| `format( tree, render_fn )` | `TreeNode<T>` (any `T`) |
| `format_aligned( tree )` | `TreeNode<ColumnData>` |
| `format_with_aggregation( tree, ... )` | `TreeNode<T>` (any `T`) |

### Invariants

Pre/post conditions enforced at construction time:

- **Intermediate nodes**: every non-terminal path element that does not already exist is created as a directory node (`data = None`). Callers cannot produce a tree with missing interior nodes.
- **Leaf placement**: the final element of each `insert()` path is always a leaf with `data = Some(T)`. A path with zero elements panics.
- **Type uniformity**: all leaf nodes carry the same type `T`. Mixed-type trees are not possible — the type is fixed at `TreeBuilder<T>` construction.
- **Batch equivalence**: `from_items( items, path_fn, data_fn )` produces the same tree as calling `insert()` for each item individually with the extracted path and data.

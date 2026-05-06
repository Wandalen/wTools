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
| `build()` | — | tree structure |
| `from_items( items, extract_path, extract_data )` | data must be cloneable | tree structure directly |

### Path Semantics

Each `insert()` call takes a slice of path components. Intermediate nodes are created automatically as directory nodes with no data payload. The final path component becomes a leaf carrying the supplied data value.

Produces:

```text
root
├── src
│   ├── main.rs [150]
│   └── lib.rs [300]
└── tests
    └── test.rs [50]
```

### Batch Construction

`from_items()` builds an entire tree from a slice. The caller supplies two extractor closures: one that returns the path for each item (as a list of path components), and one that returns the data value. The result is identical to calling `insert()` for each item individually.

### Input Model

Hierarchical — see `input_model/hierarchical.md`.

### Downstream

| TreeFormatter Method | Input |
|---------------------|-------|
| `format( tree, render_fn )` | generic tree with display-capable data |
| `format_aligned( tree )` | multi-column tree (ColumnData leaves) |
| `format_with_aggregation( tree, ... )` | generic tree with display-capable data |

### Invariants

Pre/post conditions enforced at construction time:

- **Intermediate nodes**: every non-terminal path element that does not already exist is created as a directory node with no data payload. Callers cannot produce a tree with missing interior nodes.
- **Leaf placement**: the final element of each `insert()` path is always a leaf carrying the inserted data value. A path with zero elements panics.
- **Type uniformity**: all leaf nodes carry the same data type. Mixed-type trees are not possible — the type is fixed at builder construction.
- **Batch equivalence**: `from_items( items, path_fn, data_fn )` produces the same tree as calling `insert()` for each item individually with the extracted path and data.

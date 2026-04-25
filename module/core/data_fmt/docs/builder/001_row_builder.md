# Builder: RowBuilder

### Scope

- **Purpose**: Document RowBuilder construction helper API and usage patterns.
- **Responsibility**: Describe how to build tabular data via fluent and mutable builder chains.
- **In Scope**: Builder methods, dual output paths, usage examples, input model reference.
- **Out of Scope**: Input type internals (see `../input_type/`), public API signatures (see `../api/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/builder.rs` | RowBuilder implementation |
| test | `tests/builder.rs` | Builder tests |
| doc | `../api/002_builders.md` | Public API surface |
| doc | `../input_type/001_table_view.md` | Output type documentation |

### Construction API

| Method | Consumes Self | Output |
|--------|:------------:|--------|
| `new( headers )` | — | Builder with column schema |
| `add_row( row )` | yes | Append row (auto-numbered) |
| `add_row_mut( row )` | no | Same, `&mut self` for loops |
| `add_row_with_name( name, row )` | yes | Append row with custom name |
| `add_row_with_name_mut( name, row )` | no | Same, `&mut self` |
| `add_row_with_detail( row, detail )` | yes | Append row with `DecoratedText` annotation |
| `add_row_with_detail_mut( row, detail )` | no | Same, `&mut self` |
| `build_view()` | yes | `TableView` (modern path) |
| `build()` | yes | `TreeNode<String>` (legacy path) |

### Dual Output

The builder maintains both representations internally in parallel:

```text
RowBuilder
├── rows (+ row_details)  → build_view() → TableView
└── root tree             → build()      → tree structure
```

### Input Model

Tabular — see `input_model/tabular.md`.

### Usage

Construct a builder with `RowBuilder::new( headers )`, add rows via `add_row` or `add_row_mut`, then finalize. Call `build_view()` for the modern `Format`-trait path (8 formatters), or `build()` for the legacy `TableShapedFormatter` path (2 formatters).

### Invariants

Pre/post conditions enforced at construction time:

- **Row length**: every row added via any `add_row*` method must have length exactly equal to `headers.len()`. Violated at insertion time causes an immediate panic. Downstream formatters never encounter ragged rows.
- **Parallel vectors**: `rows` and `row_details` are always the same length throughout the builder's lifetime. Every internal row insertion updates both vectors simultaneously; rows without explicit detail receive `None`.
- **Empty headers allowed**: constructing with an empty `headers` vec is valid; all subsequently added rows must also be empty.

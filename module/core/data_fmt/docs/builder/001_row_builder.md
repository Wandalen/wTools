# Builder: RowBuilder

### Scope

- **Purpose**: Document RowBuilder construction helper API and usage patterns.
- **Responsibility**: Describe how to build tabular data via fluent and mutable builder chains.
- **In Scope**: Builder methods, usage examples, input model reference.
- **Out of Scope**: Input type internals (see `../input_type/`), public API signatures (see `../api/`).

### APIs

| File | Relationship |
|------|-------------|
| [002_builders.md](../api/002_builders.md) | Public API surface |

### Input Types

| File | Relationship |
|------|-------------|
| [001_table_view.md](../input_type/001_table_view.md) | Output type documentation |

### Sources

| File | Relationship |
|------|-------------|
| [`src/builder.rs`](../../src/builder.rs) | RowBuilder implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/builder.rs`](../../tests/builder.rs) | Builder tests |

### Construction API

| Method | Consumes Self | Output |
|--------|:------------:|--------|
| `new( headers )` | — | Builder with column schema |
| `add_row( row )` | yes | Append row (auto-numbered) |
| `add_row_mut( row )` | no | Same, mutable reference for loops |
| `add_row_with_name( name, row )` | yes | Append row with custom name |
| `add_row_with_name_mut( name, row )` | no | Same, mutable reference |
| `add_row_with_detail( row, detail )` | yes | Append row with `DecoratedText` annotation |
| `add_row_with_detail_mut( row, detail )` | no | Same, mutable reference |
| `build_view()` | yes | `TableView` |

### Output

The builder accumulates rows and produces a `TableView` via `build_view()`, consumed by any `Format`-trait formatter.

### Input Models

| File | Relationship |
|------|-------------|
| [001_tabular.md](../input_model/001_tabular.md) | Tabular data model that this builder produces rows for |

### Usage

Construct a builder with `RowBuilder::new( headers )`, add rows via `add_row` or `add_row_mut`, then finalize with `build_view()`. Pass the resulting `TableView` to any `Format`-trait formatter (9 formatters).

### Invariants

Pre/post conditions enforced at construction time:

- **Row length**: every row added via any `add_row*` method must have length exactly equal to `headers.len()`. Violated at insertion time causes an immediate panic. Downstream formatters never encounter ragged rows.
- **Parallel vectors**: `rows` and `row_details` are always the same length throughout the builder's lifetime. Every internal row insertion updates both vectors simultaneously; rows without explicit detail receive no annotation.
- **Empty headers allowed**: constructing with an empty headers list is valid; all subsequently added rows must also be empty.

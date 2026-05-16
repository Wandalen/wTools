# Input Model: Tabular

### Scope

- **Purpose**: Define the tabular data shape: a fixed set of named columns (headers) with zero or more rows of cells where every cell belongs to exactly one column.
- **Responsibility**: Document the conceptual structure and invariants of tabular data.
- **In Scope**: Headers, rows, row details, invariants, Rust type mapping, and builder entry points.
- **Out of Scope**: Rust type details (see `../input_type/`), construction APIs (see `../builder/`).

### InputTypes

| File | Relationship |
|------|-------------|
| [001_table_view.md](../input_type/001_table_view.md) | Rust type documentation |

### Sources

| File | Relationship |
|------|-------------|
| [`src/data.rs`](../../src/data.rs) | TableView definition |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../tests/data.rs) | Data model tests |

### Data Shape

```text
headers:  [ "Name",  "Age",  "City"    ]
row 0:    [ "Alice", "30",   "Berlin"  ]
row 1:    [ "Bob",   "25",   "London"  ]
```

- **Headers** — ordered column names; define the schema.
- **Rows** — ordered sequences of cell values, one per column.
- **Row details** — optional per-row annotation line (`DecoratedText`), displayed below the row in visual formats.

### Invariants

- Every row has exactly as many cells as there are headers.
- Column order is stable across all rows.
- Cell values are strings at the model level; `DataType` metadata provides optional semantic typing.

### Downstream Connections

**Data structure representing this model:**

`TableView` is the canonical representation, produced by `RowBuilder::build_view()`.

**Formatters that consume tabular input:**

All 9 tabular formatters implement the `Format` trait and consume `TableView` directly. `TreeFormatter` is excluded — it consumes hierarchical input only.

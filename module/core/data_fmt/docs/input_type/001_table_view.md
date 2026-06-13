# Input Type: TableView

### Scope

- **Purpose**: Document the `TableView` Rust struct as the canonical tabular input type for the `Format` trait.
- **Responsibility**: Document TableView struct definition, components, construction, and formatter coverage.
- **In Scope**: Struct fields, TableMetadata, construction patterns, Format trait consumption, and backward compatibility.
- **Out of Scope**: Conceptual shape (see `../input_model/`), formatter behavior (see `../feature/`).

### Input Models

| File | Relationship |
|------|-------------|
| [001_tabular.md](../input_model/001_tabular.md) | Conceptual data shape |

### APIs

| File | Relationship |
|------|-------------|
| [001_data_types.md](../api/001_data_types.md) | Public API surface |

### Sources

| File | Relationship |
|------|-------------|
| [`src/data.rs`](../../src/data.rs) | TableView struct definition |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../tests/data.rs) | TableView tests |

### Type Definition

`TableView` has three public fields: `metadata` holds column names and type classifications; `rows` holds cell data as a matrix of strings; `row_details` holds an optional per-row annotation line parallel to `rows`.

### Components

| Field | Role |
|-------|------|
| `metadata` | Column names and data types |
| `rows` | Cell data, one inner vec per row |
| `row_details` | Optional per-row annotation line (parallel to `rows`) |

`TableMetadata` carries column names and per-column semantic type classifications (`String`, `Integer`, `Boolean`, `Path`).

### Construction

The preferred construction path is `RowBuilder::build_view()`, which validates row length at each insertion. `TableView::new()` and `TableView::with_details()` allow direct construction when the caller already holds a headers vector and a row matrix.

### Traits

| File | Relationship |
|------|-------------|
| [001_format.md](../trait/001_format.md) | `Format` trait that consumes `TableView` as its input type |

### Formatter Coverage

| Formatter | Implements `Format` |
|-----------|:-------------------:|
| `TableFormatter` | yes |
| `LogfmtFormatter` | yes |
| `HtmlFormatter` | yes |
| `SqlFormatter` | yes |
| `JsonFormatter` | yes |
| `YamlFormatter` | yes |
| `TomlFormatter` | yes |
| `TextFormatter` | yes |
| `ExpandedFormatter` | yes |
| `TreeFormatter` | **no** |

9 of 10 formatters accept `TableView` via `Format`.


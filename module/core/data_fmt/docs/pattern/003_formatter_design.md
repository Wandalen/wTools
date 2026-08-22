# Pattern: Formatter Design

### Scope

- **Purpose**: Document the formatter trait hierarchy and the TableShapedView decoupling pattern.
- **Responsibility**: Complete description of how formatters consume data and expose output APIs.
- **In Scope**: `Format` trait, `TableShapedFormatter` trait, `TableShapedView` decoupling, `format()`/`write_to()` output surface.
- **Out of Scope**: Per-formatter configuration (see `api/003_config_types.md`), formatter list (see `001_three_layer_architecture.md`).

### Traits

| File | Relationship |
|------|-------------|
| [001_format.md](../trait/001_format.md) | Format trait definition |
| [002_table_shaped_formatter.md](../trait/002_table_shaped_formatter.md) | TableShapedFormatter trait — removed in v0.3.0 |
| [003_table_shaped_view.md](../trait/003_table_shaped_view.md) | TableShapedView trait definition |

### APIs

| File | Relationship |
|------|-------------|
| [004_formatters.md](../api/004_formatters.md) | Formatter API surface |

### Docs

| File | Relationship |
|------|-------------|
| [`../architecture.md`](../architecture.md) | Original combined architecture document (retained per migration rules) |

### Problem

The formatter layer must support multiple formatter types with different data requirements under a single unified API. Some formatters operate on flat row/column data; others work directly on hierarchical tree structures. Without a clear design, adding a new formatter requires changing caller code or creating formatter-specific data preparation paths.

### Solution

The formatter layer is unified under a single trait axis today. The `Format` trait provides the universal output interface shared by every formatter, consuming `TableView` values (built via `RowBuilder::build_view()`). Historically, a second axis existed: the `TableShapedFormatter` trait provided polymorphism for the subset of formatters that operated on flat row/column data. It was removed in v0.3.0; `Format` is now the sole formatter interface. Tree-specific formatters (`TreeFormatter`) operate on hierarchical tree data directly rather than through `TableView`.

#### TableShapedView Decoupling

`TableFormatter` and `ExpandedFormatter` do not call `TableShapedView` internally — both consume `TableView` (built via `RowBuilder::build_view()`), whose `rows` field holds `DecoratedText` cells (text plus optional ANSI color and independent bold/dim weight flags), not plain strings. `TableShapedView` is a separate, standalone trait (blanket-implemented for any `TreeNode<T: Display>` in `src/data.rs`) available to callers that already hold a table-shaped `TreeNode` and want direct `Vec<String>` header/row extraction without going through `RowBuilder` — see `trait/003_table_shaped_view.md`.

`TreeFormatter` renders hierarchical tree data directly using method-level generics, independent of both `TableView` and `TableShapedView`. Its format methods accept tree references directly, producing box-drawing output with configurable symbols and indentation.

#### TableShapedFormatter Trait (Removed in v0.3.0)

> **Removed in v0.3.0.** Use `Format` trait with `RowBuilder::build_view()` instead.

The `TableShapedFormatter` trait was the original polymorphism interface between `TableFormatter` and `ExpandedFormatter`. It was removed — not merely deprecated — in the v0.3.0 API cleanup; the `Format` trait is the sole canonical interface. See `trait/002_table_shaped_formatter.md` for the removed trait's historical definition.

#### Output Surface

All formatters support both output modes:

- `format()` — returns `String`
- `write_to()` — writes to any `io::Write`

### Applicability

Apply this pattern when adding a new formatter to the library. New formatters must implement `Format` — `TableShapedFormatter` no longer exists as an option. Build or accept a `TableView` (typically via `RowBuilder::build_view()`) when the formatter produces tabular output; operate on tree data directly when the formatter produces hierarchical output.

### Consequences

Decoupling via `TableView` prevents formatters from depending on tree traversal details. The `Format` trait is the canonical extension point — new formatters implement `Format`; the removed `TableShapedFormatter` is no longer an option. The dual `format()`/`write_to()` surface satisfies both in-memory and streaming output needs without separate formatter types. `TableShapedFormatter` was removed in v0.3.0 rather than preserved, eliminating the maintenance burden of two parallel interfaces — `Format` is the sole formatter interface today.

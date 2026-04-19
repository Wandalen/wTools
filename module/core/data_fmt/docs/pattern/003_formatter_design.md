# Pattern: Formatter Design

### Scope

- **Purpose**: Document the formatter trait hierarchy and the TableShapedView decoupling pattern.
- **Responsibility**: Complete description of how formatters consume data and expose output APIs.
- **In Scope**: `Format` trait, `TableShapedFormatter` trait, `TableShapedView` decoupling, `format()`/`write_to()` output surface.
- **Out of Scope**: Per-formatter configuration (see `api/003_config_types.md`), formatter list (see `001_three_layer_architecture.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../architecture.md` | Original combined architecture document (retained per migration rules) |
| doc | `../trait/001_format.md` | Format trait definition |
| doc | `../trait/002_table_shaped_formatter.md` | TableShapedFormatter trait definition |
| doc | `../trait/003_table_shaped_view.md` | TableShapedView trait definition |
| doc | `../api/004_formatters.md` | Formatter API surface |

### Problem

The formatter layer must support multiple formatter types with different data requirements under a single unified API. Some formatters operate on flat row/column data; others work directly on hierarchical tree structures. Without a clear design, adding a new formatter requires changing caller code or creating formatter-specific data preparation paths.

### Solution

The formatter layer uses two orthogonal trait axes. The `Format` trait provides the universal output interface shared by every formatter. The `TableShapedFormatter` trait (deprecated) provides polymorphism for the subset of formatters that operate on flat row/column data extracted via `TableShapedView`. Tree-specific formatters bypass `TableShapedView` and operate on `TreeNode< T >` directly.

#### TableShapedView Decoupling

`TableFormatter` and `ExpandedFormatter` use the `TableShapedView` trait to extract headers and rows from any `TreeNode< T >` where `T : Display`. This decouples formatting logic from tree internals — formatters work with flat vectors of strings.

`TreeFormatter` renders `TreeNode< T >` directly using method-level generics rather than relying on `TableShapedView`. Its format methods accept `&TreeNode< T >` where `T : Display`, producing box-drawing output with configurable symbols and indentation.

#### TableShapedFormatter Trait (Deprecated)

> **Deprecated since 0.1.0.** Use `Format` trait with `RowBuilder::build_view()` instead.

The `TableShapedFormatter` trait was the original polymorphism interface between `TableFormatter` and `ExpandedFormatter`. It is now deprecated; the `Format` trait is the canonical interface.

#### Output Surface

All formatters support both output modes:

- `format()` — returns `String`
- `write_to()` — writes to any `io::Write`

### Applicability

Apply this pattern when adding a new formatter to the library. New formatters must implement `Format` (not `TableShapedFormatter`). Use `TableShapedView` to extract flat row/column data when the formatter produces tabular output; operate on `TreeNode< T >` directly when the formatter produces hierarchical output.

### Consequences

Decoupling via `TableShapedView` prevents formatters from depending on tree traversal details. The `Format` trait is the canonical extension point — new formatters implement `Format`, not the deprecated `TableShapedFormatter`. The dual `format()`/`write_to()` surface satisfies both in-memory and streaming output needs without separate formatter types. The deprecated `TableShapedFormatter` adds maintenance burden but is preserved for backward compatibility in 0.x versions.

### Sources

| File | Notes |
|------|-------|
| [../architecture.md](../architecture.md) | Original source; section "Formatter Design" extracted into this instance |

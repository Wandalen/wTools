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

### Description

The formatter layer uses two orthogonal trait axes. The `Format` trait provides the universal output interface shared by every formatter. The `TableShapedFormatter` trait provides polymorphism for the subset of formatters that operate on flat row/column data extracted via `TableShapedView`. Tree-specific formatters bypass `TableShapedView` and operate on `TreeNode< T >` directly.

### Structure

#### TableShapedView Decoupling

`TableFormatter` and `ExpandedFormatter` use the `TableShapedView` trait to extract headers and rows from any `TreeNode< T >` where `T : Display`. This decouples formatting logic from tree internals — formatters work with flat vectors of strings.

`TreeFormatter` renders `TreeNode< T >` directly using method-level generics rather than relying on `TableShapedView`. Its `format()` and `format_aligned()` methods accept `&TreeNode< T >` where `T : Display`, producing box-drawing output with configurable symbols and indentation.

#### TableShapedFormatter Trait

The `TableShapedFormatter` trait provides polymorphism between `TableFormatter` and `ExpandedFormatter`:

```rust
pub trait TableShapedFormatter
{
  fn format( &self, tree : &TreeNode< String > ) -> String;
}
```

#### Output Surface

All formatters support both output modes:

- `format()` — returns `String`
- `write_to()` — writes to any `io::Write`

### Rationale

Decoupling via `TableShapedView` prevents formatters from depending on tree traversal details. Adding a new table-shaped formatter requires only implementing `TableShapedFormatter` — no changes to data structures. The dual `format()`/`write_to()` surface satisfies both in-memory and streaming output needs without separate formatter types.

### Sources

| File | Notes |
|------|-------|
| [../architecture.md](../architecture.md) | Original source; section "Formatter Design" extracted into this instance |

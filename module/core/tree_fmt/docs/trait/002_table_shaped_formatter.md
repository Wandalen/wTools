# Trait: TableShapedFormatter

### Scope

- **Purpose**: Document the TableShapedFormatter interface contract, implementors, and coverage.
- **Responsibility**: Define the legacy formatting trait and its relationship to the modern Format trait.
- **In Scope**: Trait definition, implementor table, input type, migration path to Format trait.
- **Out of Scope**: Formatter implementation (see `../feature/`), variant output (see `../variant/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/mod.rs` | TableShapedFormatter trait definition |
| test | `tests/formatters.rs` | Formatter trait tests |

### Definition

```rust
pub trait TableShapedFormatter
{
  fn format( &self, tree : &TreeNode< String > ) -> String;
}
```

### Implementors

| Formatter | Also Implements `Format` |
|-----------|:------------------------:|
| `TableFormatter` | yes |
| `ExpandedFormatter` | **no** |

### Input Type

`TreeNode<String>` — a table encoded as a tree where:
- Root's children are rows (named `"1"`, `"2"`, ...)
- Each row's children are cells (named by column header, data = cell value)

Produced by `RowBuilder::build()`.

### Relationship to Format Trait

`TableShapedFormatter` is the older interface. `Format` is the modern replacement.

| Aspect | `TableShapedFormatter` | `Format` |
|--------|----------------------|----------|
| Input | `&TreeNode<String>` | `&TableView` |
| Output | `String` | `Result<String, FormatError>` |
| Implementors | 2 | 8 |
| Error handling | None (infallible) | `FormatError` |
| Status | Legacy | Current |

### Migration Path

For `TableFormatter`: switch from `TableShapedFormatter::format()` to `Format::format()` by changing `build()` to `build_view()`.

For `ExpandedFormatter`: no `Format` impl exists yet — must continue using `TableShapedFormatter` with `RowBuilder::build()`.

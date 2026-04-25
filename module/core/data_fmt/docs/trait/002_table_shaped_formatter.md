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

### Signature

> **Deprecated since 0.1.0.** Use `Format` trait with `RowBuilder::build_view()` instead.

`TableShapedFormatter` has one method. It takes an immutable reference to self and an immutable reference to a table-encoded tree; returns a formatted string without error handling. Unlike `Format`, formatting is infallible — there is no error wrapper.

### Implementors

| Formatter | Also Implements `Format` |
|-----------|:------------------------:|
| `TableFormatter` | yes |
| `ExpandedFormatter` | **no** |

### Coverage Gaps

| Formatter | Status | Note |
|-----------|--------|------|
| `ExpandedFormatter` | Implements `TableShapedFormatter` but not `Format` | No modern `Format` impl exists — callers must continue using the deprecated path |
| All other formatters | Do not implement `TableShapedFormatter` | Intentional — they implement `Format` directly with the modern `TableView` input |

### Input Type

Table-encoded tree — a tree where:
- Root's children are rows (named `"1"`, `"2"`, ...)
- Each row's children are cells (named by column header, data = cell value)

Produced by `RowBuilder::build()`.

### Relationship to Format Trait

`TableShapedFormatter` is the older interface. `Format` is the modern replacement.

| Aspect | `TableShapedFormatter` | `Format` |
|--------|----------------------|----------|
| Input | table-encoded tree | `&TableView` |
| Output | formatted string (infallible) | formatted string or error |
| Implementors | 2 | 8 |
| Error handling | None (infallible) | `FormatError` |
| Status | **Deprecated** | Current |

### Migration Path

For `TableFormatter`: switch from `TableShapedFormatter::format()` to `Format::format()` by changing `build()` to `build_view()`.

For `ExpandedFormatter`: no `Format` impl exists yet — must continue using `TableShapedFormatter` with `RowBuilder::build()`.

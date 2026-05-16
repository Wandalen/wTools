# Trait: TableShapedFormatter

> **Removed in v0.3.0.** Use the `Format` trait with `RowBuilder::build_view()` instead.

### Scope

- **Purpose**: Document the removed TableShapedFormatter interface contract for historical reference.
- **Responsibility**: Record the legacy formatting trait, its former implementors, and the migration path.
- **In Scope**: Former trait signature, former implementors, migration guide.
- **Out of Scope**: Active formatter contracts (see `001_format.md`), formatter implementation (see `../feature/`).

### Sources
| File | Relationship |
|------|-------------|
| `src/formatters/mod.rs` | Former trait definition — removed in v0.3.0 |

### Tests
| File | Relationship |
|------|-------------|
| `tests/formatters.rs` | Tests migrated off deprecated API in v0.3.0 |

### Signature

`TableShapedFormatter` had one method: took `&self` and `&TreeNode< String >` (table-encoded tree); returned a formatted `String` without error handling. Formatting was infallible — no error wrapper.

### Former Implementors

| Formatter | Also Implemented `Format` |
|-----------|:-------------------------:|
| `TableFormatter` | yes |
| `ExpandedFormatter` | yes (since v0.3.0) |

### Migration Path

For all callers:
1. Replace `RowBuilder::build()` with `RowBuilder::build_view()` to get a `TableView`.
2. Replace `TableShapedFormatter::format( &tree )` with `Format::format( &formatter, &view ).unwrap()`.
3. Remove `use data_fmt::formatters::TableShapedFormatter` import.

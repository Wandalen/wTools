# Feature: API Cleanup v0.3.0

### Scope

- **Purpose**: Remove all backward-compatibility shims from the public API and complete the unified Format interface by adding `ExpandedFormatter` as a `Format` implementor.
- **Responsibility**: Document the v0.3.0 breaking-change boundary, the items removed, and the migration path for each.
- **In Scope**: Removed APIs (TableShapedFormatter trait, RowBuilder::build(), format(&TreeNode), to_tree_node()), added API (impl Format for ExpandedFormatter), internal cleanup (conversions.rs, write_to signatures, ansi_str comment).
- **Out of Scope**: Formatter algorithm details (see `../algorithm/`), variant output examples (see `../variant/`), stable promotion (see `../../../doc/layers.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/expanded.rs` | ExpandedFormatter Format impl to add |
| source | `src/formatters/mod.rs` | TableShapedFormatter trait to remove |
| source | `src/table_tree.rs` | RowBuilder::build() and root field to remove |
| source | `src/data.rs` | TableView::to_tree_node() to remove |
| source | `src/conversions.rs` | flatten_to_table_tree return type fix |
| test | `tests/formatters.rs` | Tests to migrate off deprecated APIs |
| test | `tests/fluent_api.rs` | Tests to migrate off deprecated APIs |
| doc | `../trait/001_format.md` | Format trait — gains ExpandedFormatter coverage |
| doc | `../trait/002_table_shaped_formatter.md` | Deprecated trait — to be deleted |
| doc | `../formatter/002_expanded_formatter.md` | ExpandedFormatter — gains Format impl |
| doc | `../api/002_builders.md` | Builders API — build() removed |

### Design

#### Background

v0.1.0 introduced the `Format` trait + `TableView` as the modern unified API, retaining the
older `TableShapedFormatter` + `TreeNode<String>` path with a deprecation notice.
v0.2.0 added `build_view()` to `RowBuilder` as the new terminal operation alongside the old `build()`.

v0.3.0 removes all deprecated paths. After this release there is no backward-compat code in the
codebase — one API, one data model, one trait.

#### What Changes

**Added:**

| Item | Type | Note |
|------|------|------|
| `impl Format for ExpandedFormatter` | new impl | Completes 9-of-10 formatter coverage |

**Removed:**

| Item | Type | Replacement |
|------|------|-------------|
| `TableShapedFormatter` trait | public trait | Use `Format` trait |
| `RowBuilder::build()` | method | Use `RowBuilder::build_view()` |
| `TableFormatter::format(&TreeNode<String>)` | method | Use `Format::format(&TableView)` |
| `ExpandedFormatter::format(&TreeNode<String>)` | method | Use `Format::format(&TableView)` |
| `TableView::to_tree_node()` | method | No replacement needed (zero callers) |
| `pub use formatters::TableShapedFormatter` in lib.rs | re-export | Removed with the trait |

**Internal fixes (non-breaking):**

| Item | Change | Reason |
|------|--------|--------|
| `conversions::flatten_to_table_tree` | returns `TableView` instead of `TreeNode<String>` | Removes internal dependency on deleted TreeNode-building path |
| `TableFormatter::write_to` | accepts `&TableView` instead of `&TreeNode<String>` | Aligns with Format trait input type |
| `ExpandedFormatter::write_to` | accepts `&TableView` instead of `&TreeNode<String>` | Same |
| `src/ansi_str.rs` comment | removes "backward compatibility" framing | Re-exports are active, not compat shims |

#### Migration Guide

| From | To | Change |
|------|----|--------|
| `RowBuilder::new(...).add_row(...).build()` | `.build_view()` | Final method only |
| `formatter.format( &tree )` | `Format::format( &formatter, &view ).unwrap()` | Trait call + unwrap |
| `let tree : TreeNode<String>` | `let view : TableView` | Variable type |
| `use data_fmt::formatters::TableShapedFormatter` | remove import | Trait gone |
| `write_to( &tree, &mut w )` | `write_to( &view, &mut w )` | Input type only |

#### Invariants Preserved

- `TreeBuilder::build()` is NOT affected. Only `RowBuilder::build()` is removed.
- `format_tree<T>()` on both formatters is NOT removed (genuine purpose: flatten hierarchical trees). Internals updated to call `Format::format()` internally.
- `TableShapedView` trait is NOT affected. Only `TableShapedFormatter` is removed.
- `ExpandedFormatter` record separator now uses 1-based row index (`(idx+1).to_string()`) instead of `row_node.name` since `TableView` rows have no names.

### Acceptance Criteria

- AC-001: `grep -r "TableShapedFormatter" src/ tests/` returns 0 matches
- AC-002: `grep -r "allow( deprecated )" src/ tests/` returns 0 matches
- AC-003: `grep -n "impl.*Format.*for ExpandedFormatter" src/formatters/expanded.rs` returns 1 match
- AC-004: `grep -n "fn build\b" src/table_tree.rs` returns 0 matches (RowBuilder::build deleted)
- AC-005: `w3 .test level::3` exits 0 with all tests passing
- AC-006: `grep '^version' Cargo.toml` returns `version = "0.3.0"`

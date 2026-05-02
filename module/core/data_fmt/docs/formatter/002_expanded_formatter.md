# Formatter: ExpandedFormatter

### Scope

- **Purpose**: Render tabular data as vertical key-value records, one record per row with labeled field names.
- **Responsibility**: Document the `ExpandedFormatter` struct — its trait paths, input type, and the 2 style variants it produces.
- **In Scope**: Trait implementation, input type, `ExpandedConfig` preset mechanism, per-variant feature flags.
- **Out of Scope**: Variant output details (see `../variant/010_expanded_postgres_style.md`, `011_expanded_property_style.md`), operation signatures (see `../api/004_formatters.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/expanded.rs` | `ExpandedFormatter` implementation |
| source | `src/config.rs` | `ExpandedConfig` struct and preset methods |
| doc | `../api/004_formatters.md` | Operation signatures |
| doc | `../trait/001_format.md` | Unified `Format` trait (to implement in v0.3.0) |
| doc | `../trait/002_table_shaped_formatter.md` | Deprecated `TableShapedFormatter` trait |
| doc | `../feature/006_api_cleanup_v030.md` | v0.3.0 cleanup — adds Format impl, removes deprecated path |
| doc | `../variant/010_expanded_postgres_style.md` | Variant: postgres_style |
| doc | `../variant/011_expanded_property_style.md` | Variant: property_style |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ❌ Not implemented (to add in v0.3.0) | See `../feature/006_api_cleanup_v030.md` |
| `TableShapedFormatter` | ⚠️ Deprecated since 0.1.0 | Only available interface — takes a table-encoded tree, returns formatted string; removed in v0.3.0 |

`ExpandedFormatter` is the only formatter with zero `Format` implementations. All callers must use the deprecated `TableShapedFormatter` path until v0.3.0 remediation (see `../feature/006_api_cleanup_v030.md`).

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | v0.3.0+ | `Format` trait (to add — see `../feature/006_api_cleanup_v030.md`) |
| table-encoded tree | Legacy (v0.1.0–v0.2.x) | `TableShapedFormatter` trait (deprecated, removed in v0.3.0) |

In v0.2.x: `TableView` is not accepted — use `RowBuilder::build()` (not `build_view()`) to produce the legacy input. In v0.3.0: `TableShapedFormatter` is removed and `RowBuilder::build()` is deleted; use `RowBuilder::build_view()` with the `Format` trait.

### Variants

Selection mechanism: pass an `ExpandedConfig` preset to `ExpandedFormatter::with_config(config)`.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| postgres_style | `ExpandedConfig::postgres_style()` | `expanded_postgres` |
| property_style | `ExpandedConfig::property_style()` | `expanded_property` |

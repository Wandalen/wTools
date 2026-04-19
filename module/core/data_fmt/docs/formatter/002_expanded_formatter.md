# Formatter: ExpandedFormatter

### Scope

- **Purpose**: Render tabular data as vertical key-value records, one record per row with labeled field names.
- **Responsibility**: Document the `ExpandedFormatter` struct — its deprecated-only trait path, input type, and the 2 style variants it produces.
- **In Scope**: Trait implementation (deprecated only), input type, `ExpandedConfig` preset mechanism, per-variant feature flags.
- **Out of Scope**: Variant output details (see `../variant/010_expanded_postgres_style.md`, `011_expanded_property_style.md`), operation signatures (see `../api/004_formatters.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/expanded.rs` | `ExpandedFormatter` implementation |
| source | `src/config.rs` | `ExpandedConfig` struct and preset methods |
| doc | `../api/004_formatters.md` | Operation signatures |
| doc | `../trait/002_table_shaped_formatter.md` | Deprecated `TableShapedFormatter` trait |
| doc | `../variant/010_expanded_postgres_style.md` | Variant: postgres_style |
| doc | `../variant/011_expanded_property_style.md` | Variant: property_style |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ❌ Not implemented | `ExpandedFormatter` cannot be used through the unified `Format` interface |
| `TableShapedFormatter` | ⚠️ Deprecated since 0.1.0 | Only available interface — takes `&TreeNode<String>`, returns `String` |

`ExpandedFormatter` is the only formatter with zero `Format` implementations. All callers must use the deprecated `TableShapedFormatter` path until this is remediated.

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TreeNode<String>` | Legacy only | `TableShapedFormatter` trait (deprecated) |

`TableView` is not accepted. Use `RowBuilder::build()` (not `build_view()`) to produce the required input.

### Variants

Selection mechanism: pass an `ExpandedConfig` preset to `ExpandedFormatter::with_config(config)`.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| postgres_style | `ExpandedConfig::postgres_style()` | `expanded_postgres` |
| property_style | `ExpandedConfig::property_style()` | `expanded_property` |

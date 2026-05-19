# Formatter: ExpandedFormatter

### Scope

- **Purpose**: Render tabular data as vertical key-value records, one record per row with labeled field names.
- **Responsibility**: Document the `ExpandedFormatter` struct — its `Format` trait implementation, input type, and the 2 style variants it produces.
- **In Scope**: Trait implementation, input type, `ExpandedConfig` preset mechanism, per-variant feature flags.
- **Out of Scope**: Variant output details (see `../variant/010_expanded_postgres_style.md`, `011_expanded_property_style.md`), operation signatures (see `../api/004_formatters.md`).

### Traits

| File | Relationship |
|------|-------------|
| [001_format.md](../trait/001_format.md) | `Format` trait contract |

### APIs

| File | Relationship |
|------|-------------|
| [004_formatters.md](../api/004_formatters.md) | Operation signatures |

### Variants

| File | Relationship |
|------|-------------|
| [010_expanded_postgres_style.md](../variant/010_expanded_postgres_style.md) | Variant: postgres_style |
| [011_expanded_property_style.md](../variant/011_expanded_property_style.md) | Variant: property_style |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/expanded.rs`](../../src/formatters/expanded.rs) | `ExpandedFormatter` implementation |
| [`src/config.rs`](../../src/config.rs) | `ExpandedConfig` struct and preset methods |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes a `TableView`, returns formatted string or error |

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |

### Variants

Selection mechanism: pass an `ExpandedConfig` preset to `ExpandedFormatter::with_config(config)`.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| postgres_style | `ExpandedConfig::postgres_style()` | `expanded_postgres` |
| property_style | `ExpandedConfig::property_style()` | `expanded_property` |

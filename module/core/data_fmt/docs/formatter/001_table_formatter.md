# Formatter: TableFormatter

### Scope

- **Purpose**: Render tabular data as horizontal tables with configurable borders, column sizing, and color.
- **Responsibility**: Document the `TableFormatter` struct — its trait implementations, input types, and the 9 style variants it produces.
- **In Scope**: Trait implementations, input paths, `TableConfig` preset mechanism, per-variant feature flags.
- **Out of Scope**: Variant output details (see `../variant/001_table_plain.md` through `009_table_compact.md`), operation signatures (see `../api/004_formatters.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/table/mod.rs` | `TableFormatter` implementation |
| source | `src/config.rs` | `TableConfig` struct and preset methods |
| doc | `../api/004_formatters.md` | Operation signatures |
| doc | `../trait/001_format.md` | `Format` trait contract |
| doc | `../trait/002_table_shaped_formatter.md` | Deprecated `TableShapedFormatter` trait |
| doc | `../variant/001_table_plain.md` | Variant: plain |
| doc | `../variant/002_table_minimal.md` | Variant: minimal |
| doc | `../variant/003_table_bordered.md` | Variant: bordered |
| doc | `../variant/004_table_markdown.md` | Variant: markdown |
| doc | `../variant/005_table_grid.md` | Variant: grid |
| doc | `../variant/006_table_unicode_box.md` | Variant: unicode_box |
| doc | `../variant/007_table_csv.md` | Variant: csv |
| doc | `../variant/008_table_tsv.md` | Variant: tsv |
| doc | `../variant/009_table_compact.md` | Variant: compact |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Primary path — takes a `TableView`, returns formatted string or error |
| `TableShapedFormatter` | ⚠️ Deprecated since 0.1.0 | Legacy path — takes a table-encoded tree, returns formatted string |

`TableFormatter` is the only formatter implementing both traits. New code must use `Format`.

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |
| table-encoded tree | Legacy (deprecated) | `TableShapedFormatter` trait |

### Variants

Selection mechanism: pass a `TableConfig` preset to `TableFormatter::with_config(config)`.

| Variant | Selector | Feature Flag |
|---------|----------|--------------|
| plain | `TableConfig::plain()` | `table_plain` |
| minimal | `TableConfig::minimal()` | `table_minimal` |
| bordered | `TableConfig::bordered()` | `table_bordered` |
| markdown | `TableConfig::markdown()` | `table_markdown` |
| grid | `TableConfig::grid()` | `table_grid` |
| unicode_box | `TableConfig::unicode_box()` | `table_unicode` |
| csv | `TableConfig::csv()` | `table_csv` |
| tsv | `TableConfig::tsv()` | `table_tsv` |
| compact | `TableConfig::compact()` | `table_compact` |

Each feature flag compiles the formatter independently — unused variants add no binary overhead.

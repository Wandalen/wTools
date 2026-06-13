# Formatter: TableFormatter

### Scope

- **Purpose**: Render tabular data as horizontal tables with configurable borders, column sizing, and color.
- **Responsibility**: Document the `TableFormatter` struct — its trait implementations, input types, and the 9 style variants it produces.
- **In Scope**: Trait implementations, input paths, `TableConfig` preset mechanism, per-variant feature flags.
- **Out of Scope**: Variant output details (see `../variant/001_table_plain.md` through `009_table_compact.md`), operation signatures (see `../api/004_formatters.md`).

### Traits

| File | Relationship |
|------|-------------|
| [001_format.md](../trait/001_format.md) | `Format` trait contract |
| [002_table_shaped_formatter.md](../trait/002_table_shaped_formatter.md) | `TableShapedFormatter` — removed in v0.3.0 |

### APIs

| File | Relationship |
|------|-------------|
| [004_formatters.md](../api/004_formatters.md) | Operation signatures |

### Variants

| File | Relationship |
|------|-------------|
| [001_table_plain.md](../variant/001_table_plain.md) | Variant: plain |
| [002_table_minimal.md](../variant/002_table_minimal.md) | Variant: minimal |
| [003_table_bordered.md](../variant/003_table_bordered.md) | Variant: bordered |
| [004_table_markdown.md](../variant/004_table_markdown.md) | Variant: markdown |
| [005_table_grid.md](../variant/005_table_grid.md) | Variant: grid |
| [006_table_unicode_box.md](../variant/006_table_unicode_box.md) | Variant: unicode_box |
| [007_table_csv.md](../variant/007_table_csv.md) | Variant: csv |
| [008_table_tsv.md](../variant/008_table_tsv.md) | Variant: tsv |
| [009_table_compact.md](../variant/009_table_compact.md) | Variant: compact |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | `TableFormatter` implementation |
| [`src/config.rs`](../../src/config.rs) | `TableConfig` struct and preset methods |

### Variant Selection

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

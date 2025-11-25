# Feature Selection Guide

How to choose the right cargo features for your use case.

## Quick Start

### I want everything

```toml
tree_fmt = { version = "0.5.0", features = ["all_formats"] }
```

### I want minimal binary size

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = ["table_plain"] }
```

### I want default formatters only

```toml
tree_fmt = "0.5.0"
# Includes: table_plain, expanded_postgres, tree_hierarchical, format_logfmt
```

## Decision Tree

```
What do you need to format?

├─ Tabular data
│  ├─ For CLI tools (like ps, top) → table_plain (default)
│  ├─ For export to Excel → table_csv
│  ├─ For spreadsheet paste → table_tsv
│  ├─ For GitHub README → table_markdown
│  ├─ For database output → table_bordered
│  ├─ For formal reports → table_grid
│  ├─ For modern terminals → table_unicode
│  ├─ For narrow terminals → table_compact
│  └─ For maximum simplicity → table_minimal
│
├─ Single record details
│  ├─ PostgreSQL \x mode style → expanded_postgres (default)
│  └─ Property list style → expanded_property
│
├─ Hierarchical/tree data
│  ├─ Standard tree view → tree_hierarchical (default)
│  ├─ With column alignment → tree_aligned
│  └─ With subtree totals → tree_aggregated
│
├─ Text lists
│  └─ All variants → format_text
│     (Bullets, Numbered, Sections, KeyValue, Compact)
│
├─ JSON output
│  └─ format_json (Pretty/Compact modes at runtime)
│
├─ YAML output
│  └─ format_yaml
│
├─ TOML output
│  └─ format_toml
│
├─ Logfmt logging
│  └─ format_logfmt (default)
│
├─ HTML output
│  ├─ Plain HTML → html_minimal (default)
│  ├─ Bootstrap 5 → html_bootstrap
│  ├─ Tailwind CSS → html_tailwind
│  └─ Custom classes → html_custom
│
└─ SQL output
   ├─ Standard SQL → sql_ansi (default)
   ├─ PostgreSQL → sql_postgres
   ├─ MySQL/MariaDB → sql_mysql
   └─ SQLite3 → sql_sqlite
```

## Use Case Examples

### CLI Tool (process monitor, system status)

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = ["table_plain"] }
```

**Size**: ~100 KB
**Formatters**: TableFormatter (plain variant only)

### Data Export Pipeline

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_csv",
  "table_tsv",
  "format_json",
  "format_yaml"
] }
```

**Size**: ~150 KB
**Formatters**: TableFormatter (CSV, TSV), JsonFormatter, YamlFormatter

### Documentation Generator

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_markdown",
  "html_minimal",
  "format_json"
] }
```

**Size**: ~150 KB
**Formatters**: TableFormatter (Markdown), HtmlFormatter (Minimal), JsonFormatter

### Database Administration Tool (PostgreSQL)

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_bordered",
  "expanded_postgres",
  "sql_postgres",
  "format_json"
] }
```

**Size**: ~180 KB
**Formatters**: TableFormatter (bordered), ExpandedFormatter (postgres_style), SqlFormatter (PostgreSQL), JsonFormatter

### Web Application (Bootstrap UI)

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "html_bootstrap",
  "format_json"
] }
```

**Size**: ~130 KB
**Formatters**: HtmlFormatter (Bootstrap), JsonFormatter

### Configuration File Manager

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "format_json",
  "format_yaml",
  "format_toml"
] }
```

**Size**: ~140 KB
**Formatters**: JsonFormatter, YamlFormatter, TomlFormatter

### Log Analysis Tool

```toml
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_plain",
  "format_logfmt",
  "format_json"
] }
```

**Size**: ~120 KB
**Formatters**: TableFormatter (plain), LogfmtFormatter, JsonFormatter

### Full-Featured Application

```toml
tree_fmt = { version = "0.5.0", features = ["all_formats"] }
```

**Size**: ~500 KB
**Formatters**: All 10 formatters with all 32 variants

## Feature Compilation Matrix

| Feature Flag | Formatter | Variant | Default | Binary Impact |
|-------------|-----------|---------|---------|---------------|
| `table_plain` | TableFormatter | plain | Yes | +30 KB |
| `table_minimal` | TableFormatter | minimal | No | +5 KB |
| `table_bordered` | TableFormatter | bordered | No | +8 KB |
| `table_markdown` | TableFormatter | markdown | No | +6 KB |
| `table_grid` | TableFormatter | grid | No | +10 KB |
| `table_unicode` | TableFormatter | unicode_box | No | +12 KB |
| `table_csv` | TableFormatter | csv | No | +8 KB |
| `table_tsv` | TableFormatter | tsv | No | +6 KB |
| `table_compact` | TableFormatter | compact | No | +5 KB |
| `expanded_postgres` | ExpandedFormatter | postgres_style | Yes | +15 KB |
| `expanded_property` | ExpandedFormatter | property_style | No | +5 KB |
| `tree_hierarchical` | TreeFormatter | hierarchical | Yes | +20 KB |
| `tree_aligned` | TreeFormatter | aligned | No | +8 KB |
| `tree_aggregated` | TreeFormatter | aggregated | No | +10 KB |
| `format_text` | TextFormatter | All 6 variants | Yes | +25 KB |
| `format_json` | JsonFormatter | Both modes | No | +30 KB |
| `format_yaml` | YamlFormatter | Standard | No | +40 KB |
| `format_toml` | TomlFormatter | Standard | No | +35 KB |
| `format_logfmt` | LogfmtFormatter | Standard | Yes | +10 KB |
| `html_minimal` | HtmlFormatter | Minimal | No | +20 KB |
| `html_bootstrap` | HtmlFormatter | Bootstrap | No | +8 KB |
| `html_tailwind` | HtmlFormatter | Tailwind | No | +8 KB |
| `html_custom` | HtmlFormatter | Custom | No | +6 KB |
| `sql_ansi` | SqlFormatter | ANSI | No | +25 KB |
| `sql_postgres` | SqlFormatter | PostgreSQL | No | +8 KB |
| `sql_mysql` | SqlFormatter | MySQL | No | +8 KB |
| `sql_sqlite` | SqlFormatter | SQLite | No | +8 KB |

**Note**: Binary sizes are approximate and depend on optimization level and target platform

## Meta-Features

Convenience features that enable multiple variants:

| Meta-Feature | Enables | Use When |
|-------------|---------|----------|
| `format_table` | All 9 table variants | You need all table formats |
| `format_table_visual` | All visual table variants (excludes CSV/TSV) | You need tables for display, not export |
| `format_table_export` | CSV + TSV | You need data export only |
| `format_expanded` | Both expanded variants | You need both expanded styles |
| `format_tree` | All 3 tree variants | You need all tree formats |
| `format_html` | All 4 HTML variants | You need all HTML styles |
| `format_html_basic` | Minimal + Custom | You need HTML without frameworks |
| `format_html_frameworks` | Bootstrap + Tailwind | You need framework support |
| `format_sql` | All 4 SQL variants | You support multiple databases |
| `all_formats` | Everything | You want maximum flexibility |

## Configuration Comparison

| Configuration | Features | Binary Size | Formatters | Variants |
|--------------|----------|-------------|------------|----------|
| Minimal | `table_plain` | ~100 KB | 1 | 1 |
| Default | `default` | ~200 KB | 4 | 4 |
| Export | `table_csv`, `table_tsv`, `format_json` | ~150 KB | 2 | 3 |
| Database | `table_bordered`, `expanded_postgres`, `sql_postgres` | ~180 KB | 3 | 3 |
| Web | `html_bootstrap`, `format_json` | ~130 KB | 2 | 2 |
| Config | `format_json`, `format_yaml`, `format_toml` | ~140 KB | 3 | 3 |
| Full | `all_formats` | ~500 KB | 10 | 31 |

## Migration from v0.4.0

### No changes needed (meta-features remain)

```toml
# v0.4.0 (still works in v0.5.0)
tree_fmt = { version = "0.4.0", features = ["format_json"] }

# v0.5.0 (identical behavior via meta-feature)
tree_fmt = { version = "0.5.0", features = ["format_json"] }
```

### Optimize for smaller binary

```toml
# v0.4.0 (enables all table variants)
tree_fmt = { version = "0.4.0" }

# v0.5.0 (enable only needed variants)
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_plain",
  "table_csv"
] }
```

### Granular variant selection (new in v0.5.0)

```toml
# Now possible: PostgreSQL-specific tools
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_bordered",
  "sql_postgres"
] }

# Now possible: Bootstrap-only HTML
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "html_bootstrap"
] }

# Now possible: Export pipeline only
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_csv",
  "table_tsv",
  "format_json",
  "format_yaml"
] }
```

## Troubleshooting

### Compilation error: "no method named `csv`"

**Problem**: You're trying to use `TableConfig::csv()` but didn't enable the feature

**Solution**:
```toml
tree_fmt = { version = "0.5.0", features = ["table_csv"] }
```

### Compilation error: "cannot find type `HtmlVariant`"

**Problem**: You're using HtmlFormatter but didn't enable any HTML features

**Solution**:
```toml
# Enable at least one HTML variant
tree_fmt = { version = "0.5.0", features = ["html_minimal"] }

# Or enable all HTML variants
tree_fmt = { version = "0.5.0", features = ["format_html"] }
```

### Binary size larger than expected

**Problem**: Using `default` or meta-features includes more than you need

**Solution**: Use granular features instead
```toml
# Instead of this (includes 4 formatters)
tree_fmt = "0.5.0"

# Use this (only what you need)
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_plain",
  "format_json"
] }
```

## Best Practices

1. **Start minimal**: Use `default-features = false` and add only what you need
2. **Use meta-features for convenience**: `format_table` is easier than listing 9 features
3. **Test binary size**: Use `cargo bloat --release` to measure impact
4. **Document your choice**: Add comment explaining why you chose specific features
5. **Review periodically**: Remove unused features as requirements change

## Related Documentation

- [Feature Refactoring Plan](feature_refactoring_plan.md) - Technical implementation details
- [Table of Variants](../readme.md#table-of-variants) - Complete variant reference
- [Main README](../readme.md) - Project overview

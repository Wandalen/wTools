# tree_fmt

Multi-format data visualization library supporting 10 formatters with 31 variants and 6 color themes.

## Quick Start

```rust
use tree_fmt::{ RowBuilder, TableFormatter, Format };

let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
  .add_row( vec![ "Alice".into(), "30".into() ] )
  .add_row( vec![ "Bob".into(), "25".into() ] )
  .build_view();

let formatter = TableFormatter::new();
let output = formatter.format( &view )?;
println!( "{}", output );
```

Output:
```
 Name   Age
 -----  ---
 Alice  30
 Bob    25
```

## Table of Formatters

| Formatter         | Default      | Available Variants |
|-------------------|--------------|-------------------|
| TableFormatter    | plain        | 8 variants        |
| ExpandedFormatter | postgres_style | 2 variants      |
| TreeFormatter     | hierarchical | 3 variants        |
| TextFormatter     | Bullets      | 6 variants        |
| JsonFormatter     | Pretty       | 2 variants        |
| YamlFormatter     | Standard     | 1 variant         |
| TomlFormatter     | Standard     | 1 variant         |
| LogfmtFormatter   | Standard     | 1 variant         |
| HtmlFormatter     | Minimal      | 4 variants        |
| SqlFormatter      | ANSI         | 4 variants        |

**Total**: 10 formatters with 32 variants

Each formatter provides customizable formatter parameters via its Config type.

**Note**: ColorTheme is not a formatter - it's a color scheme system (6 themes: Dark/Light/Monokai/Solarized/Nord/Dracula) that applies to visual formatters.

## Table of Variants

| Formatter | Variant | Default | Feature Flag | Char Set | Constructor | Primary Use Case |
|-----------|---------|---------|--------------|----------|-------------|------------------|
| TableFormatter | [plain](docs/variant/table_plain.md) | Yes | table_plain | ASCII | `TableConfig::plain()` | CLI tools output (ps, top, pmon) |
| TableFormatter | [minimal](docs/variant/table_minimal.md) | No | table_minimal | ASCII | `TableConfig::minimal()` | Maximum simplicity, no separators |
| TableFormatter | [bordered](docs/variant/table_bordered.md) | No | table_bordered | ASCII | `TableConfig::bordered()` | Database output, PostgreSQL-style |
| TableFormatter | [markdown](docs/variant/table_markdown.md) | No | table_markdown | ASCII | `TableConfig::markdown()` | GitHub documentation, README files |
| TableFormatter | [grid](docs/variant/table_grid.md) | No | table_grid | ASCII | `TableConfig::grid()` | Formal reports with full ASCII box |
| TableFormatter | [unicode_box](docs/variant/table_unicode_box.md) | No | table_unicode | Unicode | `TableConfig::unicode_box()` | Modern terminal UIs |
| TableFormatter | [csv](docs/variant/table_csv.md) | No | table_csv | ASCII | `TableConfig::csv()` | Data export, Excel import |
| TableFormatter | [tsv](docs/variant/table_tsv.md) | No | table_tsv | ASCII | `TableConfig::tsv()` | Spreadsheet paste, clipboard data |
| TableFormatter | [compact](docs/variant/table_compact.md) | No | table_compact | ASCII | `TableConfig::compact()` | Narrow terminals, space-constrained |
| ExpandedFormatter | [postgres_style](docs/variant/expanded_postgres_style.md) | Yes | expanded_postgres | ASCII | `ExpandedConfig::postgres_style()` | PostgreSQL \x mode |
| ExpandedFormatter | [property_style](docs/variant/expanded_property_style.md) | No | expanded_property | ASCII | `ExpandedConfig::property_style()` | Property lists with colored keys |
| TreeFormatter | [hierarchical](docs/variant/tree_hierarchical.md) | Yes | tree_hierarchical | Unicode | `format()` | Standard tree view |
| TreeFormatter | [aligned](docs/variant/tree_aligned.md) | No | tree_aligned | Unicode | `format_aligned()` | Column-aligned tree with metadata |
| TreeFormatter | [aggregated](docs/variant/tree_aggregated.md) | No | tree_aggregated | Unicode | `format_with_aggregation()` | Tree with subtree totals |
| TextFormatter | [Bullets](docs/variant/text_bullets.md) | Yes | format_text | ASCII | `TextVariant::Bullets` | Bulleted lists |
| TextFormatter | [Numbered](docs/variant/text_numbered.md) | No | format_text | ASCII | `TextVariant::Numbered` | Numbered lists |
| TextFormatter | [Sections](docs/variant/text_sections.md) | No | format_text | ASCII | `TextVariant::Sections` | Section headers |
| TextFormatter | [KeyValue](docs/variant/text_keyvalue.md) | No | format_text | ASCII | `TextVariant::KeyValue` | Key-value pairs |
| TextFormatter | [Compact](docs/variant/text_compact.md) | No | format_text | ASCII | `TextVariant::Compact` | Dense text output |
| TextFormatter | [CliHelp](docs/variant/text_cli_help.md) | No | format_text | ASCII | `TextVariant::CliHelp` | CLI help text with sections and alignment |
| JsonFormatter | [Pretty](docs/variant/json_pretty.md) | Yes | format_json | UTF-8 | mode parameter | Human-readable JSON with indentation |
| JsonFormatter | [Compact](docs/variant/json_compact.md) | No | format_json | UTF-8 | mode parameter | Minified JSON, single line |
| YamlFormatter | [Standard](docs/variant/yaml_standard.md) | Yes | format_yaml | UTF-8 | default | Standard YAML format |
| TomlFormatter | [Standard](docs/variant/toml_standard.md) | Yes | format_toml | UTF-8 | default | Standard TOML format |
| LogfmtFormatter | [Standard](docs/variant/logfmt_standard.md) | Yes | format_logfmt | ASCII | default | Logfmt structured logging |
| HtmlFormatter | [Minimal](docs/variant/html_minimal.md) | Yes | html_minimal | UTF-8 | `HtmlVariant::Minimal` | Plain HTML table, no CSS classes |
| HtmlFormatter | [Bootstrap](docs/variant/html_bootstrap.md) | No | html_bootstrap | UTF-8 | `HtmlVariant::Bootstrap` | Bootstrap 5 styling |
| HtmlFormatter | [Tailwind](docs/variant/html_tailwind.md) | No | html_tailwind | UTF-8 | `HtmlVariant::Tailwind` | Tailwind CSS classes |
| HtmlFormatter | [Custom](docs/variant/html_custom.md) | No | html_custom | UTF-8 | `HtmlVariant::Custom(String)` | User-provided CSS classes |
| SqlFormatter | [ANSI](docs/variant/sql_ansi.md) | Yes | sql_ansi | ASCII | `SqlVariant::Ansi` | Standard SQL compliant |
| SqlFormatter | [PostgreSQL](docs/variant/sql_postgresql.md) | No | sql_postgres | ASCII | `SqlVariant::PostgreSQL` | PostgreSQL-specific syntax |
| SqlFormatter | [MySQL](docs/variant/sql_mysql.md) | No | sql_mysql | ASCII | `SqlVariant::MySQL` | MySQL/MariaDB syntax |
| SqlFormatter | [SQLite](docs/variant/sql_sqlite.md) | No | sql_sqlite | ASCII | `SqlVariant::SQLite` | SQLite3 syntax |

Run any example:
```bash
cargo run --example table_format --all-features
cargo run --example unified_formats --all-features
```

## Documentation

- **Complete Specification**: See `spec.md` for API contract, design rationale, and technical details
- **Examples Directory**: See `examples/readme.md` for all available examples
- **API Documentation**: Run `cargo doc --open --all-features`
- **Module Documentation**: Each formatter has detailed doc comments with examples

## Cargo Features

### Granular Per-Variant Features

tree_fmt provides granular feature flags for minimal binary size:

```toml
# Default: 4 default variants (plain table, postgres expanded, hierarchical tree, logfmt)
tree_fmt = "0.5.0"

# Minimal CLI tool: just plain table
tree_fmt = { version = "0.5.0", default-features = false, features = [ "table_plain" ] }

# Data export pipeline: CSV, TSV, JSON
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_csv",
  "table_tsv",
  "format_json"
] }

# Documentation generator: Markdown tables + HTML
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_markdown",
  "html_minimal"
] }

# Database tool (PostgreSQL): bordered table + SQL
tree_fmt = { version = "0.5.0", default-features = false, features = [
  "table_bordered",
  "sql_postgres"
] }

# All formatters and variants
tree_fmt = { version = "0.5.0", features = [ "all_formats" ] }
```

### Meta-Features (Convenience)

```toml
# All table variants (9 total)
features = [ "format_table" ]

# Visual table variants only (excludes CSV/TSV)
features = [ "format_table_visual" ]

# Export variants only (CSV + TSV)
features = [ "format_table_export" ]

# All HTML variants (4 total)
features = [ "format_html" ]

# All SQL variants (4 total)
features = [ "format_sql" ]
```

See [docs/feature_selection_guide.md](docs/feature_selection_guide.md) for detailed examples and binary size comparison.

Each formatter provides customizable formatter parameters via its Config type (e.g., TableConfig, HtmlVariant).

## Testing

```bash
cargo nextest run --all-features
cargo test --doc --all-features
cargo clippy --all-targets --all-features
```

## Version

Current: v0.4.0 - Unified format interface with granular feature flags

See `spec.md` for complete version history and changelog.

## License

MIT

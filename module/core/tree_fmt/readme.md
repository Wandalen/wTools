# tree_fmt

[![Crates.io](https://img.shields.io/crates/v/tree_fmt.svg)](https://crates.io/crates/tree_fmt)
[![Documentation](https://docs.rs/tree_fmt/badge.svg)](https://docs.rs/tree_fmt)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./license)

Multi-format data visualization library: 10 formatters, 31 variants, zero core dependencies.

## Why tree_fmt?

**Build once, format anywhere:** Create your data structure once, then output as ASCII table, JSON, HTML, SQL, YAML, tree view, or 6 other formats - without rebuilding data. Features granular compilation (31 flags) and zero core dependencies.

## Installation

```bash
cargo add tree_fmt  # Default: table, tree, expanded, logfmt
cargo add tree_fmt --no-default-features --features table_plain  # Minimal
```

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

## Formatters & Features

| Format | Use Case | Variants | Feature Flags | Example |
|--------|----------|----------|---------------|---------|
| **Table** | CLI tools, terminals | 9 styles | `table_plain`, `table_markdown`, `table_csv`, `table_bordered`, `table_grid`, `table_unicode`, `table_minimal`, `table_tsv`, `table_compact` | [→](examples/) |
| **Tree** | File trees, hierarchies | 3 styles | `tree_hierarchical`, `tree_aligned`, `tree_aggregated` | [→](examples/) |
| **Expanded** | PostgreSQL \x mode | 2 styles | `expanded_postgres`, `expanded_property` | [→](examples/) |
| **JSON** | APIs, config files | Pretty/Compact | `format_json` | [→](examples/) |
| **HTML** | Web dashboards | 4 themes | `html_minimal`, `html_bootstrap`, `html_tailwind`, `html_custom` | [→](examples/) |
| **SQL** | Database seeds | 4 dialects | `sql_ansi`, `sql_postgres`, `sql_mysql`, `sql_sqlite` | [→](examples/) |
| **YAML** | Config export | Standard | `format_yaml` | [→](examples/) |
| **TOML** | Cargo.toml gen | Standard | `format_toml` | [→](examples/) |
| **Logfmt** | Structured logs | Standard | `format_logfmt` | [→](examples/) |
| **Text** | Docs, CLI help | 6 styles | `format_text` | [→](examples/) |

**Total: 31 variants across 10 formatters** · **[Detailed variant docs →](./docs/)**

Run examples:
```bash
cargo run --example table_format --all-features
cargo run --example unified_formats --all-features
```

## Feature Flags

```toml
# Default (4 formatters)
tree_fmt = "0.4.0"

# Minimal (single formatter)
tree_fmt = { version = "0.4.0", default-features = false, features = [ "table_plain" ] }

# Specific use case
tree_fmt = { version = "0.4.0", features = [ "table_markdown", "format_json" ] }

# Everything
tree_fmt = { version = "0.4.0", features = [ "all_formats" ] }
```

**31 granular flags available** - each variant has its own flag for minimal binary size.

**Meta-features:** `format_table` (all 9 table variants), `format_html` (all 4 HTML variants), `format_sql` (all 4 SQL dialects), `visual_formats`, `data_formats`, `all_formats`

**[Complete feature guide →](./docs/feature_selection_guide.md)**

## Documentation & Testing

- **[Complete Specification](./spec.md)** - API contract, design rationale, changelog
- **[Examples](./examples/readme.md)** - Working code for all formatters
- **[API Docs](https://docs.rs/tree_fmt)** - Generated documentation

```bash
cargo test --all-features           # Run tests
cargo doc --open --all-features    # View docs locally
```

## License

MIT - **v0.4.0** - [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/tree_fmt)

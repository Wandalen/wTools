# tree_fmt

[![Crates.io](https://img.shields.io/crates/v/tree_fmt.svg)](https://crates.io/crates/tree_fmt)
[![Documentation](https://docs.rs/tree_fmt/badge.svg)](https://docs.rs/tree_fmt)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./license)

> **Deprecated:** This crate has been renamed to [`data_fmt`](https://crates.io/crates/data_fmt).
> Use `data_fmt` for all new projects. This is the final release of `tree_fmt`.

Multi-format data visualization library: 10 formatters, 33 variants, zero core dependencies.

## Why data_fmt?

**Build once, format anywhere:** Create your data structure once, then output as ASCII table, JSON, HTML, SQL, YAML, tree view, or 6 other formats — without rebuilding data. Features granular per-variant compilation and zero core dependencies.

> The name `tree_fmt` was misleading — trees are just 1 of 10 output formats. The crate is a general-purpose **data formatter**, hence the rename to `data_fmt`.

## Installation

```bash
# Recommended — use the new name
cargo add data_fmt

# Legacy — still works, final release
cargo add tree_fmt
```

## Quick Start

```rust
// use data_fmt::{ RowBuilder, TableFormatter, Format };  // new name
use tree_fmt::{ RowBuilder, TableFormatter, Format };     // still works

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

**Total: 33 variants across 10 formatters** · **[Detailed variant docs →](./docs/)**

Run examples:
```bash
cargo run --example table_format --all-features
cargo run --example unified_formats --all-features
```

## Feature Flags

```toml
# Recommended — use data_fmt (same API, same features)
data_fmt = "0.1.0"

# Legacy — final tree_fmt release
tree_fmt = "0.12.0"

# Minimal (single formatter)
data_fmt = { version = "0.1.0", default-features = false, features = [ "table_plain" ] }

# Specific use case
data_fmt = { version = "0.1.0", features = [ "table_markdown", "format_json" ] }

# Everything
data_fmt = { version = "0.1.0", features = [ "all_formats" ] }
```

**Granular flags available** — most variants have their own feature flag for minimal binary size.

**Meta-features:** `format_table` (all 9 table variants), `format_html` (all 4 HTML variants), `format_sql` (all 4 SQL dialects), `visual_formats`, `data_formats`, `all_formats`

**[Complete feature guide →](./docs/feature_selection_guide.md)**

## Documentation & Testing

- **[Documentation](./docs/readme.md)** - Architecture, API reference, features, invariants
- **[Examples](./examples/readme.md)** - Working code for all formatters
- **[API Docs — data_fmt](https://docs.rs/data_fmt)** - Generated documentation (new name)
- **[API Docs — tree_fmt](https://docs.rs/tree_fmt)** - Generated documentation (this release)

```bash
cargo test --all-features           # Run tests
cargo doc --open --all-features    # View docs locally
```

## Migration

Replace in `Cargo.toml`:

```diff
- tree_fmt = "0.12.0"
+ data_fmt = "0.1.0"
```

Replace in source:

```diff
- use tree_fmt::{ RowBuilder, TableFormatter, Format };
+ use data_fmt::{ RowBuilder, TableFormatter, Format };
```

All types, traits, features, and feature flags are identical.

## License

MIT - **v0.12.0** (final `tree_fmt` release) - [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/tree_fmt)

# src/formatters

## Purpose
Contains one source file per output format family, each implementing the `Format` trait for its domain.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Re-exports all formatter modules and their public types |
| `format_trait.rs` | Defines the `Format` trait and `FormatError` type |
| `table/` | Table format family: plain, bordered, compact, markdown, CSV, TSV, grid, unicode-box |
| `tree/` | Tree format family: hierarchical, aligned, aggregated |
| `expanded.rs` | Expanded format family: property-style and postgres-style vertical layouts |
| `text.rs` | Text format family: bullets, numbered, sections, key-value, CLI help, compact |
| `json.rs` | JSON format family: pretty and compact output |
| `yaml.rs` | YAML format family: standard YAML output |
| `toml_fmt.rs` | TOML format family: standard TOML output |
| `logfmt.rs` | Logfmt format family: key=value log format output |
| `html.rs` | HTML format family: minimal, bootstrap, tailwind, and custom themes |
| `sql.rs` | SQL format family: ANSI, MySQL, PostgreSQL, and SQLite INSERT statements |

# data_fmt

Multi-format data visualization library — 10 formatters, 33 variants, granular feature flags.

Build your data structure once, output as table, tree, expanded, JSON, HTML, SQL, YAML, TOML, logfmt, or text.

```toml
data_fmt = { version = "0.2", features = ["all_formats"] }
```

## Why data_fmt?

The name `tree_fmt` was misleading — trees are just 1 of 10 output formats.
`data_fmt` is a general-purpose multi-format **data** formatter.

## Features

- **`RowBuilder`**: Construct tabular data (headers + rows)
- **`TreeBuilder`**: Construct trees from flat data with path-based insertion
- **10 Formatters**: Table (9 styles), Tree (3), Expanded (2), JSON, HTML (4), SQL (4), YAML, TOML, Logfmt, Text (6)
- **String Output**: All formatters return `String`, no direct console output
- **Terminal-aware**: Auto-wrap and auto-fold for wide tables

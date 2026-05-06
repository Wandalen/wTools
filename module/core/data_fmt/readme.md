# data_fmt
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=data_fmt)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/data_fmt?color=e3e8f0&logo=docs.rs)](https://docs.rs/data_fmt) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fdata_fmt%2Fexamples%2Ftree_format.rs,RUN_POSTFIX=--example%20tree_format/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

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

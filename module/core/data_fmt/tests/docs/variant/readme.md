# Variant Test Spec Doc Entity

### Scope

- **Purpose**: Provide test coverage specifications for all variant doc instances.
- **Responsibility**: Registry and overview of all variant test spec instances.
- **In Scope**: VT-N variant output contract cases in Given/When/Then format for all 33 variant elements across 10 formatters; minimum 4 cases per spec.
- **Out of Scope**: Formatter-level tests (see `../formatter/`), feature behavioral cases (see `../feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Table Plain](001_table_plain.md) | Space-separated output, dash header separator, no borders | ⏳ |
| 002 | [Table Minimal](002_table_minimal.md) | Space-separated output, no header separator, no borders | ⏳ |
| 003 | [Table Bordered](003_table_bordered.md) | ASCII pipe borders, grid header separator, outer borders | ⏳ |
| 004 | [Table Markdown](004_table_markdown.md) | Pipe-delimited rows, Markdown separator, GFM compliance | ⏳ |
| 005 | [Table Grid](005_table_grid.md) | Full ASCII grid, pipe columns, horizontal rules between rows | ⏳ |
| 006 | [Table Unicode Box](006_table_unicode_box.md) | Unicode box-drawing borders, U+2502 separator, Unicode charset | ⏳ |
| 007 | [Table CSV](007_table_csv.md) | Comma-separated values, no borders, RFC 4180, quoting | ⏳ |
| 008 | [Table TSV](008_table_tsv.md) | Tab-separated values, no borders, no header separator | ⏳ |
| 009 | [Table Compact](009_table_compact.md) | Single-space separation, no header separator, minimal output | ⏳ |
| 010 | [Expanded Postgres](010_expanded_postgres_style.md) | Vertical record layout, pipe separator, dash record dividers | ⏳ |
| 011 | [Expanded Property](011_expanded_property_style.md) | Colon key-value pairs, no record headers, compact vertical | ⏳ |
| 012 | [Tree Hierarchical](012_tree_hierarchical.md) | Unicode box-drawing connectors, hierarchical indentation | ⏳ |
| 013 | [Tree Aligned](013_tree_aligned.md) | Multi-column alignment, ColumnData, space separation | ⏳ |
| 014 | [Tree Aggregated](014_tree_aggregated.md) | Aggregated directory totals, recursive computation | ⏳ |
| 015 | [Logfmt Standard](015_logfmt_standard.md) | key=value format, space-separated pairs, quoting | ⏳ |
| 016 | [JSON Pretty](016_json_pretty.md) | Valid JSON, indented output, RFC 8259, backslash escaping | ⏳ |
| 017 | [JSON Compact](017_json_compact.md) | Valid JSON, single-line, minimal whitespace | ⏳ |
| 018 | [YAML Standard](018_yaml_standard.md) | Valid YAML, indentation-based structure, header-name keys | ⏳ |
| 019 | [TOML Standard](019_toml_standard.md) | Valid TOML, array-of-tables notation, bracket headers | ⏳ |
| 020 | [HTML Minimal](020_html_minimal.md) | Semantic HTML table, no CSS classes, thead/tbody/th/td | ⏳ |
| 021 | [HTML Bootstrap](021_html_bootstrap.md) | Bootstrap CSS classes, semantic HTML table structure | ⏳ |
| 022 | [HTML Tailwind](022_html_tailwind.md) | Tailwind CSS utility classes, semantic HTML structure | ⏳ |
| 023 | [HTML Custom](023_html_custom.md) | User-provided CSS class string, HtmlVariant::Custom | ⏳ |
| 024 | [SQL ANSI](024_sql_ansi.md) | ANSI SQL INSERT, single-quote escaping, column list | ⏳ |
| 025 | [SQL PostgreSQL](025_sql_postgresql.md) | PostgreSQL quoting, double-quoted identifiers, INSERT syntax | ⏳ |
| 026 | [SQL MySQL](026_sql_mysql.md) | MySQL backtick quoting, INSERT syntax, identifier escaping | ⏳ |
| 027 | [SQL SQLite](027_sql_sqlite.md) | SQLite quoting, INSERT syntax, single-quote escaping | ⏳ |
| 028 | [Text Bullets](028_text_bullets.md) | Bullet prefix, one item per row, no borders | ⏳ |
| 029 | [Text Numbered](029_text_numbered.md) | Sequential number prefix, 1-based numbering | ⏳ |
| 030 | [Text Sections](030_text_sections.md) | Section headers with underlines, grouped fields | ⏳ |
| 031 | [Text KeyValue](031_text_keyvalue.md) | Colon-separated key-value pairs, record blocks | ⏳ |
| 032 | [Text Compact](032_text_compact.md) | Comma-separated fields, minimal overhead | ⏳ |
| 033 | [Text CLI Help](033_text_cli_help.md) | Automatic alignment, section grouping, CLI-optimized | ⏳ |

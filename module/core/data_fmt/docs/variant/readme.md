# Variant Doc Entity

### Scope

- **Purpose**: Document every output variant with attributes, examples, and configuration.
- **Responsibility**: Registry and overview of all variant doc instances.
- **In Scope**: Per-variant attribute descriptors (46 attributes each), example output, feature flags.
- **Out of Scope**: Formatter implementation (see `feature/`), attribute definitions (see `data_structure/001_variant_attributes.md`).

Required instance sections (in order):

| Section | Heading | Required Content |
|---------|---------|-----------------|
| Scope | `### Scope` | 4 bullets: Purpose / Responsibility / In Scope / Out of Scope |
| Typed References | `### Formatters`, `### Sources`, `### Tests` | Per-type `| File | Relationship |` table; `### Sources` and `### Tests` always last |
| Identity & Classification | `### Identity & Classification` | formatter, variant, is_default, category |
| Build & Dependencies | `### Build & Dependencies` | feature_flag, runtime_deps, zero_dependency |
| Character Set & Encoding | `### Character Set & Encoding` | charset, border_charset, requires_unicode_terminal, supports_ansi_colors |
| Visual Structure | `### Visual Structure` | has_borders, border_style, column_separator, row_separator, header_separator, outer_padding, inner_padding |
| Data Representation | `### Data Representation` | machine_parseable, human_readable, supports_hierarchical, supports_tabular, preserves_structure, supports_multiline_values |
| Output Characteristics | `### Output Characteristics` | output_compactness, visual_complexity, alignment, column_alignment |
| Usage Context | `### Usage Context` | primary_use_case, terminal_optimized, file_export_suitable, streaming_friendly, grep_friendly |
| Technical Details | `### Technical Details` | escaping_rules, output_format, standards_compliance, supports_custom_colors |
| API & Construction | `### API & Construction` | constructor, config_type, customizable_parameters, builder_pattern |
| Performance & Size | `### Performance & Size` | output_overhead, memory_efficiency |
| Compatibility | `### Compatibility` | works_on_windows, works_in_ci, copy_paste_friendly |
| Example Output | `### Example Output` | Fenced code block showing representative rendered output |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Table Plain](001_table_plain.md) | Space-separated with dash separator (default) | ✅ |
| 002 | [Table Minimal](002_table_minimal.md) | No separators | ✅ |
| 003 | [Table Bordered](003_table_bordered.md) | PostgreSQL-style pipes | ✅ |
| 004 | [Table Markdown](004_table_markdown.md) | GitHub-flavored Markdown | ✅ |
| 005 | [Table Grid](005_table_grid.md) | Full ASCII grid | ✅ |
| 006 | [Table Unicode Box](006_table_unicode_box.md) | Unicode box-drawing | ✅ |
| 007 | [Table CSV](007_table_csv.md) | Comma-separated values | ✅ |
| 008 | [Table TSV](008_table_tsv.md) | Tab-separated values | ✅ |
| 009 | [Table Compact](009_table_compact.md) | Minimal spacing | ✅ |
| 010 | [Expanded Postgres](010_expanded_postgres_style.md) | PostgreSQL `\x` mode (default) | ✅ |
| 011 | [Expanded Property](011_expanded_property_style.md) | Property list style | ✅ |
| 012 | [Tree Hierarchical](012_tree_hierarchical.md) | Standard tree (default) | ✅ |
| 013 | [Tree Aligned](013_tree_aligned.md) | Column-aligned tree | ✅ |
| 014 | [Tree Aggregated](014_tree_aggregated.md) | Tree with subtree totals | ✅ |
| 015 | [Logfmt Standard](015_logfmt_standard.md) | Structured log key=value (default) | ✅ |
| 016 | [JSON Pretty](016_json_pretty.md) | Indented JSON (default) | ✅ |
| 017 | [JSON Compact](017_json_compact.md) | Minified JSON | ✅ |
| 018 | [YAML Standard](018_yaml_standard.md) | Standard YAML (default) | ✅ |
| 019 | [TOML Standard](019_toml_standard.md) | Standard TOML (default) | ✅ |
| 020 | [HTML Minimal](020_html_minimal.md) | Plain semantic HTML (default) | ✅ |
| 021 | [HTML Bootstrap](021_html_bootstrap.md) | Bootstrap 5 classes | ✅ |
| 022 | [HTML Tailwind](022_html_tailwind.md) | Tailwind CSS classes | ✅ |
| 023 | [HTML Custom](023_html_custom.md) | User-provided classes | ✅ |
| 024 | [SQL ANSI](024_sql_ansi.md) | Standard SQL (default) | ✅ |
| 025 | [SQL PostgreSQL](025_sql_postgresql.md) | PostgreSQL syntax | ✅ |
| 026 | [SQL MySQL](026_sql_mysql.md) | MySQL/MariaDB syntax | ✅ |
| 027 | [SQL SQLite](027_sql_sqlite.md) | SQLite3 syntax | ✅ |
| 028 | [Text Bullets](028_text_bullets.md) | Bulleted lists (default) | ✅ |
| 029 | [Text Numbered](029_text_numbered.md) | Numbered lists | ✅ |
| 030 | [Text Sections](030_text_sections.md) | Section headers | ✅ |
| 031 | [Text KeyValue](031_text_keyvalue.md) | Key-value pairs | ✅ |
| 032 | [Text Compact](032_text_compact.md) | Dense text | ✅ |
| 033 | [Text CliHelp](033_text_cli_help.md) | CLI help format with aligned descriptions | ✅ |

### Organization

- **Table**: 001–009
- **Expanded**: 010–011
- **Tree**: 012–014
- **Logfmt**: 015
- **JSON**: 016–017
- **YAML**: 018
- **TOML**: 019
- **HTML**: 020–023
- **SQL**: 024–027
- **Text**: 028–033

### Cross-Doc Entity Dependencies

- [Variant Attributes](../data_structure/001_variant_attributes.md) — attribute schema for all 46 per-variant attributes

# Variant Documentation

This directory contains comprehensive descriptor files for all 32 formatter variants in tree_fmt.

## Overview

Each variant has a dedicated markdown file with all 46 attributes documented:

### TableFormatter (8 variants)
- [plain](table_plain.md) - Space-separated with dash separator (default)
- [minimal](table_minimal.md) - No separators
- [bordered](table_bordered.md) - PostgreSQL-style pipes
- [markdown](table_markdown.md) - GitHub-flavored Markdown
- [grid](table_grid.md) - Full ASCII grid
- [unicode_box](table_unicode_box.md) - Unicode box-drawing
- [csv](table_csv.md) - Comma-separated values
- [tsv](table_tsv.md) - Tab-separated values
- [compact](table_compact.md) - Minimal spacing

### ExpandedFormatter (2 variants)
- [postgres_style](expanded_postgres_style.md) - PostgreSQL \x mode (default)
- [property_style](expanded_property_style.md) - Property lists

### TreeFormatter (3 variants)
- [hierarchical](tree_hierarchical.md) - Standard tree (default)
- [aligned](tree_aligned.md) - Column-aligned
- [aggregated](tree_aggregated.md) - With subtree totals

### TextFormatter (6 variants)
- [Bullets](text_bullets.md) - Bulleted lists (default)
- [Numbered](text_numbered.md) - Numbered lists
- [Sections](text_sections.md) - Section headers
- [KeyValue](text_keyvalue.md) - Key-value pairs
- [Compact](text_compact.md) - Dense text
- [CliHelp](text_cli_help.md) - CLI help text with section headers

### JsonFormatter (2 variants)
- [Pretty](json_pretty.md) - Indented JSON (default)
- [Compact](json_compact.md) - Minified JSON

### YamlFormatter (1 variant)
- [Standard](yaml_standard.md) - Standard YAML (default)

### TomlFormatter (1 variant)
- [Standard](toml_standard.md) - Standard TOML (default)

### LogfmtFormatter (1 variant)
- [Standard](logfmt_standard.md) - Logfmt logging (default)

### HtmlFormatter (4 variants)
- [Minimal](html_minimal.md) - Plain HTML (default)
- [Bootstrap](html_bootstrap.md) - Bootstrap 5 classes
- [Tailwind](html_tailwind.md) - Tailwind CSS classes
- [Custom](html_custom.md) - User-provided classes

### SqlFormatter (4 variants)
- [ANSI](sql_ansi.md) - Standard SQL (default)
- [PostgreSQL](sql_postgresql.md) - PostgreSQL syntax
- [MySQL](sql_mysql.md) - MySQL/MariaDB syntax
- [SQLite](sql_sqlite.md) - SQLite3 syntax

## Attribute Structure

Each variant descriptor contains 46 attributes organized into 12 sections:

1. **Identity & Classification** (4 attributes)
2. **Build & Dependencies** (3 attributes)
3. **Character Set & Encoding** (4 attributes)
4. **Visual Structure** (7 attributes)
5. **Data Representation** (6 attributes)
6. **Output Characteristics** (4 attributes)
7. **Usage Context** (5 attributes)
8. **Technical Details** (4 attributes)
9. **API & Construction** (4 attributes)
10. **Performance & Size** (2 attributes)
11. **Compatibility** (3 attributes)
12. **Example Output**
13. **Related Links**

See [variant_attributes.md](../variant_attributes.md) for complete attribute definitions.

## Quick Reference

| Total Formatters | Total Variants | Default Variants |
|-----------------|----------------|------------------|
| 10              | 32             | 10               |

## Related Documentation

- [Table of Variants](../../readme.md#table-of-variants) - Main table with hyperlinks
- [Variant Attributes List](../variant_attributes.md) - Complete attribute definitions
- [Main README](../../readme.md) - Project overview
- [Specification](../../spec.md) - Complete technical specification

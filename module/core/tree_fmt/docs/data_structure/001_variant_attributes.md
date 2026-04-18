# Data Structure: Variant Attributes

### Scope

- **Purpose**: Define the canonical 46-attribute schema used to describe every output format variant in this library.
- **Responsibility**: Single source of truth for attribute names, types, and example values across all variant doc instances.
- **In Scope**: All 46 per-variant attributes organized by group; attribute name, purpose, and example values for each.
- **Out of Scope**: Per-variant attribute values (see `variant/NNN_*.md` files), formatter implementation (see `feature/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | `../variant/readme.md` | Variant doc entity master file — consumes this schema |
| source | `../variant_attributes.md` | Original combined source document (retained per migration rules) |

### Schema

The schema defines 46 attributes across 10 groups. Every variant doc instance fills out all 46 attributes.

#### Identity & Classification

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 1 | `formatter` | Parent formatter name | `TableFormatter`, `HtmlFormatter`, `JsonFormatter` |
| 2 | `variant` | Variant name within the formatter | `plain`, `bordered`, `Bootstrap` |
| 3 | `is_default` | Whether this is the default variant for its formatter | `Yes`, `No` |
| 4 | `category` | Format category | `Visual`, `Data`, `Markup`, `Query`, `Logging` |

#### Build & Dependencies

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 5 | `feature_flag` | Cargo feature required to enable this variant | `default`, `format_json`, `format_html`, `format_sql` |
| 6 | `runtime_deps` | Runtime crate dependencies | `None`, `serde`, `serde+serde_json`, `serde+serde_yaml` |
| 7 | `zero_dependency` | Whether variant needs zero external crates | `Yes`, `No` |

#### Character Set & Encoding

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 8 | `charset` | Character encoding used in output | `ASCII`, `Unicode`, `UTF-8` |
| 9 | `border_charset` | Character set used for border elements specifically | `ASCII`, `Unicode`, `None` |
| 10 | `requires_unicode_terminal` | Whether the terminal must support Unicode for correct rendering | `Yes`, `No` |
| 11 | `supports_ansi_colors` | Whether ANSI color codes are supported by this variant | `Yes`, `No` |

#### Visual Structure

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 12 | `has_borders` | Whether the variant renders border characters | `Yes`, `No`, `Partial` |
| 13 | `border_style` | Border rendering style | `None`, `ASCII-Pipes`, `ASCII-Grid`, `Unicode-Box`, `Markdown` |
| 14 | `column_separator` | What character(s) separate columns | `Spaces`, `Pipes \|`, `Commas ,`, `Tabs \t`, `None` |
| 15 | `row_separator` | What character(s) separate rows | `Newline`, `Dashes`, `Grid-Lines`, `None` |
| 16 | `header_separator` | Header separator style | `None`, `Dashes`, `ASCII-Grid`, `Unicode`, `Markdown` |
| 17 | `outer_padding` | Whether padding is added at table edges | `Yes`, `No` |
| 18 | `inner_padding` | Padding within cells (count of space characters) | `0`, `1`, `2` |

#### Data Representation

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 19 | `machine_parseable` | Whether the output is designed for machine parsing | `Yes`, `No`, `Partial` |
| 20 | `human_readable` | Whether the output is designed for human reading | `Yes`, `No` |
| 21 | `supports_hierarchical` | Whether tree/nested data can be represented | `Yes`, `No` |
| 22 | `supports_tabular` | Whether rows/columns can be represented | `Yes`, `No` |
| 23 | `preserves_structure` | Whether data structure is preserved in output | `Yes`, `No`, `Partial` |
| 24 | `supports_multiline_values` | Whether cell values can contain newlines | `Yes`, `No`, `Escaped` |

#### Output Characteristics

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 25 | `output_compactness` | Space efficiency relative to raw data | `Minimal`, `Compact`, `Standard`, `Rich`, `Verbose` |
| 26 | `visual_complexity` | Visual richness of the rendered output | `Minimal`, `Simple`, `Standard`, `Rich` |
| 27 | `alignment` | Data alignment support | `Left`, `Right`, `Both`, `None` |
| 28 | `column_alignment` | Whether columns are visually aligned | `Yes`, `No` |

#### Usage Context

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 29 | `primary_use_case` | Main intended purpose | `CLI tools output`, `Database export`, `Documentation`, `Web display` |
| 30 | `terminal_optimized` | Designed for terminal display | `Yes`, `No`, `Partial` |
| 31 | `file_export_suitable` | Good for writing to files | `Yes`, `No`, `Primary` |
| 32 | `streaming_friendly` | Can be parsed or generated line-by-line | `Yes`, `No` |
| 33 | `grep_friendly` | Easy to search with grep/awk | `Yes`, `No` |

#### Technical Details

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 34 | `escaping_rules` | How special characters are escaped | `None`, `Quotes`, `HTML-Entities`, `SQL-Quotes`, `Backslash` |
| 35 | `output_format` | MIME/format type of the output | `text/plain`, `application/json`, `text/html`, `text/csv` |
| 36 | `standards_compliance` | Whether the output follows a published standard | `None`, `Markdown-GFM`, `SQL-ANSI`, `JSON-RFC8259`, `CSV-RFC4180` |
| 37 | `supports_custom_colors` | Whether formatter parameters control output colors | `Yes`, `No` |

#### API & Construction

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 38 | `constructor` | How to construct the formatter with this variant | `TableConfig::plain()`, `HtmlVariant::Bootstrap`, `default()` |
| 39 | `config_type` | Config struct name for this variant | `TableConfig`, `HtmlVariant`, `ExpandedConfig`, `None` |
| 40 | `customizable_parameters` | Count of configurable formatter parameters | `0`, `5`, `15`, `20+` |
| 41 | `builder_pattern` | Whether a fluent builder API is available | `Yes`, `No` |

#### Performance & Size

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 42 | `output_overhead` | Extra characters added relative to raw data | `Minimal`, `Low`, `Medium`, `High`, `Very-High` |
| 43 | `memory_efficiency` | Memory usage pattern during rendering | `Streaming`, `Buffered`, `Hybrid` |

#### Compatibility

| # | Attribute | Purpose | Example Values |
|---|-----------|---------|----------------|
| 44 | `works_on_windows` | Windows console compatible without modification | `Yes`, `No`, `Partial` |
| 45 | `works_in_ci` | Renders correctly in CI/CD environments | `Yes`, `No` |
| 46 | `copy_paste_friendly` | Easy to copy from terminal and paste elsewhere | `Yes`, `No`, `Partial` |

### Sources

| File | Notes |
|------|-------|
| [../variant_attributes.md](../variant_attributes.md) | Original combined source document; all 46 attribute definitions extracted into this doc instance |

# Examples

## Purpose
Demonstrates all three output formats (Table, Expanded, Tree) and library features through practical, real-world usage scenarios. Each example is self-contained and runnable.

## Organization Principles
- **Format-specific examples**: Individual formatters demonstrating each display mode
- **Comprehensive overview**: `unified_formats.rs` demonstrates the unified Format trait with multiple formatters
- All examples follow "concise but informative" design with clear doc comments

## Available Examples

### unified_formats.rs - Unified Format Interface
**Purpose**: Demonstrates the unified Format trait interface with multiple output formatters

**What it shows**:
- Using the Format trait for format-agnostic code
- Same data formatted in TABLE, JSON, YAML, and TEXT formats
- Feature-gated formatter usage patterns
- Runtime format selection with trait objects

**Run**: `cargo run --example unified_formats --all-features`

### table_format.rs - Horizontal Tabular Display
**Purpose**: Demonstrates `TableFormatter` for row-and-column table output

**What it shows**:
- Creating table data with headers and rows
- Automatic column width calculation
- Border rendering and alignment
- ANSI color support in tables

**Run**: `cargo run --example table_format`

### expanded_format.rs - Vertical Record Display (PostgreSQL Style)
**Purpose**: Demonstrates `ExpandedFormatter` for PostgreSQL `\x`-style output

**What it shows**:
- Vertical key-value pair display
- Record separators with numbering (`-[ RECORD N ]`)
- Aligned key columns with pipe separator
- Best use cases for wide tables

**Run**: `cargo run --example expanded_format`

### tree_format.rs - Hierarchical Tree Display
**Purpose**: Demonstrates `TreeFormatter` for hierarchical data visualization

**What it shows**:
- Building hierarchical trees with `TreeBuilder`
- Box-drawing characters (├──, └──, │)
- Custom data rendering with closures
- Directory/file structure visualization

**Run**: `cargo run --example tree_format`

### logfmt_format.rs - Structured Logging Output
**Purpose**: Demonstrates `LogfmtFormatter` for machine-parseable structured logs

**What it shows**:
- Logfmt format (key=value pairs) for application logging
- Automatic value escaping (spaces, quotes, newlines)
- Grep-friendly log format for observability
- Real-world log scenarios (HTTP requests, errors, metrics)

**Run**: `cargo run --example logfmt_format`

### text_format.rs - Text Formatting Variants
**Purpose**: Demonstrates `TextFormatter` with all 6 text formatting variants

**What it shows**:
- Bullets variant for quick lists and bullet points
- Numbered variant for ordered lists and step-by-step instructions
- KeyValue variant for configuration and metadata display
- Sections variant for grouped data with headers
- Compact variant for dense, space-constrained output
- CliHelp variant (NEW) for CLI help text with section headers and aligned descriptions
- Custom indentation and separator configuration

**Run**: `cargo run --example text_format --features format_text`

### command_report.rs - Command Execution Reports
**Purpose**: Demonstrates property list style for command execution output

**What it shows**:
- Property list formatting with `ExpandedConfig::property_style()`
- Colon separator with aligned values
- Clean output without record headers
- Perfect for command/process status reports

**Run**: `cargo run --example command_report`

**Sample output**:
```
Command:           sleep 10 && echo hello1
Working Directory: /
Status:            Completed
Started At:        2025-10-24 22:16:26
Completed At:      2025-10-24 22:16:36
Duration:          10 seconds
Exit Code:         0
```

**Key features**:
- Clean, readable format ideal for CLI tools, logs, and status reports
- Keys appear in gray by default (automatic with `property_style()`)
- Values perfectly aligned for easy scanning

### html_format.rs - HTML Table Output
**Purpose**: Demonstrates `HtmlFormatter` with multiple CSS framework variants

**What it shows**:
- Minimal HTML table output (no CSS classes)
- Bootstrap 5 styled tables
- Tailwind CSS styled tables
- Custom CSS class configuration

**Run**: `cargo run --example html_format --features format_html`

### sql_format.rs - SQL INSERT Statement Output
**Purpose**: Demonstrates `SqlFormatter` for generating SQL INSERT statements

**What it shows**:
- ANSI-standard SQL output
- PostgreSQL, MySQL, and SQLite dialect variants
- Proper quoting and escaping for each dialect
- Batch INSERT generation from tabular data

**Run**: `cargo run --example sql_format --features format_sql`

### sub_row_detail.rs - Sub-row Detail Lines
**Purpose**: Demonstrates per-row detail lines for supplementary context beneath table rows

**What it shows**:
- Attaching an optional detail string to any row via `add_row_with_detail`
- Mix of annotated and plain rows in one table
- Detail indented 2 spaces, no column separators or borders
- Transparent when unused (normal table rendered unchanged)

**Run**: `cargo run --example sub_row_detail`

**Sample output**:
```
Service  Status  Latency
-------  ------  -------
auth     ERROR   n/a
  connection refused: 10.0.1.5:5432
api-gw   OK      12ms
worker   WARN    340ms
  queue depth 4 821 — consider scaling
cache    OK      1ms
```

### themes.rs - Theme System Demonstration
**Purpose**: Demonstrates the theme system for styled table output

**What it shows**:
- Built-in theme selection and application
- Color and style customization via themes
- Theme composition and override patterns
- Visual comparison of available themes

**Run**: `cargo run --example themes`

### manual_test_001_truncation.rs - Manual Truncation Tests
**Purpose**: Manual verification of column truncation behavior

**Run**: `cargo run --example manual_test_001_truncation`

### manual_test_002_multiline.rs - Manual Multiline Tests
**Purpose**: Manual verification of multiline cell rendering

**Run**: `cargo run --example manual_test_002_multiline`

### manual_test_003_combined.rs - Manual Combined Feature Tests
**Purpose**: Manual verification of truncation + multiline combined

**Run**: `cargo run --example manual_test_003_combined`

### manual_test_runner.rs - Manual Test Runner
**Purpose**: Runs all manual test scenarios in sequence for visual inspection

**Run**: `cargo run --example manual_test_runner`

## Quick Start

```bash
# See unified Format trait with multiple formatters
cargo run --example unified_formats --all-features

# Explore individual formatters
cargo run --example table_format
cargo run --example expanded_format     # PostgreSQL \x style
cargo run --example tree_format
cargo run --example logfmt_format       # Structured logging
cargo run --example text_format --features format_text  # 6 text variants

# Advanced examples
cargo run --example command_report      # Property list style
cargo run --example sub_row_detail      # Per-row detail lines
```

## Common Patterns Demonstrated

- **Builder patterns**: Fluent APIs for tree and table construction
- **Generic rendering**: Custom display logic via closures
- **Format conversion**: Same data in multiple display formats
- **ANSI support**: Colored output with proper alignment
- **Column alignment**: Multi-attribute nodes with vertically aligned columns (NEW)

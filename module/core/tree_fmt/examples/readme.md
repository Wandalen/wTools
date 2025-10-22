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
```

## Common Patterns Demonstrated

- **Builder patterns**: Fluent APIs for tree and table construction
- **Generic rendering**: Custom display logic via closures
- **Format conversion**: Same data in multiple display formats
- **ANSI support**: Colored output with proper alignment
- **Column alignment**: Multi-attribute nodes with vertically aligned columns (NEW)

# Variant: Text CLI Help

### Scope

- **Purpose**: Provide CLI help text format with section headers and aligned descriptions.
- **Responsibility**: Complete attribute descriptor for this output variant preset.
- **In Scope**: All 46 variant attributes, example output, feature flag, compatibility.
- **Out of Scope**: Formatter implementation (see source), attribute schema (see `../data_structure/001_variant_attributes.md`).

### Algorithms

| File | Relationship |
|------|-------------|
| [006_cli_help_alignment.md](../algorithm/006_cli_help_alignment.md) | Two-pass section detection and description alignment algorithm |

### DataStructures

| File | Relationship |
|------|-------------|
| [001_variant_attributes.md](../data_structure/001_variant_attributes.md) | Attribute definitions for all 46 variant attributes |

### Formatters

| File | Relationship |
|------|-------------|
| [010_text_formatter.md](../formatter/010_text_formatter.md) | Parent formatter producing this variant |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/text.rs`](../../src/formatters/text.rs) | TextFormatter implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text.rs`](../../tests/text.rs) | Text output tests |

### Identity & Classification

- **formatter**: TextFormatter
- **variant**: CliHelp
- **is_default**: No
- **category**: Visual

### Build & Dependencies

- **feature_flag**: format_text
- **runtime_deps**: None
- **zero_dependency**: Yes

### Character Set & Encoding

- **charset**: ASCII
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: Yes

### Visual Structure

- **has_borders**: No
- **border_style**: None
- **column_separator**: Automatic alignment spacing
- **row_separator**: Newline
- **header_separator**: Blank line between sections
- **outer_padding**: No
- **inner_padding**: Configurable (default 2 spaces)

### Data Representation

- **machine_parseable**: No
- **human_readable**: Yes
- **supports_hierarchical**: Yes (via sections)
- **supports_tabular**: Partial (key-description pairs)
- **preserves_structure**: Full
- **supports_multiline_values**: No

### Output Characteristics

- **output_compactness**: Medium
- **visual_complexity**: Medium
- **alignment**: Left-aligned keys, descriptions aligned to longest key
- **column_alignment**: Yes (automatic description alignment)

### Usage Context

- **primary_use_case**: CLI help text, command documentation, usage guides
- **terminal_optimized**: Yes
- **file_export_suitable**: Yes
- **streaming_friendly**: Yes
- **grep_friendly**: Yes

### Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: Yes

### API & Construction

- **constructor**: `TextVariant::CliHelp` or `TextFormatter::cli_help()`
- **config_type**: TextVariant
- **customizable_parameters**: 2 (indent, separator)
- **builder_pattern**: Yes

### Performance & Size

- **output_overhead**: Low (two-pass algorithm for alignment)
- **memory_efficiency**: Buffered

### Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

### Example Output

```
USAGE:
  myapp [command] [options]

COMMANDS:
  build          Build the project
  test           Run tests
  deploy         Deploy to production

OPTIONS:
  --verbose      Enable verbose output
  --config FILE  Use custom config file
  --help         Show this help message

EXAMPLES:
  myapp build --verbose
  myapp test --config test.toml
```

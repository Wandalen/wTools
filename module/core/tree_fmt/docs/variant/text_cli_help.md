# TextFormatter: CliHelp

CLI help text format with section headers and aligned descriptions. Designed for command-line interface help output.

## Identity & Classification

- **formatter**: TextFormatter
- **variant**: CliHelp
- **is_default**: No
- **category**: Visual

## Build & Dependencies

- **feature_flag**: format_text
- **runtime_deps**: None
- **zero_dependency**: Yes

## Character Set & Encoding

- **charset**: ASCII
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: Yes

## Visual Structure

- **has_borders**: No
- **border_style**: None
- **column_separator**: Automatic alignment spacing
- **row_separator**: Newline
- **header_separator**: Blank line between sections
- **outer_padding**: No
- **inner_padding**: Configurable (default 2 spaces)

## Data Representation

- **machine_parseable**: No
- **human_readable**: Yes
- **supports_hierarchical**: Yes (via sections)
- **supports_tabular**: Partial (key-description pairs)
- **preserves_structure**: Full
- **supports_multiline_values**: No

## Output Characteristics

- **output_compactness**: Medium
- **visual_complexity**: Medium
- **alignment**: Left-aligned keys, descriptions aligned to longest key
- **column_alignment**: Yes (automatic description alignment)

## Usage Context

- **primary_use_case**: CLI help text, command documentation, usage guides
- **terminal_optimized**: Yes
- **file_export_suitable**: Yes
- **streaming_friendly**: Yes
- **grep_friendly**: Yes

## Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: Yes

## API & Construction

- **constructor**: `TextVariant::CliHelp` or `TextFormatter::cli_help()`
- **config_type**: TextVariant
- **customizable_parameters**: 2 (indent, separator)
- **builder_pattern**: Yes

## Performance & Size

- **output_overhead**: Low (two-pass algorithm for alignment)
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Data Structure Convention

For CliHelp variant, input data follows specific conventions:

- **Section header**: First column is all uppercase (or uppercase + whitespace + underscores), second column is empty
- **Key-description pair**: Both columns populated - first is key/term, second is description
- **Simple line**: Only first column populated - rendered as indented line

## Formatting Algorithm

Two-pass algorithm:
1. **First pass**: Scan all rows to identify section headers and calculate maximum key width
2. **Second pass**: Format output with proper indentation, alignment, and blank lines

## Example Output

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

## Example Code

```rust
use tree_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };

let help_view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
  .add_row( vec![ "USAGE".into(), "".into() ] )
  .add_row( vec![ "myapp [options]".into(), "".into() ] )
  .add_row( vec![ "OPTIONS".into(), "".into() ] )
  .add_row( vec![ "--verbose".into(), "Enable verbose output".into() ] )
  .add_row( vec![ "--help".into(), "Show this help message".into() ] )
  .build_view();

let formatter = TextFormatter::cli_help();
let output = formatter.format( &help_view ).unwrap();
```

## Key Features

- **Section headers**: Automatically detected and formatted with colon suffix
- **Description alignment**: All descriptions align to longest key + 2 spaces
- **Blank line separators**: Automatically added between sections for readability
- **Configurable indentation**: Default 2 spaces, customizable via `.with_indent()`
- **Mixed content**: Supports section headers, aligned pairs, and simple lines in same output

## Use Cases

- CLI application help text (`--help` output)
- Command documentation in terminal UIs
- Configuration parameter descriptions
- API endpoint documentation in CLI tools
- Plugin/extension help systems

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [TextFormatter Documentation](../../src/formatters/text.rs)
- [Example: text_format.rs](../../examples/text_format.rs)

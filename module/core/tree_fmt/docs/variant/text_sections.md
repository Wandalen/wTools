# TextFormatter: Sections

Section headers format for organized content blocks.

## Identity & Classification

- **formatter**: TextFormatter
- **variant**: Sections
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
- **column_separator**: None
- **row_separator**: Double-Newline
- **header_separator**: Underline
- **outer_padding**: No
- **inner_padding**: 0

## Data Representation

- **machine_parseable**: No
- **human_readable**: Yes
- **supports_hierarchical**: Partial
- **supports_tabular**: No
- **preserves_structure**: Partial
- **supports_multiline_values**: Yes

## Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Simple
- **alignment**: Left
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Section headers
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

- **constructor**: `TextVariant::Sections`
- **config_type**: TextVariant
- **customizable_parameters**: 1
- **builder_pattern**: No

## Performance & Size

- **output_overhead**: Low
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```
Record 1
========
Name: Alice
Age: 30
City: NYC

Record 2
========
Name: Bob
Age: 25
City: LA
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [TextFormatter Documentation](../../src/formatters/text.rs)

# TextFormatter: Compact

Dense text output format with minimal spacing for space-constrained display.

## Identity & Classification

- **formatter**: TextFormatter
- **variant**: Compact
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
- **column_separator**: Comma
- **row_separator**: Semicolon
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

## Data Representation

- **machine_parseable**: Partial
- **human_readable**: Partial
- **supports_hierarchical**: No
- **supports_tabular**: No
- **preserves_structure**: Partial
- **supports_multiline_values**: No

## Output Characteristics

- **output_compactness**: Maximum
- **visual_complexity**: Minimal
- **alignment**: None
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Dense text output
- **terminal_optimized**: Yes
- **file_export_suitable**: Partial
- **streaming_friendly**: Yes
- **grep_friendly**: Partial

## Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: No

## API & Construction

- **constructor**: `TextVariant::Compact`
- **config_type**: TextVariant
- **customizable_parameters**: 1
- **builder_pattern**: No

## Performance & Size

- **output_overhead**: Minimal
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Partial

## Example Output

```
Alice,30,NYC;Bob,25,LA
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [TextFormatter Documentation](../../src/formatters/text.rs)

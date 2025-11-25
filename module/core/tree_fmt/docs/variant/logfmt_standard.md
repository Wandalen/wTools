# LogfmtFormatter: Standard

Logfmt structured logging format with key=value pairs. Default and only logfmt variant.

## Identity & Classification

- **formatter**: LogfmtFormatter
- **variant**: Standard
- **is_default**: Yes
- **category**: Data

## Build & Dependencies

- **feature_flag**: default
- **runtime_deps**: None
- **zero_dependency**: Yes

## Character Set & Encoding

- **charset**: ASCII
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: No

## Visual Structure

- **has_borders**: No
- **border_style**: None
- **column_separator**: Space
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

## Data Representation

- **machine_parseable**: Yes
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: No

## Output Characteristics

- **output_compactness**: Compact
- **visual_complexity**: Simple
- **alignment**: None
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Logfmt structured logging
- **terminal_optimized**: Yes
- **file_export_suitable**: Primary
- **streaming_friendly**: Yes
- **grep_friendly**: Yes

## Technical Details

- **escaping_rules**: Quotes
- **output_format**: text/logfmt
- **standards_compliance**: Logfmt
- **supports_custom_colors**: No

## API & Construction

- **constructor**: default
- **config_type**: None
- **customizable_parameters**: 0
- **builder_pattern**: No

## Performance & Size

- **output_overhead**: Minimal
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```
Name=Alice Age=30 City=NYC
Name=Bob Age=25 City=LA
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [LogfmtFormatter Documentation](../../src/formatters/logfmt.rs)
- [Logfmt Format](https://brandur.org/logfmt)

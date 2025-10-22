# TableFormatter: csv

Comma-separated values format for data export and Excel import.

## Identity & Classification

- **formatter**: TableFormatter
- **variant**: csv
- **is_default**: No
- **category**: Data

## Build & Dependencies

- **feature_flag**: table_csv
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
- **column_separator**: Commas ,
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

## Data Representation

- **machine_parseable**: Yes
- **human_readable**: Partial
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: Escaped

## Output Characteristics

- **output_compactness**: Minimal
- **visual_complexity**: Minimal
- **alignment**: None
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Data export, Excel import
- **terminal_optimized**: No
- **file_export_suitable**: Primary
- **streaming_friendly**: Yes
- **grep_friendly**: No

## Technical Details

- **escaping_rules**: Quotes
- **output_format**: text/csv
- **standards_compliance**: CSV-RFC4180
- **supports_custom_colors**: No

## API & Construction

- **constructor**: `TableConfig::csv()`
- **config_type**: TableConfig
- **customizable_parameters**: 15+
- **builder_pattern**: Yes

## Performance & Size

- **output_overhead**: Minimal
- **memory_efficiency**: Streaming

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```
Name,Age,City
Alice,30,NYC
Bob,25,LA
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [TableFormatter Documentation](../../src/formatters/table.rs)
- [CSV RFC4180](https://tools.ietf.org/html/rfc4180)

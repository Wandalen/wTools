# JsonFormatter: Pretty

Human-readable JSON with indentation and newlines. Default JSON variant.

## Identity & Classification

- **formatter**: JsonFormatter
- **variant**: Pretty
- **is_default**: Yes
- **category**: Data

## Build & Dependencies

- **feature_flag**: format_json
- **runtime_deps**: serde+serde_json
- **zero_dependency**: No

## Character Set & Encoding

- **charset**: UTF-8
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: No

## Visual Structure

- **has_borders**: No
- **border_style**: None
- **column_separator**: None
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

## Data Representation

- **machine_parseable**: Yes
- **human_readable**: Yes
- **supports_hierarchical**: Yes
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: Yes

## Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Standard
- **alignment**: None
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Human-readable JSON with indentation
- **terminal_optimized**: Partial
- **file_export_suitable**: Yes
- **streaming_friendly**: No
- **grep_friendly**: Partial

## Technical Details

- **escaping_rules**: Backslash
- **output_format**: application/json
- **standards_compliance**: JSON-RFC8259
- **supports_custom_colors**: No

## API & Construction

- **constructor**: mode parameter (default)
- **config_type**: None
- **customizable_parameters**: 1
- **builder_pattern**: No

## Performance & Size

- **output_overhead**: High
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```json
{
  "rows": [
    {"Name": "Alice", "Age": "30", "City": "NYC"},
    {"Name": "Bob", "Age": "25", "City": "LA"}
  ]
}
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [JsonFormatter Documentation](../../src/formatters/json.rs)
- [JSON RFC8259](https://tools.ietf.org/html/rfc8259)

# JsonFormatter: Compact

Minified JSON output with no whitespace, single-line format.

## Identity & Classification

- **formatter**: JsonFormatter
- **variant**: Compact
- **is_default**: No
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
- **row_separator**: None
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

## Data Representation

- **machine_parseable**: Yes
- **human_readable**: No
- **supports_hierarchical**: Yes
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: Yes

## Output Characteristics

- **output_compactness**: Maximum
- **visual_complexity**: Minimal
- **alignment**: None
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Minified JSON, single line
- **terminal_optimized**: No
- **file_export_suitable**: Primary
- **streaming_friendly**: Yes
- **grep_friendly**: No

## Technical Details

- **escaping_rules**: Backslash
- **output_format**: application/json
- **standards_compliance**: JSON-RFC8259
- **supports_custom_colors**: No

## API & Construction

- **constructor**: mode parameter
- **config_type**: None
- **customizable_parameters**: 1
- **builder_pattern**: No

## Performance & Size

- **output_overhead**: Minimal
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```json
{"rows":[{"Name":"Alice","Age":"30","City":"NYC"},{"Name":"Bob","Age":"25","City":"LA"}]}
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [JsonFormatter Documentation](../../src/formatters/json.rs)
- [JSON RFC8259](https://tools.ietf.org/html/rfc8259)

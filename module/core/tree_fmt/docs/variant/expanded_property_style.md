# ExpandedFormatter: property_style

Property list format with colored keys for enhanced readability.

## Identity & Classification

- **formatter**: ExpandedFormatter
- **variant**: property_style
- **is_default**: No
- **category**: Visual

## Build & Dependencies

- **feature_flag**: expanded_property
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
- **column_separator**: Colon
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 1

## Data Representation

- **machine_parseable**: Partial
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: Yes

## Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Simple
- **alignment**: Left
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Property lists with colored keys
- **terminal_optimized**: Yes
- **file_export_suitable**: Partial
- **streaming_friendly**: Yes
- **grep_friendly**: Yes

## Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: Yes

## API & Construction

- **constructor**: `ExpandedConfig::property_style()`
- **config_type**: ExpandedConfig
- **customizable_parameters**: 8+
- **builder_pattern**: Yes

## Performance & Size

- **output_overhead**: Medium
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```
Name: Alice
Age:  30
City: NYC

Name: Bob
Age:  25
City: LA
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [ExpandedFormatter Documentation](../../src/formatters/expanded.rs)

# TableFormatter: plain

Space-separated table with dash header separator. Default variant optimized for CLI tools.

## Identity & Classification

- **formatter**: TableFormatter
- **variant**: plain
- **is_default**: Yes
- **category**: Visual

## Build & Dependencies

- **feature_flag**: table_plain
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
- **column_separator**: Spaces (2)
- **row_separator**: Newline
- **header_separator**: Dashes
- **outer_padding**: Yes
- **inner_padding**: 0

## Data Representation

- **machine_parseable**: Partial
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: No

## Output Characteristics

- **output_compactness**: Compact
- **visual_complexity**: Simple
- **alignment**: Both
- **column_alignment**: Yes

## Usage Context

- **primary_use_case**: CLI tools output (ps, top, pmon)
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

- **constructor**: `TableConfig::plain()`
- **config_type**: TableConfig
- **customizable_parameters**: 15+
- **builder_pattern**: Yes

## Performance & Size

- **output_overhead**: Low
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```
 Name   Age  City
 -----  ---  -------
 Alice  30   NYC
 Bob    25   LA
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [TableFormatter Documentation](../../src/formatters/table.rs)

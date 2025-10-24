# TableFormatter: unicode_box

Unicode box-drawing characters for modern terminal UIs.

## Identity & Classification

- **formatter**: TableFormatter
- **variant**: unicode_box
- **is_default**: No
- **category**: Visual

## Build & Dependencies

- **feature_flag**: table_unicode
- **runtime_deps**: None
- **zero_dependency**: Yes

## Character Set & Encoding

- **charset**: Unicode
- **border_charset**: Unicode
- **requires_unicode_terminal**: Yes
- **supports_ansi_colors**: Yes

## Visual Structure

- **has_borders**: Yes
- **border_style**: Unicode-Box
- **column_separator**: │
- **row_separator**: Grid-Lines
- **header_separator**: Unicode
- **outer_padding**: Yes
- **inner_padding**: 1

## Data Representation

- **machine_parseable**: No
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: No

## Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Rich
- **alignment**: Both
- **column_alignment**: Yes

## Usage Context

- **primary_use_case**: Modern terminal UIs
- **terminal_optimized**: Yes
- **file_export_suitable**: No
- **streaming_friendly**: No
- **grep_friendly**: No

## Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: Yes

## API & Construction

- **constructor**: `TableConfig::unicode_box()`
- **config_type**: TableConfig
- **customizable_parameters**: 15+
- **builder_pattern**: Yes

## Performance & Size

- **output_overhead**: High
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Partial
- **works_in_ci**: Partial
- **copy_paste_friendly**: Yes

## Example Output

```
┌───────┬─────┬──────┐
│ Name  │ Age │ City │
├───────┼─────┼──────┤
│ Alice │ 30  │ NYC  │
├───────┼─────┼──────┤
│ Bob   │ 25  │ LA   │
└───────┴─────┴──────┘
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [TableFormatter Documentation](../../src/formatters/table.rs)

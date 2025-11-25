# TreeFormatter: aggregated

Tree view with subtree totals and aggregate statistics.

## Identity & Classification

- **formatter**: TreeFormatter
- **variant**: aggregated
- **is_default**: No
- **category**: Visual

## Build & Dependencies

- **feature_flag**: tree_aggregated
- **runtime_deps**: None
- **zero_dependency**: Yes

## Character Set & Encoding

- **charset**: Unicode
- **border_charset**: Box-Drawing
- **requires_unicode_terminal**: Yes
- **supports_ansi_colors**: Yes

## Visual Structure

- **has_borders**: No
- **border_style**: Tree-Lines
- **column_separator**: None
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 1

## Data Representation

- **machine_parseable**: Partial
- **human_readable**: Yes
- **supports_hierarchical**: Yes
- **supports_tabular**: No
- **preserves_structure**: Yes
- **supports_multiline_values**: No

## Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Standard
- **alignment**: Left
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Tree with subtree totals
- **terminal_optimized**: Yes
- **file_export_suitable**: Partial
- **streaming_friendly**: Yes
- **grep_friendly**: Partial

## Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: Yes

## API & Construction

- **constructor**: `format_with_aggregation()`
- **config_type**: None
- **customizable_parameters**: 0
- **builder_pattern**: No

## Performance & Size

- **output_overhead**: Medium
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Partial
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```
Root (2 items, total: 55)
├── Alice (age: 30)
│   └── Subtotal: 30
└── Bob (age: 25)
    └── Subtotal: 25
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [TreeFormatter Documentation](../../src/formatters/tree.rs)

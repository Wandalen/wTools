# TreeFormatter: hierarchical

Standard tree view with Unicode box-drawing characters. Default tree variant.

## Identity & Classification

- **formatter**: TreeFormatter
- **variant**: hierarchical
- **is_default**: Yes
- **category**: Visual

## Build & Dependencies

- **feature_flag**: tree_hierarchical
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

- **machine_parseable**: No
- **human_readable**: Yes
- **supports_hierarchical**: Yes
- **supports_tabular**: No
- **preserves_structure**: Yes
- **supports_multiline_values**: No

## Output Characteristics

- **output_compactness**: Compact
- **visual_complexity**: Standard
- **alignment**: Left
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Standard tree view
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

- **constructor**: `format()`
- **config_type**: None
- **customizable_parameters**: 0
- **builder_pattern**: No

## Performance & Size

- **output_overhead**: Low
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Partial
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```
Root
├── Alice
│   └── Age: 30
└── Bob
    └── Age: 25
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [TreeFormatter Documentation](../../src/formatters/tree.rs)

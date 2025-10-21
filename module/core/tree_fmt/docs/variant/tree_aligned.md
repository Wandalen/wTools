# TreeFormatter: aligned

Column-aligned tree view with metadata columns for structured display.

## Identity & Classification

- **formatter**: TreeFormatter
- **variant**: aligned
- **is_default**: No
- **category**: Visual

## Build & Dependencies

- **feature_flag**: tree_aligned
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
- **column_separator**: Spaces
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 1

## Data Representation

- **machine_parseable**: Partial
- **human_readable**: Yes
- **supports_hierarchical**: Yes
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: No

## Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Standard
- **alignment**: Both
- **column_alignment**: Yes

## Usage Context

- **primary_use_case**: Column-aligned tree with metadata
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

- **constructor**: `format_aligned()`
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
Root                Size    Modified
├── Alice           1.2KB   2024-01-15
│   └── Age: 30     128B    2024-01-15
└── Bob             980B    2024-01-14
    └── Age: 25     96B     2024-01-14
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [TreeFormatter Documentation](../../src/formatters/tree.rs)

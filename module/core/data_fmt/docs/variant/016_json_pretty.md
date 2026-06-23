# Variant: JSON Pretty

### Scope

- **Purpose**: Provide human-readable JSON output with indentation and newlines.
- **Responsibility**: Complete attribute descriptor for this output variant preset.
- **In Scope**: All 46 variant attributes, example output, feature flag, compatibility.
- **Out of Scope**: Formatter implementation (see source), attribute schema (see `../data_structure/001_variant_attributes.md`).

### DataStructures

| File | Relationship |
|------|-------------|
| [001_variant_attributes.md](../data_structure/001_variant_attributes.md) | Attribute definitions for all 46 variant attributes |

### Formatters

| File | Relationship |
|------|-------------|
| [005_json_formatter.md](../formatter/005_json_formatter.md) | Parent formatter producing this variant |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/json.rs`](../../src/formatters/json.rs) | JsonFormatter implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/json.rs`](../../tests/json.rs) | JSON output tests |

### Identity & Classification

- **formatter**: JsonFormatter
- **variant**: Pretty
- **is_default**: Yes
- **category**: Data

### Build & Dependencies

- **feature_flag**: format_json
- **runtime_deps**: serde+serde_json
- **zero_dependency**: No

### Character Set & Encoding

- **charset**: UTF-8
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: No

### Visual Structure

- **has_borders**: No
- **border_style**: None
- **column_separator**: None
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

### Data Representation

- **machine_parseable**: Yes
- **human_readable**: Yes
- **supports_hierarchical**: Yes
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: Yes

### Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Standard
- **alignment**: None
- **column_alignment**: No

### Usage Context

- **primary_use_case**: Human-readable JSON with indentation
- **terminal_optimized**: Partial
- **file_export_suitable**: Yes
- **streaming_friendly**: No
- **grep_friendly**: Partial

### Technical Details

- **escaping_rules**: Backslash
- **output_format**: application/json
- **standards_compliance**: JSON-RFC8259
- **supports_custom_colors**: No

### API & Construction

- **constructor**: mode parameter (default)
- **config_type**: None
- **customizable_parameters**: 1
- **builder_pattern**: No

### Performance & Size

- **output_overhead**: High
- **memory_efficiency**: Buffered

### Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

### Example Output

```json
{
  "rows": [
    {"Name": "Alice", "Age": "30", "City": "NYC"},
    {"Name": "Bob", "Age": "25", "City": "LA"}
  ]
}
```

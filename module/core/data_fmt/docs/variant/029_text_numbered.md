# Variant: Text Numbered

### Scope

- **Purpose**: Provide a numbered list format for sequential item display.
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
| [010_text_formatter.md](../formatter/010_text_formatter.md) | Parent formatter producing this variant |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/text.rs`](../../src/formatters/text.rs) | TextFormatter implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text.rs`](../../tests/text.rs) | Text output tests |

### Identity & Classification

- **formatter**: TextFormatter
- **variant**: Numbered
- **is_default**: No
- **category**: Visual

### Build & Dependencies

- **feature_flag**: format_text
- **runtime_deps**: None
- **zero_dependency**: Yes

### Character Set & Encoding

- **charset**: ASCII
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: Yes

### Visual Structure

- **has_borders**: No
- **border_style**: None
- **column_separator**: None
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

### Data Representation

- **machine_parseable**: Partial
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: No
- **preserves_structure**: Partial
- **supports_multiline_values**: Yes

### Output Characteristics

- **output_compactness**: Compact
- **visual_complexity**: Simple
- **alignment**: Left
- **column_alignment**: No

### Usage Context

- **primary_use_case**: Numbered lists
- **terminal_optimized**: Yes
- **file_export_suitable**: Yes
- **streaming_friendly**: Yes
- **grep_friendly**: Yes

### Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: Yes

### API & Construction

- **constructor**: `TextVariant::Numbered`
- **config_type**: TextVariant
- **customizable_parameters**: 1
- **builder_pattern**: No

### Performance & Size

- **output_overhead**: Minimal
- **memory_efficiency**: Buffered

### Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

### Example Output

```
1. Name: Alice, Age: 30, City: NYC
2. Name: Bob, Age: 25, City: LA
```

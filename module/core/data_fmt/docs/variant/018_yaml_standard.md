# Variant: YAML Standard

### Scope

- **Purpose**: Provide standard YAML format with indentation and flow control.
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
| [006_yaml_formatter.md](../formatter/006_yaml_formatter.md) | Parent formatter producing this variant |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/yaml.rs`](../../src/formatters/yaml.rs) | YamlFormatter implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/yaml.rs`](../../tests/yaml.rs) | YAML output tests |

### Identity & Classification

- **formatter**: YamlFormatter
- **variant**: Standard
- **is_default**: Yes
- **category**: Data

### Build & Dependencies

- **feature_flag**: format_yaml
- **runtime_deps**: serde+serde_yaml
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
- **inner_padding**: 2

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

- **primary_use_case**: Standard YAML format
- **terminal_optimized**: Partial
- **file_export_suitable**: Primary
- **streaming_friendly**: No
- **grep_friendly**: Yes

### Technical Details

- **escaping_rules**: YAML-Quoting
- **output_format**: application/yaml
- **standards_compliance**: YAML-1.2
- **supports_custom_colors**: No

### API & Construction

- **constructor**: default
- **config_type**: None
- **customizable_parameters**: 0
- **builder_pattern**: No

### Performance & Size

- **output_overhead**: Medium
- **memory_efficiency**: Buffered

### Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

### Example Output

```yaml
rows:
  - Name: Alice
    Age: "30"
    City: NYC
  - Name: Bob
    Age: "25"
    City: LA
```

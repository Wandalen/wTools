# Variant: TOML Standard

### Scope

- **Purpose**: Provide standard TOML format with sections and key-value pairs.
- **Responsibility**: Complete attribute descriptor for this output variant preset.
- **In Scope**: All 46 variant attributes, example output, feature flag, compatibility.
- **Out of Scope**: Formatter implementation (see source), attribute schema (see `../data_structure/001_variant_attributes.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/toml_fmt.rs` | TomlFormatter implementation |
| test | `tests/toml_fmt.rs` | TOML output tests |
| doc | `../data_structure/001_variant_attributes.md` | Attribute definitions for all 46 variant attributes |

### Identity & Classification

- **formatter**: TomlFormatter
- **variant**: Standard
- **is_default**: Yes
- **category**: Data

### Build & Dependencies

- **feature_flag**: format_toml
- **runtime_deps**: serde+toml
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
- **row_separator**: Double-Newline
- **header_separator**: Brackets
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

- **primary_use_case**: Standard TOML format
- **terminal_optimized**: Partial
- **file_export_suitable**: Primary
- **streaming_friendly**: No
- **grep_friendly**: Yes

### Technical Details

- **escaping_rules**: Backslash
- **output_format**: application/toml
- **standards_compliance**: TOML-v1.0.0
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

```toml
[[rows]]
Name = "Alice"
Age = "30"
City = "NYC"

[[rows]]
Name = "Bob"
Age = "25"
City = "LA"
```

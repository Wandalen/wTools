# Variant: Table Markdown

### Scope

- **Purpose**: Provide GitHub-flavored Markdown table format for documentation embedding.
- **Responsibility**: Complete attribute descriptor for this output variant preset.
- **In Scope**: All 46 variant attributes, example output, feature flag, compatibility.
- **Out of Scope**: Formatter implementation (see source), attribute schema (see `../data_structure/001_variant_attributes.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/table.rs` | TableFormatter implementation |
| test | `tests/table_styles_presets.rs` | Preset configuration and output tests |
| doc | `../data_structure/001_variant_attributes.md` | Attribute definitions for all 46 variant attributes |

### Identity & Classification

- **formatter**: TableFormatter
- **variant**: markdown
- **is_default**: No
- **category**: Markup

### Build & Dependencies

- **feature_flag**: table_markdown
- **runtime_deps**: None
- **zero_dependency**: Yes

### Character Set & Encoding

- **charset**: ASCII
- **border_charset**: ASCII
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: No

### Visual Structure

- **has_borders**: Yes
- **border_style**: Markdown
- **column_separator**: Pipes |
- **row_separator**: Newline
- **header_separator**: Markdown
- **outer_padding**: Yes
- **inner_padding**: 1

### Data Representation

- **machine_parseable**: Yes
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: No

### Output Characteristics

- **output_compactness**: Standard
- **visual_complexity**: Simple
- **alignment**: Both
- **column_alignment**: Yes

### Usage Context

- **primary_use_case**: GitHub documentation, README files
- **terminal_optimized**: Partial
- **file_export_suitable**: Primary
- **streaming_friendly**: Yes
- **grep_friendly**: Partial

### Technical Details

- **escaping_rules**: None
- **output_format**: text/markdown
- **standards_compliance**: Markdown-GFM
- **supports_custom_colors**: No

### API & Construction

- **constructor**: `TableConfig::markdown()`
- **config_type**: TableConfig
- **customizable_parameters**: 15+
- **builder_pattern**: Yes

### Performance & Size

- **output_overhead**: Medium
- **memory_efficiency**: Buffered

### Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

### Example Output

```markdown
| Name  | Age | City |
|-------|-----|------|
| Alice | 30  | NYC  |
| Bob   | 25  | LA   |
```

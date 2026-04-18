# Variant: Table Bordered

### Scope

- **Purpose**: Provide a traditional pipe-separated table with ASCII grid for PostgreSQL-style output.
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
- **variant**: bordered
- **is_default**: No
- **category**: Visual

### Build & Dependencies

- **feature_flag**: table_bordered
- **runtime_deps**: None
- **zero_dependency**: Yes

### Character Set & Encoding

- **charset**: ASCII
- **border_charset**: ASCII
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: Yes

### Visual Structure

- **has_borders**: Yes
- **border_style**: ASCII-Pipes
- **column_separator**: Pipes |
- **row_separator**: Newline
- **header_separator**: ASCII-Grid
- **outer_padding**: Yes
- **inner_padding**: 1

### Data Representation

- **machine_parseable**: Partial
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: No

### Output Characteristics

- **output_compactness**: Standard
- **visual_complexity**: Standard
- **alignment**: Both
- **column_alignment**: Yes

### Usage Context

- **primary_use_case**: Database output, PostgreSQL-style
- **terminal_optimized**: Yes
- **file_export_suitable**: No
- **streaming_friendly**: Yes
- **grep_friendly**: Partial

### Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: Yes

### API & Construction

- **constructor**: `TableConfig::bordered()`
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

```
 Name  | Age | City
-------+-----+--------
 Alice |  30 | NYC
 Bob   |  25 | LA
```

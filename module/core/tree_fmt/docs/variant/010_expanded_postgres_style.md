# Variant: Expanded Postgres

### Scope

- **Purpose**: Provide vertical record display with field separators optimized for PostgreSQL \x mode.
- **Responsibility**: Complete attribute descriptor for this output variant preset.
- **In Scope**: All 46 variant attributes, example output, feature flag, compatibility.
- **Out of Scope**: Formatter implementation (see source), attribute schema (see `../data_structure/001_variant_attributes.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/expanded.rs` | ExpandedFormatter implementation |
| test | `tests/expanded_behavior.rs` | Expanded formatter behavior tests |
| doc | `../data_structure/001_variant_attributes.md` | Attribute definitions for all 46 variant attributes |

### Identity & Classification

- **formatter**: ExpandedFormatter
- **variant**: postgres_style
- **is_default**: Yes
- **category**: Visual

### Build & Dependencies

- **feature_flag**: expanded_postgres
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
- **column_separator**: Pipe
- **row_separator**: Dashes
- **header_separator**: Dashes
- **outer_padding**: No
- **inner_padding**: 1

### Data Representation

- **machine_parseable**: Partial
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: Yes

### Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Simple
- **alignment**: Left
- **column_alignment**: No

### Usage Context

- **primary_use_case**: PostgreSQL \x mode
- **terminal_optimized**: Yes
- **file_export_suitable**: Partial
- **streaming_friendly**: Yes
- **grep_friendly**: Yes

### Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: PostgreSQL
- **supports_custom_colors**: Yes

### API & Construction

- **constructor**: `ExpandedConfig::postgres_style()`
- **config_type**: ExpandedConfig
- **customizable_parameters**: 8+
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
-[ RECORD 1 ]----
Name | Alice
Age  | 30
City | NYC
-[ RECORD 2 ]----
Name | Bob
Age  | 25
City | LA
```

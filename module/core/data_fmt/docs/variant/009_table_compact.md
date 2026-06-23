# Variant: Table Compact

### Scope

- **Purpose**: Provide a single-space separator table with minimal padding for narrow terminals.
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
| [001_table_formatter.md](../formatter/001_table_formatter.md) | Parent formatter producing this variant |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | TableFormatter implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/table_styles_presets.rs`](../../tests/table_styles_presets.rs) | Preset configuration and output tests |

### Identity & Classification

- **formatter**: TableFormatter
- **variant**: compact
- **is_default**: No
- **category**: Visual

### Build & Dependencies

- **feature_flag**: table_compact
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
- **column_separator**: Spaces (1)
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

### Data Representation

- **machine_parseable**: Partial
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: No

### Output Characteristics

- **output_compactness**: Minimal
- **visual_complexity**: Minimal
- **alignment**: Both
- **column_alignment**: Yes

### Usage Context

- **primary_use_case**: Narrow terminals, space-constrained
- **terminal_optimized**: Yes
- **file_export_suitable**: No
- **streaming_friendly**: Yes
- **grep_friendly**: Yes

### Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: Yes

### API & Construction

- **constructor**: `TableConfig::compact()`
- **config_type**: TableConfig
- **customizable_parameters**: 15+
- **builder_pattern**: Yes

### Performance & Size

- **output_overhead**: Minimal
- **memory_efficiency**: Buffered

### Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

### Example Output

```
Name Age City
Alice 30 NYC
Bob 25 LA
```

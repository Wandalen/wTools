# Variant: Table TSV

### Scope

- **Purpose**: Provide tab-separated values format for spreadsheet paste and clipboard data.
- **Responsibility**: Complete attribute descriptor for this output variant preset.
- **In Scope**: All 46 variant attributes, example output, feature flag, compatibility.
- **Out of Scope**: Formatter implementation (see source), attribute schema (see `../data_structure/001_variant_attributes.md`).

### DataStructures

| File | Relationship |
|------|-------------|
| [001_variant_attributes.md](../data_structure/001_variant_attributes.md) | Attribute definitions for all 46 variant attributes |

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
- **variant**: tsv
- **is_default**: No
- **category**: Data

### Build & Dependencies

- **feature_flag**: table_tsv
- **runtime_deps**: None
- **zero_dependency**: Yes

### Character Set & Encoding

- **charset**: ASCII
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: No

### Visual Structure

- **has_borders**: No
- **border_style**: None
- **column_separator**: Tabs \t
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

### Data Representation

- **machine_parseable**: Yes
- **human_readable**: Partial
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: No

### Output Characteristics

- **output_compactness**: Minimal
- **visual_complexity**: Minimal
- **alignment**: None
- **column_alignment**: No

### Usage Context

- **primary_use_case**: Spreadsheet paste, clipboard data
- **terminal_optimized**: No
- **file_export_suitable**: Primary
- **streaming_friendly**: Yes
- **grep_friendly**: Partial

### Technical Details

- **escaping_rules**: None
- **output_format**: text/tab-separated-values
- **standards_compliance**: None
- **supports_custom_colors**: No

### API & Construction

- **constructor**: `TableConfig::tsv()`
- **config_type**: TableConfig
- **customizable_parameters**: 15+
- **builder_pattern**: Yes

### Performance & Size

- **output_overhead**: Minimal
- **memory_efficiency**: Streaming

### Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

### Example Output

```
Name	Age	City
Alice	30	NYC
Bob	25	LA
```

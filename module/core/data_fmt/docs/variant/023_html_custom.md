# Variant: HTML Custom

### Scope

- **Purpose**: Provide an HTML table with user-provided CSS classes for customized styling.
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
| [`src/formatters/html.rs`](../../src/formatters/html.rs) | HtmlFormatter implementation |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/html.rs`](../../tests/html.rs) | HTML output tests |

### Identity & Classification

- **formatter**: HtmlFormatter
- **variant**: Custom
- **is_default**: No
- **category**: Markup

### Build & Dependencies

- **feature_flag**: html_custom
- **runtime_deps**: None
- **zero_dependency**: Yes

### Character Set & Encoding

- **charset**: UTF-8
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: No

### Visual Structure

- **has_borders**: Partial
- **border_style**: CSS-Defined
- **column_separator**: None
- **row_separator**: None
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

### Data Representation

- **machine_parseable**: Yes
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: Yes

### Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Variable
- **alignment**: None
- **column_alignment**: No

### Usage Context

- **primary_use_case**: User-provided CSS classes
- **terminal_optimized**: No
- **file_export_suitable**: Primary
- **streaming_friendly**: No
- **grep_friendly**: No

### Technical Details

- **escaping_rules**: HTML-Entities
- **output_format**: text/html
- **standards_compliance**: HTML5
- **supports_custom_colors**: No

### API & Construction

- **constructor**: `HtmlVariant::Custom(String)`
- **config_type**: HtmlVariant
- **customizable_parameters**: Unlimited
- **builder_pattern**: No

### Performance & Size

- **output_overhead**: Very-High
- **memory_efficiency**: Buffered

### Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: No

### Example Output

```html
<table class="my-custom-table">
  <thead class="my-header">
    <tr><th>Name</th><th>Age</th><th>City</th></tr>
  </thead>
  <tbody class="my-body">
    <tr><td>Alice</td><td>30</td><td>NYC</td></tr>
    <tr><td>Bob</td><td>25</td><td>LA</td></tr>
  </tbody>
</table>
```

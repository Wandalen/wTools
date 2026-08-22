# Variant: HTML Tailwind

### Scope

- **Purpose**: Provide an HTML table with Tailwind CSS classes for utility-first styled display.
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
| [008_html_formatter.md](../formatter/008_html_formatter.md) | Parent formatter producing this variant |

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
- **variant**: Tailwind
- **is_default**: No
- **category**: Markup

### Build & Dependencies

- **feature_flag**: html_tailwind
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
- **visual_complexity**: Rich
- **alignment**: None
- **column_alignment**: No

### Usage Context

- **primary_use_case**: Tailwind CSS classes
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

- **constructor**: `HtmlVariant::Tailwind`
- **config_type**: HtmlVariant
- **customizable_parameters**: 1
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
<table class="min-w-full divide-y divide-gray-200">
  <thead>
    <tr><th>Name</th><th>Age</th><th>City</th></tr>
  </thead>
  <tbody>
    <tr><td>Alice</td><td>30</td><td>NYC</td></tr>
    <tr><td>Bob</td><td>25</td><td>LA</td></tr>
  </tbody>
</table>
```

Note: the current implementation applies the variant's CSS class only to the
`<table>` tag itself; `<thead>`, `<th>`, `<tbody>`, and `<td>` are always
emitted with no class attribute (verified via `variant_class()` and `format()`
in `src/formatters/html.rs`, which never writes a class on any element other
than `<table>`).

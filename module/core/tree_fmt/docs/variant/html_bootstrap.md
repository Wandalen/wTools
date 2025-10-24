# HtmlFormatter: Bootstrap

HTML table with Bootstrap 5 CSS classes for styled web display.

## Identity & Classification

- **formatter**: HtmlFormatter
- **variant**: Bootstrap
- **is_default**: No
- **category**: Markup

## Build & Dependencies

- **feature_flag**: html_bootstrap
- **runtime_deps**: None
- **zero_dependency**: Yes

## Character Set & Encoding

- **charset**: UTF-8
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: No

## Visual Structure

- **has_borders**: Partial
- **border_style**: CSS-Defined
- **column_separator**: None
- **row_separator**: None
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

## Data Representation

- **machine_parseable**: Yes
- **human_readable**: Yes
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: Yes

## Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Rich
- **alignment**: None
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Bootstrap 5 styling for web dashboards
- **terminal_optimized**: No
- **file_export_suitable**: Primary
- **streaming_friendly**: No
- **grep_friendly**: No

## Technical Details

- **escaping_rules**: HTML-Entities
- **output_format**: text/html
- **standards_compliance**: HTML5
- **supports_custom_colors**: No

## API & Construction

- **constructor**: `HtmlVariant::Bootstrap`
- **config_type**: HtmlVariant
- **customizable_parameters**: 2
- **builder_pattern**: No

## Performance & Size

- **output_overhead**: Very-High
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: No

## Example Output

```html
<table class="table table-striped">
  <thead>
    <tr><th>Name</th><th>Age</th><th>City</th></tr>
  </thead>
  <tbody>
    <tr><td>Alice</td><td>30</td><td>NYC</td></tr>
    <tr><td>Bob</td><td>25</td><td>LA</td></tr>
  </tbody>
</table>
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [HtmlFormatter Documentation](../../src/formatters/html.rs)
- [Bootstrap 5 Tables](https://getbootstrap.com/docs/5.0/content/tables/)

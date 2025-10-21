# HtmlFormatter: Tailwind

HTML table with Tailwind CSS classes for utility-first styled display.

## Identity & Classification

- **formatter**: HtmlFormatter
- **variant**: Tailwind
- **is_default**: No
- **category**: Markup

## Build & Dependencies

- **feature_flag**: html_tailwind
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

- **primary_use_case**: Tailwind CSS classes
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

- **constructor**: `HtmlVariant::Tailwind`
- **config_type**: HtmlVariant
- **customizable_parameters**: 1
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
<table class="min-w-full divide-y divide-gray-200">
  <thead class="bg-gray-50">
    <tr><th class="px-6 py-3">Name</th><th class="px-6 py-3">Age</th><th class="px-6 py-3">City</th></tr>
  </thead>
  <tbody class="divide-y divide-gray-200">
    <tr><td class="px-6 py-4">Alice</td><td class="px-6 py-4">30</td><td class="px-6 py-4">NYC</td></tr>
    <tr><td class="px-6 py-4">Bob</td><td class="px-6 py-4">25</td><td class="px-6 py-4">LA</td></tr>
  </tbody>
</table>
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [HtmlFormatter Documentation](../../src/formatters/html.rs)
- [Tailwind CSS](https://tailwindcss.com)

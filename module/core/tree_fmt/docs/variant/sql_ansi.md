# SqlFormatter: ANSI

Standard SQL-compliant INSERT statements. Default SQL variant.

## Identity & Classification

- **formatter**: SqlFormatter
- **variant**: ANSI
- **is_default**: Yes
- **category**: Data

## Build & Dependencies

- **feature_flag**: sql_ansi
- **runtime_deps**: None
- **zero_dependency**: Yes

## Character Set & Encoding

- **charset**: ASCII
- **border_charset**: None
- **requires_unicode_terminal**: No
- **supports_ansi_colors**: No

## Visual Structure

- **has_borders**: No
- **border_style**: None
- **column_separator**: Comma
- **row_separator**: Semicolon
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 0

## Data Representation

- **machine_parseable**: Yes
- **human_readable**: Partial
- **supports_hierarchical**: No
- **supports_tabular**: Yes
- **preserves_structure**: Yes
- **supports_multiline_values**: Yes

## Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Standard
- **alignment**: None
- **column_alignment**: No

## Usage Context

- **primary_use_case**: Standard SQL compliant
- **terminal_optimized**: No
- **file_export_suitable**: Primary
- **streaming_friendly**: Yes
- **grep_friendly**: Partial

## Technical Details

- **escaping_rules**: Single-Quote
- **output_format**: text/sql
- **standards_compliance**: SQL-92
- **supports_custom_colors**: No

## API & Construction

- **constructor**: `SqlVariant::Ansi`
- **config_type**: SqlVariant
- **customizable_parameters**: 2
- **builder_pattern**: No

## Performance & Size

- **output_overhead**: Medium
- **memory_efficiency**: Buffered

## Compatibility

- **works_on_windows**: Yes
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

## Example Output

```sql
INSERT INTO table_name (Name, Age, City) VALUES ('Alice', '30', 'NYC');
INSERT INTO table_name (Name, Age, City) VALUES ('Bob', '25', 'LA');
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [SqlFormatter Documentation](../../src/formatters/sql.rs)
- [SQL-92 Standard](https://en.wikipedia.org/wiki/SQL-92)

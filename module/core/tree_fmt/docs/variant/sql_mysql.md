# SqlFormatter: MySQL

MySQL/MariaDB-specific INSERT statements with dialect support.

## Identity & Classification

- **formatter**: SqlFormatter
- **variant**: MySQL
- **is_default**: No
- **category**: Data

## Build & Dependencies

- **feature_flag**: sql_mysql
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

- **primary_use_case**: MySQL/MariaDB syntax
- **terminal_optimized**: No
- **file_export_suitable**: Primary
- **streaming_friendly**: Yes
- **grep_friendly**: Partial

## Technical Details

- **escaping_rules**: Single-Quote
- **output_format**: text/sql
- **standards_compliance**: MySQL
- **supports_custom_colors**: No

## API & Construction

- **constructor**: `SqlVariant::MySQL`
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
INSERT INTO `table_name` (`Name`, `Age`, `City`) VALUES ('Alice', '30', 'NYC');
INSERT INTO `table_name` (`Name`, `Age`, `City`) VALUES ('Bob', '25', 'LA');
```

## Related

- [Table of Variants](../../readme.md#table-of-variants)
- [SqlFormatter Documentation](../../src/formatters/sql.rs)
- [MySQL INSERT](https://dev.mysql.com/doc/refman/8.0/en/insert.html)

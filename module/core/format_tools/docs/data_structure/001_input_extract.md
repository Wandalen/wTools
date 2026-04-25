# Data Structure: Input Extract

### Scope

- **Purpose**: Hold the extracted column names and row data ready for consumption by a table output formatter.
- **Responsibility**: Documents the input extract structure — its fields, constraints, and the operations callers and formatters perform on it.
- **In Scope**: Column name list, row data, header flag, and how formatters read from this structure.
- **Out of Scope**: Output accumulation (→ data_structure/002_context.md), format selection (→ pattern/002_format_strategy.md).

### Abstract

Input Extract is the intermediate representation between data extraction and formatting. It holds two parallel collections: a list of column names and a sequence of rows, where each row is a list of string values aligned with the column names. An optional header flag controls whether the column names are rendered as a header row in the output.

### Structure

Contains: an ordered list of column name strings; a sequence of rows where each row is an ordered list of string values; a flag indicating whether column names should be rendered as a header. Column count is fixed at construction — each row must have the same number of values as there are column names.

### Operations

**Construction**: Built from explicitly provided column names and row data, typically by the table construction utilities or the reflection-based conversion path.

**Read by formatter**: The output format implementation reads column names, iterates over rows, and reads each cell value. The structure is read-only during formatting.

**Column count query**: Formatters query the column count to determine separator widths.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/table.rs` | Table construction utilities that produce InputExtract |
| source | `src/format/output_format.rs` | Formatter interface that consumes InputExtract |
| test | `tests/inc/table_test.rs` | InputExtract construction and formatting tests |
| doc | `docs/api/003_table_formatting_api.md` | API that operates on this structure |
| doc | `docs/feature/002_table_formatting.md` | Feature that uses this structure |

# API: Table Formatting API

### Scope

- **Purpose**: Provide traits and utilities for formatting structured data as a formatted plain-text table.
- **Responsibility**: Documents the public interface for table formatting — the format trait, the table conversion trait, raw construction, and compatibility policy.
- **In Scope**: The pluggable output format interface, the table conversion trait, and the raw vector table construction function.
- **Out of Scope**: Specific format implementations (grid/records/keys — those are in feature/002), field macros (→ api/002), fallback conversion (→ api/001).

### Abstract

The table formatting API provides two complementary entry points: a pluggable format interface for implementing custom output layouts, and a table conversion trait for types that can be formatted as tables using reflection. A raw vector construction path is also available for callers that build tables from explicitly provided column names and rows.

### Operations

**Output format interface**: The pluggable format interface accepts a table input structure and an output buffer, and writes the formatted string into the buffer. Implementors define how columns, rows, headers, and separators are rendered. Three implementations are provided: grid layout, records layout, key-value layout.

**Table conversion trait**: A trait implemented by types that can present themselves as tables. Provides methods to format the type as a table string using any compatible output format. Works with types implementing the reflection interface from reflect_tools.

**Raw vector construction**: A function accepting column names as a list of string values and rows as a list of lists of string values. Constructs a table from explicitly provided data without requiring reflection.

### Error Handling

Format operations return a result indicating whether the write to the output buffer succeeded. Buffer write failures propagate to the caller. No panics on malformed input; missing or empty data produces empty output.

### Compatibility Guarantees

The pluggable format interface is stable; new implementations can be added without breaking existing callers. The table conversion trait is stable. Raw construction function signature is stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/output_format.rs` | Pluggable format interface |
| source | `src/format/output_format/table.rs` | Grid layout implementation |
| source | `src/format/output_format/records.rs` | Records layout implementation |
| source | `src/format/output_format/keys.rs` | Key-value layout implementation |
| source | `src/format/as_table.rs` | Table conversion trait |
| source | `src/format/table.rs` | Table construction utilities |
| test | `tests/inc/format_table_test.rs` | Grid layout tests |
| test | `tests/inc/format_records_test.rs` | Records layout tests |
| test | `tests/inc/table_test.rs` | Table utility tests |
| doc | `docs/feature/002_table_formatting.md` | Feature description |
| doc | `docs/data_structure/001_input_extract.md` | Input structure consumed by this API |
| doc | `docs/data_structure/002_context.md` | Output buffer written by this API |
| doc | `docs/pattern/002_format_strategy.md` | Strategy pattern behind this API |

# Feature: Table Formatting

### Scope

- **Purpose**: Render structured tabular data as a formatted plain-text string in one of three selectable layouts.
- **Responsibility**: Documents the table formatting capability — its layouts, format selection mechanism, input model, and all associated artifacts.
- **In Scope**: Grid table layout, vertical records layout, key-value layout, pluggable format interface, raw vector table construction, header support.
- **Out of Scope**: Fallback string conversion (→ feature/001), field extraction macros (→ feature/003), color or terminal styling (→ invariant/002).

### Design

Table formatting accepts structured input — a list of column names and a sequence of rows — and produces a formatted plain-text string in one of three layouts:

- **Grid layout**: Traditional table with column headers and ASCII separator rows, suited for many rows with few columns.
- **Records layout**: Vertical format where each row appears as a labeled block with one field per line, suited for few rows with many columns.
- **Key-value layout**: Each field presented as a labeled pair, suited for single-record display.

The output format is selected by the caller at invocation time through a pluggable interface. The core formatting engine is decoupled from any specific layout, allowing custom layouts to be added without modifying the engine. The engine writes into an output buffer incrementally; the buffer is returned as the formatted string.

Column headers are optional. All separators and borders use plain ASCII characters. No color codes, escape sequences, or terminal-specific bytes are emitted.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/output_format.rs` | Output format module and pluggable interface |
| source | `src/format/output_format/table.rs` | Grid table layout implementation |
| source | `src/format/output_format/records.rs` | Vertical records layout implementation |
| source | `src/format/output_format/keys.rs` | Key-value layout implementation |
| source | `src/format/as_table.rs` | Table conversion trait |
| source | `src/format/table.rs` | Table construction utilities |
| test | `tests/inc/format_table_test.rs` | Grid table layout tests |
| test | `tests/inc/format_records_test.rs` | Records layout tests |
| test | `tests/inc/table_test.rs` | Table utility tests |
| test | `tests/inc/tabe_foreign_test.rs` | Foreign type integration tests |
| test | `tests/inc/fields_test.rs` | Field reflection integration tests |
| doc | `docs/api/003_table_formatting_api.md` | Public API for table formatting |
| doc | `docs/pattern/002_format_strategy.md` | Pluggable format strategy pattern |
| doc | `docs/data_structure/001_input_extract.md` | Table input representation |
| doc | `docs/data_structure/002_context.md` | Table output buffer |
| doc | `docs/invariant/001_pure_data_transformation.md` | No I/O constraint |
| doc | `docs/invariant/002_no_color_styling.md` | Plain text output constraint |

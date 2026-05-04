# Data Structure: Context

### Scope

- **Purpose**: Accumulate the formatted string output incrementally as the table formatter writes each row, separator, and header.
- **Responsibility**: Documents the context buffer structure — its role, how formatters write to it, and how callers retrieve the result.
- **In Scope**: Output buffer accumulation, incremental write semantics, result extraction.
- **Out of Scope**: Input data (→ data_structure/001_input_extract.md), format selection (→ pattern/002_format_strategy.md).

### Abstract

Context is the write target for all formatting operations. It accumulates string fragments incrementally — headers, separators, cell values, row terminators — as the formatter processes the input data. When formatting completes, the caller extracts the final string from the context. The context satisfies the pure-transformation invariant: it holds only in-memory string data and performs no I/O.

### Structure

Contains: an internal string buffer that grows as content is appended; write position tracking. The buffer is allocated once and grows as needed. No fixed capacity limit is imposed.

### Operations

**Write fragment**: Appends a string fragment to the buffer. Called repeatedly by the formatter for each piece of output (cell value, separator character, newline).

**Extract result**: Returns the accumulated buffer content as a string. Called once after formatting completes. The context may be reset and reused for subsequent formatting operations.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/output_format.rs` | Formatter interface that writes to Context |
| source | `src/format/output_format/table.rs` | Grid layout writes to Context |
| source | `src/format/output_format/records.rs` | Records layout writes to Context |
| source | `src/format/output_format/keys.rs` | Key-value layout writes to Context |
| doc | `docs/api/003_table_formatting_api.md` | API that uses Context as its output target |
| doc | `docs/invariant/001_pure_data_transformation.md` | In-memory buffer satisfies the no-I/O constraint |
| doc | `docs/feature/002_table_formatting.md` | Feature that produces output into Context |

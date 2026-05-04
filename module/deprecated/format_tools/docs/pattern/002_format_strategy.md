# Pattern: Format Strategy

### Scope

- **Purpose**: Decouple the table formatting engine from any specific output layout by routing through a common pluggable interface.
- **Responsibility**: Documents the format strategy pattern — the problem it solves, the interface structure, when to apply it, and its trade-offs.
- **In Scope**: The pluggable output format interface, runtime layout selection, extension with custom layouts.
- **Out of Scope**: Fallback chain dispatch (→ pattern/001), data extraction and reflection (→ feature/005).

### Problem

Multiple output layouts are needed for the same tabular data (grid, vertical records, key-value). Hardcoding layout logic into the formatting engine would require modifications to add layouts, and callers could not inject custom layouts without modifying library code.

### Solution

Define a common interface that all output format implementations must satisfy. The interface accepts a table input structure and an output buffer and renders the layout into the buffer. The engine invokes the interface without knowing which implementation is active. Callers select the desired format at call time and pass it to the engine.

Three implementations are provided: grid layout, records layout, key-value layout. All three satisfy the same interface. Custom implementations can be defined outside the library and used without any library modification.

### Applicability

Apply this pattern when:
- Multiple output representations of the same data exist.
- New representations may be added in the future.
- The caller should choose the representation at call time.
- The core engine must remain independent of layout-specific logic.

### Consequences

**Benefits**: New layouts can be added without modifying the engine; callers control layout selection; the engine is testable with any layout including test doubles; the interface is small and easy to implement.

**Trade-offs**: Caller must pass the format object explicitly; the interface introduces an indirection layer (negligible cost for formatting operations).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/format/output_format.rs` | Pluggable format interface definition |
| source | `src/format/output_format/table.rs` | Grid layout strategy |
| source | `src/format/output_format/records.rs` | Records layout strategy |
| source | `src/format/output_format/keys.rs` | Key-value layout strategy |
| test | `tests/inc/format_table_test.rs` | Grid strategy tests |
| test | `tests/inc/format_records_test.rs` | Records strategy tests |
| doc | `docs/feature/002_table_formatting.md` | Feature that applies this pattern |
| doc | `docs/api/003_table_formatting_api.md` | API built on this pattern |
| doc | `docs/data_structure/001_input_extract.md` | Input consumed by format strategies |
| doc | `docs/data_structure/002_context.md` | Output buffer written by format strategies |

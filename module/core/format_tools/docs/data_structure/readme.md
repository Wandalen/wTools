# Data Structure Doc Entity

### Scope

- **Purpose**: Document the core data structures used internally by the table formatting engine.
- **Responsibility**: Index of data structure doc instances, each describing one in-memory structure's purpose, fields, and operations.
- **In Scope**: Data structures that hold table input or output state within the formatting pipeline.
- **Out of Scope**: Wrapper and marker types (→ api/004), formatting algorithms (→ feature/002), API contracts (→ api/003).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Input Extract](001_input_extract.md) | Holds extracted column names and row data as input to the formatter | ✅ |
| 002 | [Context](002_context.md) | Accumulates formatted string output during table rendering | ✅ |

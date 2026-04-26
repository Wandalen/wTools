# API Doc Entity

### Scope

- **Purpose**: Define the public programmatic interface contracts for `reflect_tools`.
- **Responsibility**: Document operations, error conditions, and compatibility for reflection and fields APIs.
- **In Scope**: Reflection operations, fields iteration operations, return value semantics.
- **Out of Scope**: Feature design decisions (→ `feature/`); correctness properties (→ `invariant/`); internal data types (→ `data_structure/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Reflection API](001_reflection_api.md) | Entity reflection operations contract | ✅ |
| 002 | [Fields API](002_fields_api.md) | Fields trait iteration contract | ✅ |

# Feature Doc Entity

### Scope

- **Purpose**: Document user-facing capabilities exposed by `reflect_tools`.
- **Responsibility**: Provide navigational hubs linking source, test, and doc artifacts for each feature.
- **In Scope**: Feature scope, design decisions, artifact cross-references for both reflection subsystems.
- **Out of Scope**: API contracts (→ `api/`); correctness properties (→ `invariant/`); data type internals (→ `data_structure/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Entity Reflection](001_entity_reflection.md) | Type introspection via Instance/Entity trait pair | ✅ |
| 002 | [Fields Iteration](002_fields_iteration.md) | Key-value iteration over standard collections | ✅ |

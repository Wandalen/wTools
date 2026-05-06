# Invariant Doc Entity

### Scope

- **Purpose**: Define correctness properties that reflection implementations must always maintain.
- **Responsibility**: Document behavioral invariants, enforcement mechanisms, and violation consequences.
- **In Scope**: Container ordering semantics, element iteration consistency guarantees.
- **Out of Scope**: API operation details (→ `api/`); feature design decisions (→ `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Container Ordering](001_container_ordering.md) | is_ordered() correctness for all container types | ✅ |
| 002 | [Element Iteration Contract](002_element_iteration_contract.md) | elements().count() must equal len() | ✅ |

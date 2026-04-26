# Invariant Doc Entity

### Scope

- **Purpose**: Define correctness constraints that the derive macro must always maintain.
- **Responsibility**: Document behavioral invariants, how they are enforced, and consequences of violation.
- **In Scope**: Input type restrictions, attribute handling constraints, compile-time rejection behavior.
- **Out of Scope**: API operation details (→ `api/`); feature design decisions (→ `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Struct-Only Input](001_struct_only_input.md) | Derive macro accepts only struct types | ✅ |

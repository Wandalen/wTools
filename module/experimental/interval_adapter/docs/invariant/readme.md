# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral constraints of `interval_adapter` — properties that the crate explicitly does not provide and why.
- **Responsibility**: Collect one doc instance per out-of-scope constraint; each instance owns the invariant statement, rationale, and enforcement.
- **In Scope**: Formal invariant statements defining boundaries, rationale for each boundary, and enforcement via compile-time or design.
- **Out of Scope**: Feature design (→ `feature/`); API signatures (→ `api/`); pattern decisions (→ `pattern/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Integer Endpoints Only](001_integer_endpoints_only.md) | Interval endpoints must satisfy the endpoint constraint — no floats | ✅ |
| 002 | [No Validation](002_no_validation.md) | Left ≤ right is not checked — caller is responsible | ✅ |
| 003 | [No Set Operations](003_no_set_operations.md) | No union, intersection, containment, or arithmetic on intervals | ✅ |

# Pattern Doc Entity

### Scope

- **Purpose**: Document reusable design patterns employed in `program_tools` that callers may recognize and adapt in their own code.
- **Responsibility**: Catalogs each pattern — its context, problem, solution structure, and consequences.
- **In Scope**: Builder hierarchy design; layered configuration override precedence.
- **Out of Scope**: Implementation detail (→ `api/`); feature behavior (→ `feature/`); correctness guarantees (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Builder Hierarchy](001_builder_hierarchy.md) | Three-level Former builder chain for structured execution plan construction | ✅ |
| 002 | [Layered Configuration](002_layered_configuration.md) | Override precedence: CLI flags over environment over programmatic defaults | ✅ |

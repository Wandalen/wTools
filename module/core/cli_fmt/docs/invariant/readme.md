# Invariant Doc Entity

### Scope

- **Purpose**: Define behavioral contracts that must always hold for this crate.
- **Responsibility**: Architectural boundary enforcement between cli_fmt (CLI-specific) and strs_tools (general-purpose).
- **In Scope**: Instance 001 — architectural boundary between `cli_fmt` and `strs_tools`.
- **Out of Scope**: Desired behavior and processing logic — see `feature/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Architectural Boundary](001_architectural_boundary.md) | cli_fmt vs strs_tools separation boundary | ✅ |

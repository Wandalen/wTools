# Invariant Doc Entity

### Scope

- **Purpose**: Define invariants that must hold across all config_hierarchy implementations.
- **Responsibility**: Formal invariant specifications for resolution behavior and file persistence.
- **In Scope**: Resolution ordering, persistence contracts, and app naming constraints.
- **Out of Scope**: Feature descriptions (→ feature/), API signatures (→ api/)

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Resolution Hierarchy](001_resolution_hierarchy.md) | Precedence ordering of config sources | ✅ |
| 002 | [File Persistence Contracts](002_file_persistence_contracts.md) | Contracts for config file persistence | ✅ |
| 003 | [App Name Constraints](003_app_name_constraints.md) | Valid app name format constraints | ✅ |

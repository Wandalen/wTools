# Integration Doc Entity

### Scope

- **Purpose**: Document how this crate integrates with external systems and consumers.
- **Responsibility**: Collect all integration doc instances for external dependencies and consumers.
- **In Scope**: One instance per external dependency or consumer relationship.
- **Out of Scope**: Internal implementation — see `feature/` and `api/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [macro_tools](001_macro_tools.md) | Syntax parsing utilities used during macro expansion | ✅ |
| 002 | [former_types](002_former_types.md) | Runtime types referenced in generated macro output | ✅ |
| 003 | [former](003_former.md) | Consumer facade that re-exports this crate's macros | ✅ |

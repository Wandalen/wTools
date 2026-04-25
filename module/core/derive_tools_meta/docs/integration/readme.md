# Integration Doc Entity

### Scope

- **Purpose**: Document how `derive_tools_meta` integrates with external systems and consumers.
- **Responsibility**: Master index for all integration doc instances in this crate.
- **In Scope**: Instance 001 — macro_tools dependency; Instance 002 — derive_tools consumer facade.
- **Out of Scope**: Internal implementation — see `feature/` and `api/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [macro_tools](001_macro_tools.md) | Syntax parsing utilities used in macro expansion | ✅ |
| 002 | [derive_tools](002_derive_tools.md) | Consumer facade that re-exports macros from this crate | ✅ |

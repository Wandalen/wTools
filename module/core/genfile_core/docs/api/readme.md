# API Doc Entity

### Scope

- **Purpose**: Documents the public programmatic interface exposed to library consumers.
- **Responsibility**: Index of all API doc instances for genfile_core.
- **In Scope**: Public traits, value types, construction patterns, generation interface, and error contract.
- **Out of Scope**: Internal implementation details (→ `feature/`), non-functional constraints (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Template Value API](001_template_value_api.md) | Public trait and built-in value type contract | ✅ |
| 002 | [Parameter API](002_parameter_api.md) | Parameter definition and collection contract | ✅ |
| 003 | [Generation API](003_generation_api.md) | Template holder and archive generation contract | ✅ |
| 004 | [Error Contract](004_error_contract.md) | Typed error surface and handling guidance | ✅ |

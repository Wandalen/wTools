# Algorithm Doc Entity

### Scope

- **Purpose**: Document the computational procedures underlying variadic_from's code generation.
- **Responsibility**: Lists all algorithm doc instances, each describing one code generation procedure.
- **In Scope**: VariadicFrom derive macro algorithm — field analysis, impl selection, convenience impl rules.
- **Out of Scope**: Trait API definitions → `api/`; feature navigation → `feature/`; correctness properties → `invariant/`.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [VariadicFrom Derive](001_variadic_from_derive.md) | Code generation algorithm for FromN impls | ✅ |

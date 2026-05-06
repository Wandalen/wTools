# API Doc Entity

### Scope

- **Purpose**: Provide a reference for all public programmatic interfaces exposed by `variadic_from`.
- **Responsibility**: Lists all api doc instances, each specifying one public interface.
- **In Scope**: From1, From2, From3 trait definitions; from! macro interface.
- **Out of Scope**: Implementation details → `algorithm/`; correctness properties → `invariant/`.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [FromN Traits](001_from_n_traits.md) | N-argument constructor trait interface | ✅ |
| 002 | [from! Macro](002_from_macro.md) | Variadic argument dispatch macro | ✅ |

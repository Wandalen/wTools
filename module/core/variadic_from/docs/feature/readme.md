# Feature Doc Entity

### Scope

- **Purpose**: Collect navigational hubs for user-facing variadic construction capabilities.
- **Responsibility**: Lists all feature doc instances, each pointing to the artifacts for one user-facing capability.
- **In Scope**: Variadic construction features for structs with 1–3 fields.
- **Out of Scope**: Implementation details → `algorithm/`; trait API → `api/`; correctness properties → `invariant/`.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Variadic Construction](001_variadic_construction.md) | Instantiate structs from 0–3 arguments | ✅ |

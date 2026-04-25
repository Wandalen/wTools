# API Doc Entity

### Scope

- **Purpose**: Document the public macro API of `meta_tools` so consumers know which macros are available and under what feature conditions they are accessible.
- **Responsibility**: Define the complete public macro surface — all re-exported macros, their feature gates, and compatibility guarantees — as the authoritative reference for downstream crates.
- **In Scope**: All macros re-exported from `meta_tools` (from `for_each`, `impls_index`, `impls_index_meta`, `mod_interface_meta`, `paste`), their feature flags, and the `dependency` sub-module.
- **Out of Scope**: Feature scope rationale (see `feature/`), proc-macro implementation details, internal helpers.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Macros](001_macros.md) | Complete macro API surface and feature gate mapping | ✅ |

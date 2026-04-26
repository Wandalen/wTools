# Feature: Error Aggregation

### Scope

- **Purpose**: Provide error handling utilities through the wtools error module alias.
- **Responsibility**: Document the error category feature flags, the aliased module, and its typed/untyped sub-features.
- **In Scope**: Feature flags in the error namespace, exposed module alias, typed and untyped error sub-features.
- **Out of Scope**: Error handling implementation details (see error_tools docs/).

### Design

The error category re-exports error_tools under the short alias `error`. It offers two complementary error paradigms selectable via sub-features.

| Flag | Enables |
|------|---------|
| `error` | Base sub-crate inclusion (activates error_tools enabled feature) |
| `error_default` | typed and untyped error support |
| `error_full` | All error sub-features (same as default) |
| `error_no_std` | no_std support |
| `error_typed` | Typed error support (structured, enum-based errors) |
| `error_untyped` | Untyped error support (dynamic, trait-object errors) |

The error_tools crate is the sole permitted error handling mechanism in the wTools ecosystem. Direct use of third-party error crates is prohibited in application code.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 174-194) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |

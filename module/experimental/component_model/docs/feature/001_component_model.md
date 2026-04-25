# Feature: Component Model

### Scope

- **Purpose**: Enable zero-boilerplate, type-safe component assignment for configuration builders and fluent APIs via a single derive macro.
- **Responsibility**: Navigational hub collecting all source, test, and documentation artifacts for the component model feature across the three-crate ecosystem.
- **In Scope**: The ComponentModel derive macro, the Assign trait system, popular types support, no-std compatibility, and the absorption pattern.
- **Out of Scope**: Builder pattern scaffolding (→ `former` crate); runtime component loading or dependency injection.

### Design

The component model addresses a common pattern in configuration-heavy code: a struct with many fields where each field can be set independently. Instead of hand-writing one setter method per field, a single derive macro generates all setters, dispatching on the value's type.

Core capabilities:

| Capability | Entry point |
|------------|-------------|
| Type-safe assignment | `#[derive(ComponentModel)]` on any named struct |
| Mutating setter | `struct_instance.assign(value)` |
| Builder chaining | `struct_instance.impute(value).impute(other_value)` |
| Multiple fields at once | `struct_instance.components_assign((v1, v2, v3))` |
| Popular types (Duration, PathBuf) | Automatic — recognized at derive time |
| No-std environments | `no_std` + `use_alloc` feature flags |

The single import point (`use component_model::*`) gives access to the derive macros and trait definitions together, hiding the three-crate internal structure from callers.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Re-export aggregation — the single public entry point |
| source | `component_model_meta/src/lib.rs` | Derive macro proc-macro entry points |
| source | `component_model_types/src/component.rs` | Assign, OptionExt, AssignWithType trait definitions |
| test | `tests/tests.rs` | Integration test suite for derive macros |
| test | `tests/smoke_test.rs` | Smoke tests verifying basic build and publish health |
| doc | [pattern/001_absorption_pattern.md](../pattern/001_absorption_pattern.md) | Why the feature is split across three crates |
| doc | `component_model_meta/docs/api/001_derive_macros.md` | Derive macro API reference |
| doc | `component_model_types/docs/api/001_assign_trait.md` | Assign trait API reference |
| doc | `component_model_types/docs/feature/001_component_assignment.md` | Component assignment feature (types crate view) |

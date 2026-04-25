# Feature: Typed Errors

### Scope

- **Purpose**: Enable structured, named error types with compile-time field capture.
- **Responsibility**: Documents the typed error backend feature — its activation, design, and relationship to the facade.
- **In Scope**: Re-exporting the error derivation macro and providing access to the thiserror namespace via the dependency module.
- **Out of Scope**: Defining specific typed error variants, controlling display formatting, or integrating with untyped error handling.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_error_facade.md](001_error_facade.md) | Unified facade that includes this component |
| doc | [invariant/002_zero_cost_facade.md](../invariant/002_zero_cost_facade.md) | Pure re-export mandate |

### Design

**Activation:** Enabled by the `error_typed` feature flag. When the feature is absent the module is excluded entirely from compilation.

**Derive Macro Access:** The derive macro is re-exported at the crate root so consumers annotate error types with `#[derive(Error)]` using just the `error_tools` import. When the macro's procedural backend requires an explicit namespace reference, consumers must bring `thiserror` into scope via `error_tools::dependency::thiserror`.

**Upstream Library:** `thiserror` — zero-overhead derive macro crate; produces the same code as manually implementing the standard error and display traits.

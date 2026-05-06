# Feature: Typed Errors

### Scope

- **Purpose**: Enable structured, named error types with structured field capture.
- **Responsibility**: Documents the typed error backend feature — its activation, design, and relationship to the facade.
- **In Scope**: Exposing error type derivation and access to the upstream library namespace via the dependency module.
- **Out of Scope**: Defining specific typed error variants, controlling display formatting, or integrating with untyped error handling.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_error_facade.md](001_error_facade.md) | Unified facade that includes this component |
| doc | [invariant/002_zero_cost_facade.md](../invariant/002_zero_cost_facade.md) | Pass-through mandate |

### Design

**Activation:** Enabled by the `error_typed` feature flag. When the feature is absent the module is excluded entirely from compilation.

**Derive Macro Access:** The error type derivation mechanism is available at the crate root using just the `error_tools` import. When the derivation mechanism's backend requires an explicit upstream namespace reference, the upstream library is accessible via the dependency module.

**Upstream Library:** `thiserror` — zero-overhead code generation crate; produces identical output to a hand-written implementation.

# Feature: Typed Errors

### Scope

**Purpose:** Enable structured, named error types with compile-time field capture.

**Responsibility:** Re-export the derive macro and associated types from the `thiserror` library under the `error_tools` namespace, so consumers can define typed error enums without a direct `thiserror` dependency.

**In Scope:**
- Re-exporting the error derivation macro for use in consumer crates
- Providing access to `thiserror` via the `dependency` module when explicit namespace access is required

**Out of Scope:**
- Defining specific typed error variants (consumer responsibility)
- Controlling error display formatting beyond what the derive macro provides
- Integration with untyped error handling (see Error Facade feature)

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Feature | feature/001_error_facade.md | Unified facade that includes this component |
| Invariant | invariant/002_zero_cost_facade.md | Pure re-export mandate |

### Design

**Activation:** Enabled by the `error_typed` feature flag. When the feature is absent the module is excluded entirely from compilation.

**Derive Macro Access:** The derive macro is re-exported at the crate root so consumers annotate error types with `#[derive(Error)]` using just the `error_tools` import. When the macro's procedural backend requires an explicit namespace reference, consumers must bring `thiserror` into scope via `error_tools::dependency::thiserror`.

**Upstream Library:** `thiserror` — zero-overhead derive macro crate; produces the same code as manually implementing the standard error and display traits.

# Feature: Untyped Errors

### Scope

- **Purpose**: Enable lightweight, context-enriched error propagation without defining named error types.
- **Responsibility**: Documents the untyped error backend feature — its activation, re-exported surface, and relationship to the facade.
- **In Scope**: Exposing the dynamic error type, context-chaining mechanism, and error construction operations from anyhow.
- **Out of Scope**: Typed error derivation, structured error recovery or matching on specific error types, and serialization of dynamic error chains.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_error_facade.md](001_error_facade.md) | Unified facade that includes this component |
| doc | [invariant/002_zero_cost_facade.md](../invariant/002_zero_cost_facade.md) | Pass-through mandate |

### Design

**Activation:** Enabled by the `error_untyped` feature flag. When the feature is absent the module is excluded entirely from compilation.

**Re-exported Surface:**
- Error — heap-allocated, type-erased error type; carries a chain of context messages
- Result — the standard error-carrying return type, fixed to the dynamic error
- Ok — success-value constructor
- Context — context-attachment operations available on any fallible value
- anyhow macro — construct a dynamic error from a format string
- bail macro — construct and immediately return a dynamic error
- ensure macro — conditional bail; return an error if a condition is false
- format_err macro — alias for the anyhow macro; construct a dynamic error value without returning it

**Upstream Library:** `anyhow` — heap-allocated dynamic error crate; requires heap allocation support.

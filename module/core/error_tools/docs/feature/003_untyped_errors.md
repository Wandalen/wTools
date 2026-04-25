# Feature: Untyped Errors

### Scope

**Purpose:** Enable lightweight, context-enriched error propagation without defining named error types.

**Responsibility:** Re-export the dynamic error type, context-chaining trait, and error construction macros from the `anyhow` library under the `error_tools` namespace, so consumers can handle arbitrary errors without a direct `anyhow` dependency.

**In Scope:**
- Re-exporting the boxed dynamic error type and its result alias
- Re-exporting context-chaining methods for attaching human-readable messages to propagated errors
- Re-exporting error construction macros for creating ad-hoc error values inline

**Out of Scope:**
- Typed error derivation (see Typed Errors feature)
- Structured error recovery or matching on error variants
- Serialization of dynamic error chains

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Feature | feature/001_error_facade.md | Unified facade that includes this component |
| Invariant | invariant/002_zero_cost_facade.md | Pure re-export mandate |

### Design

**Activation:** Enabled by the `error_untyped` feature flag. When the feature is absent the module is excluded entirely from compilation.

**Re-exported Surface:**
- Error — heap-allocated, type-erased error type; carries a chain of context messages
- Result — type alias fixing the error type to the dynamic error; simplifies return signatures
- Ok — success-value constructor for use in closures and match arms
- Context — extension trait adding context-attachment methods to any result or option type
- anyhow macro — construct a dynamic error from a format string
- bail macro — construct and immediately return a dynamic error
- ensure macro — conditional bail; return an error if a condition is false
- format_err macro — alias for the anyhow macro; construct a dynamic error value without returning it

**Upstream Library:** `anyhow` — heap-allocated dynamic error crate; requires `alloc`.

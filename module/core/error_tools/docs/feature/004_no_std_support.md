# Feature: No-Std Support

### Scope

**Purpose:** Allow `error_tools` to operate in constrained environments without the standard library.

**Responsibility:** Provide feature flags that strip standard-library dependencies and optionally introduce heap allocation via the `alloc` crate for environments where the full standard library is unavailable.

**In Scope:**
- Declaring the `no_std` feature to signal standard-library exclusion
- Declaring the `use_alloc` feature to enable heap allocation via the `alloc` crate
- Defining the dependency relationship between `use_alloc` and `no_std`

**Out of Scope:**
- Implementing no_std-specific error types
- Providing a fully allocation-free error API
- Integration with embedded-specific runtime crates

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Feature | feature/001_error_facade.md | Unified facade that includes this component |
| Invariant | invariant/003_alloc_requirement.md | use_alloc requires no_std invariant |

### Design

**Feature Pair:** `no_std` declares the intent to exclude the standard library. `use_alloc` re-introduces heap allocation by depending on `no_std` — it cannot be activated alone. The relationship enforces that heap allocation is only used when the standard library is explicitly opted out.

**Current Limitation:** The untyped error component requires `alloc`. In practice, untyped error functionality is available in no_std environments only when `use_alloc` is also enabled.

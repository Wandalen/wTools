# Feature: No-Std Support

### Scope

- **Purpose**: Allow `error_tools` to operate in constrained environments without the standard library.
- **Responsibility**: Documents the no_std feature pair — its flags, dependency relationship, and current limitations.
- **In Scope**: Declaring the `no_std` and `use_alloc` feature flags and defining the dependency relationship between them.
- **Out of Scope**: Implementing no_std-specific error types, providing allocation-free error APIs, or integrating with embedded runtime crates.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_error_facade.md](001_error_facade.md) | Unified facade that includes this component |
| doc | [invariant/003_alloc_requirement.md](../invariant/003_alloc_requirement.md) | use_alloc requires no_std invariant |

### Design

**Feature Pair:** `no_std` declares the intent to exclude the standard library. `use_alloc` re-introduces heap allocation by depending on `no_std` — it cannot be activated alone. The relationship enforces that heap allocation is only used when the standard library is explicitly opted out.

**Current Limitation:** The untyped error component requires `alloc`. In practice, untyped error functionality is available in no_std environments only when `use_alloc` is also enabled.

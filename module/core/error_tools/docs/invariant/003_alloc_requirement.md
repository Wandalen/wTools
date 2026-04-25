# Invariant: Alloc Feature Requires No-Std

### Scope

- **Purpose**: Enforce a coherent feature flag dependency so that heap allocation in constrained environments is always paired with standard-library exclusion.
- **Responsibility**: Documents the alloc-requires-no_std invariant — its statement, build-level enforcement, and violation consequences.
- **In Scope**: The dependency rule that `use_alloc` cannot be activated without `no_std`.
- **Out of Scope**: The no_std feature itself or allocation-free error API design — see `feature/004_no_std_support.md`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/004_no_std_support.md](../feature/004_no_std_support.md) | No-std and alloc feature pair |

### Invariant Statement

The `use_alloc` feature depends on `no_std`. Activating `use_alloc` without `no_std` is a build configuration error because the feature dependency declaration enforces this relationship at the build tool level.

### Enforcement Mechanism

The build tool enforces transitive feature activation via the feature dependency declaration. Any consumer enabling `use_alloc` automatically enables `no_std`. The reverse is not enforced — `no_std` may be activated alone without `use_alloc`.

### Violation Consequences

If `use_alloc` could be activated without `no_std`, the crate would attempt to use both the standard library and heap allocation support simultaneously, producing conflicting runtime definitions depending on the target environment.

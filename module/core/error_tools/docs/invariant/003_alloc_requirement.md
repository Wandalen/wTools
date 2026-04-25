# Invariant: Alloc Feature Requires No-Std

### Scope

- **Purpose**: Enforce a coherent feature flag dependency so that heap allocation in constrained environments is always paired with standard-library exclusion.
- **Responsibility**: Documents the alloc-requires-no_std invariant — its statement, Cargo-level enforcement, and violation consequences.
- **In Scope**: The dependency rule that `use_alloc` cannot be activated without `no_std`.
- **Out of Scope**: The no_std feature itself or allocation-free error API design — see `feature/004_no_std_support.md`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/004_no_std_support.md](../feature/004_no_std_support.md) | No-std and alloc feature pair |

### Invariant Statement

The `use_alloc` feature depends on `no_std`. Activating `use_alloc` without `no_std` is a Cargo manifest error because `use_alloc = ["no_std"]` in the feature declaration enforces the dependency at the tool level.

### Enforcement Mechanism

Cargo feature dependency declaration `use_alloc = ["no_std"]` in the manifest. Cargo enforces transitive feature activation, so any consumer enabling `use_alloc` automatically enables `no_std`. The reverse is not enforced — `no_std` may be activated alone without `use_alloc`.

### Violation Consequences

If `use_alloc` could be activated without `no_std`, the crate would attempt to use both the standard library and the `alloc` crate simultaneously, producing compilation ambiguity and potentially duplicate symbol errors depending on the target platform.

# Invariant: Comparison Functions Work Across Heterogeneous Types

### Scope

- **Purpose**: Guarantee that all four comparison functions accept reference pairs of differing types without requiring a shared type bound.
- **Responsibility**: Documents the type-agnostic design contract — why it exists and what would be violated by adding type equality constraints.
- **In Scope**: same_data, same_ptr, same_size, same_region parameter type requirements.
- **Out of Scope**: Type safety of the unsafe memcmp call — that is documented separately in invariant/002.

### Invariant Statement

All four comparison functions accept two independently typed references. Neither parameter is constrained to match the type of the other. This allows comparing a slice against an array, a struct against its byte representation, or any other pair of values where the types differ but the memory relationship is meaningful.

### Enforcement Mechanism

Each function is defined with two independent type parameters — one per reference — with no constraint requiring those types to match. The standard library pointer equality function requires both references to share the same type; mem_tools deliberately does not impose this constraint.

### Violation Consequences

Adding a type equality constraint (requiring both parameters to share the same type or a common bound) would break the primary use case of comparing heterogeneous references and reduce same_ptr to a thin wrapper around the standard library pointer equality function.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/mem.rs` | Implements all four functions with independent type parameters |
| test | `tests/corner_cases_test.rs` | Cross-type comparison tests that exercise heterogeneous pairs |
| doc | [feature/001_memory_comparison.md](../feature/001_memory_comparison.md) | Memory comparison feature subject to this invariant |
| doc | [api/001_comparison_functions.md](../api/001_comparison_functions.md) | Public interface contract for all four functions |
| doc | [invariant/002_size_guarded_data_comparison.md](002_size_guarded_data_comparison.md) | Companion safety invariant for same_data byte comparison |

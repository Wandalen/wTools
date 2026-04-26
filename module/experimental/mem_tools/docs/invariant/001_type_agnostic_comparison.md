# Invariant: Comparison Functions Work Across Heterogeneous Types

### Scope

- **Purpose**: Guarantee that all four comparison functions accept reference pairs of differing types without requiring a shared type bound.
- **Responsibility**: Documents the type-agnostic design contract — why it exists and what would be violated by adding type equality constraints.
- **In Scope**: same_data, same_ptr, same_size, same_region parameter type requirements.
- **Out of Scope**: Type safety of the unsafe memcmp call — that is documented separately in source safety comments.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_memory_comparison.md](../feature/001_memory_comparison.md) | Memory comparison feature subject to this invariant |

### Invariant Statement

All four comparison functions accept two independently typed references. Neither parameter is constrained to match the type of the other. This allows comparing a slice against an array, a struct against its byte representation, or any other pair of values where the types differ but the memory relationship is meaningful.

### Enforcement Mechanism

- Source inspection: all four functions are generic over two independent type parameters T1 and T2, both with ?Sized bounds.
- The standard library std::ptr::eq requires both references to share the same type; mem_tools deliberately does not impose this constraint.

### Violation Consequences

Adding a type equality constraint (T1 = T2 or a shared trait bound requiring the same type) would break the primary use case of comparing heterogeneous references and reduce same_ptr to a thin wrapper around std::ptr::eq.

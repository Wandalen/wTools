# Feature: Memory and Pointer Comparison

### Scope

- **Purpose**: Provide safe utility functions for comparing memory addresses, sizes, and contents across heterogeneous types.
- **Responsibility**: Documents the memory comparison feature — its four functions and the type-agnostic design that distinguishes them from standard library equivalents.
- **In Scope**: same_data, same_ptr, same_size, same_region and the semantics of cross-type pointer comparison.
- **Out of Scope**: Memory allocation, deallocation, mutation, or pointer arithmetic.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_comparison_functions.md](../api/001_comparison_functions.md) | Public comparison function interface |
| doc | [invariant/001_type_agnostic_comparison.md](../invariant/001_type_agnostic_comparison.md) | Functions work across different types without requiring same type |

### Design

The memory comparison feature provides four functions for inspecting pointer identity, size equality, and data equality. All functions accept references to any two values, regardless of whether those values share a type.

The feature composes cleanly: same_region combines same_ptr and same_size, and same_data performs a bytewise comparison only after verifying same_size. The design allows comparison between a slice and an array, or any other heterogeneous reference pair, which the standard library pointer equality function does not support.

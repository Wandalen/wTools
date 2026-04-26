# API: Memory Comparison Functions

### Scope

- **Purpose**: Define the public interface for the four memory and pointer comparison functions.
- **Responsibility**: Documents the signature contract, parameter requirements, and return semantics for each comparison function.
- **In Scope**: same_data, same_ptr, same_size, same_region function signatures and behavior.
- **Out of Scope**: Internal unsafe implementation details, FFI linkage to memcmp.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_memory_comparison.md](../feature/001_memory_comparison.md) | Memory comparison feature context |

### Abstract

A set of four utility functions for comparing memory addresses, sizes, and byte contents across independently typed references. Available when the enabled feature is active. All functions accept two references of independently sized types and return a boolean.

### Operations

- same_ptr — returns true if both references point to the same memory address, regardless of type. Extends the capability of the standard library pointer equality function to heterogeneous type pairs.

- same_size — returns true if the data behind both references has the same size in bytes, as reported by the memory intrinsic size query.

- same_region — returns true if both same_ptr and same_size hold simultaneously; i.e., both references point to the same address with the same byte count.

- same_data — returns true if both references point to the same number of bytes containing identical content. Returns false immediately if same_size fails; otherwise performs a bytewise memory comparison.

All functions are exported from the crate's orphan namespace and are accessible without the prelude.

### Error Handling

All four functions are infallible — they return a boolean and never panic, propagate errors, or invoke undefined behavior. same_data uses an unsafe FFI comparison internally but the function's public contract is fully safe; the safety invariants are documented in the source implementation.

### Compatibility Guarantees

All four function names and their parameter contract (two independently typed references, boolean return) are stable across minor versions. The enabled feature name is permanent. The same_region composition guarantee (same_ptr AND same_size) is stable.

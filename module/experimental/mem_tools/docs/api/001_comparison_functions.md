# API: Memory Comparison Functions

### Scope

- **Purpose**: Define the public interface for the four memory and pointer comparison functions.
- **Responsibility**: Documents the signature contract, parameter requirements, and return semantics for each comparison function.
- **In Scope**: same_data, same_ptr, same_size, same_region function signatures and behavior.
- **Out of Scope**: Internal unsafe implementation details, FFI linkage to memcmp.

### Abstract

A set of four utility functions for comparing memory addresses, sizes, and byte contents across independently typed references. Available when the enabled feature is active. All functions accept two references of independently typed, potentially unsized values and return a boolean. The functions are composable: same_region is defined as same_ptr AND same_size; same_data short-circuits on same_size before performing byte comparison.

### Operations

- same_ptr — returns true if both references point to the same memory address, regardless of type. Accepts two references with independent type parameters and no constraint requiring those types to match, extending pointer equality to heterogeneous pairs where the standard library function would require a shared type.

- same_size — returns true if the data behind both references occupies the same number of bytes, as reported by the memory intrinsic size query. Works across DSTs (slices, trait objects) because size is measured at runtime from the fat pointer metadata, not from a compile-time type size.

- same_region — returns true if both same_ptr and same_size hold simultaneously. Callers use this when they need to confirm that two references are not merely aliases but identical fat-pointer handles covering the exact same memory region.

- same_data — returns true if both references point to the same number of bytes containing identical content. Returns false immediately if same_size returns false; otherwise performs a bytewise memory comparison using the platform's system library comparison function. The function is fully safe despite the internal unsafe call — the size pre-check makes out-of-bounds access structurally impossible.

All functions are exported from the crate's orphan namespace and are accessible without the prelude. All four functions are available only when the enabled feature is active.

### Error Handling

All four functions are infallible — they return a boolean and never panic, propagate errors, or invoke undefined behavior. same_data uses an unsafe FFI comparison internally but the function's public contract is fully safe; the safety invariants are documented in the source implementation and in invariant/002.

### Compatibility Guarantees

All four function names and their parameter contract (two independently typed references, boolean return) are stable across minor versions. The enabled feature name is permanent. The same_region composition guarantee (same_ptr AND same_size) is stable. The same_data size-pre-check behavior (false when sizes differ, no byte comparison performed) is stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/mem.rs` | Implementation of all four comparison functions |
| source | `src/lib.rs` | Namespace re-export hub exposing the public API |
| config | `Cargo.toml` | enabled feature gate that activates these functions |
| test | `tests/mem_tools_tests.rs` | Primary test aggregator using the_module alias pattern |
| test | `tests/smoke_test.rs` | Published and local smoke tests |
| doc | [feature/001_memory_comparison.md](../feature/001_memory_comparison.md) | Memory comparison feature context and design rationale |
| doc | [invariant/001_type_agnostic_comparison.md](../invariant/001_type_agnostic_comparison.md) | Type-agnostic parameter contract for all four functions |
| doc | [invariant/002_size_guarded_data_comparison.md](../invariant/002_size_guarded_data_comparison.md) | Size-guarded safety contract for same_data |

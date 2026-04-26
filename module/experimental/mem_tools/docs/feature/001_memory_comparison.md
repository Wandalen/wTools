# Feature: Memory and Pointer Comparison

### Scope

- **Purpose**: Provide type-agnostic utility functions for comparing memory addresses, sizes, and byte contents across independently typed references.
- **Responsibility**: Documents the memory comparison feature — its four comparison modes, cross-type design rationale, safety model, and zero-dependency constraint that distinguish it from standard library alternatives.
- **In Scope**: Four comparison functions (address identity, size equality, combined region, byte-content); cross-type reference pairs; no_std support; zero production dependencies; size-guarded safety model for byte comparison.
- **Out of Scope**: Memory allocation, copying, initialization, smart pointers, alignment utilities, type conversions, memory layout inspection, and deep structural equality — all served by standard library or dedicated ecosystem crates.

### Design

The crate provides four functions that partition comparison into distinct, composable modes: address identity, size equality, the conjunction of the two, and byte-content equality. Each function pays only for what it tests; callers can combine them as needed.

All functions accept references to any two values regardless of whether those values share a type. This extends comparison to heterogeneous pairs — a slice against a byte array, a struct against a reinterpreted region — which the standard library pointer equality function does not support because it requires both arguments to share the same type.

The byte-content comparison uses a well-tested system library comparison function that benefits from platform-optimized implementation on all major architectures. Safety is maintained by always validating that both regions are the same size before performing the comparison; same_data returns false immediately when sizes differ, avoiding any out-of-bounds read.

The crate has zero production dependencies: all comparison logic uses only core language intrinsics and a single C standard library function available on every platform. This makes the crate usable in no_std and embedded contexts without pulling in any external code.

**Boundaries**:
- vs standard library pointer equality: standard library requires both references to share the same type; mem_tools removes that constraint
- vs compile-time memory assertion utilities (e.g., diagnostics_tools): those check layout properties at compile time; mem_tools performs runtime value comparisons
- vs raw C function bindings (e.g., libc): those expose unsafe raw interfaces; mem_tools wraps them with a fully safe public contract

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/mem.rs` | Implementation of all four comparison functions |
| source | `src/lib.rs` | Namespace re-export hub exposing the public API |
| test | `tests/mem_tools_tests.rs` | Primary test aggregator using the_module alias pattern |
| test | `tests/smoke_test.rs` | Published and local smoke tests |
| test | `tests/corner_cases_test.rs` | Cross-type, empty-value, and edge-case coverage |
| doc | [api/001_comparison_functions.md](../api/001_comparison_functions.md) | Public interface contract for all four functions |
| doc | [invariant/001_type_agnostic_comparison.md](../invariant/001_type_agnostic_comparison.md) | Type-agnostic parameter contract for all four functions |
| doc | [invariant/002_size_guarded_data_comparison.md](../invariant/002_size_guarded_data_comparison.md) | Size-guarded safety contract for same_data |

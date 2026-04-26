# Feature: Memory Layout Assertions

### Scope

- **Purpose**: Provide compile-time assertions for verifying memory size and alignment properties across types and values.
- **Responsibility**: Documents the memory layout assertions feature — its four macros and their compile-time verification behavior.
- **In Scope**: The cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size macros.
- **Out of Scope**: Runtime memory inspection, address-level pointer comparison, allocation behavior.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/003_memory_layout_macros.md](../api/003_memory_layout_macros.md) | Public memory layout assertion macro set |
| doc | [invariant/003_compiletime_zero_overhead.md](../invariant/003_compiletime_zero_overhead.md) | Compile-time assertions introduce no runtime overhead |

### Design

The memory layout assertions feature provides macros for verifying memory size and alignment properties at compile time. All assertions are evaluated by the compiler — no runtime cost is incurred and no code is generated in the output binary.

The feature offers two macro pairs. Type-level macros (cta_type_same_size, cta_type_same_align) accept type parameters and compare their compile-time size or alignment. Value-level macros (cta_ptr_same_size, cta_mem_same_size) accept expressions and compare the size of the data they reference.

This feature is gated by the diagnostics_memory_layout feature flag.

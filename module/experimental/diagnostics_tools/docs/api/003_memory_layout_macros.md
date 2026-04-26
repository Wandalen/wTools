# API: Memory Layout Assertion Macros

### Scope

- **Purpose**: Define the public memory layout assertion macro set exposed by the diagnostics_memory_layout feature.
- **Responsibility**: Documents the interface contract for all memory layout macros — their parameter forms and compile-error behavior.
- **In Scope**: cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size.
- **Out of Scope**: Runtime memory comparison, pointer identity, compile-time condition assertions.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/003_memory_layout_assertions.md](../feature/003_memory_layout_assertions.md) | Memory layout assertions feature context |

### Abstract

A macro set for verifying memory size and alignment properties at compile time. Available when the diagnostics_memory_layout feature is enabled. All assertions produce compile errors on failure; no runtime code is emitted.

### Operations

**Type-level macros** — accept two type parameters:

- cta_type_same_size — asserts at compile time that two types have identical size in bytes.
- cta_type_same_align — asserts at compile time that two types have identical alignment.

**Value-level macros** — accept two expression parameters:

- cta_ptr_same_size — asserts at compile time that the data behind two references has the same size.
- cta_mem_same_size — asserts at compile time that two values have the same size; delegates to cta_ptr_same_size by taking references automatically.

All macros evaluate entirely at compile time. All are exported at the crate root and available in the prelude when the feature is enabled.

### Error Handling

Assertion failure produces a compile_error with a message identifying the compared types or expressions. This is a compile-time failure — no runtime error or panic occurs. The macros cannot fail in any runtime sense.

### Compatibility Guarantees

All four macros are stable across minor versions. The type-level and value-level groupings are permanent. The diagnostics_memory_layout feature name is permanent. The compile_error message format is not guaranteed stable across major versions.

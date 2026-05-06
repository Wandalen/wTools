# Invariant: Compile-Time Assertions Introduce No Runtime Overhead

### Scope

- **Purpose**: Guarantee that compile-time and memory layout assertion macros add zero cost to compiled binaries.
- **Responsibility**: Documents the zero-overhead contract for cta_true, cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size.
- **In Scope**: All macros gated by diagnostics_compiletime_assertions and diagnostics_memory_layout feature flags.
- **Out of Scope**: Runtime assertion macros — those intentionally execute at runtime.

### Invariant Statement

All compile-time assertion macros (cta_true, cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size) evaluate entirely during compilation. They produce either a compile error (on failure) or nothing (on success). No instructions are emitted into the compiled binary.

### Enforcement Mechanism

- cta_true uses cfg(not(...)) and compile_error!, which are pure compiler directives with no codegen.
- Memory layout macros use const fn closures that are immediately discarded after type-checking; the compiler never emits them into the binary.

### Violation Consequences

Any change that causes these macros to emit runtime instructions would impose unexpected binary-size and execution costs on consumers who chose them specifically for their zero-overhead guarantee.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/002_compiletime_assertions.md](../feature/002_compiletime_assertions.md) | Compile-time assertions feature subject to this invariant |
| doc | [feature/003_memory_layout_assertions.md](../feature/003_memory_layout_assertions.md) | Memory layout assertions feature subject to this invariant |
| source | [src/diag/cta.rs](../../src/diag/cta.rs) | cta_true implementation that must emit no runtime instructions |
| source | [src/diag/layout.rs](../../src/diag/layout.rs) | Memory layout macro implementations that must emit no runtime instructions |
| test | [tests/inc/cta_test.rs](../../tests/inc/cta_test.rs) | Tests for compile-time assertion macro behavior |
| test | [tests/inc/layout_test.rs](../../tests/inc/layout_test.rs) | Tests for memory layout assertion macro behavior |

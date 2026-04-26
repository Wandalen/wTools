# Feature: Compile-Time Assertions

### Scope

- **Purpose**: Enable cfg condition validation that fails the build when the condition does not hold.
- **Responsibility**: Documents the compile-time assertions feature — the cta_true macro and its behavior.
- **In Scope**: The cta_true macro and the diagnostics_compiletime_assertions feature flag.
- **Out of Scope**: Runtime assertion behavior, memory layout validation.

### Design

The compile-time assertions feature provides a macro for asserting cfg conditions at compile time. When the asserted condition does not hold, the macro emits a compile_error with a message that includes the condition text, identifying the exact predicate that failed.

This feature is gated by the diagnostics_compiletime_assertions feature flag. All evaluation happens during compilation — the feature introduces no runtime overhead and produces no generated binary code.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/002_compiletime_assertion_macros.md](../api/002_compiletime_assertion_macros.md) | Public compile-time assertion macro set |
| doc | [invariant/003_compiletime_zero_overhead.md](../invariant/003_compiletime_zero_overhead.md) | Compile-time assertions introduce no runtime overhead |
| source | [src/diag/cta.rs](../../src/diag/cta.rs) | Compile-time assertion macro implementations |
| test | [tests/inc/cta_test.rs](../../tests/inc/cta_test.rs) | Behavioral tests for compile-time assertion macros |

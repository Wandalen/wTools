# API: Compile-Time Assertion Macros

### Scope

- **Purpose**: Define the public compile-time assertion macro set exposed by the diagnostics_compiletime_assertions feature.
- **Responsibility**: Documents the interface contract for cta_true — its invocation forms and compile-error behavior.
- **In Scope**: The cta_true macro and its two invocation forms (with and without a custom message).
- **Out of Scope**: Memory layout assertions, runtime assertion behavior.

### Abstract

A macro set for asserting cfg conditions at compile time. Available when the diagnostics_compiletime_assertions feature is enabled. All evaluation happens during compilation — no runtime code is generated.

### Operations

- cta_true — asserts that a cfg predicate holds at compile time.

  Two invocation forms are supported. With a condition alone, the generated compile error includes the stringified condition text. With a condition and a message expression, the generated compile error uses the provided message.

  An empty invocation (no arguments) is a no-op.

All macros are exported at the crate root and available in the prelude.

### Error Handling

Assertion failure produces a compile_error with a message identifying the failed predicate. This is a compile-time failure — the build is rejected; no runtime error or panic occurs. The macro cannot fail in any runtime sense.

### Compatibility Guarantees

The cta_true macro and its two invocation forms are stable across minor versions. The diagnostics_compiletime_assertions feature name is permanent. The compile_error message format (including stringified condition text) is not guaranteed stable across major versions.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/002_compiletime_assertions.md](../feature/002_compiletime_assertions.md) | Compile-time assertions feature context |
| source | [src/diag/cta.rs](../../src/diag/cta.rs) | Compile-time assertion macro implementations |
| test | [tests/inc/cta_test.rs](../../tests/inc/cta_test.rs) | Behavioral tests for compile-time assertion macros |

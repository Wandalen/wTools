# API: AsyncFrom

### Scope

- **Purpose**: Provide the primary async conversion trait for infallible conversions from a source type to the implementing type.
- **Responsibility**: Defines the AsyncFrom trait contract — its single required method, thread-safety context, and its relationship to AsyncInto.
- **In Scope**: AsyncFrom trait, the async conversion method, the async_trait attribution requirement, and feature flag `async_from`.
- **Out of Scope**: Blanket implementation details (see `docs/api/002_async_into.md`) and fallible conversions (see `docs/api/003_async_try_from.md`).

### Abstract

AsyncFrom is a single-method trait for asynchronous, infallible type conversion. The implementor receives a source value and returns an instance of the implementing type via an async function. The async_trait macro bridges the async method into a boxed future, satisfying object-safety and trait bounds across executors.

### Operations

`async_from` — Consumes the source value and produces an instance of the implementing type. The operation is infallible; panic or process termination are the only failure modes. The implementor is responsible for all awaiting performed within the method body.

### Error Handling

No error handling — this trait is infallible by design. Any operation that may fail must use AsyncTryFrom instead.

### Compatibility Guarantees

Gated behind the `async_from` feature flag. Implementations require the implementing type to have a statically known size. The async_trait macro is required for all implementations; this is a compile-time constraint enforced by the trait definition.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | AsyncFrom trait definition |
| test | `tests/inc/basic_test.rs` | Integration tests for AsyncFrom |
| test | `tests/manual_corner_cases_test.rs` | AsyncFrom edge cases (empty, whitespace, boundaries, overflow) |
| test | `tests/additional_corner_cases_test.rs` | AsyncFrom corner cases (zero, max, negative, overflow, format parsing) |
| doc | `docs/feature/001_infallible_async_conversion.md` | Feature scope and design rationale |
| doc | `docs/api/002_async_into.md` | AsyncInto blanket derived from this trait |
| doc | `docs/invariant/001_send_bounds.md` | Thread-safety constraint on blanket impl |
| doc | `docs/pattern/001_std_mirror_pattern.md` | Std mirror design pattern |

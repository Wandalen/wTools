# API: AsyncTryFrom

### Scope

- **Purpose**: Provide the primary async conversion trait for fallible conversions from a source type to the implementing type.
- **Responsibility**: Defines the AsyncTryFrom trait contract — its associated error type, the single required method, and the result-based return.
- **In Scope**: AsyncTryFrom trait, the associated error type, the async fallible conversion method, and feature flag `async_try_from`.
- **Out of Scope**: Blanket implementation details (see `docs/api/004_async_try_into.md`) and infallible conversions (see `docs/api/001_async_from.md`).

### Abstract

AsyncTryFrom is the fallible counterpart to AsyncFrom. Implementations define an associated error type that supports diagnostic output, and return a value on success or that error on failure. The pattern mirrors the standard TryFrom trait with async execution substituted for synchronous execution.

### Operations

`async_try_from` — Attempts to convert a source value into an instance of the implementing type. Returns the converted value on success or the associated error on failure. The implementor chooses the error type; it must support diagnostic output.

### Error Handling

Returns either the converted value or the implementor-defined error. The diagnostic output requirement on the error type enables debugging. Callers use standard result combinators after awaiting.

### Compatibility Guarantees

Gated behind the `async_try_from` feature flag, which is independent of `async_from`. Requires the implementing type to have a statically known size. The async_trait macro is required for all implementations.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | AsyncTryFrom trait definition |
| test | `tests/inc/basic_test.rs` | Integration tests for AsyncTryFrom |
| test | `tests/manual_corner_cases_test.rs` | Edge cases and error propagation |
| test | `tests/additional_corner_cases_test.rs` | AsyncTryFrom corner cases (leading zeros, spaces, floats, unicode, hex) |
| doc | `docs/feature/002_fallible_async_conversion.md` | Feature scope and design rationale |
| doc | `docs/api/001_async_from.md` | Infallible async conversion counterpart |
| doc | `docs/api/004_async_try_into.md` | AsyncTryInto blanket derived from this trait |
| doc | `docs/invariant/001_send_bounds.md` | Thread-safety constraint on blanket impl |
| doc | `docs/pattern/001_std_mirror_pattern.md` | Std mirror design pattern |

# Feature: Fallible Async Conversion

### Scope

- **Purpose**: Enable type conversions that require async operations and may fail, with explicit error propagation.
- **Responsibility**: Documents the AsyncTryFrom and AsyncTryInto traits — their design rationale, scope, and cross-references to all related artifacts.
- **In Scope**: AsyncTryFrom trait definition, AsyncTryInto blanket implementation, feature flag `async_try_from`, associated error type, and usage patterns.
- **Out of Scope**: Infallible conversions (see `docs/feature/001_infallible_async_conversion.md`) and async runtime configuration.

### Design

AsyncTryFrom mirrors the standard TryFrom trait, extended to async contexts. Implementors define an async fallible conversion method that returns a value on success or an associated error on failure; the error type must support diagnostic output. AsyncTryInto derives from AsyncTryFrom via a blanket implementation, maintaining the same TryFrom/TryInto relationship as the standard library.

The feature flag `async_try_from` is independent of `async_from` — a crate may enable one without the other, allowing callers to choose only the conversion model that fits their domain.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | AsyncTryFrom and AsyncTryInto trait definitions and blanket impl |
| test | `tests/inc/basic_test.rs` | Integration tests for AsyncTryFrom and AsyncTryInto |
| test | `tests/additional_corner_cases_test.rs` | Readme examples and fallible conversion corner cases |
| test | `tests/manual_corner_cases_test.rs` | Edge cases including parse error propagation |
| test | `tests/send_bounds_validation_test.rs` | Thread-safety bounds for blanket impls in multi-threaded runtimes |
| doc | `docs/feature/001_infallible_async_conversion.md` | Complementary infallible conversion feature |
| doc | `docs/api/003_async_try_from.md` | AsyncTryFrom public API contract |
| doc | `docs/api/004_async_try_into.md` | AsyncTryInto public API contract |
| doc | `docs/invariant/001_send_bounds.md` | Thread-safety requirement on async trait methods |
| doc | `docs/pattern/001_std_mirror_pattern.md` | Std TryFrom/TryInto mirror design pattern |

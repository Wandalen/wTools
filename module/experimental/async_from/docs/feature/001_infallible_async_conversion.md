# Feature: Infallible Async Conversion

### Scope

- **Purpose**: Enable type conversions that require async operations without failure handling overhead.
- **Responsibility**: Documents the AsyncFrom and AsyncInto traits — their design rationale, scope, and cross-references to all related artifacts.
- **In Scope**: AsyncFrom trait definition, AsyncInto blanket implementation, feature flag `async_from`, and usage patterns.
- **Out of Scope**: Fallible conversions (see `docs/feature/002_fallible_async_conversion.md`), error handling, and async runtime specifics.

### Design

AsyncFrom mirrors the standard From trait, extended to async contexts via the async_trait macro. Implementors define an async conversion method that receives a source value and returns the implementing type. AsyncInto is a blanket implementation derived from AsyncFrom — no manual implementation required; it follows the same From/Into relationship as the standard library.

The trait pair targets conversions that are infallible but require async work — network lookups, database queries, IO-bound initialization. The feature flag `async_from` gates both traits, allowing downstream crates to opt in to only infallible conversions without pulling in error-handling types.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | AsyncFrom and AsyncInto trait definitions and blanket impl |
| test | `tests/inc/basic_test.rs` | Integration tests for AsyncFrom and AsyncInto |
| test | `tests/additional_corner_cases_test.rs` | Readme examples and infallible conversion corner cases |
| test | `tests/manual_corner_cases_test.rs` | Edge cases for AsyncFrom and AsyncInto (boundaries, overflow) |
| test | `tests/send_bounds_validation_test.rs` | Thread-safety bounds for blanket impls in multi-threaded runtimes |
| doc | `docs/feature/002_fallible_async_conversion.md` | Complementary fallible conversion feature |
| doc | `docs/api/001_async_from.md` | AsyncFrom public API contract |
| doc | `docs/api/002_async_into.md` | AsyncInto public API contract |
| doc | `docs/invariant/001_send_bounds.md` | Thread-safety requirement on async trait methods |
| doc | `docs/pattern/001_std_mirror_pattern.md` | Std From/Into mirror design pattern |

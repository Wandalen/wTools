# API: Debug Assertion Macros

### Scope

- **Purpose**: Provide identity and non-identity assertion macros that are stripped from release builds.
- **Responsibility**: Documents the debug assertion macro API — its four forms, error behavior, and compatibility guarantees.
- **In Scope**: The four macro forms for equality and inequality assertion, active only under debug assertions.
- **Out of Scope**: Release-build assertions, assertions on ordering or containment, and panic-free assertion variants.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_error_facade.md](../feature/001_error_facade.md) | Facade that exposes this API surface |

### Abstract

Four debug-only macros that assert equality or inequality between two expressions. All four are compiled to zero in release builds — no binary cost. The short-form and canonical-alias pairs exist to serve distinct readability conventions at different call sites.

### Operations

**debug_assert_id** — asserts that two expressions are equal. Compiled to the standard equality assertion under debug assertions; the macro body is elided entirely in release builds.

**debug_assert_identical** — canonical alias of debug_assert_id. Delegates directly to debug_assert_id when debug assertions are active. Use when the "identical" terminology better matches the calling context.

**debug_assert_ni** — asserts that two expressions are not equal. Compiled to the standard inequality assertion under debug assertions; elided in release builds.

**debug_assert_not_identical** — canonical alias of debug_assert_ni. Delegates directly to debug_assert_ni when debug assertions are active.

### Error Handling

All macros panic with a standard equality-failure message when the assertion fails in a debug build. They produce no output and take no action in release builds. The panic message follows the same format as the standard assertion macros.

### Compatibility Guarantees

The four macro names are stable. The two aliases (debug_assert_identical, debug_assert_not_identical) are permanent rather than deprecated — they serve distinct readability purposes at different call sites.

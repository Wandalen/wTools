# API: Debug Assertion Macros

### Scope

**Purpose:** Provide identity and non-identity assertion macros that are stripped from release builds.

**Responsibility:** Expose four macro forms for asserting equality and inequality between two expressions, active only when debug assertions are enabled, so they impose no cost in release builds.

**In Scope:**
- debug_assert_id — assert two expressions are equal in debug builds
- debug_assert_identical — canonical alias of debug_assert_id
- debug_assert_ni — assert two expressions are not equal in debug builds
- debug_assert_not_identical — canonical alias of debug_assert_ni

**Out of Scope:**
- Assertions that remain active in release builds (use the standard assert_eq and assert_ne instead)
- Assertions on ordering, containment, or custom predicates
- Panic-free assertion variants

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Feature | feature/001_error_facade.md | Facade that exposes this API surface |

### Abstract

**debug_assert_id**

Asserts that two expressions are equal. Compiled to the standard equality assertion under debug assertions; the macro body is elided entirely in release builds. Accepts any argument pattern accepted by the standard equality assertion macro.

**debug_assert_identical**

Alias of debug_assert_id. Delegates directly to debug_assert_id when debug assertions are active. Use when the "identical" terminology better matches the calling context.

**debug_assert_ni**

Asserts that two expressions are not equal. Compiled to the standard inequality assertion under debug assertions; elided in release builds.

**debug_assert_not_identical**

Alias of debug_assert_ni. Delegates directly to debug_assert_ni when debug assertions are active.

### Error Handling

All macros panic with a standard equality-failure message when the assertion fails in a debug build. They produce no output and take no action in release builds.

### Compatibility Guarantees

The four macro names are stable. The two aliases (debug_assert_identical, debug_assert_not_identical) are permanent rather than deprecated — they serve distinct readability purposes at different call sites.

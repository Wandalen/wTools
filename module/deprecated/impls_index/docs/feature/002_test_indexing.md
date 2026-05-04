# Feature: Test Indexing

### Scope

- **Purpose**: Provide structured test organization by automatically injecting the test attribute into indexed test functions.
- **Responsibility**: Document the test-specific indexing capability — `tests_impls!`, `tests_impls_optional!`, and `tests_index!`.
- **In Scope**: Test attribute injection at macro expansion time, optional vs strict test indexing, test index invocation.
- **Out of Scope**: Generic function indexing without test semantics (→ `feature/001`).

### Design

Test indexing mirrors function indexing but adds the test attribute to each wrapped function during expansion. A test function indexed with `tests_impls!` is not registered as a test until `tests_index!` invokes it — the lazy expansion pattern applies here exactly as in generic function indexing.

| Variant | Macro | Characteristic |
|---------|-------|----------------|
| Strict | `tests_impls!` | Wraps functions in macros with test attribute; unused macro is a compile error |
| Optional | `tests_impls_optional!` | Same as strict; unused macro permitted |
| Invocation | `tests_index!` | Alias for `index!`; expands test macros into `#[test]` functions |

The test attribute injection happens at macro expansion time, not at definition time. This means a function indexed with `tests_impls!` but never invoked through `tests_index!` never becomes a test function — it remains a lazy macro definition.

This design allows selectively materializing only the test functions needed in a given scope, with unused indexed test macros caught by the compiler under the strict variant.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implsindex/impls.rs` | Implements `tests_impls!`, `tests_impls_optional!`, `tests_index!` alias |
| test | `tests/inc/tests_index_test.rs` | Validates `tests_index!` invocation syntax variants |
| test | `tests/inc/impls_basic_test.rs` | Validates cross-variant basic test indexing behavior |
| doc | `docs/api/001_indexing_macros.md` | API contract for `tests_impls!` and `tests_impls_optional!` |
| doc | `docs/api/002_invocation_macros.md` | API contract for `tests_index!` |
| doc | `docs/invariant/001_unused_macro_enforcement.md` | Enforcement invariant — applies to `tests_impls!` strict variant |

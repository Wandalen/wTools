# Feature: Function Indexing

### Scope

- **Purpose**: Enable separation of function structure from implementation by wrapping functions in named macros, making the code index explicit.
- **Responsibility**: Document the core function indexing capability — all impls variants and `index!` invocation.
- **In Scope**: Wrapping functions in named macros, invoking indexed functions, renaming during invocation, strict vs optional enforcement.
- **Out of Scope**: Test-specific indexing with auto test-attribute injection (→ `feature/002`), function definition utilities (→ `feature/003`).

### Design

Function indexing operates in two phases. In the definition phase, functions are wrapped in named macros using an `impls` variant — the functions are not yet generated. In the invocation phase, `index!` lists the functions to materialize, expanding each macro into an actual function definition.

This separation makes the module structure explicit: a reader scanning the `index!` call sees the complete list of functions without reading their bodies.

Three indexing variants are available, differing in implementation and capability:

| Variant | Macro | Characteristic |
|---------|-------|----------------|
| Basic | `impls1!` | Simplest variant; single-step wrapping |
| Callback | `impls2!` | Alternative internal mechanism; same output as Basic variant |
| Advanced | `impls3!` / `impls!` | Supports `as NewName` rename at invocation time |

The `index!` macro accepts a comma-separated list of function names. Each entry can optionally rename the function using `Name as Alias` syntax — but rename support requires the function to have been indexed with `impls3!`.

Two enforcement modes:
- **Strict** (`impls1!`, `impls2!`, `impls3!`): unused indexed macros are a compile error.
- **Optional** (`impls_optional!`): unused indexed macros are permitted — for conditional compilation scenarios.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implsindex/impls.rs` | Implements `index!`, `impls1!`, `impls2!`, `impls_optional!` |
| source | `src/implsindex/mod.rs` | Module aggregation and namespace re-export |
| test | `tests/inc/impls1_test.rs` | Validates `impls1!` basic cases and visibility preservation |
| test | `tests/inc/impls2_test.rs` | Validates `impls2!` callback-based expansion |
| test | `tests/inc/impls3_test.rs` | Validates `impls3!` with renaming and alias invocation |
| test | `tests/inc/impls_optional_test.rs` | Validates optional indexing produces no unused-macro errors |
| test | `tests/inc/index_test.rs` | Validates `index!` syntax variants (empty, comma, rename) |
| doc | `docs/api/001_indexing_macros.md` | API contract for all function-wrapping macros |
| doc | `docs/api/002_invocation_macros.md` | API contract for `index!` invocation |
| doc | `docs/invariant/001_unused_macro_enforcement.md` | Enforcement invariant for strict indexing macros |

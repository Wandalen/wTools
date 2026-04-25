# Feature: Function Utilities

### Scope

- **Purpose**: Provide token-tree-level function manipulation for advanced macro composition and internal indexing machinery.
- **Responsibility**: Document the function utility toolkit — `fn_name!`, `fn_rename!`, `fns!`, `fns2!`.
- **In Scope**: Extracting function names, renaming functions by token substitution, iterating multiple function definitions via callback dispatch.
- **Out of Scope**: High-level indexing features (→ `feature/001`), test-specific indexing (→ `feature/002`).

### Design

These utilities expose the token-tree parsing machinery underlying `impls2!`. They operate directly on function definitions as raw token sequences, enabling external macro authors to build higher-level abstractions without reimplementing the parsing layer.

| Utility | Macro | Operation |
|---------|-------|-----------|
| Name extraction | `fn_name!` | Extract the identifier from a function definition token tree |
| Renaming | `fn_rename!` | Substitute the name in a function definition while preserving all other tokens |
| Multi-function iterator | `fns!` | Split multiple function items and dispatch each to a `@Callback` macro |
| Simple iterator | `fns2!` | Item-based alternative to `fns!` with simpler parsing |

`fns!` handles the full complexity of function syntax: visibility modifiers, generics, where clauses, parameter lists, return types, and bodies. Each function is passed individually to the callback macro, enabling per-function transformation.

`fns2!` uses the simpler `$item` matcher, which is less capable with complex generics but has lower parsing overhead for straightforward cases.

Both iterator macros use the `@Callback { path }` argument convention, accepting any macro path that takes a single function item as its argument. `impls2!` uses `$crate::_impls_callback` as its callback.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implsindex/func.rs` | Implements `fn_name!`, `fn_rename!`, `fns!`, `fns2!` |
| source | `src/implsindex/impls.rs` | Uses `fns!` internally via `impls2!` and `_impls_callback` |
| test | `tests/inc/func_test.rs` | Validates all function utility macros and edge cases |
| doc | `docs/api/003_utility_macros.md` | API contract for the function utility macros |

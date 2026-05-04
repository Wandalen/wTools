# Feature: Function Utilities

### Scope

- **Purpose**: Provide low-level function definition manipulation for advanced macro composition and internal indexing machinery.
- **Responsibility**: Document the function utility toolkit — `fn_name!`, `fn_rename!`, `fns!`, `fns2!`.
- **In Scope**: Extracting function names, renaming functions, iterating multiple function definitions via callback dispatch.
- **Out of Scope**: High-level indexing features (→ `feature/001`), test-specific indexing (→ `feature/002`).

### Design

These utilities expose the function definition parsing capabilities used internally by the indexing macros. They operate on function definitions directly, enabling external macro authors to build higher-level abstractions without reimplementing the parsing layer.

| Utility | Macro | Operation |
|---------|-------|-----------|
| Name extraction | `fn_name!` | Extract the identifier from a function definition |
| Renaming | `fn_rename!` | Substitute the name in a function definition while preserving all other tokens |
| Multi-function iterator | `fns!` | Split multiple function items and dispatch each to a callback macro |
| Simple iterator | `fns2!` | Item-based alternative to `fns!` with simpler parsing |

`fns!` handles the full complexity of function syntax: visibility modifiers, generics, where clauses, parameter lists, return types, and bodies. Each function is passed individually to the callback macro, enabling per-function transformation.

`fns2!` uses item-level matching, which is less capable with complex generics but has lower overhead for straightforward cases.

Both iterator macros use a callback dispatch convention, accepting any macro path that takes a single function item as its argument. `impls2!` uses an internal callback macro registered in the crate namespace as its dispatch target.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implsindex/func.rs` | Implements `fn_name!`, `fn_rename!`, `fns!`, `fns2!` |
| source | `src/implsindex/impls.rs` | Consumes `fns!` in the indexing macro expansion path |
| test | `tests/inc/func_test.rs` | Validates all function utility macros and edge cases |
| doc | `docs/api/003_utility_macros.md` | API contract for the function utility macros |

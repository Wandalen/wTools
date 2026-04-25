# API: Utility Macros

### Scope

- **Purpose**: Define the API contract for low-level function token-tree manipulation macros.
- **Responsibility**: Document operations, error handling, and compatibility for `fn_name!`, `fn_rename!`, `fns!`, `fns2!`.
- **In Scope**: Function identifier extraction, function renaming by token substitution, multi-function callback dispatch.
- **Out of Scope**: High-level function indexing (→ `api/001`), index invocation (→ `api/002`).

### Abstract

These utilities expose token-tree parsing machinery for function definitions. They enable composition of higher-level macros without reimplementing function syntax parsing. They are used internally by `impls2!` and are available as part of the public API for external macro authors.

### Operations

- **`fn_name!`** — Accepts a function definition token tree. Returns the function identifier as a token. Scans for the `fn` keyword, then captures the identifier that follows. Handles any leading tokens before `fn` by recursion.
- **`fn_rename!`** — Accepts three arguments: `@Prefix { … }` (tokens before `fn`), `@Name { ident }` (new name), `@Postfix { fn OldName … }` (the original function from `fn` onward). Returns the function with the name replaced while preserving all other tokens.
- **`fns!`** — Accepts `@Callback { path }` and a sequence of function definitions. For each function, invokes the callback macro passing the single function as its argument. Handles visibility, generics, where clauses, parameter lists, return types, and function bodies.
- **`fns2!`** — Accepts `@Callback { path }` and a sequence of `$item` token trees. Simpler than `fns!` — delegates each item directly to the callback without structured parsing. Lower overhead for simple cases.

### Error Handling

- `fn_name!` applied to input containing no `fn` keyword exhausts its token recursion and produces a compile error.
- `fn_rename!` requires a syntactically complete function in the `@Postfix` argument; incomplete input produces a compile error.
- `fns!` requires each function to be syntactically complete and the callback path to resolve to an accessible macro; violations produce compile errors.
- `fns2!` passes items verbatim; input validity depends on what the callback macro accepts.

### Compatibility Guarantees

- All utility macros are part of the public API and exported through the `exposed` namespace.
- The `@Callback { path }` convention used by `fns!` and `fns2!` is stable.
- Token-level behavior of `fn_rename!` and `fn_name!` may be affected by future changes to Rust macro hygiene rules, but the observable output for standard function syntax is stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implsindex/func.rs` | Implements `fn_name!`, `fn_rename!`, `fns!`, `fns2!` |
| source | `src/implsindex/impls.rs` | Uses `fns!` internally via `impls2!` with `_impls_callback` |
| test | `tests/inc/func_test.rs` | Validates all utility macro operations |
| doc | `docs/feature/003_function_utilities.md` | User-facing function utilities feature |
| doc | `docs/api/001_indexing_macros.md` | Indexing macros that consume `fns!` internally via `impls2!` |

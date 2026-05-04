# API: Utility Macros

### Scope

- **Purpose**: Define the API contract for low-level function definition manipulation macros.
- **Responsibility**: Document operations, error handling, and compatibility for `fn_name!`, `fn_rename!`, `fns!`, `fns2!`.
- **In Scope**: Function identifier extraction, function renaming, multi-function callback dispatch.
- **Out of Scope**: High-level function indexing (→ `api/001`), index invocation (→ `api/002`).

### Abstract

These utilities expose function definition parsing capabilities used by the indexing macros. They enable composition of higher-level macros without reimplementing function syntax parsing, and are available as part of the public API for external macro authors.

### Operations

- **`fn_name!`** — Accepts a function definition. Returns the function name identifier. Scans past any leading tokens to locate the function keyword, then captures the identifier that follows.
- **`fn_rename!`** — Accepts the original function split into three positional arguments: leading tokens, the replacement identifier, and the function body from the keyword onward. Returns the reconstructed function with the name substituted while all other tokens are preserved.
- **`fns!`** — Accepts a callback macro path and a sequence of function definitions. For each function, invokes the callback macro passing the single function as its argument. Handles visibility, generics, where clauses, parameter lists, return types, and function bodies.
- **`fns2!`** — Accepts a callback macro path and a sequence of items. Simpler than `fns!` — delegates each item directly to the callback without structured parsing. Lower overhead for simple cases.

### Error Handling

- `fn_name!` produces a compile error when the input contains no function definition.
- `fn_rename!` requires a syntactically complete function; incomplete input produces a compile error.
- `fns!` requires each function to be syntactically complete and the callback path to resolve to an accessible macro; violations produce compile errors.
- `fns2!` does not validate item shape; any invalid input is reported by the callback macro as a compile error.

### Compatibility Guarantees

- All utility macros are part of the public API and exported through the `exposed` namespace.
- The callback dispatch convention used by `fns!` and `fns2!` is stable.
- Behavior of `fn_rename!` and `fn_name!` may be affected by future changes to Rust macro hygiene rules, but the observable output for standard function syntax is stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implsindex/func.rs` | Implements `fn_name!`, `fn_rename!`, `fns!`, `fns2!` |
| source | `src/implsindex/impls.rs` | Consumes `fns!` utility in the indexing macro expansion path |
| test | `tests/inc/func_test.rs` | Validates all utility macro operations |
| doc | `docs/feature/003_function_utilities.md` | User-facing function utilities feature |
| doc | `docs/api/001_indexing_macros.md` | Indexing macros that consume `fns!` internally |

# Feature: Macro Code Generation

### Scope

- **Purpose**: Parse a block of function definitions and emit one `macro_rules!` wrapper per function
- **Responsibility**: Transform structured function input into named, invocable macro definitions
- **In Scope**: Parsing `ItemFn`, optional (`?`) marker, code emission via `macro_rules!`
- **Out of Scope**: Runtime dispatch, trait resolution, type inference, behavioral testing (belongs in `impls_index`)

### Design

The feature accepts a brace-delimited block of function items, each optionally prefixed with `?`. For each function item:

- A `macro_rules!` definition is generated with two arms:
  - `()` — direct invocation: expands to the function body inline
  - `(as $Name: ident)` — rename arm: delegates to `fn_rename!` using the given identifier
- Functions prefixed with `?` receive `#[allow(unused_macros)]`; functions without the prefix receive `#[deny(unused_macros)]` to enforce usage

The macro name matches the function name, providing a one-to-one mapping between function definitions and generated macros.

Parsing uses a custom `Item2` type that wraps `Option<Token![?]>` and `syn::ItemFn`, and a `Many<T>` container type that collects multiple `Item2` values from the token stream. Token emission is handled via `ToTokens`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/impls.rs` | Contains `Item2`, `Items2`, `Many<T>`, and `ToTokens` impl |
| Source | `src/lib.rs` | Exports the `impls3` proc-macro entry point |
| Test | `tests/smoke_test.rs` | Compilation tests for basic function variants |
| Test | `tests/corner_cases_test.rs` | Compilation tests for edge cases and attribute handling |
| Parent crate | `impls_index/src/` | Re-exports `impls3`; provides full public API (`impls1`, `impls2`) |

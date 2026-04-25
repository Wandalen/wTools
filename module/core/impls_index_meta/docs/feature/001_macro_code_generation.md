# Feature: Macro Code Generation

### Scope

- **Purpose**: Parse a block of function definitions and emit one `macro_rules!` wrapper per function
- **Responsibility**: Transform structured function input into named, invocable macro definitions
- **In Scope**: Parsing function items and optional (`?`) markers, code emission
- **Out of Scope**: Runtime dispatch, trait resolution, type inference, behavioral testing (belongs in `impls_index`)

### Design

The feature accepts a brace-delimited block of function items, each optionally prefixed with `?`. For each function item:

- A `macro_rules!` definition is generated with two arms:
  - `()` — direct invocation: expands to the function body inline
  - `(as $Name: ident)` — rename arm: delegates to `fn_rename!` using the given identifier
- Functions prefixed with `?` receive `#[allow(unused_macros)]`; functions without the prefix receive `#[deny(unused_macros)]` to enforce usage

The macro name matches the function name, providing a one-to-one mapping between function definitions and generated macros.

Parsing collects all function items from the token stream. Code emission converts each collected function item into its corresponding `macro_rules!` definition.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| invariant | `../invariant/001_compile_time_only.md` | Compile-time only guarantee for this feature |

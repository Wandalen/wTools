# Invariant: Compile-Time Arg Count

### Scope

- **Purpose**: Ensure that calls to from! with more than three arguments produce a compile-time error, not silent incorrect behavior.
- **Responsibility**: Documents the argument count invariant — its statement, enforcement, and consequences.
- **In Scope**: The four-or-more argument case of the from! macro.
- **Out of Scope**: The from! macro's normal dispatch paths → `api/002`; field count boundary → `invariant/001`.

### Invariant Statement

For any invocation of from! with four or more arguments: the macro expansion emits a compile_error! with the message "Too many arguments", preventing compilation. No code path executes at runtime.

### Enforcement Mechanism

The from! macro definition contains an exhaustive catch-all pattern after the three explicit arm matches (0, 1, 2, and 3 arguments). Any invocation with more than three arguments matches only this catch-all, which unconditionally expands to compile_error!. This is a compile-time enforcement — no runtime path exists.

### Violation Consequences

This invariant is mechanically enforced by the macro rules and cannot be violated at runtime. Without it, a four-argument invocation would produce a non-obvious missing-trait-implementation error from the compiler. The explicit compile_error! provides a clear, actionable error message pointing directly to the from! call site.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [`src/variadic.rs`](../../src/variadic.rs) | from! macro catch-all arm emitting compile_error! |
| doc | [`docs/api/002_from_macro.md`](../api/002_from_macro.md) | from! macro API specification |
| doc | [`docs/feature/001_variadic_construction.md`](../feature/001_variadic_construction.md) | Feature hub for variadic construction |
| test | [`tests/compile_fail.rs`](../../tests/compile_fail.rs) | Compile-fail test runner |
| test | [`tests/compile_fail/test_from_macro_too_many_args.rs`](../../tests/compile_fail/test_from_macro_too_many_args.rs) | Specific test for >3 argument invocation |

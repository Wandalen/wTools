# API: from! Macro

### Scope

- **Purpose**: Provide a single call-site entry point that dispatches to the correct constructor based on argument count.
- **Responsibility**: Documents the from! macro interface — dispatch rules, argument limits, and error behavior.
- **In Scope**: from! argument dispatch for 0–3 args; compile-time error for >3 args.
- **Out of Scope**: Trait definitions dispatched to → `api/001`; compile-time invariant details → `invariant/002`.

### Abstract

A declarative macro that selects the right constructor based on the number of arguments at the call site. It eliminates the need to call From1::from1, From2::from2, or From3::from3 by name — the caller writes from!(a, b) and the macro expands to the correct trait call. With no arguments, it calls Default::default().

### Operations

Dispatch table by argument count:
- **0 arguments**: Calls Default::default() on the target type.
- **1 argument**: Calls From1::from1 with the single argument.
- **2 arguments**: Calls From2::from2 with both arguments.
- **3 arguments**: Calls From3::from3 with all three arguments.
- **4 or more arguments**: Emits compile_error! with the message "Too many arguments".

### Error Handling

The only error condition is providing more than three arguments. This is caught at compile time by the catch-all pattern, which emits compile_error! — no runtime error is possible.

### Compatibility Guarantees

The macro relies on the underlying FromN traits being implemented. If a target type does not implement From1, from!(a) will produce a trait-not-implemented compile error. The macro itself imposes no additional constraints beyond what the traits require.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [`src/variadic.rs`](../../src/variadic.rs) | from! macro definition |
| doc | [`docs/api/001_from_n_traits.md`](001_from_n_traits.md) | FromN trait API dispatched to by this macro |
| doc | [`docs/invariant/002_compile_time_arg_count.md`](../invariant/002_compile_time_arg_count.md) | Argument count invariant: >3 args → compile error |
| doc | [`docs/feature/001_variadic_construction.md`](../feature/001_variadic_construction.md) | Feature hub for variadic construction |
| test | [`tests/compile_fail.rs`](../../tests/compile_fail.rs) | Compile-fail tests for the >3 argument case |
| test | [`tests/variadic_from_tests.rs`](../../tests/variadic_from_tests.rs) | from! macro usage tests |

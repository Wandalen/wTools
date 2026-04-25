# Invariant: Compile-Time Only

### Scope

- **Purpose**: Guarantee that all macro errors surface at compile time, never at runtime
- **Responsibility**: Enforce that `impls3!` never panics, aborts, or fails after compilation succeeds
- **In Scope**: Parse errors, unsupported item types, malformed input
- **Out of Scope**: Runtime behavior of the generated function bodies (governed by caller code)

### Statement

If `impls3! { ... }` compiles without error, the generated `macro_rules!` definitions are syntactically and structurally valid Rust. No failure mode can arise from the macro infrastructure after compilation.

Conversely, any violation of the accepted input grammar (non-function items, malformed syntax) causes a compile-time error with a diagnostic message — never a silent incorrect expansion or a runtime panic.

### Enforcement Mechanism

- Input parsed via `syn` in `src/impls.rs`; parse failure returns a compile error via `syn::Error::into_compile_error()`
- The proc-macro entry point in `src/lib.rs` returns `proc_macro::TokenStream`; no `panic!` or `unwrap()` in the code generation path
- Non-function items (structs, consts, etc.) are rejected at parse time with "Expected a function item"

### Violation Consequences

Generating invalid `macro_rules!` output or panicking in the proc-macro would corrupt the caller's compilation unit, produce cryptic errors at the invocation site, or silently produce wrong code. The compile-time contract prevents all of these.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/impls.rs` | Contains parse + emit logic; enforces this invariant |
| Source | `src/lib.rs` | Entry point; returns compile errors via `TokenStream` |
| Feature | `docs/feature/001_macro_code_generation.md` | Describes the generation pipeline this invariant governs |
| Test | `tests/corner_cases_test.rs` | Compilation tests verify the guarantee across all input variants |

# Invariant: Compile-Time Only

### Scope

- **Purpose**: Guarantee that all macro errors surface at compile time, never at runtime
- **Responsibility**: Enforce that `impls3!` never panics, aborts, or fails after compilation succeeds
- **In Scope**: Parse errors, unsupported item types, malformed input
- **Out of Scope**: Runtime behavior of the generated function bodies (governed by caller code)

### Invariant Statement

If `impls3! { ... }` compiles without error, the generated `macro_rules!` definitions are syntactically and structurally valid Rust. No failure mode can arise from the macro infrastructure after compilation.

Conversely, any violation of the accepted input grammar (non-function items, malformed syntax) causes a compile-time error with a diagnostic message — never a silent incorrect expansion or a runtime panic.

### Enforcement Mechanism

- Parse failures produce compile-time diagnostics at the macro invocation site
- Non-function items (structs, consts, enums, etc.) are rejected during parsing with a descriptive error message
- The code generation path contains no fallible runtime operations — any input violation surfaces as a compile error, never as a runtime failure

### Violation Consequences

Generating invalid `macro_rules!` output or panicking in the proc-macro would corrupt the caller's compilation unit, produce cryptic errors at the invocation site, or silently produce wrong code. The compile-time contract prevents all of these.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../feature/001_macro_code_generation.md` | The generation feature this invariant governs |

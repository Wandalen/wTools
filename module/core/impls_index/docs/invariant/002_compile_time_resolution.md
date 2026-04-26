# Invariant: Compile-Time Resolution

### Scope

- **Purpose**: Guarantee that all `impls_index` macro operations introduce zero runtime overhead.
- **Responsibility**: Define the compile-time-only invariant for all macros in the crate.
- **In Scope**: All macros — indexing, invocation, and utilities; the no-standard-library declaration.
- **Out of Scope**: Runtime behavior of the generated functions themselves (the generated code is user-authored).

### Invariant Statement

All operations performed by `impls_index` macros — wrapping functions in named macros, invoking those macros to emit function definitions, extracting or renaming function identifiers — complete entirely during the Rust compilation phase. No `impls_index` operation executes at program startup, at any function call site, or at any other runtime point.

### Enforcement Mechanism

The crate is composed entirely of declarative and procedural macro mechanisms. Both mechanism types operate exclusively during the compiler's expansion phase, which precedes code generation. Neither has any runtime representation — they produce code at build time and are absent from the running program.

The crate carries a no-standard-library declaration, confirming that no runtime services (allocator, operating system interface, thread-local storage) are required or imported. This declaration is a structural proof: a crate with runtime dependencies cannot carry this declaration without explicit remediation. The absence of this declaration from any future version of the crate would be an immediate signal of invariant breakage.

### Violation Consequences

This invariant cannot be violated through normal use of the current API — the macro system provides the enforcement by construction. A future change introducing runtime state (such as a function registry, lazy initialization table, or dynamic dispatch mechanism) would break this invariant. Such a change would also invalidate the no-standard-library declaration, require an allocator, and introduce initialization order dependencies between compilation units. These downstream effects serve as additional protection against accidental runtime state introduction.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Declares `no_std`; confirms absence of runtime dependency |
| source | `src/implsindex/impls.rs` | All indexing and invocation macros — compile-time only |
| source | `src/implsindex/func.rs` | All utility macros — compile-time only |
| doc | `docs/api/001_indexing_macros.md` | API contract with no-runtime guarantee |
| doc | `docs/pattern/001_two_crate_proc_macro.md` | Two-crate pattern that keeps proc macro isolated from runtime crate |

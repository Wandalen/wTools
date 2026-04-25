# Invariant: Unused Macro Enforcement

### Scope

- **Purpose**: Ensure every function indexed with a strict macro is intentionally materialized, preventing silent dead code accumulation.
- **Responsibility**: Define the unused-macro enforcement invariant for strict indexing macros (`impls1!`, `impls2!`, `impls3!`, `tests_impls!`).
- **In Scope**: Compile-time enforcement via deny lint; behavior of strict vs optional macro variants.
- **Out of Scope**: Optional variants that explicitly relax this invariant (`impls_optional!`, `tests_impls_optional!`).

### Invariant Statement

For every function indexed with a strict indexing macro (`impls1!`, `impls2!`, `impls3!`, `tests_impls!`), the generated named macro must be invoked at least once through `index!` or `tests_index!`. If any generated named macro is unused, the compiler emits an error before linking begins.

Formally: for every `f` indexed in a strict `impls` call, there exists exactly one corresponding `index!` entry naming `f` (directly or via an alias) in the same scope.

### Enforcement Mechanism

Each strict indexing macro wraps its generated named macro in a `deny(unused_macros)` lint attribute. The Rust compiler treats any macro definition under this lint that is never invoked as a hard error. This check fires during the compilation of the crate containing the `index!` call — before code generation, linking, or test execution.

The optional variants (`impls_optional!`, `tests_impls_optional!`) emit `allow(unused_macros)` instead, explicitly opting out of enforcement for scenarios where conditional compilation or partial usage is intentional.

### Violation Consequences

A violation means a function was indexed but never invoked through `index!`. The immediate consequence is a compile error — the affected crate does not compile and no binary or test artifact is produced. There is no silent failure, no runtime detection, and no deferred discovery. Dead indexed functions cannot reach a shipped binary.

If the intention is to allow some indexed functions to remain unused, switch to `impls_optional!` or remove the unused function from the `impls` call entirely.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implsindex/impls.rs` | Emits `deny(unused_macros)` / `allow(unused_macros)` around generated macros |
| doc | `docs/feature/001_function_indexing.md` | Feature description covering strict vs optional distinction |
| doc | `docs/feature/002_test_indexing.md` | Feature description — enforcement applies to `tests_impls!` strict variant |
| doc | `docs/api/001_indexing_macros.md` | API contract describing enforcement behavior per macro variant |

# Invariant: Declarative Macros Only

### Scope

- **Purpose**: Ensure all macros provided by format_tools are declarative, keeping compile times low and the dependency graph minimal.
- **Responsibility**: States the declarative-macros constraint, how it is enforced, and what breaks if violated.
- **In Scope**: Absence of procedural macro infrastructure (proc-macro crate type, code generation framework dependencies) from format_tools.
- **Out of Scope**: Proc macros in other crates that callers may use alongside format_tools.

### Invariant Statement

All macros in format_tools are defined using the declarative macro system. format_tools does not declare a proc-macro crate, does not depend on proc-macro compilation infrastructure, and does not use code-generation frameworks at compile time. Verified by: no proc-macro crate type declaration in Cargo.toml; no code generation framework dependencies.

### Enforcement Mechanism

Cargo.toml does not declare a proc-macro crate type. No code generation framework dependencies are listed. Verified by Cargo.toml inspection and dependency tree.

### Violation Consequences

Introducing proc macros would require splitting format_tools into two crates (a library and a proc-macro crate), add code generation framework compile-time dependencies, and significantly increase incremental build times — all for formatting macros that compile correctly as declarative macros.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `Cargo.toml` | Dependency manifest — absence of proc-macro infrastructure enforces this invariant |
| source | `src/format/to_string_with_fallback.rs` | Declarative macro implementations |
| doc | `docs/feature/001_fallback_string_conversion.md` | Fallback macros constrained by this invariant |
| doc | `docs/feature/003_field_formatting_macros.md` | Field macros constrained by this invariant |
| doc | `docs/api/001_fallback_conversion_api.md` | API surface constrained by this invariant |

# Invariant: macro_tools Standardization

### Scope

- **Purpose**: Ensure `clone_dyn_meta` uses only `macro_tools` abstractions for all proc-macro implementation.
- **Responsibility**: Prohibit direct dependencies on `proc-macro2`, `quote`, or `syn` in production code.
- **In Scope**: All syntax parsing, code generation, and error handling in `clone_dyn_meta`.
- **Out of Scope**: Test code, build scripts.

### Invariant Statement

`clone_dyn_meta` must not directly depend on `proc-macro2`, `quote`, or `syn` in its `[dependencies]`. All syntax manipulation must go through `macro_tools` abstractions. `macro_tools` provides these transitively.

### Enforcement Mechanism

- `Cargo.toml` has no direct `proc-macro2`, `quote`, or `syn` entries under `[dependencies]`.
- Code review must reject any `use proc_macro2::` or `use quote::` imports in `src/`.
- `cargo +nightly udeps` detects any drift where a dependency is taken directly.

### Violation Consequences

Direct dependency bypasses `macro_tools`' version management, potentially introducing API inconsistencies across the workspace and breaking the workspace-wide standardization contract established for all proc-macro crates.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature | `../feature/001_clone_dyn_macro.md` | Implementation that must follow this constraint |

# Invariant: Traditional Module Pattern

### Scope

- **Purpose**: Prohibit `iter_tools` from depending on `mod_interface` and require the manual namespace chain.
- **Responsibility**: State the prohibition, explain the enforcement mechanism, and document the circular dependency that would result from violation.
- **In Scope**: The `own → orphan → exposed → prelude` namespace chain in `src/iter.rs` and `src/lib.rs`.
- **Out of Scope**: The module organization of crates that do use `mod_interface`.

### Invariant Statement

`iter_tools` must never add `mod_interface` as a dependency. All namespace re-export chaining must be implemented manually using the `own → orphan → exposed → prelude` pattern directly in source files.

### Enforcement Mechanism

`mod_interface` is absent from `Cargo.toml`. The manual namespace chain is implemented in `src/iter.rs` (the `own`, `orphan`, `exposed`, and `prelude` sub-modules of `mod private`) and re-exported from `src/lib.rs`. Code review must reject any PR that adds `mod_interface` to the dependency tree.

### Violation Consequences

Adding `mod_interface` as a dependency creates a circular dependency: `iter_tools → mod_interface → macro_tools → iter_tools`. This causes a build failure across the entire workspace. The build system cannot resolve a circular crate dependency.

### Cross-References

| To | Type | Rationale |
|----|------|-----------|
| [src/iter.rs](../../src/iter.rs) | src | Manual `mod private` with `own`, `orphan`, `exposed`, `prelude` sub-modules. |
| [src/lib.rs](../../src/lib.rs) | src | Top-level re-export from `mod iter`. |
| [Cargo.toml](../../Cargo.toml) | config | `mod_interface` is intentionally absent. |
| [feature/001_itertools_reexports.md](../feature/001_itertools_reexports.md) | doc | Re-exports feature that depends on this namespace chain. |
| [feature/002_clonable_boxed_iterators.md](../feature/002_clonable_boxed_iterators.md) | doc | Clonable boxed iterators exposed via this namespace chain. |
| [feature/003_iter_ext.md](../feature/003_iter_ext.md) | doc | `IterExt` extension exposed via this namespace chain. |

### Sources

| Source | Section |
|--------|---------|
| spec.md (deleted) | § Architecture § Module Organization |
| spec.md (deleted) | § Design Decisions § No mod_interface |

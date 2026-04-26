# Feature: Module Interface Pattern

### Scope

- **Purpose**: Provide a procedural macro that generates the four standard namespace layers (`own`, `orphan`, `exposed`, `prelude`) for any module, enforcing workspace-wide module organization conventions.
- **Responsibility**: Document the scope and cross-references for the `mod_interface!` macro re-exported by `meta_tools`.
- **In Scope**: `mod_interface!` macro invocation, the four generated namespace layers, the `mod_interface` feature flag.
- **Out of Scope**: `mod_interface_meta` internals, layer visibility semantics (see the `mod_interface` crate docs).

### Design

`mod_interface!` is always available because `mod_interface_meta` is a mandatory dependency — the facade's own `meta` module uses it to organize its namespace. The optional `mod_interface` feature flag controls only whether the higher-level declarative `mod_interface` crate is included.

Callers invoke `mod_interface!` with a list of `layer <name>;` declarations. The macro generates `pub mod own {}`, `pub mod orphan {}`, `pub mod exposed {}`, and `pub mod prelude {}` sub-modules with appropriate re-export chains, standardizing module structure across the workspace.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/dependency.rs` | `mod_interface_meta::mod_interface` re-export (unconditional) |
| source | `src/meta/mod.rs` | `mod_interface!` usage demonstrating the pattern |
| test | `tests/inc/mod.rs` | Cross-crate inclusion of `mod_interface` test suite |
| doc | `docs/api/001_macros.md` | `mod_interface!` macro signature |
| doc | `docs/pattern/001_facade_aggregation.md` | Context: why `mod_interface_meta` is a mandatory dependency |

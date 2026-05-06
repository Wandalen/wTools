# Invariant: Types Crate Pattern

### Scope

- **Purpose**: Document the structural constraint that keeps `component_model_types` dependency-free from runtime and macro crates to prevent circular dependencies.
- **Responsibility**: States the invariant, explains what makes it necessary, and captures the consequences of violating it.
- **In Scope**: The dependency boundary this crate must maintain; the three-crate ecosystem structure that relies on it.
- **Out of Scope**: The Assign trait surface (→ `api/001_assign_trait.md`); the feature capabilities enabled by this isolation (→ `feature/001_component_assignment.md`).

### Invariant Statement

This crate MUST NOT take dependencies on `component_model` or `component_model_meta`. Both of those crates depend on `component_model_types`; if this crate depended on either of them, the dependency graph would be cyclic and the workspace would fail to compile.

The three-crate structure that makes this invariant necessary:

```
component_model_types   ← types only; both others depend on this
       ↑                          ↑
component_model          component_model_meta
(runtime)                (proc-macro)
```

Adding any dependency on `component_model` or `component_model_meta` into this crate collapses the shared-types design and reintroduces the circular dependency problem this crate exists to solve.

### Enforcement Mechanism

Enforced at compile time by Cargo's dependency resolver. A circular dependency produces an error at `cargo build` or `cargo check` time. No runtime check is needed or possible.

The constraint applies to transitive dependencies as well: this crate must not depend on any crate that itself depends on `component_model` or `component_model_meta`. This is why `test_tools` cannot be used in dev-dependencies — `test_tools` pulls in `impls_index_meta`, which pulls in `macro_tools`, which depends on `component_model_types`, completing a cycle.

### Violation Consequences

A dependency violation produces a compilation failure at build time, so no violation can be shipped. However, attempting such a dependency during development wastes time diagnosing a confusing build failure. The design intent must be understood to avoid the attempt in the first place.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `Cargo.toml` | Dependency declarations that must respect this invariant |
| source | `Cargo.toml` dev-dependencies comment | Documents why `test_tools` is excluded |
| doc | [feature/001_component_assignment.md](../feature/001_component_assignment.md) | The feature this isolation makes possible |
| doc | [api/001_assign_trait.md](../api/001_assign_trait.md) | The trait surface shared through this isolation |

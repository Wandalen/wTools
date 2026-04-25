# Invariant: Exclusive Error Dependency

### Scope

- **Purpose**: Guarantee consistent error handling behaviour across the workspace by preventing mixed error frameworks.
- **Responsibility**: Documents the exclusive dependency invariant — its statement, enforcement points, and violation consequences.
- **In Scope**: The mandate that consumers access error primitives only through `error_tools`, never from `anyhow` or `thiserror` directly.
- **Out of Scope**: Implementation of specific error types or error display formatting — consumer responsibility.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_error_facade.md](../feature/001_error_facade.md) | Facade that owns the error namespace |

### Invariant Statement

Any crate in the workspace that uses `error_tools` must not also directly depend on `anyhow` or `thiserror`. All error handling primitives are accessed through the `error_tools` exposed interface only.

### Enforcement Mechanism

- Workspace dependency declarations: `anyhow` and `thiserror` appear only in `error_tools/Cargo.toml` as optional dependencies, not in consumer crate configuration
- Code review: any direct access to `anyhow` or `thiserror` in a consumer crate bypassing the facade is a violation
- The dependency re-export module provides access to the upstream library namespace for the edge case where the derivation mechanism's backend requires an explicit namespace reference — this is the only permitted indirect access pattern

### Violation Consequences

A violation weakens the facade guarantee: consumers become coupled to upstream library versions independently of `error_tools`, creating the same version-skew fragmentation the facade was designed to prevent.

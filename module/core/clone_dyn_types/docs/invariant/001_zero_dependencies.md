# Invariant: Zero Dependencies

### Scope

**Purpose**: Prevent circular dependency chains in the wTools workspace.
**In Scope**: `[dependencies]` section in `Cargo.toml`.
**Out of Scope**: `[dev-dependencies]`; `[build-dependencies]`.

### Statement

`clone_dyn_types` MUST have zero entries in `[dependencies]` in `Cargo.toml` at all
times. The crate produces no output that requires any runtime dependency.

### Enforcement

Enforced by Cargo dependency resolution — any production dependency addition is
immediately visible in `cargo tree` output and workspace `Cargo.lock` changes.
CI workspace diff review catches violations on every push.

### Violation Consequences

A production dependency introduces a potential circular dependency chain in the
wTools workspace (e.g., `macro_tools -> clone_dyn_types -> X -> macro_tools`).
Breaks `no_std` compatibility for any dependency requiring `std`. Adds compile-time
overhead to every crate in the workspace that transitively depends on `clone_dyn_types`.

### Cross-References

- `feature/001_no_std_support.md` — zero deps are required for no_std compatibility

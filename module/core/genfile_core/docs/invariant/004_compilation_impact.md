# Invariant: Compilation Impact

### Scope

- **Purpose**: Limits the build-time cost that adding genfile_core as a dependency imposes on downstream crates.
- **Responsibility**: Documents the maximum allowable clean build time increase and its measurement.
- **In Scope**: Clean build time delta for a crate that adds genfile_core as a dependency.
- **Out of Scope**: Incremental build time, CI total build time.

### Invariant Statement

Adding genfile_core as a dependency must not increase the clean build time of a dependent project by more than 5 seconds, measured on standard development hardware.

### Enforcement Mechanism

Compared by measuring clean build time of a reference crate (e.g., `willbe`) before and after adding genfile_core as a dependency. The minimal dependency footprint (Handlebars, serde, base64, regex, error_tools) is the primary lever for compliance.

### Violation Consequences

A large compilation overhead discourages adoption and slows CI across the wTools workspace. Every unnecessary dependency added to genfile_core propagates this cost to all dependents.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `Cargo.toml` | Dependency declarations that drive compilation cost |

# Invariant: Selective Compilation

### Scope

- Defines the feature-flag contract that governs which macros are compiled.
- Covers the constraint that each macro must be independently disableable.
- Audience: contributors adding new macros or modifying the feature set.
- Does not cover which features are enabled by default — see `Cargo.toml`.

### Invariant Statement

Each derive macro in this crate must be guarded by a dedicated feature flag.
A consumer must be able to include any subset of macros without pulling in the
compilation cost of unused macros.

The `enabled` feature activates the crate as a whole. Individual macro features
depend on `enabled` and are themselves opt-in. The `full` feature activates all
individual macro features.

No macro implementation may be compiled unconditionally. All macro code is
inside a feature gate.

### Enforcement Mechanism

Each macro's source code is wrapped in a `cfg` attribute conditioned on its
feature flag. If the flag is absent from the dependent's feature list, the
compiler will not compile that macro's implementation.

Contributors adding a new macro must also add a corresponding feature flag in
`Cargo.toml` and guard the macro source with that flag.

### Violation Consequences

An unconditional macro increases compile time for all consumers even when the
macro is never used. It also enlarges the dependency surface unnecessarily.

A macro that does not respect the `enabled` gate may activate when the crate
is listed as a dependency but not explicitly opted in, violating the workspace
feature convention for this project.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature | docs/feature/001_derive_macros.md | Full macro collection covered by this contract |
| api | docs/api/001_derive_api.md | Individual macro availability per flag |

### Sources

- [../../spec.md](../../spec.md) — Feature Flags table and Design Rationale: Why Feature-Gated Derives section

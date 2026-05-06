# Invariant: Feature Flag Gating

### Scope

- **Purpose**: Enforce that every proc-macro entry point is independently disableable via a feature flag.
- **Responsibility**: Document the derive_former feature flag contract and its enforcement mechanism.
- **In Scope**: The `derive_former` feature, its dependency on `enabled`, and violation consequences.
- **Out of Scope**: What the macro generates — see `feature/001_former_derive.md`.

### Invariant Statement

Every proc-macro entry point in this crate must be gated behind a dedicated feature flag.
The `Former` derive macro is gated behind the `derive_former` feature. Removing or
renaming this flag is a breaking change. Adding a new macro requires adding a corresponding
new feature flag before the macro entry point is declared.

The `enabled` feature activates all transitive dependency features required for any macro
in this crate to function. A crate with `derive_former` active but `enabled` inactive
must not compile successfully.

### Enforcement Mechanism

Feature flag gating is implemented by applying conditional compilation attributes to each
proc-macro entry point. The compiler enforces that gated items are absent when the flag is
not active. The feature dependency relationship between each macro flag and the shared
enabled flag is declared in the crate manifest.

### Violation Consequences

A macro that is not feature-gated increases compile cost for all consumers regardless of
whether they need the macro. Removing an existing flag breaks consumers who have
explicitly disabled it. Both directions of violation degrade the opt-in contract that
`former_meta` provides.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/lib.rs` | Proc-macro entry points gated by the derive_former feature |
| config | `../../Cargo.toml` | Declares derive_former feature and its enabled dependency |
| doc | `../api/001_derive_api.md` | The macro API gated by this invariant |
| doc | `../feature/001_former_derive.md` | The behavior enabled by derive_former flag |

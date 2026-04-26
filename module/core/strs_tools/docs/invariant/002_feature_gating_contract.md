# Invariant: Feature Gating Contract

### Scope

- **Purpose**: Guarantee that all optional capabilities are gated behind Cargo features and that enabling no features produces a compilable, empty-surface crate.
- **Responsibility**: Defines the feature gating invariant: the `default` feature set must be empty and every optional capability must be reachable only through an explicit feature activation.
- **In Scope**: Empty `default` feature, the `enabled` opt-in feature, `full` aggregate feature, per-capability feature flags.
- **Out of Scope**: SIMD-specific feature behaviour (`invariant/003`); no_std compatibility (`invariant/004`).

### Invariant

The `default` feature set is empty. A downstream crate that depends on `strs_tools` without specifying features receives no public API surface and no transitive dependencies beyond the mandatory ones.

Every optional capability — splitting, indentation, isolation, number parsing, command parsing, ANSI utilities, SIMD acceleration, parser integration — is activated only when the corresponding Cargo feature is explicitly enabled by the caller or by the `full` aggregate feature.

The `enabled` feature is the minimum opt-in that unlocks the crate's core error handling integration. Capabilities build on top of `enabled`; none of them activate it implicitly except through the `full` feature aggregate.

A crate compiled with only `enabled` active must compile without warnings and produce no public symbols beyond the crate root.

### Sources

- `../../architecture.md` — Feature Dependencies section; feature hierarchy and dependency list migrated to this invariant.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `Cargo.toml` | Feature declarations and dependency activation |
| test | `tests/issue_002_example_feature_guards.rs` | Feature guard correctness verification |
| doc | `docs/feature/007_simd_acceleration.md` | SIMD feature flag behaviour |
| doc | `docs/invariant/004_no_std_alloc_contract.md` | No-std compatibility scope dependent on feature gating |

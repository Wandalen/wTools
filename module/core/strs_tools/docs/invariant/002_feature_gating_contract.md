# Invariant: Feature Gating Contract

### Scope

- **Purpose**: Guarantee that optional capabilities are individually gated behind Cargo features and that the feature hierarchy follows the workspace-standard enabled/full pattern.
- **Responsibility**: Defines the feature gating invariant: the `default` feature set includes `enabled` plus core capabilities for out-of-the-box usability, and every optional capability is reachable through an explicit feature activation.
- **In Scope**: Default feature set composition, the `enabled` opt-in feature, `full` aggregate feature, per-capability feature flags, workspace-internal `default-features = false` pattern.
- **Out of Scope**: SIMD-specific feature behaviour (`invariant/003`); no_std compatibility (`invariant/004`).

### Invariant

The `default` feature set includes `enabled` and commonly-needed core capabilities (`string_indentation`, `string_parse_number`), ensuring the crate works out of the box for external consumers via `cargo add`.

Every optional capability — splitting, indentation, isolation, number parsing, command parsing, ANSI utilities, SIMD acceleration, parser integration — is gated behind its own Cargo feature flag. The `full` feature activates all capabilities simultaneously.

The `enabled` feature is the minimum opt-in that unlocks the crate's core error handling integration. Capabilities build on top of `enabled`; the `full` feature includes all of them.

Workspace-internal dependents use `default-features = false` in `[workspace.dependencies]` to opt in selectively. External consumers receive the default set automatically, which provides a useful working crate without requiring feature selection.

A crate compiled with only `enabled` active must compile without warnings and produce no public symbols beyond the crate root.

### Sources

- [Cargo.toml](../../Cargo.toml) — Feature declarations and dependency activation

### Tests

- [tests/issue_002_example_feature_guards.rs](../../tests/issue_002_example_feature_guards.rs) — Feature guard correctness verification

### Features

- [007_simd_acceleration.md](../feature/007_simd_acceleration.md) — SIMD feature flag behaviour

### Invariants

- [004_no_std_alloc_contract.md](../invariant/004_no_std_alloc_contract.md) — No-std compatibility scope dependent on feature gating

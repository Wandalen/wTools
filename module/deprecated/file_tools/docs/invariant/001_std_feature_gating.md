# Invariant: Std Feature Gating

### Scope

- **Purpose**: Guarantee that `TempDir` is never exposed in `no_std` configurations, even when both `enabled` and `no_std` features are activated simultaneously.
- **Responsibility**: Documents the compile-time invariant that `TempDir` requires `std` and the mechanism that enforces it under `--all-features`.
- **In Scope**: The `cfg_attr` guard in `lib.rs`, the `--all-features` conflict scenario, the feature flag combination that triggers the guard.
- **Out of Scope**: Runtime panics, build-time lints unrelated to feature gating.

### Invariant Statement

`TempDir` is only accessible when `feature = "enabled"` is active AND `feature = "no_std"` is inactive. When both features are activated simultaneously (e.g., via `cargo test --all-features`), the `no_std` activation is suppressed for the `enabled` path, keeping `TempDir` available rather than silently absent.

### Enforcement Mechanism

`lib.rs` line 1 carries:
```
#![ cfg_attr( all( feature = "no_std", not( feature = "enabled" ) ), no_std ) ]
```
This applies `#![no_std]` only when `no_std` is active AND `enabled` is NOT active. When both are active (the `--all-features` case), the condition is false, `std` remains available, and `TempDir` is exposed normally.

All `TempDir` items in `fs.rs` are additionally guarded with `#[cfg(all(feature = "enabled", not(feature = "no_std")))]`.

### Violation Consequences

If the `cfg_attr` guard were absent:
- `cargo test --all-features` would activate `no_std`, making `std::path::PathBuf` and `std::fs` unavailable.
- `TempDir` would fail to compile under `--all-features`, breaking CI for dependent crates.
- The failure would be silent in workspaces that don't run `--all-features` — only discovered during workspace-wide CI runs.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/fs/lib.rs` | `cfg_attr` guard at crate root (line 1) |
| source | `../../src/fs/fs.rs` | `cfg(all(feature = "enabled", not(feature = "no_std")))` guards on TempDir items |
| test | `../../tests/feature_conflict_all_features_bug.rs` | Bug reproducer verifying invariant holds under `--all-features` |
| doc | `../feature/001_temp_dir_raii.md` | Feature context; note on `no_std` limitation |
| doc | `../api/001_temp_dir.md` | Availability constraint in Compatibility Guarantees |

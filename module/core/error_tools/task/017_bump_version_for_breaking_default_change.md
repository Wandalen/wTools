# Bump version to 0.40.0 for breaking default-features change

## Description

The local `error_tools` crate has a breaking change relative to the published `0.39.0`:
`default` features changed from `["enabled", "error_typed", "error_untyped"]` to `[]`.

Any downstream crate depending on `error_tools` without explicit `features = [...]` will silently
lose the entire public API (no `ErrWith`, no `thiserror` integration, no `anyhow` integration)
upon upgrading. This is a semver-breaking change.

Since `error_tools` is pre-1.0, the convention for breaking changes is a minor version bump
(`0.39.x` → `0.40.0`). The version in `Cargo.toml` must be updated before the next publish,
and all version references in `readme.md` documentation examples must be updated to match.

## Requirements

- Update `Cargo.toml` `[package] version` from `"0.39.0"` to `"0.40.0"`
- Update all `version = "0.39"` references in `readme.md` to `"0.40"`
- The `readme.md` already documents the opt-in model change — verify the migration section
  is clear and includes a before/after example for users coming from 0.38/0.39
- Run `cargo check` and `cargo test --all-features` to confirm version bump does not break build

## Acceptance Criteria

- `Cargo.toml` declares `version = "0.40.0"`
- All `readme.md` usage examples reference `version = "0.40"` (not `"0.39"`)
- Migration guide covers: "add `features = [\"full\"]` to restore previous default behaviour"
- `cargo check` exits 0
- `cargo nextest run --all-features` exits 0

## Outcomes


# genfile_core Feature Drift — Publish Failure Fix

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 📥 (Backlog)

## Goal

Document and fix the publication version drift between local `genfile_core` (which has
`enabled`) and the crates.io published version `0.10.0` (which does not). Running
`will .publish.diff` on `willbe` fails because `cargo package` strips `path =` and
resolves `genfile_core` from the registry — finding v0.10.0 without `enabled`.

**Fix:** Bump `genfile_core` to `0.11.0` and update the workspace dep constraint from
`~0.10.0` to `~0.11.0`. Publish `genfile_core` 0.11.0 before any downstream crates.

## In Scope

- `module/core/genfile_core/Cargo.toml` — version bump `0.10.0 → 0.11.0`
- `Cargo.toml` (workspace root) — constraint bump `~0.10.0 → ~0.11.0` for `genfile_core`
- `tests/inc/publish/bug_genfile_core_feature_drift_test.rs` — regression tests (DONE)
- `tests/inc/publish/mod.rs` — module registration (DONE)

## Out of Scope

- Changes to `genfile_core` feature set or API
- Other workspace crates (they inherit the workspace dep update)

## Root Cause Analysis

`genfile_core` v0.10.0 was published (commit `f0d5e9ee`) **before** the `enabled`
feature was added locally (commit `bb2374a2`). The local source had `enabled`; the
registry version did not. No version bump accompanied the feature addition.

`willbe/Cargo.toml:90` references `genfile_core = { workspace = true, features = ["enabled"] }`.

- **Local builds:** resolve via `path = "module/core/genfile_core"` — succeed.
- **`cargo package`:** strips `path`, resolves `~0.10.0` from crates.io — finds no `enabled` feature → fails.

**Error observed:**
```
package `willbe` depends on `genfile_core` with feature `enabled` but `genfile_core`
does not have that feature.
available features: archive, binary, default, external_content, filesystem, full, json,
  parameter_discovery, renderer, serialization, template, yaml
```

**Publication order:** `genfile_core` 0.11.0 must be published before `willbe`.

## Requirements

1. `genfile_core/Cargo.toml` must declare `version = "0.11.0"` — DONE
2. Workspace `Cargo.toml` must declare `version = "~0.11.0"` for `genfile_core` — DONE
3. Regression tests must pass confirming both constraints — DONE (139/139 tests pass)
4. `genfile_core` 0.11.0 must be published to crates.io before `willbe`

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Verify code changes are in place** — run `cargo nextest run -p willbe --all-features -E 'test(genfile_core)'` (should be 2 PASS)
2. **Publish `genfile_core` 0.11.0** — `cargo publish -p genfile_core`
3. **Verify willbe packages cleanly** — `cargo package -p willbe --no-verify`
4. **Publish downstream crates** in dependency order as needed

## Acceptance Criteria

- `cargo nextest run -p willbe --all-features -E 'test(genfile_core)'` → 2 PASS
- `cargo package -p willbe --no-verify` → exits 0 (no feature resolution error)
- `genfile_core` 0.11.0 visible on crates.io

## Validation

### Checklist

Desired answer for every question is YES.

- [x] V1 — Version bump applied: `genfile_core/Cargo.toml` shows `version = "0.11.0"`?
- [x] V2 — Workspace constraint updated: workspace `Cargo.toml` shows `version = "~0.11.0"` for `genfile_core`?
- [x] V3 — Regression tests pass: both `bug_genfile_core_feature_drift_test` tests are PASS?
- [ ] V4 — Published: `genfile_core` 0.11.0 visible on crates.io?
- [ ] V5 — Package check passes: `cargo package -p willbe --no-verify` exits 0?

## Outcomes

*(To be filled when task is acknowledged and closed.)*

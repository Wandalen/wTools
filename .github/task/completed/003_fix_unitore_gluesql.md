# 003: Fix unitore gluesql Incompatibility

## Execution State

- **Executor Type:** any
- **Actor:** claude-opus-4-6
- **Claimed At:** 2026-04-18T15:30:00
- **Status:** ✅ (Completed)
- **Validated By:** claude-opus-4-6
- **Validation Date:** 2026-04-18

## Goal

The `unitore` crate fails to compile because `gluesql = "0.16.3"` produces E0283 (type inference ambiguity) and E0277 (trait bound) errors on current Rust. Bump to `gluesql = "0.19"` and `gluesql_sled_storage = "0.19"` in `module/experimental/unitore/Cargo.toml`, then fix all API breakage across unitore's ~15 source files and 6 test files. Success: `cargo check -p unitore --all-features` compiles clean; `cargo nextest run -p unitore --all-features` passes all tests.

## In Scope

- `module/experimental/unitore/Cargo.toml` — bump `gluesql` and `gluesql_sled_storage` from `0.16.3` to `0.19`
- All `module/experimental/unitore/src/**/*.rs` files — fix API migration breakage
- All `module/experimental/unitore/tests/*.rs` files — fix test compilation
- Modules affected: `entity/` (table, config, feed, frame), `sled_adapter/` (mod, table, config, feed, frame), `action/` (table, query, config, feed, frame), `command/` (table, query, config, feed, frame)

## Out of Scope

- Upgrading other dependencies in unitore
- Refactoring unitore architecture
- Adding new features to unitore
- Fixing CI workflow files (separate task 002)
- Migrating unitore spec.md to docs/ (pre-existing violation, separate effort)

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Must not affect compilation of any other workspace crate
- gluesql 0.19 API changes must be fully migrated — no commented-out code or workarounds

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note constraints
2. **Bump versions** — edit `Cargo.toml` dependencies
3. **Catalog errors** — `cargo check -p unitore --all-features 2>&1` and list all errors
4. **Fix source files** — address each compilation error, working module by module
5. **Fix test files** — address test compilation errors
6. **Green state** — `cargo check -p unitore --all-features` and `cargo nextest run -p unitore --all-features` must pass
7. **Verify workspace** — `cargo check --workspace --all-features --exclude unitore` still passes
8. **Submit for Validation** — trigger SUBMIT transition (⏳ → 🔍)
9. **Update task status** — on validation pass, set ✅

## Acceptance Criteria

- `module/experimental/unitore/Cargo.toml` declares `gluesql = "0.19"` and `gluesql_sled_storage = "0.19"`
- `cargo check -p unitore --all-features` produces zero errors
- `cargo nextest run -p unitore --all-features` passes all tests
- No other workspace crate is affected (`cargo check --workspace --exclude unitore` clean)
- No `#[ignore]`, `#[cfg(not(...))]` or workaround markers added
- No commented-out old API code retained

## Validation

### Checklist

Desired answer for every question is YES.

**Dependency version**
- [ ] C1 — Does `Cargo.toml` declare `gluesql = "0.19"` (not 0.16)?
- [ ] C2 — Does `Cargo.toml` declare `gluesql_sled_storage = "0.19"` (not 0.16)?

**Compilation**
- [ ] C3 — Does `cargo check -p unitore --all-features` succeed?
- [ ] C4 — Do all unitore tests pass?

**Isolation**
- [ ] C5 — Are other workspace crates unaffected?

**Out of Scope**
- [ ] C6 — Is unitore spec.md unchanged (not migrated in this task)?
- [ ] C7 — Are CI workflow files unchanged (not fixed in this task)?

### Measurements

- [ ] M1 — gluesql version: `grep "gluesql.*0.19" module/experimental/unitore/Cargo.toml | wc -l` → 2 (was: 0)
- [ ] M2 — compilation clean: `cargo check -p unitore --all-features 2>&1 | grep "^error" | wc -l` → 0 (was: 3+)
- [ ] M3 — tests pass: `cargo nextest run -p unitore --all-features 2>&1 | grep "test result" | grep "ok"` → present

### Invariants

- [ ] I1 — workspace clean: `cargo check --workspace --all-features --exclude unitore 2>&1 | grep "^error" | wc -l` → 0
- [ ] I2 — no new warnings: `RUSTFLAGS="-D warnings" cargo check -p unitore --all-features` → success

### Anti-faking checks

- [ ] AF1 — no feature-gating: `grep "#\[cfg" module/experimental/unitore/src/**/*.rs | grep -i "gluesql\|sled" | wc -l` → 0
- [ ] AF2 — no disabled tests: `grep "#\[ignore\]" module/experimental/unitore/tests/*.rs | wc -l` → 0
- [ ] AF3 — no workarounds: `grep -r "HACK\|FIXME.*gluesql\|TODO.*revert" module/experimental/unitore/ | wc -l` → 0

## References

- Companion plan: `-plan/002_fix_cicd_workflow_failures.plan.md` (Phase 3)
- gluesql changelog: version jump 0.16.3 → 0.19.0 (3 major versions)
- Affected source files: ~15 in src/, 6 in tests/

## Outcomes

The gluesql 0.16.3 → 0.19.0 upgrade was minimal: only `sled_adapter/mod.rs` needed changes. gluesql 0.19.0 added a `Planner` trait bound to `Glue<T>`, requiring: (1) import `Planner` from `gluesql::core::store`, (2) add `+ Planner` to all three trait bounds on `FeedStorage<S>` (struct def, Debug impl, Store impl). No other source files needed changes — the rest of the API (Payload, Value, SledStorage, table builder, Execute) remained compatible.

Test compilation revealed a pre-existing issue: `pth::path::unique_folder_name()` requires the `path_unique_folder_name` feature, but unitore only declared `features = ["default"]` for pth. Added `"path_unique_folder_name"` to pth features in unitore's Cargo.toml. Also removed stale "compilation errors" note from Cargo.toml since unitore now compiles clean.

Final: `cargo check -p unitore --all-features` → clean. `cargo nextest run -p unitore --all-features` → 11/11 pass. Pre-existing willbe errors (staleness.rs HashSet mismatch) are unrelated.

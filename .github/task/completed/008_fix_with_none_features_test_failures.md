
# 008: Fix with_none_features Test Failures (P2)

## Execution State

- **Executor Type:** any
- **Actor:** self
- **Claimed At:** 2026-04-18
- **Status:** ✅ (Completed)
- **Validated By:** self
- **Validation Date:** 2026-04-18

## Goal

`workspace_push.yml` runs `will .test ... with_none_features:1` which executes
`cargo test --no-default-features` for every crate. For crates where the entire
public API is gated behind `#[cfg(feature = "enabled")]`, test files that import
those symbols fail to compile with E0432/E0433. At least 7 crates are currently
failing every push (implements, collection_tools, component_model_types, clone_dyn_types,
is_slice, proper_tools, color_tools) — each produces a failing CI job.

Root cause: test files lack `#![cfg(feature = "enabled")]` inner attribute, so the
compiler tries to compile their `use` statements and test bodies even when the feature
is absent.

Fix: add `#![cfg(feature = "enabled")]` as the first source line to every test file
that imports items from a `enabled`-gated crate API. This is the established pattern —
`clone_dyn_types/tests/additional_corner_cases_test.rs:5` already does it correctly.

## In Scope

All test files (`.rs`) in `tests/` dirs of the 7 known failing crates that lack
a top-level `#![cfg(feature = "enabled")]` guard and import crate symbols.

### Known affected files

| Crate | Path | File |
|-------|------|------|
| clone_dyn_types | `module/core/clone_dyn_types/tests/` | `clone_arrays_test.rs` |
| clone_dyn_types | `module/core/clone_dyn_types/tests/` | `clone_tuples_test.rs` |
| clone_dyn_types | `module/core/clone_dyn_types/tests/` | `smoke_test.rs` |
| clone_dyn_types | `module/core/clone_dyn_types/tests/` | `tests.rs` |
| color_tools | `module/core/color_tools/tests/` | `colorful_text_test.rs` |
| implements | `module/experimental/implements/tests/` | `tests.rs` |
| is_slice | `module/experimental/is_slice/tests/` | `is_slice_tests.rs` |
| component_model_types | `module/experimental/component_model_types/tests/` | `smoke_test.rs` |
| component_model_types | `module/experimental/component_model_types/tests/` | `corner_cases.rs` |

Note: `collection_tools` and `proper_tools` may have additional affected files —
verify by running `will .test module/core/collection_tools/ dry:0 with_none_features:1`
and checking the actual error output.

## Out of Scope

- Changing any `src/` code or feature definitions in Cargo.toml
- Fixing unrelated test failures
- Running `cargo fmt` (forbidden by project rules)

## Work Procedure

1. For each crate in the failing list, run locally with none-features to reproduce:
   `cd module/core/clone_dyn_types && cargo test --no-default-features 2>&1 | head -40`
2. For each test file that emits E0432/E0433: add `#![cfg(feature = "enabled")]`
   as the **very first line** of the file (before any doc comments or `use` statements)
3. Re-run the same `cargo test --no-default-features` command — must produce zero compile errors
   (if no tests run because all are cfg'd out, that is the expected outcome)
4. After fixing all 7+ crates, run `w3 .test level::3` in each fixed crate directory to
   confirm no regressions with default features
5. Commit all test file changes together

## Outcomes

After completion:
- Zero E0432/E0433 failures when running `cargo test --no-default-features` in any of the 7 crates
- `workspace_push.yml` matrix: all jobs pass the "Run tests" step
- No test behavior changed under normal feature activation (`default = ["enabled"]`)

## Acceptance Criteria

- `cargo test --no-default-features` succeeds (zero compile errors) in each of the 7 crates
- `w3 .test level::3` passes in each of the 7 crates
- CI run after commit shows zero `with_none_features` failures

## Validation

### Checklist

- [ ] C1 — Every affected test file has `#![cfg(feature = "enabled")]` as first line
- [ ] C2 — `cargo test --no-default-features` exits 0 in all 7 crates
- [ ] C3 — `w3 .test level::3` passes in all 7 crates
- [ ] C4 — No new test file added without the guard

### Measurements

- [ ] M1 — count of failing jobs in next CI run → 0 (down from 7+)
- [ ] M2 — per-crate: `cd module/core/clone_dyn_types && cargo test --no-default-features; echo $?` → 0
   (repeat for each of the 7 crates)

### Invariants

- [ ] I1 — No `src/` file modified in this task
- [ ] I2 — No Cargo.toml modified in this task

### Anti-faking checks

- [ ] AF1 — Guard is file-level `#![cfg...]` not test-level `#[cfg...]`: confirm via `head -1` of each modified file
- [ ] AF2 — Run before/after: `cargo test --no-default-features` output before shows errors, after shows "test result: ok"

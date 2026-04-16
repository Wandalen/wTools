# Remove Integration Feature Gate from Test Files

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)

## Goal

All 11 test files gated with `#![cfg(feature = "integration")]` run unconditionally,
removing a redundant legacy feature flag while preserving identical test behavior under
`w3 .test l::3`.

MOST breakdown:
- **Motivated** — `integration = []` is an empty legacy flag; tests only run with
  `--all-features`, not with `cargo nextest run` alone. Removing the gate makes tests
  first-class citizens that run in all contexts.
- **Observable** — gate is absent from all 11 files; `w3 .test l::3` passes with zero
  failures and zero warnings; running `cargo nextest run` (no flags) also compiles and
  runs the affected tests.
- **Scoped** — delete one `#![cfg(feature = "integration")]` line from each of the 11
  files; no logic changes; no new files.
- **Testable** — `w3 .test l::3` green; `grep -rn 'cfg(feature = "integration")' tests/`
  returns zero matches.

## In Scope

- Remove `#![cfg(feature = "integration")]` from 11 test files:
  `data.rs`, `builder.rs`, `column_data.rs`, `debug_alignment.rs`, `flatten_config.rs`,
  `fluent_api.rs`, `formatters.rs`, `reproduce_alignment_problem.rs`,
  `reproduce_willbe3_alignment.rs`, `table_styles_outputs.rs`, `verify_alignment_correct.rs`
- Fix any compilation errors exposed by the removal

## Out of Scope

- Removing the `integration = []` feature entry from `Cargo.toml` (kept for downstream
  compatibility — marked "Legacy integration flag")
- Any logic changes to tests

## Description

The `integration = []` feature was added as a legacy organizational flag. Because it is
an empty feature, gating tests behind it means they only compile and run when
`--all-features` (or `--features integration`) is passed. The standard dev command
`w3 .test l::3` uses `--all-features`, so in practice the tests do run. However, running
`cargo nextest run` alone silently skips 11 test files. Removing the gate makes the tests
unconditional, which is the correct long-term state.

## Requirements

- All work must strictly adhere to all applicable rulebooks (`kbase .rulebooks`)
- No logic changes to any test
- No new files created

## Acceptance Criteria

- `grep -rn 'cfg(feature = "integration")' tests/` returns zero matches
- `cargo nextest run` (without `--all-features`) compiles and runs all previously-gated tests
- `w3 .test l::3` passes with zero failures and zero warnings

## Work Procedure

1. For each of the 11 files: remove the `#![cfg(feature = "integration")]` line
2. Run `w3 .test l::3` — confirm green
3. Verify: `grep -rn 'cfg(feature = "integration")' tests/` → zero matches
4. Update task status in `task/readme.md`

## Validation List

- [x] `grep -rn 'cfg(feature = "integration")' tests/` returns zero matches?
- [x] `w3 .test l::3` passes with zero failures, zero warnings? *(309 nextest + 73 doc, 0 clippy)*
- [x] Were any compilation errors introduced by the removal and fixed? *(None — clean removal)*

## Validation Procedure

**VP1 — Gate absence**
`grep -rn 'cfg(feature = "integration")' tests/` — expect zero matches.

**VP2 — Full test suite**
`w3 .test l::3` — expect 0 failures, 0 warnings.

## Outcomes

*(Completed. Task delivered and verified per acceptance criteria.)*

# Add feature-flag line filtering passthrough test — FT-41

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Reopen Count:** 0
- **State:** ✅ (Completed)
- **Closes:** null
- **Blocked Reason:** null
- **Dir:** .
- **Validated By:** MAAV(VG-1..VG-4 PASS)
- **Validation Date:** 2026-06-23

## Goal

Add a test that verifies FT-41 — `process_output` passes content through unchanged with `lines_omitted == 0` when compiled without cli_fmt's `string_split` feature — (Motivated: the passthrough implementation at `src/output.rs:387-394` exists for feature configurations where `string_split` or `std` is absent, but the existing `output` composite feature always enables both, making the passthrough structurally unreachable from the current test suite; a dedicated minimal feature `output_passthrough` is required to compile the output module without `string_split`, making the passthrough path reachable for testing; Observable: a new test function `feature_flag_line_filtering_passthrough` in `tests/output_passthrough.rs` passes under `cargo nextest run --test output_passthrough --features output_passthrough`; `tests/docs/feature/001_output_processing.md` FT-41 `⏳` marker removed; `w3 .test l::3` passes; Scoped: `Cargo.toml`, `src/lib.rs`, `tests/output_passthrough.rs`, `tests/readme.md`, and `tests/docs/feature/001_output_processing.md` — no changes to `src/output.rs` logic or `tests/output.rs`; Testable: `cargo nextest run --test output_passthrough --features output_passthrough` shows one PASS, `w3 .test l::3` shows 0 failures).

## In Scope

- `Cargo.toml` — add `output_passthrough = [ "enabled", "std" ]` feature; do NOT add to `full` or `default`
- `src/lib.rs` — expand `pub mod output` gate from `#[ cfg( feature = "output" ) ]` to `#[ cfg( any( feature = "output", feature = "output_passthrough" ) ) ]`; apply the same expansion to the two direct `output::orphan::*` re-exports (`own` mod and `prelude` mod); `exposed` and `orphan` inherit without direct re-exports and need no change
- `tests/output_passthrough.rs` — new test file, file-level gate `#![ cfg( feature = "output_passthrough" ) ]`; contains one test function `feature_flag_line_filtering_passthrough`; does NOT import `strs_tools::string::lines::*`; does NOT duplicate the file-level `#![cfg(feature = "output")]` gate present in `tests/output.rs`
- `tests/readme.md` — register new test file in Responsibility Table; update test count in Test Coverage section
- `tests/docs/feature/001_output_processing.md` — remove `⏳` from FT-41 entry in `### Tests` table; replace with `FT-41: \`feature_flag_line_filtering_passthrough\``

## Out of Scope

- `src/output.rs` — no logic changes; passthrough implementation is verified as-is
- `tests/output.rs` — no changes; existing file-level gate and all 53 tests remain unchanged
- `tests/help.rs` — not involved
- Other feature flags — only `output_passthrough` is added; existing feature semantics unchanged
- The `full` and `default` feature sets — `output_passthrough` must NOT be added to either

## Requirements

- 2-space indentation per codestyle rulebook
- `output_passthrough` must activate `enabled` and `std` only — not `string_split`; this is what makes the passthrough fire
- The test function must assert both `result.content == input` and `result.lines_omitted == 0`
- Assertion messages must include `(FT-41)` for traceability
- `tests/output_passthrough.rs` must have a file-level doc comment explaining why the file exists (passthrough test for feature-flag path)

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Confirm passthrough is unreachable from standard suite** — Verify that `output = [ "enabled", "std", "string_split" ]` in `Cargo.toml` means the `not(all(string_split, std))` guard at `src/output.rs:387` never fires when compiled with `--all-features` or `--features output`. Confirm `tests/output.rs` is gated with `#![ cfg( feature = "output" ) ]`, making all existing tests incompatible with a no-`string_split` build.
2. **Add `output_passthrough` to `Cargo.toml`** — Insert `output_passthrough = [ "enabled", "std" ]` in the `[features]` section, after the `output` line and before `cli_help_template`. Do NOT add to `full` or `default`.
3. **Expand gate in `src/lib.rs`** — Change `#[ cfg( feature = "output" ) ]` on `pub mod output;` to `#[ cfg( any( feature = "output", feature = "output_passthrough" ) ) ]`. Apply the same expansion to the two `output::orphan::*` re-exports: the one in `own` mod and the one in `prelude` mod. (`exposed` inherits via `own::*` and needs no direct change.)
4. **Create `tests/output_passthrough.rs`** — New file. File-level gate `#![ cfg( feature = "output_passthrough" ) ]`. Include a doc comment explaining purpose. Import `use cli_fmt::output::*;`. Add one test function:
   ```rust
   #[ test ]
   fn feature_flag_line_filtering_passthrough()
   {
     let input = "line1\nline2\nline3";
     let config = OutputConfig::default().with_head( 2 );
     let result = process_output( input, "", &config );
     assert_eq!( result.content, input, "FT-41: passthrough returns content unchanged when string_split absent" );
     assert_eq!( result.lines_omitted, 0, "FT-41: passthrough reports zero lines omitted when string_split absent" );
   }
   ```
5. **Run passthrough test** — `cargo nextest run --test output_passthrough --features output_passthrough` — must show `feature_flag_line_filtering_passthrough` PASSED.
6. **Update `tests/readme.md`** — Add `output_passthrough.rs` row to Responsibility Table; update test count comment.
7. **Update `tests/docs/feature/001_output_processing.md`** — In `### Tests` table, remove `⏳ FT-41: \`feature_flag_line_filtering_passthrough\` (pending task 008)` and replace with `FT-41: \`feature_flag_line_filtering_passthrough\``.
8. **Run Level 3** — `w3 .test l::3` — must pass with 0 failures and 0 clippy warnings.

## Test Matrix

| # | Input / Config | Target Test Function | Expected Assertion |
|---|----------------|---------------------|-------------------|
| T01 | `"line1\nline2\nline3"`, `OutputConfig::default().with_head(2)`, compiled with `output_passthrough` (no `string_split`) | `feature_flag_line_filtering_passthrough` | `result.content == "line1\nline2\nline3"` — passthrough returns content unchanged; `result.lines_omitted == 0` — head limit silently ignored |

## Acceptance Criteria

- `Cargo.toml` has `output_passthrough = ["enabled", "std"]` and it is NOT in `full` or `default`
- `src/lib.rs` `pub mod output` gate reads `any(feature = "output", feature = "output_passthrough")`
- `tests/output_passthrough.rs` exists with the correct file-level gate and one test function
- `cargo nextest run --test output_passthrough --features output_passthrough` shows `feature_flag_line_filtering_passthrough` PASSED
- FT-41 `⏳` marker removed from `tests/docs/feature/001_output_processing.md`
- `w3 .test l::3` passes with 0 failures and 0 clippy warnings

## Validation

### Checklist

Desired answer for every question is YES.

- [x] C1 — Does `Cargo.toml` have `output_passthrough = ["enabled", "std"]` and is it absent from `full`/`default`?
- [x] C2 — Does `src/lib.rs` gate `pub mod output` with `any(output, output_passthrough)`?
- [x] C3 — Does `tests/output_passthrough.rs` exist with `#![cfg(feature = "output_passthrough")]` file gate?
- [x] C4 — Does `cargo nextest run --test output_passthrough --no-default-features --features output_passthrough` show the new test PASSED?
- [x] C5 — Is FT-41 `⏳` marker removed from `tests/docs/feature/001_output_processing.md`?
- [x] C6 — Does `w3 .test l::3` pass with 0 failures?

### Measurements

- [x] M1 — `cargo nextest run --test output_passthrough --no-default-features --features output_passthrough 2>&1 | grep PASS` → shows `feature_flag_line_filtering_passthrough`
- [x] M2 — `cargo nextest run --all-features 2>&1 | grep feature_flag` → NOT shown (test does not run under all-features since `output_passthrough` is not in `full`)

### Invariants

- [x] I1 — `w3 .test level::3` → 0 failures, 0 clippy warnings
- [x] I2 — decisions gate: `task/decisions.md` exists and Index shows no ❓ Open entries

## Related Documentation

- `tests/docs/feature/001_output_processing.md` — FT-41 spec case whose `⏳` marker this task resolves
- `docs/feature/001_output_processing.md` — Feature behavioral requirements: "when either [string_split or std] is absent, the filtering stage passes content through unchanged with zero lines reported as omitted"
- `docs/api/001_output_api.md` — `process_output` API contract

## History

- **[2026-06-23]** `CREATED` — Add feature-flag passthrough test for FT-41: introduce `output_passthrough` feature and `tests/output_passthrough.rs` with `feature_flag_line_filtering_passthrough` test function.
- **[2026-06-23]** `COMPLETED` — All acceptance criteria satisfied. `Cargo.toml` has `output_passthrough = ["enabled", "std"]` (absent from `full`/`default`). `src/lib.rs` gates `pub mod output` and two `orphan::*` re-exports with `any(output, output_passthrough)`. `tests/output_passthrough.rs` created with linter-refined gate `all(output_passthrough, not(string_split))`. FT-41 `⏳` marker removed from `tests/docs/feature/001_output_processing.md`; dedicated row added for `output_passthrough.rs`. Passthrough PASS: `feature_flag_line_filtering_passthrough` PASSED with `--no-default-features --features output_passthrough`. Level 3 PASS: 85/85 nextest + 6 doc + 0 clippy. Correction over task spec: invocation requires `--no-default-features` (default features include `output` → `string_split`, masking the passthrough branch).

## Outcomes

`output_passthrough` feature added to `Cargo.toml` (`enabled + std`, absent from `full`/`default`). Three cfg gates in `src/lib.rs` expanded to `any(output, output_passthrough)`: `pub mod output`, `own::output::orphan::*`, `prelude::output::orphan::*`. `tests/output_passthrough.rs` created with one test function `feature_flag_line_filtering_passthrough`. FT-41 spec entry resolved: `⏳` removed, dedicated `tests/output_passthrough.rs` row added to `### Tests` table. `tests/readme.md` updated: new Responsibility Table row and invocation note.

Passthrough confirmed PASS with `cargo nextest run --test output_passthrough --no-default-features --features output_passthrough`. Level 3 PASS: 85 nextest + 6 doc + 0 clippy (standard suite unchanged).

**Key discovery:** task spec commands were missing `--no-default-features`; default features include `output = ["enabled", "std", "string_split"]`, which activates `string_split` and makes the filtering branch active — defeating the passthrough test. Linter also refined the file-level gate to `all(output_passthrough, not(string_split))` for extra safety.

## Verification Record

- **Verified By:** MAAV — 4 independent subagents (VG-1 Scope Coherence, VG-2 MOST Goal Quality, VG-3 Value/YAGNI, VG-4 Implementation Readiness)
- **Verification Date:** 2026-06-23
- **Result:** PASS — all 4 dimensions pass

**VG-1 Scope Coherence:** PASS — In Scope names 5 specific files with precise change descriptions; Out of Scope names 6 excluded areas; observable outcome has 3 verifiable artifacts; no overlap between In/Out of Scope.

**VG-2 MOST Goal Quality:** PASS — Motivated (structural dead-code gap with named code path and spec obligation); Observable (new test file, function name, ⏳ removal, two shell commands); Scoped (5 named files, 6 excluded); Testable (two concrete commands with expected output, one negative check).

**VG-3 Value/YAGNI (Adversarial):** PASS — FT-41 confirmed present in `tests/docs/feature/001_output_processing.md` line 250 with `⏳` marker; behavioral requirement confirmed in `docs/feature/001_output_processing.md` line 25; `output_passthrough` is test-only infrastructure for existing production code; no YAGNI violation.

**VG-4 Implementation Readiness:** PASS — 8 executable steps with file paths and exact code; Test Matrix has 1 row covering the passthrough scenario; Acceptance Criteria are binary-checkable; technical feasibility confirmed against actual `Cargo.toml`, `src/lib.rs`, and `src/output.rs:387-394`. Minor inaccuracy corrected before record: "four re-exports" changed to "two direct sites" (agent 4 finding; `exposed` inherits via `own::*`, `orphan` has no output re-export).

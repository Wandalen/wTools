# Build Test Surface for genfile_core

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Reopen Count:** 0
- **State:** 🔄 (In Progress)
- **Closes:** null
- **Blocked Reason:** null
- **Dir:** module/core/genfile_core
- **Validated By:** null
- **Validation Date:** null

## Goal

Create the `tests/docs/` test surface for `genfile_core` — 17 feature spec files under `tests/docs/feature/` and 7 invariant spec files under `tests/docs/invariant/` following `test_surface.rulebook.md` conventions (Given/When/Then cases, FT-/IN- case prefixes, NN_ 2-digit file prefixes, minimum case counts), add a `docs/` row to `tests/readme.md` Responsibility Table, fix 58 stale FR-N comments in test files to reference `docs/feature/NNN` IDs, and add targeted test functions covering the four behavioral gaps: `check_missing_mandatory`, `RealFileSystem` I/O, `MemoryFileSystem::create_directory_all` no-op, and error message content. This task is motivated by the test surface audit which found 10 Coverage Gate failures and zero spec files — delivery is blocked until the Coverage Gate passes. Success is measured by: `w3 .test l::3` passes with 0 failures; `grep -r 'FR[0-9]' tests/ --include='*.rs' | wc -l` returns 0; `grep -rh '^| FT-' tests/docs/feature/*.md | wc -l` returns ≥68 (17 files × ≥4 cases each); `grep -rh '^| IN-' tests/docs/invariant/*.md | wc -l` returns ≥14 (7 files × ≥2 cases each); `grep -l 'Responsibility Table' tests/docs/readme.md tests/docs/feature/readme.md tests/docs/invariant/readme.md | wc -l` returns 3; and `grep -c 'docs/' tests/readme.md` returns ≥1 (the `docs/` row is present in `tests/readme.md` Responsibility Table).

## In Scope

- `tests/docs/` directory with `readme.md` and Responsibility Table
- `tests/docs/feature/` directory with `readme.md` and 17 spec files mirroring `docs/feature/001_` through `docs/feature/017_`, each renamed to `NN_` 2-digit prefix (e.g., `docs/feature/001_template_value_trait.md` → `tests/docs/feature/01_template_value_trait.md`), each with ≥4 FT-prefixed Given/When/Then cases
- `tests/docs/invariant/` directory with `readme.md` and 7 spec files mirroring `docs/invariant/001_` through `docs/invariant/007_`, each renamed to `NN_` 2-digit prefix (e.g., `docs/invariant/001_rendering_performance.md` → `tests/docs/invariant/01_rendering_performance.md`), each with ≥2 IN-prefixed cases
- `tests/readme.md` — add `docs/` row to Responsibility Table; add `docs/` to Directory Structure tree
- Stale FR-N comment replacement in all test files under `tests/inc/` and `tests/security.rs` (58 total)
- New test function: `check_missing_mandatory_returns_empty_when_all_filled` in `tests/inc/template_error_test.rs`
- New test function: `check_missing_mandatory_returns_names_when_mandatory_missing` in `tests/inc/template_error_test.rs`
- New test function: `real_file_system_write_creates_parent_dirs_and_file` in `tests/inc/filesystem_test.rs`
- New test function: `real_file_system_read_returns_written_bytes` in `tests/inc/filesystem_test.rs`
- New test function: `memory_file_system_create_directory_all_is_noop` in `tests/inc/filesystem_test.rs`
- New test function: `error_missing_parameters_message_includes_param_name` in `tests/inc/template_error_test.rs`

## Out of Scope

- Benchmark infrastructure for `invariant/001_rendering_performance.md` and `invariant/002_memory_efficiency.md` (requires `benches/` setup and `cargo criterion` — separate task)
- `cargo semver-checks` CI integration for `invariant/007_backward_compatibility.md` (requires CI config changes — separate task)
- `cargo tarpaulin` coverage measurement for `invariant/003_test_coverage.md` (requires CI config — separate task)
- Custom generic type combinations (`Template<CustomV, CustomR, FS>`) — low priority edge cases for later
- Large template (≈10KB) performance tests — belongs in benchmark infrastructure task
- Source code changes (this task adds tests and docs only)

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Spec files must follow `test_surface.rulebook.md` conventions: `NN_` 2-digit prefix, FT-/IN- case IDs, Given/When/Then format, min 4 cases (feature) / min 2 cases (invariant)
- New test functions must follow `test_organization_universal.rulebook.md` — real implementations, no mocking, loud failures
- All files must use 2-space indentation and custom code style per `code_style.rulebook.md`
- `tests/docs/readme.md` must have a Responsibility Table listing all files in that directory
- `tests/docs/feature/readme.md` and `tests/docs/invariant/readme.md` must each have a Responsibility Table

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks and verify preconditions** — `kbase .rulebooks`; note constraints on test spec format, file naming, code style. Verify source directories exist: `ls docs/feature/*.md | grep -v readme | wc -l` → 17; `ls docs/invariant/*.md | grep -v readme | wc -l` → 7. If either count does not match the expected value, STOP and resolve the discrepancy before proceeding — Steps 4 and 5 derive spec file names directly from these source instances.
2. **Verify Test Matrix** — review that the Test Matrix below covers all behavioral scenarios in In Scope. If any scenario is missing a row, add it now before proceeding. The matrix is the contract; implementation (Steps 3–8) satisfies it row by row.
3. **Create spec directory structure** — `tests/docs/`, `tests/docs/feature/`, `tests/docs/invariant/`, each with `readme.md` and Responsibility Table. Update `tests/readme.md` Responsibility Table with `docs/` row and directory tree entry.
4. **Write 17 feature spec files** — one spec per `docs/feature/NNN_*.md` instance. Each spec: `NN_` prefix, ≥4 FT-prefixed Given/When/Then cases covering the scenarios from the feature doc. Derive cases directly from the feature doc Design section and from existing test assertions in the corresponding `tests/inc/` file.
5. **Write 7 invariant spec files** — one spec per `docs/invariant/NNN_*.md` instance. Each spec: ≥2 IN-prefixed cases. For invariants without enforcement infrastructure (001, 002, 003, 004, 007), write cases with `When` describing the tooling command and `Then` as the expected measurement; mark cases `[PENDING — infrastructure not yet in place]`.
6. **Fix stale FR-N references** — in all `tests/inc/*.rs` and `tests/security.rs`, replace `// FR1:` through `// FR20:` with `// docs/feature/NNN:` using the mapping: FR1→001, FR2→002, FR3→003, FR4→004, FR5→005, FR6→005, FR7→005, FR8→006, FR9→007, FR10→008, FR11→009, FR12→008, FR13→010, FR14→011, FR15→012, FR16→013, FR17→014, FR18→016, FR19→017, FR20→017. Concrete example: `// FR1: validate path` becomes `// docs/feature/001: validate path`. Verify with `grep -r 'FR[0-9]' tests/ --include='*.rs' | wc -l` → 0.
7. **Write new test functions** — add the 6 named test functions to `tests/inc/` files listed in In Scope. These are additions to test files only — no `src/` modifications. All 6 tests call behaviors that exist in the current `src/` implementation and are expected to compile and pass without any source changes. Run `w3 .test l::3` after each addition to confirm. If a test fails to compile or fails at runtime, investigate the root cause (likely an unexpected API change or misidentified method signature). Do not delete or skip the failing test — treat it as a task-blocking issue, document the failure in the Outcomes section, and request user review before taking any further action.
8. **Confirm no source changes** — run `git diff src/ && git status --short src/` and verify both commands produce no output (no modifications and no untracked new files in `src/`). All 6 test functions must pass with the `src/` codebase unchanged from before Step 7.
9. **Green state** — `w3 .test l::3` must pass with zero failures and zero warnings.
10. **Refactor if needed** — ensure no test file exceeds 1500 lines; split if needed.
11. **Submit for Validation** — trigger SUBMIT transition (⏳ → 🔍). An independent validator executes the 8-step procedure per `validation.rulebook.md`. A NO or deviation triggers REJECT (🔍 → ⏳); fix all gaps, resubmit.
12. **Update task state** — on validation pass, set ✅ in `task/readme.md`, recalculate advisability to 0 (Priority=0), re-sort index, move file to `task/completed/`.

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | tests/docs/ directory created | tests/readme.md | Responsibility Table has `docs/` row; Directory Structure tree shows `docs/` |
| T02 | tests/docs/feature/ contains 17 files | `docs/feature/` instance count confirmed in Step 1 (expected: 17) | `ls tests/docs/feature/*.md \| grep -v readme \| wc -l` → 17 (matches Step 1 verified count); all file names use `NN_` 2-digit prefix |
| T03 | tests/docs/invariant/ contains 7 files | `docs/invariant/` instance count confirmed in Step 1 (expected: 7) | `ls tests/docs/invariant/*.md \| grep -v readme \| wc -l` → 7 (matches Step 1 verified count); all file names use `NN_` 2-digit prefix |
| T04 | Feature spec file naming | test_surface.rulebook.md § Naming | All spec files use 2-digit `NN_` prefix (not 3-digit) |
| T05 | Feature spec case IDs | test_surface.rulebook.md § Case Format | All feature cases use `FT-` prefix; invariant cases use `IN-` prefix |
| T06 | Feature spec minimum cases | Each spec file in `tests/docs/feature/` and `tests/docs/invariant/` | `grep -rh '^| FT-' tests/docs/feature/*.md | wc -l` returns ≥68; `grep -rh '^| IN-' tests/docs/invariant/*.md | wc -l` returns ≥14 |
| T07 | Stale FR-N refs in test files | test files `tests/inc/*.rs`, `tests/security.rs` | `grep -r 'FR[0-9]' tests/ --include='*.rs'` returns 0 matches |
| T08 | check_missing_mandatory — all filled | `Template` with all mandatory params filled | Returns empty Vec<String> |
| T09 | check_missing_mandatory — one missing | `Template` with one mandatory param unfilled | Returns Vec containing that param name |
| T10 | RealFileSystem write — creates parent dirs | `RealFileSystem::write` to nested path in a temp dir | Parent dirs and file created on disk; path exists and is non-empty after write (read-back byte equality verified separately by T14) |
| T11 | MemoryFileSystem create_directory_all | `MemoryFileSystem::create_directory_all(any_path)` | Returns `Ok(())`; subsequent `read` on a path under that directory returns `Err` indicating the file does not exist (no phantom directory entries stored in the in-memory map) |
| T12 | Error::MissingParameters message | `Error::MissingParameters(vec!["foo"])` | `to_string()` output contains `"foo"` |
| T14 | RealFileSystem read — returns written bytes | `RealFileSystem::read` after a `write` call | Returns `Ok(s)` where `s` is byte-for-byte identical to the content passed to `write`; test uses a unique temp dir per test run |
| T13 | All existing tests still pass | Full test suite | `w3 .test l::3` → 0 failures, 0 warnings |

## Acceptance Criteria

- `tests/docs/feature/` contains exactly 17 spec files with `NN_` prefix names
- `tests/docs/invariant/` contains exactly 7 spec files with `NN_` prefix names
- Each feature spec has ≥4 FT-prefixed Given/When/Then cases
- Each invariant spec has ≥2 IN-prefixed cases
- `tests/docs/readme.md`, `tests/docs/feature/readme.md`, `tests/docs/invariant/readme.md` all exist and contain Responsibility Tables
- `tests/readme.md` Responsibility Table has a `docs/` row; Directory Structure tree includes `docs/`
- `grep -r 'FR[0-9]' tests/ --include='*.rs' | wc -l` returns 0
- `check_missing_mandatory` is covered by ≥2 dedicated test functions in `template_error_test.rs`
- `RealFileSystem::write` to a nested path creates all parent dirs and the file on real disk; `RealFileSystem::read` on the same path returns content byte-for-byte identical to what was written
- `MemoryFileSystem::create_directory_all` no-op behavior has a dedicated test
- `Error::MissingParameters` message content (contains param name) has a dedicated test
- Every Test Matrix row has a corresponding passing test
- `w3 .test l::3` passes with 0 failures and 0 warnings

## Validation

**Execution:** The procedure for walking this section is defined in `validation.rulebook.md`. The executor does NOT self-validate — an independent validator performs the walk after SUBMIT transition (⏳ → 🔍).

### Checklist

Desired answer for every question is YES.

**Test Surface Structure**
- [ ] C1 — Does `tests/docs/` exist with `readme.md` containing a Responsibility Table?
- [ ] C2 — Does `tests/docs/feature/` contain exactly 17 spec files?
- [ ] C3 — Does `tests/docs/invariant/` contain exactly 7 spec files?
- [ ] C4 — Do all spec files use `NN_` (2-digit, not 3-digit) prefix?
- [ ] C5 — Do all feature spec cases use `FT-` prefix?
- [ ] C6 — Do all invariant spec cases use `IN-` prefix?

**Cross-Reference Cleanup**
- [ ] C7 — Does `grep -r 'FR[0-9]' tests/ --include='*.rs'` return 0 matches?
- [ ] C8 — Does `tests/readme.md` have a `docs/` row in its Responsibility Table?

**New Test Coverage**
- [ ] C9 — Does `tests/inc/template_error_test.rs` contain `check_missing_mandatory` test functions?
- [ ] C10 — Does `tests/inc/filesystem_test.rs` contain real filesystem write/read tests?
- [ ] C11 — Does `tests/inc/filesystem_test.rs` contain a `create_directory_all` no-op test?
- [ ] C12 — Does any test assert that `Error::MissingParameters` message contains the param name?

**Out of Scope Confirmation**
- [ ] C13 — Is `benches/` absent (no benchmark infrastructure added)?
- [ ] C14 — Are all changes confined to `tests/` (no `src/` modifications)?

### Measurements

- [ ] M1 — spec file count: `ls tests/docs/feature/*.md | grep -v readme | wc -l` → 17 (was: 0)
- [ ] M2 — invariant spec count: `ls tests/docs/invariant/*.md | grep -v readme | wc -l` → 7 (was: 0)
- [ ] M3 — stale FR refs: `grep -r 'FR[0-9]' tests/ --include='*.rs' | wc -l` → 0 (was: 58)
- [ ] M4 — test suite: `w3 .test l::3` → 0 failures (was: 0 failures)
- [ ] M5 — readme files: `grep -l 'Responsibility Table' tests/docs/readme.md tests/docs/feature/readme.md tests/docs/invariant/readme.md | wc -l` → 3 (was: 0)
- [ ] M6 — feature case count: `grep -rh '^| FT-' tests/docs/feature/*.md | wc -l` → ≥68 (was: 0)
- [ ] M7 — invariant case count: `grep -rh '^| IN-' tests/docs/invariant/*.md | wc -l` → ≥14 (was: 0)
- [ ] M8 — tests/readme.md docs/ row: `grep -c 'docs/' tests/readme.md` → ≥1 (was: 0)

### Invariants

- [ ] I1 — test suite: `w3 .test l::3` → 0 failures
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check` → 0 warnings

### Anti-faking checks

- [ ] AF1 — spec file content: `head -20 tests/docs/feature/01_template_value_trait.md` → contains `FT-01` case ID
- [ ] AF2 — FR-N cleanup: `grep -r 'FR1\b\|FR2\b\|FR3\b' tests/ --include='*.rs'` → 0 matches
- [ ] AF3 — new tests: `grep -n 'check_missing_mandatory\|create_directory_all.*noop\|MissingParameters.*message' tests/inc/template_error_test.rs tests/inc/filesystem_test.rs` → finds test functions

## Outcomes

*(Steps 1–10 completed 2026-05-31. Awaiting formal validation (Step 11).)*

**Steps completed:**
- Steps 1–5: test surface structure, 17 feature specs + 7 invariant specs created in a prior session
- Step 6: 0 FR-N refs remain in tests/ (was 58)
- Step 7: 6 new test functions added to `tests/inc/` — all 169 tests pass
- Step 8: no src/ changes from this task; pre-existing uncommitted changes (`D src/archive.rs`, `M src/error.rs`) are from an unrelated refactor of the archive module (archive.rs split into archive/ dir)
- Step 9: `w3 .test l::3` → 169 passed, 0 failed, 0 warnings
- Step 10: no test file exceeds 1500 lines (max: 830 lines in `inc/archive_test.rs`)

**Measurements (final):**
- M1: 17 feature spec files ✅
- M2: 7 invariant spec files ✅
- M3: 0 FR refs ✅
- M4: 0 test failures ✅
- M5: 3 readme files with Responsibility Table ✅
- M6: 69 FT- cases (≥68) ✅
- M7: 14 IN- cases (≥14) ✅
- M8: 4 docs/ refs in tests/readme.md (≥1) ✅

## Verification Record

- **Date:** 2026-05-31
- **Result:** PASS — all 4 dimensions passed on 9th verification gate run
- **D1 Scope Coherence:** PASS — In/Out Scope non-contradicting; T10/T14 non-overlapping; Step 1 STOP condition; Step 7 blocks silent test deletion
- **D2 MOST Goal:** PASS — all 6 observable commands present in Goal section and M1–M8; concrete motivation (10 Coverage Gate failures blocking delivery); scoped to `genfile_core/tests/`
- **D3 Value/YAGNI:** PASS — every In Scope item traces to a measured gap; Out of Scope properly defers benchmark/CI infrastructure
- **D4 Implementation Readiness:** PASS — all T-rows concrete and observable; T10 write / T14 read distinct; byte-for-byte language in T14; Step 7 escalate-not-delete; Step 8 git diff + git status

## History

*(append-only — newest entry last; never edit or remove past entries)*

- **[2026-05-31]** `CREATED` — Task filed after test surface audit found 10 Coverage Gate failures and 16 behavioral gaps (P1–P16). Goal: unblock delivery by creating tests/docs/ and filling critical behavioral gaps.
- **[2026-05-31]** `VERIFIED` — Passed 4-dimension Verification Gate (D1/D2/D3/D4 all PASS). Promoted from unverified/.
- **[2026-05-31]** `IN_PROGRESS` — Steps 1–10 complete; 6 new test functions added (169 tests pass, 0 failures). All M1–M8 measurements pass. Pre-existing src/ changes (archive refactor) confirmed unrelated to this task. Awaiting validation (Step 11).

# Wire and verify compile-fail test infrastructure for collection_tools

## Execution State

- **Executor Type:** any
- **Actor:** claude-sonnet-4-6
- **Claimed At:** 2026_05_04
- **Status:** ✅ (Complete)

## Goal

Wire the collection_tools compile-fail test infrastructure and achieve a Level 3 pass so FT-02, FT-06, FT-07, and AP-09 are actively verified by automated tests. (Motivated: four spec cases in `tests/docs/` are marked ✅ but backed by untracked test files that have never been committed or confirmed to pass against the current compiler; Observable: `compile_fail_test.rs::compile_fail`, and all three functions in `feature_gate_compile_fail_test.rs` pass under `--all-features`; Scoped: `tests/compile_fail_test.rs`, `tests/feature_gate_compile_fail_test.rs`, `tests/compile_fail/`, `Cargo.toml` dev-dependencies; Testable: `cd module/core/collection_tools && w3 .test level::3` exits 0.)

Two compile-fail test drivers cover the four ⚠️ spec cases: `compile_fail_test.rs` runs trybuild against `tests/compile_fail/into_hmap_no_annotation.rs` (FT-02); `feature_gate_compile_fail_test.rs` uses subprocess `cargo check` to verify feature-gate isolation (FT-07, FT-06, AP-09). Both files exist on disk but are untracked — the trybuild golden file `into_hmap_no_annotation.stderr` was generated at authorship time but may need regeneration if the compiler version has changed.

The Cargo.toml already contains `trybuild = { workspace = true }` in dev-dependencies. No additional Cargo entries are expected to be needed, but `[[test]]` entries with `required-features` may be required if the auto-discovered tests fail without features.

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/compile_fail_test.rs` — verify trybuild runner works and test function fires under `--all-features`
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/feature_gate_compile_fail_test.rs` — verify all 3 subprocess feature-gate tests pass
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/compile_fail/into_hmap_no_annotation.rs` — verify fixture is correct
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/compile_fail/into_hmap_no_annotation.stderr` — regenerate if compiler output changed
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/Cargo.toml` — add `[[test]]` entries with `required-features` if needed

## Out of Scope

- Documentation updates (already completed by doc_tsk)
- Changes to existing runtime tests (`tests.rs`, `manual_corner_cases_test.rs`, `no_std_alloc_test.rs`, `heap_macro_availability_test.rs`, `inc/`)
- Changes to `module/experimental/config_hierarchy/` or any other crate

## Requirements

- All work must strictly adhere to all applicable rulebooks
  (discover via `kbase .rulebooks`)
- Trybuild golden files must be generated from actual compiler output — never hand-edited to match
- No new `#[ignore]` attributes — if a test can't run in CI, fix the root cause
- `feature_gate_compile_fail_test.rs` uses subprocess `cargo check`; the `CARGO` env var must be respected (already in the file at line 46)

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks` from `module/core/collection_tools/`; note test_organization rules for compile-fail patterns.
2. **Read spec docs** — Read `tests/docs/feature/02_into_constructors.md` (FT-02, FT-06), `tests/docs/feature/01_collection_constructors.md` (FT-07), `tests/docs/api/01_collection_macros.md` (AP-09) as source of truth for expected compile-fail behavior.
3. **Run current test suite** — `cd module/core/collection_tools && RUSTFLAGS="-D warnings" cargo nextest run --all-features --no-fail-fast` to identify which tests pass and which fail.
4. **Fix trybuild golden file if stale** — If `compile_fail_test.rs` fails because `.stderr` doesn't match current output: delete `tests/compile_fail/into_hmap_no_annotation.stderr`, then run `RUSTFLAGS="-D warnings" cargo test --test compile_fail_test --features collection_into_constructors 2>&1` once to regenerate; trybuild writes the new golden file automatically on first run.
5. **Fix subprocess tests if failing** — If `feature_gate_compile_fail_test.rs` tests fail: inspect the `cargo check` output from within the test (add `eprintln!` temporarily); common causes: wrong `CARGO_TARGET_DIR`, feature name mismatch, or path resolution for `CARGO_MANIFEST_DIR`.
6. **Add `[[test]]` entries if needed** — If any test requires specific features to avoid compilation errors or noise, add `[[test]]` block with `required-features` to `Cargo.toml`; match the pattern of `module/experimental/config_hierarchy/Cargo.toml` entries.
7. **Validate** — Run `w3 .test level::3` from `module/core/collection_tools/` → all tests pass, 0 warnings, 0 clippy issues.
8. **Walk Validation Checklist** — Check every item. Every answer must be YES.
9. **Update task status** — Mark 🎯 → ✅ in `task/readme.md` Tasks Index; move file to `task/completed/`; write Outcomes.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `into_hmap!{"a" => 1}` without type annotation | `compile_fail/into_hmap_no_annotation.rs` via trybuild | Compilation fails; compiler error matches `.stderr` golden file |
| `collection_tools::vec![1,2,3]` with only `enabled` feature | `feature_gate_compile_fail_test.rs::strict_macros_absent_without_collection_constructors` | `cargo check` exits non-zero; test asserts true |
| `collection_tools::vec![1,2,3]` with `enabled + collection_into_constructors` | `feature_gate_compile_fail_test.rs::strict_macros_absent_with_only_into_feature` | `cargo check` exits non-zero; test asserts true |
| `collection_tools::into_hmap!{"a"=>1}` with `enabled + collection_constructors` | `feature_gate_compile_fail_test.rs::into_macros_absent_with_only_strict_feature` | `cargo check` exits non-zero; test asserts true |
| All existing runtime tests | `--all-features` | All previously passing tests continue to pass; 0 regressions |

## Acceptance Criteria

- `cargo nextest run -p collection_tools --all-features` exits 0 with all tests passing, including the 4 compile-fail test functions
- `compile_fail_test.rs::compile_fail` exists, is not ignored, and passes under `--all-features`
- All 3 functions in `feature_gate_compile_fail_test.rs` (`strict_macros_absent_without_collection_constructors`, `strict_macros_absent_with_only_into_feature`, `into_macros_absent_with_only_strict_feature`) pass
- `tests/compile_fail/into_hmap_no_annotation.stderr` file exists with non-trivial content (>100 bytes) matching current compiler output
- `w3 .test level::3` from `module/core/collection_tools/` exits 0

## Validation

### Checklist

Desired answer for every question is YES.

**Compile-fail test functions**
- [ ] Does `compile_fail_test.rs::compile_fail` test function exist and pass under `--all-features`?
- [ ] Do all 3 functions in `feature_gate_compile_fail_test.rs` pass?
- [ ] Is trybuild using the current compiler output (golden file matches and does not regenerate)?

**Infrastructure**
- [ ] Does `tests/compile_fail/into_hmap_no_annotation.stderr` exist with >100 bytes?
- [ ] Is `trybuild = { workspace = true }` present in `Cargo.toml` dev-dependencies?

**No regressions**
- [ ] Do all previously passing tests still pass (124/124 nextest + 60 doc tests)?

**Out of Scope confirmation**
- [ ] Are `tests.rs`, `manual_corner_cases_test.rs`, `no_std_alloc_test.rs`, `heap_macro_availability_test.rs`, and `inc/` contents unchanged?

### Measurements

**M1 — Compile-fail functions present**
Command: `grep -c "^fn " /home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/feature_gate_compile_fail_test.rs`
Before: 4 (check_compile_fails helper + 3 test fns). Expected: 4. Deviation: functions removed or renamed.

**M2 — Level 3 gate**
Command: `w3 .test level::3` from `module/core/collection_tools/`
Before: unknown (untracked files not run by CI). Expected: exit 0. Deviation: any failure or warning.

**M3 — Golden file size**
Command: `wc -c /home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/compile_fail/into_hmap_no_annotation.stderr`
Before: 1.7K (1700 bytes). Expected: >100 bytes. Deviation: empty or absent file.

### Invariants

- [ ] I1 — full suite: `cargo nextest run -p collection_tools --all-features` → 0 failures
- [ ] I2 — clippy clean: `cargo clippy -p collection_tools --all-targets --all-features -- -D warnings` → exit 0

### Anti-faking checks

**AF1 — Test functions not stubs**
Check: `grep -c "assert!" /home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/feature_gate_compile_fail_test.rs`
Expected: 3. Why: each of the 3 test functions must have a real assertion — not just be an empty pass.

**AF2 — Golden file is non-trivial**
Check: `wc -c /home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/compile_fail/into_hmap_no_annotation.stderr`
Expected: >100 bytes. Why: a trivially empty `.stderr` makes trybuild pass without verifying actual compiler output.

**AF3 — compile_fail_test guarded correctly**
Check: `grep -c "cfg.*collection_into_constructors" /home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/compile_fail_test.rs`
Expected: 1. Why: test function must be cfg-guarded to only run when the relevant feature is active.

## Outcomes

All 4 compile-fail test functions verified passing at Level 3 (2026_05_04).

**Validation results:**

| Check | Result |
|-------|--------|
| `compile_fail_test.rs::compile_fail` exists and passes | YES — 7.9s trybuild run, golden file matched |
| All 3 feature-gate functions pass | YES — all 3 subprocess `cargo check` calls exit non-zero as expected |
| Golden file `.stderr` is non-trivial | YES — 1764 bytes |
| `trybuild` in Cargo.toml dev-dependencies | YES — line 64 |
| Level 3 PASS (`w3 .test level::3`) | YES — 128/128 nextest + 60 doc + 0 clippy |
| No regressions (all 124 prior tests still pass) | YES — 128 total = 124 prior + 4 new compile-fail tests |

**Anti-faking results:**

| Check | Expected | Actual | Pass |
|-------|----------|--------|------|
| AF1: `assert!` count in feature_gate file | 3 | 3 | YES |
| AF2: `.stderr` size | >100 bytes | 1764 bytes | YES |
| AF3: cfg guard count in compile_fail_test | 1 | 1 | YES |
| M1: `fn` count in feature_gate file | 4 | 4 | YES |

**Files verified:**
- `tests/compile_fail_test.rs` — trybuild runner for FT-02, cfg-gated on `collection_into_constructors`
- `tests/feature_gate_compile_fail_test.rs` — subprocess cargo check for FT-07, into/FT-06, AP-09
- `tests/compile_fail/into_hmap_no_annotation.rs` — trybuild fixture; line 15 matches golden file reference
- `tests/compile_fail/into_hmap_no_annotation.stderr` — 1764-byte golden file from current compiler

No file modifications required — infrastructure was correctly wired and all tests passed on first verification run.

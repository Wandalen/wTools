# Create test surface and fill test coverage gaps

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Start Time:** null
- **Prior State:** null
- **Reopen Count:** 0
- **State:** ❓ (Unverified)
- **Closes:** null
- **Dir:** module/core/strs_tools
- **Validated By:** null
- **Validation Date:** null

## Goal

Create a complete `tests/docs/` test surface (18 spec files + 5 readmes) and fill the 3 remaining test code coverage gaps: SIMD acceleration (zero tests), expand thin indentation tests, and expand thin number parsing tests. ANSI test code already exists and is wired via `inc/mod.rs`. Between-isolation and count-limit are doc-reality gaps (documented but not implemented) — docs updated to mark these as planned extensions. Success: all 18 spec files have ≥ minimum case counts with ≥ 80% of cases at ✅ status, and `w3 .test level::3` passes with the new test code.

## In Scope

- Create `tests/docs/` directory tree: 18 spec files (8 feature + 3 api + 4 invariant + 3 algorithm) and 5 readme.md files
- Populate each spec with Given/When/Then cases meeting minimum counts (FT≥4, AP≥4, IN≥2, AC≥4)
- Write test code for F-007/I-003/G-001 SIMD: feature activation, scalar-vs-SIMD equivalence, delimiter search algorithm
- Expand F-002 indentation tests: empty source, newlines-only, long prefix boundaries
- Expand F-004 number parsing tests: scientific notation, invalid input, overflow boundaries
- Update `tests/readme.md` with `docs/` entry
- ~~Write ANSI test code~~ — CANCELLED: all 5 operations already have dedicated test files in `tests/inc/ansi_*_test.rs`, now wired into `inc/mod.rs`
- ~~Write between-isolation tests~~ — CANCELLED: `isolate_between()` not implemented (doc-reality gap; docs updated)
- ~~Write count-limit tests~~ — CANCELLED: `BasicSplitBuilder` has no limit setter (doc-reality gap; docs updated)

## Out of Scope

- Modifying any source code in `src/` — this task is test-only
- Task 011 `visual_width` implementation — separate task
- Upgrading `unicode-width` dependency version — separate task
- Downstream crate changes (`data_fmt`, `cli_fmt`)
- Between-isolation implementation — `isolate_between()` doesn't exist; requires src/ changes (separate feature task)
- Count-limit implementation — `BasicSplitBuilder` has no limit setter; requires src/ changes (separate feature task)

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)

## Delivery Requirements

- Test spec files follow `l1_imp_surface.rulebook.md § Spec` format
- Each spec uses Given/When/Then with type-appropriate prefix (FT-, AP-, IN-, AC-)
- Readme parity: each `tests/docs/*/readme.md` row count = non-readme .md file count
- All new test code compiles and passes: `w3 .test level::3` with zero failures
- No function exceeds 50 lines; public items have `///` doc comments
- Independent validation passes per `validation.rulebook.md`
- Task state updated to ✅ on validation pass; file moved to `task/completed/`

## Work Procedure

1. ~~Read all 18 `docs/` instances and existing `tests/` code to ground spec content~~ ✅ (completed 2026-06-23)
2. ~~Create `tests/docs/readme.md` with Responsibility Table~~ ✅ (completed 2026-06-23)
3. ~~For each entity type (feature, api, invariant, algorithm):~~ ✅ (completed 2026-06-23)
   ~~a. Create `tests/docs/{type}/readme.md` with Overview Table~~
   ~~b. Create each spec file `tests/docs/{type}/NNN_name.md` with GWT cases~~
   ~~c. Mark existing test coverage as ✅, gaps as ⏳~~
4. ~~Update `tests/readme.md` to add `docs/` directory entry~~ ✅ (completed 2026-06-23)
5. ~~Write ANSI utility test code~~ ❌ CANCELLED (VF-1: all 5 ANSI operations already tested in `tests/inc/ansi_*_test.rs`, now wired into `inc/mod.rs`)
6. Write SIMD test code: `tests/simd_tests.rs` (feature-gated on `simd`) covering SIMD activation, scalar equivalence, delimiter search
7. ~~Add between-isolation test cases~~ ❌ CANCELLED (VF-2: `isolate_between()` not implemented; docs updated to mark as planned)
8. ~~Add count-limit test cases~~ ❌ CANCELLED (VF-3: no limit setter in builder API; docs updated to mark as planned)
9. Expand `tests/inc/indentation_test.rs` with boundary cases (audit existing tests first per VF-5)
10. Expand `tests/inc/number_test.rs` with scientific notation, invalid input, overflow (audit existing tests first per VF-5)
11. Update spec ⏳ markers to ✅ as test code is written
12. Run `w3 .test level::3` and fix any failures

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | Same input, simd enabled | `SplitFastIterator` split | Identical segments to scalar path |
| T02 | Same input, simd disabled | scalar split | Identical segments to SIMD path |
| T03 | Single-byte delimiter, simd on | `SimdStringSearch::find` | Correct byte offset |
| T04 | Multi-delimiter, simd on | `SimdStringSearch::find_any` | All offsets match scalar |
| T05 | Empty string | indentation | Returns empty string |
| T06 | `"\n\n\n"` (newlines only) | indentation | Each line gets prefix+postfix |
| T07 | Long prefix exceeding line width | indentation | Prefix applied without truncation |
| T08 | `"1.5e10"` | number parsing | Parses as f64 scientific notation |
| T09 | `"not_a_number"` | number parsing | Returns parse error |
| T10 | `"99999999999999999999"` | number parsing (i32) | Returns overflow error |
| T11 | `"1.7976931348623157e+308"` | number parsing (f64) | Parses as f64 near max boundary |

## Acceptance Criteria

- 18 spec files exist in `tests/docs/` with correct GWT format and minimum case counts
- 5 readme.md files exist with correct Overview Tables matching file counts
- SIMD feature-gated tests pass with `--all-features` and produce identical results to scalar
- Indentation tests cover empty input, newlines-only, and long prefix boundaries
- Number parsing tests cover scientific notation, invalid input, and overflow boundaries
- `w3 .test level::3` passes with zero failures and zero warnings
- `tests/readme.md` lists `docs/` directory

## Validation

**Execution:** The procedure for walking this section is defined in `validation.rulebook.md`. The executor does NOT self-validate — an independent validator performs the walk after RELEASE transition.

### Checklist

**Test surface structure**
- [ ] C1 — Do 18 spec files exist under `tests/docs/` matching all 18 `docs/` instances?
- [ ] C2 — Do 5 readme.md files exist (root + 4 entity types)?
- [ ] C3 — Does each spec have ≥ minimum case count for its type?
- [ ] C4 — Does each readme Overview Table row count equal non-readme file count?

**SIMD coverage**
- [ ] C5 — Does a SIMD test file exist with `cfg(feature = "simd")` gate?
- [ ] C6 — Does it verify scalar-SIMD output equivalence?

**Expansion coverage**
- [ ] C7 — Does `indentation_test.rs` test empty input and newlines-only?
- [ ] C8 — Does `number_test.rs` test scientific notation and invalid input?

### Measurements

- [ ] M1 — spec file count: `find tests/docs -name '*.md' -not -name 'readme.md' | wc -l` = 18
- [ ] M2 — readme count: `find tests/docs -name 'readme.md' | wc -l` = 5
- [ ] M3 — new SIMD test function count: `grep -r '#\[ *test *\]' tests/simd_tests.rs | wc -l` >= 4

### Invariants

- [ ] I1 — test suite: `w3 .test level::3` passes with 0 failures
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check --all-features` passes

### Anti-faking checks

- [ ] AF1 — spec content: `grep -r 'Given:' tests/docs/ | wc -l` >= 64
- [ ] AF2 — SIMD test real: `grep -c 'simd\|SplitFastIterator\|SimdStringSearch\|scalar' tests/simd_tests.rs` >= 4

## Related Documentation

- `docs/feature/001_string_splitting.md` — F-001 scope (count-limit marked as planned extension)
- `docs/feature/002_text_indentation.md` — F-002 scope (thin coverage)
- `docs/feature/003_string_isolation.md` — F-003 scope (between-isolation marked as planned extension)
- `docs/feature/004_number_parsing.md` — F-004 scope (thin coverage)
- `docs/feature/007_simd_acceleration.md` — F-007 scope (zero tests)
- `docs/invariant/003_simd_fallback_contract.md` — I-003 scope (zero tests)
- `docs/algorithm/001_simd_delimiter_search.md` — G-001 scope (zero tests)
- `tests/docs/` — test surface spec files (created by this task)
- `task/unverified/011_add_visual_width_display_columns.md` — Related: 011 (visual_width in same ANSI module)

## History

- **[2026-06-23]** `CREATED` — Create test surface specs and fill 6 critical coverage gaps (ANSI 4/5, SIMD 0%, between-isolation, count-limit, thin indentation/number tests).
- **[2026-06-23]** `UPDATED` — Test surface (steps 1-4) completed during docs/ normalization session: 18 spec files + 5 readmes created in `tests/docs/`, `tests/readme.md` updated with `docs/` entry. Remaining: test code implementation (steps 5-12).
- **[2026-06-23]** `VERIFY` — MAAV Verification Gate: 2/4 PASS (Scope Coherence, MOST Goal), 2/4 FAIL (Value/YAGNI, Implementation Readiness). Task remains ❓ Unverified. See Verification Findings below.
- **[2026-06-23]** `REVISED` — Applied VF-1/VF-2/VF-3 resolutions: removed ANSI (tests exist), between-isolation (API missing), count-limit (API missing) from scope. Updated docs/ to mark between-isolation and count-limit as planned extensions. Reduced from 6 to 3 coverage gaps. Revised Goal, In Scope, Test Matrix (T01-T11), Acceptance Criteria, Validation Checklist.

## Verification Findings

**Date:** 2026-06-23
**Result:** FAIL (2/4 dimensions)
**Passed:** Scope Coherence, MOST Goal Quality
**Failed:** Value/YAGNI, Implementation Readiness

### Finding VF-1: ANSI coverage claim is factually false

The task claims "4/5 operations untested" for ANSI utilities. This is incorrect — all 5 operations already have dedicated test files with substantial coverage:
- `has_ansi()` — tests in `tests/inc/ansi_detect_test.rs`
- `parse_segments()` — tests in `tests/inc/ansi_parse_test.rs`
- `strip()` — tests in `tests/inc/ansi_strip_test.rs`
- `visual_len()` — tests in `tests/inc/ansi_visual_test.rs`
- `truncate` — tests in `tests/inc/ansi_truncate_test.rs`

**Resolution:** Remove step 5 (creating `tests/ansi_utility_tests.rs`), T01-T05 from Test Matrix, C5-C6 and AF2 from Validation. Or reframe as expanding existing test files with genuinely missing edge cases.

### Finding VF-2: Between-isolation targets unimplemented feature

The task scopes testing of a "between isolation" mode (step 7, T10-T11, C9), but `src/string/isolate.rs` only implements `isolate_left()` and `isolate_right()`. No `isolate_between()` function exists. Writing tests for a non-existent API contradicts the Out of Scope constraint "Modifying any source code in `src/`."

**Resolution:** Remove step 7, T10-T11, C9. Or create a prerequisite implementation task and gate this work on its completion.

### Finding VF-3: Count-limit may target unimplemented feature

The task scopes testing of split count-limit (step 8, T12, C10). The adversarial agent found no `limit`, `max_splits`, or `count` field in the split builder or iterator. If the feature is not implemented, this has the same issue as VF-2.

**Resolution:** Verify whether `SplitFastIterator` or its builder actually exposes a count-limit parameter. If not, remove step 8, T12, C10.

### Finding VF-4: Test Matrix coverage gaps

- Count-limit: only T12 tests limit=2. Missing: limit=0, limit=1, limit exceeding segment count.
- Indentation: "long prefix boundaries" listed in In Scope but no Test Matrix row covers it.
- Number parsing: overflow tested only for i32 (T17). No f64 boundary or underflow scenario.

**Resolution:** Add missing Test Matrix rows for all In Scope scenarios.

### Finding VF-5: Indentation/number expansion may be partially redundant

Existing `indentation_test.rs` may already test empty string and newlines-only. Existing `number_test.rs` may already cover basic invalid input. The task should audit existing tests before declaring gaps.

**Resolution:** Audit existing test files and update In Scope to reflect only genuinely missing scenarios.

# Build API Test Surface for genfile_core

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Reopen Count:** 0
- **State:** 🎯 (Verified)
- **Closes:** null
- **Blocked Reason:** null
- **Dir:** module/core/genfile_core
- **Validated By:** null
- **Validation Date:** null

## Goal

Create the `tests/docs/api/` test surface for `genfile_core` — 4 API contract spec files under `tests/docs/api/` following `test_surface.rulebook.md` conventions (AT- case prefixes, Given/When/Then format, minimum 4 cases each), with a `tests/docs/api/readme.md` Responsibility Table, and an `api/` row added to `tests/docs/readme.md`. This task is motivated by the addition of the `api/` doc entity (4 instances: Template Value API, Parameter API, Generation API, Error Contract) during the doc normalization session on 2026-05-31 — the Coverage Gate for api/ contract behaviors has zero spec files until delivery. Success is measured by: `ls tests/docs/api/*.md | grep -v readme | wc -l` returns 4; each spec has ≥4 AT-prefixed Given/When/Then cases; `tests/docs/readme.md` contains an `api/` row; `w3 .test l::3` passes with 0 failures and 0 warnings.

## In Scope

- `tests/docs/api/` directory with `readme.md` and Responsibility Table listing all 4 spec files
- 4 API contract spec files under `tests/docs/api/`, each using 2-digit `NN_` prefix matching its `docs/api/NNN_*.md` source:
  - `tests/docs/api/01_template_value_api.md` — spec for `docs/api/001_template_value_api.md`
  - `tests/docs/api/02_parameter_api.md` — spec for `docs/api/002_parameter_api.md`
  - `tests/docs/api/03_generation_api.md` — spec for `docs/api/003_generation_api.md`
  - `tests/docs/api/04_error_contract.md` — spec for `docs/api/004_error_contract.md`
- `tests/docs/readme.md` — add `api/` row to the Responsibility Table (create the file if task 001 has not yet run; otherwise add the row to the existing file)

## Out of Scope

- `tests/docs/feature/` spec files — 17 feature specs (→ task 001)
- `tests/docs/invariant/` spec files — 7 invariant specs (→ task 001)
- Stale FR-N comment replacement in test source files (→ task 001)
- New test function implementations in `tests/inc/` (→ task 001)
- Source code changes to `src/` — all api/ contract behaviors are already implemented
- Benchmark or CI infrastructure for performance/memory/coverage invariants (separate task)
- Test helper or fixture infrastructure in `tests/inc/` — spec files only

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Spec files must follow `test_surface.rulebook.md` for api/ entity type: `NN_` 2-digit prefix, AT- case IDs, Given/When/Then format, minimum 4 cases per spec file; if the rulebook specifies a different prefix for api/ entities, use the rulebook-specified prefix
- Each spec case derives directly from the corresponding `docs/api/NNN_*.md` Design section — each case maps to one observable contract behavior callable or matchable by library consumers
- All files must use 2-space indentation per `code_style.rulebook.md`
- `tests/docs/api/readme.md` must have a Responsibility Table listing all files in that directory

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note constraints on test spec format, prefix convention for api/ entities, file naming, and minimum case counts.
2. **Check `tests/docs/` state** — if `tests/docs/readme.md` does not exist (task 001 not yet run), create it with Responsibility Table entries for `feature/`, `invariant/`, and `api/` directories. If it already exists, add only the `api/` row.
3. **Create spec directory** — `tests/docs/api/` with `readme.md` and Responsibility Table listing the 4 spec files to be created.
4. **Derive spec cases** — for each of the 4 api/ instances, read `docs/api/NNN_*.md` Design section and enumerate all observable contract behaviors visible to library consumers. Each distinct behavior becomes one Given/When/Then case.
5. **Write `01_template_value_api.md`** — ≥4 AT-prefixed cases covering: String/Number/Bool/List `to_template_string()` conversions, `is_empty()` semantics, and custom-type plug-in contract.
6. **Write `02_parameter_api.md`** — ≥4 AT-prefixed cases covering: descriptor construction (name, mandatory flag, default value, description), collection construction, mandatory-name listing, and default propagation semantics.
7. **Write `03_generation_api.md`** — ≥4 AT-prefixed cases covering: holder generation with all params filled, archive loading and generation, missing-mandatory pre-check, and template vs. static content dispatch.
8. **Write `04_error_contract.md`** — ≥4 AT-prefixed cases covering: all 4 error variant identities (render failure, missing parameters, filesystem I/O, invalid template), user-fixable vs. system-error distinction, and diagnostic context (param name / file path / template context) present in error messages.
9. **Verify structure** — `ls tests/docs/api/*.md | grep -v readme | wc -l` → 4; inspect each spec to confirm ≥4 AT-prefixed cases.
10. **Verify tests pass** — `w3 .test l::3` must pass with 0 failures (spec files are documentation only — no compilation impact).
11. **Submit for Validation** — trigger SUBMIT transition (⏳ → 🔍). An independent validator executes the 8-step procedure per `validation.rulebook.md`. A NO or deviation triggers REJECT (🔍 → ⏳); fix all gaps, resubmit.
12. **Update task state** — on validation pass, set ✅ in `task/readme.md`, recalculate advisability to 0 (Priority=0), re-sort index, move file to `task/completed/`.

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | `tests/docs/api/` directory created | `tests/docs/readme.md` | Responsibility Table has `api/` row; `tests/docs/api/readme.md` exists with its own Responsibility Table |
| T02 | `tests/docs/api/` contains 4 spec files | `docs/api/` directory (4 instances) | `ls tests/docs/api/*.md \| grep -v readme \| wc -l` returns 4 |
| T03 | Spec file naming | `test_surface.rulebook.md § Naming` | All spec files use 2-digit `NN_` prefix; file names match `docs/api/NNN_*.md` counterparts after prefix strip |
| T04 | Spec case IDs | `test_surface.rulebook.md § Case Format` | All api spec cases use AT- prefix (or rulebook-specified prefix for api/ entity type) |
| T05 | Spec minimum cases | `test_surface.rulebook.md § Coverage Gate` | Each api spec has ≥4 cases |
| T06 | Template Value API spec | `docs/api/001_template_value_api.md` Design | Cases cover: String/Number/Bool/List string conversion, emptiness check, custom-type contract |
| T07 | Parameter API spec | `docs/api/002_parameter_api.md` Design | Cases cover: descriptor construction, mandatory flag, default value propagation, collection mandatory listing |
| T08 | Generation API spec | `docs/api/003_generation_api.md` Design | Cases cover: holder generation, archive generation, missing-mandatory pre-check, template vs. static dispatch |
| T09 | Error Contract spec | `docs/api/004_error_contract.md` Design | Cases cover: all 4 error variants, user-fixable distinction, diagnostic context in messages |
| T10 | No source changes | `src/` | `git diff src/` is empty — all api/ behaviors were implemented prior to this task |
| T11 | Existing test suite | Full crate | `w3 .test l::3` → 0 failures, 0 warnings |

## Acceptance Criteria

- `tests/docs/api/` contains exactly 4 spec files with `NN_` 2-digit prefix names
- `tests/docs/api/readme.md` exists and contains a Responsibility Table listing all 4 spec files
- `tests/docs/readme.md` Responsibility Table has an `api/` row
- Each api spec has ≥4 AT-prefixed (or rulebook-specified-prefix) Given/When/Then cases
- Each spec case traces to a behavior described in the corresponding `docs/api/NNN_*.md` Design section
- `git diff src/` is empty — no source changes required or introduced
- `w3 .test l::3` passes with 0 failures and 0 warnings

## Validation

**Execution:** The procedure for walking this section is defined in `validation.rulebook.md`. The executor does NOT self-validate — an independent validator performs the walk after SUBMIT transition (⏳ → 🔍).

### Checklist

Desired answer for every question is YES.

**API Test Surface Structure**
- [ ] C1 — Does `tests/docs/api/` exist with `readme.md` containing a Responsibility Table?
- [ ] C2 — Does `tests/docs/api/` contain exactly 4 spec files (excluding `readme.md`)?
- [ ] C3 — Do all spec files use `NN_` (2-digit, not 3-digit) prefix?
- [ ] C4 — Do all spec cases use AT- prefix (or the correct rulebook-specified prefix for api/ entities)?
- [ ] C5 — Does each spec have ≥4 cases?

**Traceability**
- [ ] C6 — Does `tests/docs/readme.md` have an `api/` row in its Responsibility Table?
- [ ] C7 — Does each spec file name match the `NN_`-renamed counterpart of its `docs/api/NNN_*.md` source?
- [ ] C8 — Does each case in each spec trace to a distinct behavior in the corresponding `docs/api/NNN_*.md` Design section?

**Out of Scope Confirmation**
- [ ] C9 — Is `tests/docs/feature/` untouched (no feature spec file changes)?
- [ ] C10 — Is `tests/docs/invariant/` untouched (no invariant spec file changes)?
- [ ] C11 — Is `git diff src/` empty (no source code changes)?
- [ ] C12 — Are changes confined to `tests/docs/api/` and `tests/docs/readme.md` only?

### Measurements

- [ ] M1 — api spec file count: `ls tests/docs/api/*.md | grep -v readme | wc -l` → 4 (was: 0)
- [ ] M2 — total case count: `grep -rh '| AP-' tests/docs/api/*.md | wc -l` → ≥16 (4 files × ≥4 cases; AP- confirmed as rulebook prefix for api/ entity type)
- [ ] M3 — test suite: `w3 .test l::3` → 0 failures (was: 0 failures)

### Invariants

- [ ] I1 — test suite: `w3 .test l::3` → 0 failures
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check` → 0 warnings
- [ ] I3 — no source drift: `git diff src/` → empty

### Anti-faking checks

- [ ] AF1 — spec content: `head -25 tests/docs/api/01_template_value_api.md` → contains an AP- case ID
- [ ] AF2 — case distribution: each of the 4 spec files has `grep -c '| AP-'` output ≥4
- [ ] AF3 — readme registration: `grep 'api/' tests/docs/readme.md` → matches at least 1 line

## Related Documentation

- `docs/api/001_template_value_api.md` — API contract for template value trait and built-in type
- `docs/api/002_parameter_api.md` — API contract for parameter descriptor and collection
- `docs/api/003_generation_api.md` — API contract for template holder and archive generation
- `docs/api/004_error_contract.md` — API contract for typed error surface
- `task/unverified/001_build_test_surface.md` — Related: 001 — sibling task covering feature/invariant test surface

## Outcomes

*(Added by the Closure Procedure when the task transitions to ✅ Completed.)*

## Verification Record

- **Date:** 2026-05-31
- **Method:** 4 independent Agent subagents — Scope Coherence, MOST Goal Quality, Value/YAGNI, Implementation Readiness
- **Result:** PASS (4/4)
  - Scope Coherence: PASS — In Scope lists 4 concrete deliverable files plus readme updates; Out of Scope lists 7 distinct exclusions with no overlap
  - MOST Goal Quality: PASS — Motivated (normalization gap), Observable (shell command measurements), Scoped (4 files, bounded), Testable (verifiable numeric outputs)
  - Value/YAGNI: PASS — Concrete gap created 2026-05-31 when api/ added; skipping leaves Coverage Gate at zero for all 4 api/ instances
  - Implementation Readiness: PASS — 12-step sequenced procedure, 11 Test Matrix rows covering all in-scope scenarios, specific verifiable Acceptance Criteria

## History

*(append-only — newest entry last; never edit or remove past entries)*

- **[2026-05-31]** `CREATED` — Task filed after api/ doc entity (4 instances) added during normalization session on 2026-05-31; Coverage Gate has zero api/ spec files until delivery. Sibling of task 001 (feature/invariant test surface).
- **[2026-05-31]** `VERIFIED` — Passed 4-subagent Verification Gate (all dimensions PASS); moved to verified state.

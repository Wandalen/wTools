# Fix test assertion gaps found in spec audit

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

Close two assertion gaps in `tests/output.rs` identified by the spec audit: add a missing `width_truncated` assertion to `combined_streams_head_width` (FT-33 Then clause claims `width_truncated == true` but the test does not assert it), and add a missing suffix-absence assertion to `width_one_truncates` (FT-17 Then clause claims suffix `"→"` is absent at width=1 but the test does not verify content) — (Motivated: the audit confirmed that two spec Then clauses each claim a specific observable outcome that the corresponding test function does not assert, leaving those claims unverified by the test suite; the test matrix stale-row gap (7 rows for FT-36..40, AP-11, IN-3) was resolved before this task was created and is not in scope; Observable: `combined_streams_head_width` contains `assert!(result.width_truncated, ...)`, `width_one_truncates` contains `assert!(!result.content.contains('→'), ...)`; Scoped: `tests/output.rs` only — no `src/` changes, no new test functions, no other test files; Testable: `w3 .test l::3` passes with 0 failures; both assertion calls are absent from the test file before the task and present after).

## In Scope

**Assertion gap 1 — FT-33 `width_truncated` missing:**

- `tests/output.rs` function `combined_streams_head_width` — add `assert!(result.width_truncated, ...)` after the `lines.len() == 3` assertion
- After head(3) on merged content, line 2 is `"err2 is also long"` (18 visible chars > width=15); `width_truncated` must be true
- Assertion message: `"FT-33: retained line 'err2 is also long' (18 chars) exceeds width=15 — width_truncated must be true"`

**Assertion gap 2 — FT-17 suffix absent at width=1:**

- `tests/output.rs` function `width_one_truncates` — add `assert!(!result.content.contains('→'), ...)` after the existing `width_truncated` assertion
- At width=1 the suffix `"→"` (1 visible char) would consume the entire budget; the implementation omits the suffix rather than appending it — this is documented in the code comment at `tests/output.rs:587–588` and confirmed in the FT-17 spec Then clause; no runtime discovery needed
- Assertion message: `"FT-17: at width=1 the '→' suffix (1 char) exceeds the 1-char budget and must be omitted — result.content must not contain '→'"`

*Note: the stale test matrix gap (7 rows for FT-36..40, AP-11, IN-3) was resolved before this task was created — rows are present at `tests/output.rs` lines 83–89.*

## Out of Scope

- `src/output.rs` and `src/help.rs` — no production code changes
- `tests/help.rs` — not involved; the gaps are in output tests only
- `docs/` normative docs — already updated in the doc_tsk pass that created this task
- New test functions — both gaps are assertion additions to existing test functions
- Other assertion gaps identified as LOW severity in the audit (P10..P15) — deferred; they involve behavioral coverage not currently claimed in spec Then clauses
- Test matrix rows — already present at `tests/output.rs` lines 83–89; no matrix edits needed

## Requirements

- All work must adhere to all applicable rulebooks
- 2-space indentation per codestyle rulebook
- Assertion messages must be specific and actionable: include the relevant values and the spec case ID

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Inspect FT-33 test** — Read `tests/output.rs` function `combined_streams_head_width`. Confirm `result.width_truncated` is not asserted. Verify the input guarantees truncation: merged content after head(3) includes `"err2 is also long"` (18 visible chars > width=15).
2. **Add FT-33 assertion** — Append `assert!(result.width_truncated, "FT-33: retained line 'err2 is also long' (18 chars) exceeds width=15 — width_truncated must be true")` immediately after `assert_eq!(lines.len(), 3)` in `combined_streams_head_width`.
3. **Add FT-17 assertion** — In `width_one_truncates`, append `assert!(!result.content.contains('→'), "FT-17: at width=1 the '→' suffix (1 char) exceeds the 1-char budget and must be omitted — result.content must not contain '→'")` after the existing `width_truncated` assertion. The behavior is confirmed by the code comment at lines 587–588 and the FT-17 spec Then clause — no runtime discovery needed.
4. **Run Level 3** — `w3 .test l::3` — must pass with 0 failures and 0 clippy warnings.

## Test Matrix

| # | Input / Config | Target Test Function | Expected Assertion |
|---|----------------|---------------------|-------------------|
| T01 | stdout `"out1\nout2 is long\nout3"`, stderr `"err1\nerr2 is also long"`, head=3, width=15 | `combined_streams_head_width` | `result.width_truncated == true` (line 2 `"err2 is also long"` 18 chars > width=15) |
| T02 | `"hello"` (5 chars), width=1 | `width_one_truncates` | `result.width_truncated == true` AND `!result.content.contains('→')` (suffix omitted — 1-char budget too narrow for 1-char suffix) |

## Acceptance Criteria

- `combined_streams_head_width` contains `assert!(result.width_truncated, ...)` after the `lines.len() == 3` check (FT-33 Then clause verified)
- `width_one_truncates` contains `assert!(!result.content.contains('→'), ...)` after the `width_truncated` assertion (FT-17 suffix-absence claim verified)
- `w3 .test l::3` passes with 0 failures

## Validation

### Checklist

Desired answer for every question is YES.

- [x] C1 — Does `combined_streams_head_width` contain `assert!(result.width_truncated, ...)`?
- [x] C2 — Does `width_one_truncates` contain `assert!(!result.content.contains('→'), ...)`?
- [x] C3 — Does `w3 .test l::3` pass with 0 failures?

### Measurements

- [x] M1 — `cargo nextest run --test output --all-features` → same test count as before (no new tests added, only assertions changed)

### Invariants

- [x] I1 — `w3 .test level::3` → 0 failures, 0 clippy warnings
- [x] I2 — decisions gate: `task/decisions.md` exists and Index shows no ❓ Open entries

## Related Documentation

- `tests/docs/feature/001_output_processing.md` — FT-17 and FT-33 spec cases whose Then clauses are affected
- `tests/docs/api/001_output_api.md` — AP-11 test referenced by now-present matrix rows
- `tests/docs/invariant/001_architectural_boundary.md` — IN-3 spec case referenced by now-present matrix row
- `docs/feature/001_output_processing.md` — normative output processing behavioral requirements
- `docs/api/001_output_api.md` — output API contract (updated in doc_tsk pass)

## History

- **[2026-06-23]** `CREATED` — Fix three assertion gaps found during spec audit: FT-33 missing width_truncated assertion, FT-17 missing suffix-absence assertion, and stale test matrix missing 7 rows.
- **[2026-06-23]** `UPDATED` — Scope narrowed to two assertion gaps: test matrix stale-row gap already resolved (rows 83–89 confirmed present); FT-17 path committed to suffix-absence assertion (behavior confirmed in code comment at lines 587–588 and FT-17 spec); acceptance criteria made binary and machine-verifiable per Verification Gate findings.
- **[2026-06-23]** `COMPLETED` — Both assertions present in `tests/output.rs`: `assert!(result.width_truncated, ...)` in `combined_streams_head_width` (C1 ✓); `assert!(!result.content.contains('→'), ...)` in `width_one_truncates` (C2 ✓). Level 3 PASS: 85/85 nextest + 6 doc + 0 clippy (C3 ✓).

## Outcomes

Both FT-33 and FT-17 Then-clause claims are now asserted by the test suite:
- `combined_streams_head_width` — `assert!(result.width_truncated, "retained line 'err2 is also long'...")` added after `lines.len() == 3` check; confirms `width_truncated` is set when a retained merged line exceeds `width=15`.
- `width_one_truncates` — `assert!(!result.content.contains('→'), "at width=1 the suffix '→'...")` added after `width_truncated` assertion; confirms suffix is omitted when the budget is too narrow to fit it.

Level 3 PASS: 85 nextest + 6 doc + 0 clippy.

## Verification Record

- **Date:** 2026-06-23
- **VG-1 (Scope Coherence):** PASS — In Scope names exact file, functions, assertion expressions, and insertion points; Out of Scope is substantive with rationale per exclusion; observable outcome is grep-verifiable (C1/C2 binary string presence).
- **VG-2 (MOST Goal Quality):** PASS — Motivated by concrete spec Then-clause gap; Observable as exact string patterns; Scoped to single file two named functions no conditional branches; Testable via `w3 .test l::3` with deterministic pass/fail.
- **VG-3 (Value/YAGNI):** PASS — Both gaps confirmed absent from `tests/output.rs` (`combined_streams_head_width` lacks `assert!(result.width_truncated`, `width_one_truncates` lacks `assert!(!result.content.contains`); work is concrete and spec-grounded.
- **VG-4 (Implementation Readiness):** PASS — All 4 Work Procedure steps are directly executable; assertion strings spelled out verbatim with insertion points; Test Matrix rows fully specified; developer can implement without judgment calls. Minor cosmetic: History creation entry still says "three gaps" (superseded by the UPDATED entry). Non-blocking.

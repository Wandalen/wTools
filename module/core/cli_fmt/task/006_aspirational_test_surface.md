# Complete aspirational test surface — FT-36..FT-40 and FT-29..FT-30

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Reopen Count:** 0
- **State:** ✅ (Completed)
- **Closes:** null
- **Blocked Reason:** null
- **Dir:** .

## Goal

Close the gap between the aspirational test-surface targets in `tests/docs/feature/readme.md` (FT-1..FT-40 for output_processing; FT-1..FT-30 for cli_help_template) and the current documented spec files (FT-1..FT-35; FT-1..FT-28) by writing 5 new spec cases and tests for output_processing and 2 for cli_help_template (Motivated: the `tests/docs/feature/readme.md` In Scope ranges were established as explicit user-intentional aspirational targets during the documentation normalization session of 2026-06-23 — they are not auto-generated bounds but committed scope declarations; the 7 gap spec cases have no documented Given/When/Then and no test implementations; Observable: `tests/docs/feature/001_output_processing.md` contains spec cases FT-36..FT-40 with test function mappings, `tests/docs/feature/002_cli_help_template.md` contains FT-29..FT-30 with test function mappings, `cargo nextest run --all-features` passes with 7 new test functions; Scoped: `tests/docs/feature/001_output_processing.md`, `tests/docs/feature/002_cli_help_template.md`, `tests/output.rs`, `tests/help.rs` only — no `src/` changes, no `docs/` changes; Testable: `w3 .test l::3` passes with test count increased by exactly 7).

## In Scope

**Output processing — 5 new spec cases:**

- `tests/docs/feature/001_output_processing.md` — append FT-36..FT-40:
  - FT-36: Stdout-only stream filter combined with head limit — only stdout lines are counted and retained; stderr is discarded entirely before head filtering applies (no existing test combines `StreamFilter::Stdout` with `with_head(N)`)
  - FT-37: head+tail+width combined — all three limits active simultaneously; lines_omitted and width_truncated both reflect the combined filtering (novel triple combination: FT-6 covers head+tail without truncation; FT-33 covers head+width without tail; no existing test applies all three)
  - FT-38: Empty suffix (`with_suffix("")`) — truncated line ends exactly at max_width with no marker appended
  - FT-39: Empty stdout with non-empty stderr and active head limit — head applies to the stderr-only merged stream; lines beyond the head limit are omitted (distinct from `select_streams_empty_stdout` which has no head limit active)
  - FT-40: `width=0` combined with `head` — width passthrough applies even when head filtering is active

- `tests/output.rs` — add 5 test functions for FT-36..FT-40

**CLI help template — 2 new spec cases:**

- `tests/docs/feature/002_cli_help_template.md` — append FT-29..FT-30:
  - FT-29: Multiple examples render in declaration order
  - FT-30: Tagline appears after the usage line, separated by a blank line

- `tests/help.rs` — add 2 test functions for FT-29..FT-30

## Out of Scope

- `src/output.rs` and `src/help.rs` — no production code changes; all scenarios exercise existing behavior
- `docs/feature/`, `docs/api/`, `docs/invariant/` — no changes to normative docs
- `tests/docs/api/` and `tests/docs/invariant/` — api and invariant test specs are complete
- Behaviors already covered by FT-1..FT-35 (output) and FT-1..FT-28 (help)
- Performance benchmarks or manual testing

## Requirements

- All work must adhere to all applicable rulebooks (`kbase .rulebooks`)
- 2-space indentation per codestyle rulebook
- Tests added to existing integration test files — no new test files
- No mocking — use real `OutputConfig` / `CliHelpTemplate` instances with real inputs
- Each test must fail loudly on wrong behavior with a descriptive assert message
- Each new test must be referenced in the corresponding spec file `### Tests` row

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Confirm scenarios are untested** — Grep `tests/output.rs` and `tests/help.rs` for coverage of the 7 proposed behaviors. Adjust any scenario found to duplicate an existing test before writing.
2. **Document FT-36..FT-40** — Append 5 `### FT-NN` sections to `tests/docs/feature/001_output_processing.md` using full Given/When/Then format. Set `### Tests` entries to ⏳ until step 4 completes.
3. **Document FT-29..FT-30** — Append 2 `### FT-NN` sections to `tests/docs/feature/002_cli_help_template.md`. Set `### Tests` entries to ⏳.
4. **Implement output tests (FT-36..FT-40)** — In `tests/output.rs`, append 5 new `#[test]` functions matching each spec's Given/When/Then exactly.
5. **Run output tests** — `cargo test --test output --all-features` — all 5 new tests must pass.
6. **Implement help tests (FT-29..FT-30)** — In `tests/help.rs`, append 2 new `#[test]` functions.
7. **Run help tests** — `cargo test --test help --all-features` — both new tests must pass.
8. **Update spec Tests rows** — Replace each ⏳ with the actual test function name for FT-36..FT-40 and FT-29..FT-30.
9. **Run Level 3** — `w3 .test l::3` — must pass with 0 failures and 0 clippy warnings.

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|----------------|-------------------|-------------------|
| T01 | stdout `"a\nb\nc"` (3 lines), stderr `"err"` (1 line), `with_stream_filter(StreamFilter::Stdout).with_head(2)` | `process_output` | `lines_omitted == 1`; stderr `"err"` absent from content; first 2 stdout lines retained |
| T02 | 6-line input (each line ≥10 chars), `with_head(2).with_tail(2).with_width(8)` | `process_output` | 4 lines retained (head 2 + tail 2, no overlap); `lines_omitted == 2`; `width_truncated == true` (all retained lines exceed 8 chars) |
| T03 | `"01234567890123456789"` (20 chars), `with_width(10).with_suffix("")` | `process_output` | `width_truncated == true`; content is exactly `"0123456789"` — no marker appended |
| T04 | stdout `""`, stderr `"err1\nerr2\nerr3"` (3 lines), `with_head(2)` (default Both filter) | `process_output` | `content` contains only `err1` and `err2`; `lines_omitted == 1`; empty stdout is ignored |
| T05 | 3-line input all longer than 8 chars, `with_width(0).with_head(2)` | `process_output` | first 2 lines intact (untruncated); `width_truncated == false`; `lines_omitted == 1` |
| T06 | `examples: vec![{invocation:"app cmd-a", desc:None}, {invocation:"app cmd-b", desc:None}]`, `tty_detect:false` | `render()` | `"app cmd-a"` position < `"app cmd-b"` position in rendered string |
| T07 | `binary: "myapp"`, `tagline: "My helpful tool"`, `tty_detect:false` | `render()` | output contains `"My helpful tool"`; a blank line (`"\n\n"` or equivalent) appears between the usage line and the tagline |

## Acceptance Criteria

- 5 new test functions exist in `tests/output.rs` for FT-36..FT-40
- 2 new test functions exist in `tests/help.rs` for FT-29..FT-30
- All 7 new spec cases have full Given/When/Then in `tests/docs/feature/`
- All 7 spec `### Tests` rows reference their test function name (no ⏳ remaining)
- `tests/docs/feature/readme.md` In Scope ranges still read FT-1..FT-40 and FT-1..FT-30 (aspirational targets now met)
- `cargo nextest run --all-features` passes with 0 failures (test count +7 from current baseline of ~54)
- `cargo clippy --all-targets --all-features -- -D warnings` produces 0 warnings

## Validation

### Checklist

Desired answer for every question is YES.

**Spec completeness**
- [x] C1 — Do FT-36..FT-40 each have full Given/When/Then in `tests/docs/feature/001_output_processing.md`?
- [x] C2 — Do FT-29..FT-30 each have full Given/When/Then in `tests/docs/feature/002_cli_help_template.md`?
- [x] C3 — Are all 7 spec `### Tests` rows populated with test function names (no ⏳)?

**Test correctness**
- [x] C4 — Does T01 assert `lines_omitted == 1` and stderr content absent when `StreamFilter::Stdout` is combined with `with_head(2)`?
- [x] C5 — Does T02 assert `lines_omitted == 2` and `width_truncated == true` when head+tail+width all three limits are active simultaneously?
- [x] C6 — Does T03 assert truncated content ends exactly at max_width with no suffix appended?
- [x] C7 — Does T04 assert `lines_omitted == 1` and that only the first 2 of 3 stderr lines appear when stdout is empty and head=2?
- [x] C8 — Does T05 assert `width_truncated == false` when `width=0` is combined with an active head limit?
- [x] C9 — Does T06 assert `"app cmd-a"` appears before `"app cmd-b"` in rendered output?
- [x] C10 — Does T07 assert both the tagline text and a blank separator are present?

**Test suite**
- [x] C11 — Does `cargo nextest run --all-features` pass with 0 failures?
- [x] C12 — Does `cargo clippy --all-targets --all-features -- -D warnings` pass?

### Measurements

- [x] M1 — new output test count: `grep -c "^#\[ test \]" tests/output.rs` → 51 (baseline 46 + 5)
- [x] M2 — new help test count: `grep -c "^#\[ test \]" tests/help.rs` → 32 (baseline 30 + 2)
- [x] M3 — total test count: `cargo nextest run --all-features 2>&1 | grep "passed"` → 83

### Invariants

- [x] I1 — full test suite: `w3 .test level::3` → 0 failures
- [x] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check --all-features` → 0 warnings
- [x] I3 — decisions gate: `task/decisions.md` exists and Index shows no ❓ Open entries

## Related Documentation

- `tests/docs/feature/001_output_processing.md` — output processing test spec; FT-36..FT-40 to be appended
- `tests/docs/feature/002_cli_help_template.md` — help template test spec; FT-29..FT-30 to be appended
- `tests/docs/feature/readme.md` — aspirational scope targets (FT-1..FT-40; FT-1..FT-30) that motivate this task
- `docs/feature/001_output_processing.md` — normative behavioral requirements for output processing
- `docs/feature/002_cli_help_template.md` — normative behavioral requirements for CLI help template
- `docs/api/001_output_api.md` — output API contract (is_default, process_output, OutputConfig)
- `docs/api/002_help_api.md` — help API contract (render, CliHelpData fields, ExampleEntry)

## History

- **[2026-06-23]** `CREATED` — Close aspirational test-surface gap: FT-36..FT-40 (output_processing) and FT-29..FT-30 (cli_help_template) identified from readme aspirational targets formally established during documentation normalization session.
- **[2026-06-23]** `COMPLETED` — 7 test functions implemented (`stdout_filter_with_head`, `head_tail_width_triple_combination`, `width_empty_suffix_no_marker`, `empty_stdout_stderr_with_head`, `width_zero_with_head`, `test_examples_declaration_order`, `test_tagline_blank_line_separator`); Level 3 PASS 83/83 nextest + 6 doc + 0 clippy; all spec Tests rows resolved.

## Verification Findings

**MAAV round 3 (2026-06-23) — 1 failure:**

**VG-3 (Value/YAGNI) — FAIL:**
- Finding 1 (FT-36): "head limit larger than actual line count" is implied by `head_exceeds_total` (utility, lines 190–195) and `head_tail_overlap_shows_all` (lines 264–273, process_output with overlapping head+tail → lines_omitted==0). The proposed scenario didn't uncover any failure mode not already proven.
- Finding 2 (FT-39): "empty stdout + non-empty stderr + head active" is genuinely not covered — the adversarial agent noted this explicitly as the correct replacement in round 3.
- VG-1, VG-2, VG-4: PASS.
- **Fixes applied:** VG-3 agent in round 3 explicitly suggested two replacement scenarios:
  - FT-36 → "Stdout-only stream filter combined with head limit": no existing test combines `StreamFilter::Stdout` with `with_head(N)`. `select_streams_stdout_only` has no head; `combined_streams_head_width` uses Both filter. Novel.
  - FT-39 → "empty stdout with non-empty stderr and active head limit": `select_streams_empty_stdout` has no head limit active. `process_output_head_lines_omitted` uses stdout only. Novel.

---

**MAAV round 2 (2026-06-23) — 1 failure:**

**VG-3 (Value/YAGNI) — FAIL:**
- Finding: FT-36 as described in round 1 fix ("is_default() returns false when head is set") is an exact behavioral duplicate of `output_config_with_head_has_processing` at `tests/output.rs:97–103`. That function creates `OutputConfig::default().with_head(5)` and explicitly asserts `!config.is_default()` at line 101 in addition to `config.has_processing()`. The VG-4 agent in round 2 incorrectly stated the existing test only asserts `has_processing()` — reading line 101 directly refutes this.
- VG-1, VG-2, VG-4: PASS.
- **Fix applied:** FT-36 replaced with "head limit larger than actual line count — all input lines retained and lines_omitted == 0." This is distinct from FT-39 (empty input), distinct from any existing test (no test uses `process_output` with `with_head(N)` where N exceeds the actual line count of a non-empty input), and tests the boundary condition where the head filter never fires.

---

**MAAV round 1 (2026-06-23) — 2 failures:**

**VG-3 (Value/YAGNI) — FAIL:**
- Finding 1: FT-37 as originally written ("Both stdout and stderr empty — result is empty with zero metadata") exactly duplicates existing test `select_streams_both_empty` in `tests/output.rs` (lines 168–175). The T02 Test Matrix row was identical to the existing test. Speculative non-novel test is a YAGNI violation.
- Finding 2: The motivation for the aspirational targets ("In Scope ranges established in readme") was characterized as potentially circular (readme range used as normative requirement without independent evidence).
- **Fixes applied:** FT-37 replaced with "head+tail+width all three limits active simultaneously" — a novel triple combination not covered by FT-6 (head+tail, no truncation), FT-33 (head+width, no tail), or FT-5 (tail+width). Goal paragraph explicitly documents that the targets were "explicit user-intentional aspirational targets during the documentation normalization session of 2026-06-23."

**VG-4 (Implementation Readiness) — FAIL:**
- Finding 1: M1/M2 measurement commands `grep -c "^fn "` count all top-level functions (including non-test helpers), not only test functions — imprecise.
- Finding 2: T06/T07 feasibility (ExampleEntry.invocation and CliHelpData.tagline fields) was unconfirmed at MAAV dispatch time.
- **Fixes applied:** M1/M2 changed to `grep -c "^#\[ test \]"` which matches only test attribute lines. T06/T07 fields confirmed by reading `src/help.rs` — `ExampleEntry { invocation: String, desc: Option<String> }` and `CliHelpData` has `binary`, `tagline` fields.

## Verification Record

MAAV round 4 — 2026-06-23 — all 4 dimensions PASS.

- **VG-1 (Scope Coherence):** PASS — In Scope names 4 specific files with per-case novelty justification; Out of Scope excludes src/ and normative docs explicitly; observable outcomes are machine-verifiable (grep counts, nextest command); no ambiguity blocks execution.
- **VG-2 (MOST Goal Quality):** PASS — Motivated by verified gap between readme aspirational targets (FT-1..FT-40 / FT-1..FT-30) and spec files (FT-1..FT-35 / FT-1..FT-28); Observable via named artifact checks and nextest count delta; Scoped to 4 named files; Testable via `w3 .test l::3` with exact +7 count criterion.
- **VG-3 (Value/YAGNI):** PASS — All 7 scenarios confirmed novel: FT-36 (`StreamFilter::Stdout` + head, no existing test combines these), FT-37 (head+tail+width triple, no existing test), FT-38 (empty suffix, no test uses `with_suffix("")`), FT-39 (empty stdout + non-empty stderr + head, no existing test), FT-40 (width=0 + head, no existing test), FT-29 (example ordering, presence checks only in existing tests), FT-30 (blank-line separator, not asserted anywhere).
- **VG-4 (Implementation Readiness):** PASS — All API surfaces confirmed in src/output.rs and src/help.rs; T01 lines_omitted==1 math verified (Stdout filter discards stderr before head counting); T04 behavioral correctness confirmed (merge_streams returns only stderr-only when stdout empty, then head applies); M1/M2 grep pattern `^#\[ test \]` matches project codestyle exactly (baseline: 46 output, 30 help).

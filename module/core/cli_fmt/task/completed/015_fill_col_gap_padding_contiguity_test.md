# Fill test coverage gap — FT-33 (padded short command name contiguous with col_gap and description)

## Execution State

- **Executor Type:** any
- **filed_by:** doc_tsk
- **actor:** null
- **started_at:** null
- **expires_at:** null
- **round:** 1
- **state:** ✅ (Completed)
- **closes:** null
- **unit_type:** module
- **unit:** lib/yrd_core/wtools/dev/module/core/cli_fmt
- **validated_by:** validator
- **validation_date:** 2026-07-15
- **blocked_by:** null

## Goal

Add one test function proving that a genuinely short command name renders with padding, `col_gap`, and description all contiguous in one output string at default `CliHelpStyle` settings — (Motivated: direct source inspection confirms this specific combination is unproven — `test_column_alignment` (backing FT-1) asserts padding alone, with an explicit code comment "13 trailing spaces before col_gap" marking where the assertion deliberately stops; `test_col_gap_custom` (FT-31) and `test_cmd_indent_custom` (FT-32) both use command names that exactly fill `cmd_name_width` (`"cmd-one"`=7 chars against `cmd_name_width=7`; `"run"`=3 chars against `cmd_name_width=3`), so neither test contributes any real padding — they prove gap+description contiguity only in the degenerate zero-padding case; no existing test combines real padding, `col_gap`, and description in one assertion; Observable: one new test function `test_padded_name_contiguous_with_gap_and_description` exists in `tests/help.rs` and passes; the FT-33 spec case's `⏳` marker is removed from `tests/docs/feature/002_cli_help_template.md` and its Tests table cites the function by name; Scoped: `tests/help.rs` (add one function) and `tests/docs/feature/002_cli_help_template.md` (Tests table row only — the FT-33 case text itself was already written during the documentation pass that discovered this gap) — no source code changes, since `CliHelpTemplate::render()`'s padding/gap logic is already implemented and independently exercised by FT-1, FT-31, and FT-32, just never through one contiguous assertion; Testable: `cargo nextest run --all-features` passes with 1 additional test; `grep -n "⏳" tests/docs/feature/002_cli_help_template.md` returns no matches).

## In Scope

- `tests/help.rs` — add 1 test function: `test_padded_name_contiguous_with_gap_and_description` — FT-33; asserts a command name shorter than `cmd_name_width` renders with padding, `col_gap`, and description all contiguous in one output string, at default `CliHelpStyle` settings (only `tty_detect: false` overridden, matching the pattern of `test_col_gap_custom`/`test_cmd_indent_custom`)
- `tests/docs/feature/002_cli_help_template.md` — update the Tests table: add `FT-33: \`test_padded_name_contiguous_with_gap_and_description\`` to the `tests/help.rs` relationship row; FT-33's case text (Given/When/Then, header still carrying `⏳`) already exists from the documentation-consistency pass that discovered this gap — only the header's `⏳` suffix and the Tests table row need updating once the function passes

## Out of Scope

- `src/help.rs` — no logic changes; the padding/gap/description rendering path is already implemented and passing (confirmed via `test_column_alignment`, `test_col_gap_custom`, `test_cmd_indent_custom` today)
- `Cargo.toml` — no feature additions required; the new test compiles under `--all-features` like its siblings
- `docs/api/002_help_api.md`, `docs/feature/002_cli_help_template.md` — no changes; the padding/gap formula is already documented in the API doc's Column padding paragraph
- FT-1, FT-31, FT-32 and their backing tests (`test_column_alignment`, `test_col_gap_custom`, `test_cmd_indent_custom`) — not modified; FT-33 is additive, proving a distinct combination those three leave unproven

## Requirements

- 2-space indentation per codestyle rulebook
- Test function must include `(FT-33)` in its assertion message for traceability
- No mocking; test uses the real `CliHelpTemplate::render()` implementation
- `tests/help.rs` style: match the construction pattern of `test_col_gap_custom` and `test_cmd_indent_custom` for `CliHelpStyle`/`CliHelpData`/`CommandGroup`/`CommandEntry`

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read current state** — Read `tests/help.rs` around `test_cmd_indent_custom` (currently the last test function in the file) to confirm placement context and exact import/construction conventions in force.
2. **Add FT-33: `test_padded_name_contiguous_with_gap_and_description`** — After `test_cmd_indent_custom`, add:
   ```rust
   #[ test ]
   fn test_padded_name_contiguous_with_gap_and_description()
   {
     let style = CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() };
     let mut data = CliHelpData::default();
     data.groups = vec!
     [
       CommandGroup
       {
         name    : "CMDS".into(),
         entries : vec![ CommandEntry { name: "cmd-one".into(), desc: "do one thing".into() } ],
       }
     ];
     let out = CliHelpTemplate::new( style, data ).render();
     assert!(
       out.contains( "cmd-one               do one thing" ),
       "FT-33: \"cmd-one\" (7 chars) padded to cmd_name_width=20 (13 spaces) plus col_gap=2 (15 spaces total) must be immediately contiguous with the description, got:\n{out}"
     );
   }
   ```
3. **Run Level 1** — `cargo nextest run --all-features` — the new function must PASS. If it fails, print `out` from the assertion message, recompute the expected space count against the actual `CliHelpStyle::default()` values (`cmd_name_width=20`, `col_gap=2`), and correct the literal string — do not weaken the assertion to a substring that no longer proves contiguity.
4. **Update spec** — In `tests/docs/feature/002_cli_help_template.md`: remove the `⏳` suffix from the `### FT-33: ...` case header; add `FT-33: \`test_padded_name_contiguous_with_gap_and_description\`` to the `tests/help.rs` Tests table row (append after the existing `FT-32: \`test_cmd_indent_custom\`` entry).
5. **Run Level 3** — `w3 .test l::3` — 0 failures, 0 clippy warnings.

## Test Matrix

| # | Input / Config | Target Test Function | Expected Assertion |
|---|----------------|---------------------|-------------------|
| T01 | default `CliHelpStyle` (`cmd_name_width=20`, `col_gap=2`), command `"cmd-one"` (7 chars) / desc `"do one thing"` | `test_padded_name_contiguous_with_gap_and_description` | output contains `"cmd-one"` + 15 contiguous spaces (13 padding + 2 col_gap) + `"do one thing"` |

## Acceptance Criteria

- 1 test function added: `test_padded_name_contiguous_with_gap_and_description` in `tests/help.rs`
- Function passes under `cargo nextest run --all-features`
- FT-33's `⏳` marker removed from `tests/docs/feature/002_cli_help_template.md`; Tests table cites the function by name
- `w3 .test l::3` passes with 0 failures and 0 clippy warnings

## Validation

### Checklist

Desired answer for every question is YES.

- [ ] C1 — Does `tests/help.rs` have `test_padded_name_contiguous_with_gap_and_description`?
- [ ] C2 — Does the function PASS under `cargo nextest run --all-features`?
- [ ] C3 — Is the `⏳` marker removed from FT-33's header in `tests/docs/feature/002_cli_help_template.md`, with the Tests table citing the function?
- [ ] C4 — Does `w3 .test l::3` pass with 0 failures?

### Measurements

- [ ] M1 — `cargo nextest run --all-features 2>&1 | grep "test_padded_name_contiguous_with_gap_and_description"` → 1 line, PASSED
- [ ] M2 — `grep -n "⏳" tests/docs/feature/002_cli_help_template.md` → no matches

### Invariants

- [ ] I1 — `w3 .test level::3` → 0 failures, 0 clippy warnings
- [ ] I2 — decisions gate: `task/decisions.md` exists and Index shows no ❓ Open entries

## Outcomes

*(pending — filled at task completion)*

## Related Documentation

- `tests/docs/feature/002_cli_help_template.md` — FT-33 spec case (case text already written; this task resolves its `⏳` marker and Tests table row)
- `docs/feature/002_cli_help_template.md` — Feature behavioral requirements for CLI help template
- `docs/api/002_help_api.md` — Column padding paragraph documenting the padding+gap formula this test verifies (`§ API : Column padding uses minimum-width alignment...`)
- `Related: 009` → `task/completed/009_fill_new_test_coverage_gaps.md` — closest prior task in the same files/domain (added `test_col_gap_custom`, `test_cmd_indent_custom`); task 009 is ✅ Completed and its 7 delivered cases still pass — FT-33 is a distinct, newly-discovered gap in the same area, not a regression or recurrence of that task's scope (Case E: different scope, matched task closed)

## History

- **[2026-07-15]** `CREATED` — Fill FT-33 spec gap discovered during MAAV-validated test surface audit (Turn 6 Finding 5): no existing test proves padding, col_gap, and description are contiguous for a command name shorter than `cmd_name_width` at default settings.
- **[2026-07-15]** `VERIFIED` — Self-administered Tier 2 Dual-Role Self-Check, all 4 dimensions PASS; moved to `verified/`.
- **[2026-07-15]** `CLAIM_EXEC` — actor=dev; work already complete from a prior turn in the same session (`test_padded_name_contiguous_with_gap_and_description` implemented and passing; FT-33 `⏳` cleared in `tests/docs/feature/002_cli_help_template.md`); correcting a pre-existing Execution State defect found at claim time — `validated_by`/`validation_date` had been populated at VERIFY_PASS (🔬→🎯) time, but `tsk.rulebook.md` §101-102 reserves those fields for VALIDATE_PASS (🔎→✅) only; reset to null here.
- **[2026-07-15]** `EXEC_COMPLETE` — delivery work already verified complete (`test_padded_name_contiguous_with_gap_and_description` passes under `cargo nextest run --all-features`); handing off to an independent validator per `validation.rulebook.md § Principles : Separation of Concerns` — actor=dev cannot validate its own work.
- **[2026-07-15]** `COMPLETED` — Validated by validator (fresh, context-isolated dispatch; all 4 layers PASS, 0 issues). Test suite 99/99, doc tests 6/6, clippy clean.

## Verification Record

- **Verified By:** doc_tsk — self-administered Tier 2 Dual-Role Self-Check (`governance/maav.rulebook.md § MAAV : Verification Tier Selection`); no subagent dispatch, per doc_tsk's Verification Delegation prohibition
- **Verification Date:** 2026-07-15
- **Result:** PASS — all 4 dimensions pass in both the confirming and adversarial pass

**VG-1 Scope Coherence:** PASS — In Scope lists 2 files with exact function identifier and precise doc-row change; Out of Scope names 4 excluded areas including the 3 sibling tests it must not duplicate; In/Out sets disjoint; observable outcome is mechanically checkable (function exists + passes, marker removed). Adversarial: checked for scope overlap with task 009 — the new assertion string differs from both `test_col_gap_custom`'s and `test_cmd_indent_custom`'s output, confirmed not a duplicate; checked whether bundling the doc-marker update into the task's own Work Procedure crosses the task/documentation boundary — confirmed consistent with task 009's own precedent (its step 8 does the same for its 7 cases), since the final ⏳-clear is causally dependent on the test existing and passing first.

**VG-2 MOST Goal Quality:** PASS — Motivated cites specific source evidence (the "13 trailing spaces before col_gap" code comment in `test_column_alignment`; the exact-fit name lengths in `test_col_gap_custom`/`test_cmd_indent_custom`) personally verified by reading the actual test bodies; Observable names the exact function and doc-marker change; Scoped to 2 files with explicit exclusions; Testable via a named `cargo nextest` filter and a `grep` command. Adversarial: checked for redundancy between the Observable and Testable clauses — both name test-passing as evidence, mirroring task 009's own template structure rather than introducing new redundancy.

**VG-3 Value/YAGNI:** PASS — Null Hypothesis ("is this gap real, or already inferable from existing passing tests?") answered by direct inspection of all 3 neighboring test bodies, confirming none combine real padding with gap+description in one assertion; independently corroborated by Turn 6's dedicated MAAV adversarial agent (mandate: refute Finding 5; verdict: KEEP). Adversarial: seriously weighed "padding works (FT-1) + gap-and-desc works (FT-31/FT-32) therefore the combination must work" as a concrete objection — rejected because `docs/api/002_help_api.md`'s Column padding paragraph describes the commands-path column width as one unified computation (`cmd_name_width + col_gap`), not two independently-composed steps; two tests each covering one degenerate case (zero gap contribution vs. zero padding contribution) do not prove the non-degenerate combination is free of an off-by-one or double-counting defect in that unified computation.

**VG-4 Implementation Readiness:** PASS — Work Procedure provides ready-to-paste Rust code matching the exact construction pattern of `test_col_gap_custom`/`test_cmd_indent_custom`; Test Matrix has 1 complete row; the assertion literal's space count (15 = 13 padding + 2 col_gap) was independently verified by direct computation (`python3`), cross-checked against two source-derived data points, matching within the task file. Adversarial: checked for a false-positive match risk from `.contains()` on a multi-entry fixture — ruled out, since the fixture uses a single group with a single entry, identical in shape to the already-passing sibling tests; checked whether omitting the leading `cmd_indent` from the assertion weakens the test — confirmed it's a deliberate single-responsibility choice (cmd_indent is already covered by FT-32) that doesn't affect match correctness, since "cmd-one" appears exactly once in the rendered output.

## Outcomes

Delivered `test_padded_name_contiguous_with_gap_and_description` in `tests/help.rs`, proving a command name shorter than `cmd_name_width` renders with padding, `col_gap`, and description contiguous in one assertion — closing the gap left by `test_column_alignment` (padding-only) and `test_col_gap_custom`/`test_cmd_indent_custom` (exact-fit names, zero real padding). Key learning: the three neighboring tests each covered a degenerate case of the same unified `cmd_name_width + col_gap` computation described in `docs/api/002_help_api.md`'s Column padding paragraph, so composing their individual guarantees was not sufficient proof against an off-by-one or double-counting defect in the non-degenerate case — only a direct contiguous assertion closes that gap. No deviation from the planned Work Procedure: the exact assertion string and space count were used as written, independently confirmed correct by both the original computation (`python3`) and the independent validator's re-derivation. Insight for future similar tasks: when a padding/gap/description formula is documented as one unified computation shared across multiple render paths (commands, legacy options, option_groups, arguments), audit every sibling path for the same contiguity gap rather than treating the first fix as complete — this exact pattern later surfaced FT-34 (legacy options) and FT-35 (option_groups) as analogous gaps, addressed separately in the same working session outside this task's own scope.

### Validation Results

- **Validated by:** validator
- **Date:** 2026-07-15
- **Verdict:** PASS

#### Checklist

- [x] C1 — Does `tests/help.rs` have `test_padded_name_contiguous_with_gap_and_description`? — YES: `tests/help.rs:986` defines `fn test_padded_name_contiguous_with_gap_and_description()`; confirmed present via direct `Read` of the file.
- [x] C2 — Does the function PASS under `cargo nextest run --all-features`? — YES: independently re-ran `cargo nextest run --all-features` from the crate root; output line `PASS [ 0.019s] (30/99) cli_fmt::help test_padded_name_contiguous_with_gap_and_description`; overall summary `99 tests run: 99 passed, 0 skipped`.
- [x] C3 — Is the `⏳` marker removed from FT-33's header in `tests/docs/feature/002_cli_help_template.md`, with the Tests table citing the function? — YES: `tests/docs/feature/002_cli_help_template.md:202` header reads `### FT-33: Padded short command name is immediately contiguous with col_gap and description at default settings` (no `⏳` suffix); `grep -n "⏳" tests/docs/feature/002_cli_help_template.md` returns zero matches (exit code 1); Tests table row at line 236 contains `FT-33: \`test_padded_name_contiguous_with_gap_and_description\` (T-B13)`.
- [x] C4 — Does `w3 .test l::3` pass with 0 failures? — YES: `w3` binary resolved at `/home/user1/.cargo/bin/w3`; ran the equivalent Level 3 command set directly (`RUSTFLAGS="-D warnings" cargo nextest run --all-features` → `99 tests run: 99 passed, 0 skipped`; `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` → `test result: ok. 6 passed; 0 failed`; `cargo clippy --all-targets --all-features -- -D warnings` → clean finish, zero warning lines emitted).

#### Measurements

- [x] M1 — test filter: `cargo nextest run --all-features 2>&1 | grep "test_padded_name_contiguous_with_gap_and_description"` — MET (expected 1 line, PASSED): actual output is exactly 1 line — `PASS [ 0.019s] (30/99) cli_fmt::help test_padded_name_contiguous_with_gap_and_description`.
- [x] M2 — pending-marker grep: `grep -n "⏳" tests/docs/feature/002_cli_help_template.md` — MET (expected no matches): command executed, returned empty stdout with exit code 1 (grep's "no matches" signal), confirmed no `⏳` anywhere in the file.

#### Invariants

- [x] I1 — test suite + doc tests + clippy: `RUSTFLAGS="-D warnings" cargo nextest run --all-features && RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features && cargo clippy --all-targets --all-features -- -D warnings` — HOLD: nextest `99 tests run: 99 passed, 0 skipped`; doc tests `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`; clippy finished cleanly with `-D warnings` active and produced zero warning/error lines.
- [x] I2 — decisions gate: `task/decisions.md` exists and Index shows no ❓ Open entries — HOLD: contrary to the possibility flagged for investigation, `task/decisions.md` DOES exist in this crate (`/home/user1/pro/lib/yrd_core/wtools/dev/module/core/cli_fmt/task/decisions.md`, confirmed via direct `Read`); its `## Index` table contains exactly one row, `| *(none)* | | | | | |`, i.e. zero ❓ Open entries. The gate mechanism is present and functioning as intended — no task-authoring defect to record for this item.

#### Anti-faking checks

*(Derived by validator per `validation.rulebook.md § Procedure : Pre-Walk Gate - Missing Validation Section` — the task's `## Validation` section contained Checklist, Measurements, and Invariants layers but no `### Anti-faking checks` layer. AF1–AF3 below were derived from the Acceptance Criteria, In Scope, and Out of Scope sections, specifically targeting the two shortcuts the task's own Work Procedure step 3 explicitly warns against (assertion weakening) and the Out of Scope boundary (no `src/help.rs` changes), plus the standard disabled-test guard.)*

- [x] AF1 — assertion not weakened to a non-contiguous substring: direct inspection of `tests/help.rs:999-1002` — PASS: the assertion is `out.contains( "cmd-one               do one thing" )` — a single contiguous string spanning name+padding+gap+description with no wildcard/split checks; independently recomputed via `python3` (`'cmd-one' + ' '*15 + 'do one thing'`) and the output matched the literal in the file exactly, confirming the 15-space run (13 padding + 2 col_gap) was not shortened or split into weaker separate assertions.
- [x] AF2 — Out of Scope boundary respected, `src/help.rs` unmodified: `git diff --stat -- src/help.rs` — PASS: command returned empty output (zero lines), confirming no working-tree changes to `src/help.rs`, matching the task's Out of Scope claim that "the padding/gap/description rendering path is already implemented."
- [x] AF3 — no disabled-test marker on the new function: `grep -n "#\[ignore\]\|#\[ ignore\]" tests/help.rs` scoped to the new function's body (`tests/help.rs:986-1003`) — PASS: no `#[ignore]` attribute present; function is a plain `#[ test ]` and was observed to execute (not skip) in the nextest run (`99 tests run: 99 passed, 0 skipped` — 0 skipped confirms no ignore markers fired anywhere in the suite).

**Environmental observation (non-blocking):** `git diff --stat -- tests/help.rs tests/docs/feature/002_cli_help_template.md` shows a larger uncommitted delta than task 015's own In Scope (128 added lines in `tests/help.rs` covering FT-33 AND FT-34/`test_padded_opt_name_contiguous_with_gap_and_description` AND FT-35/`test_option_group_differential_padding_within_group` AND T-B15/prelude re-export test; similarly the doc file's diff touches FT-31/FT-32 header `⏳`-removal alongside FT-33/34/35 additions). These additional tests/doc entries are outside task 015's declared In Scope (which names only FT-33) but are unrelated siblings already present, passing, and not conflicting with FT-33's own claim — no separate task file for FT-34/FT-35 exists in this crate's `task/` tree, suggesting they were delivered in the same uncommitted working-tree state from adjacent work. This does not affect task 015's own verdict since its specific deliverable (FT-33 test function, doc marker, table row) is independently correct and isolated from the FT-34/FT-35 additions by disjoint file byte-ranges — recorded here for audit-trail completeness, not as a Blocking Finding against this task.

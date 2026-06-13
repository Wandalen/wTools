# Fill test coverage gaps — implement all ⬜ spec cases across algorithm, invariant, and feature specs

## Execution State

- **Executor Type:** any
- **Actor:** dev
- **Claimed At:** 2026-05-16
- **Status:** ✅ (Complete)

## Goal

Implement the 56 remaining missing Rust test functions across existing test files to bring all ⬜ spec cases to ✅ status in the test surface spec files under `tests/docs/`, eliminating the gap between the documented test surface and its actual test implementation. (Motivated: spec completeness prevents silent behavioral regressions in areas not yet exercised; Observable: every ⬜ row in the spec Case Index tables becomes ✅ and `w3 .test level::3` passes clean; Scoped: additions only to existing test files under `tests/` — no source code changes; Testable: `cargo nextest run --all-features 2>&1 | grep -E "PASS|FAIL"` shows 0 failures and the new test names are present.)

The spec files in `tests/docs/` describe the intended behavioral contract of the crate. There are now 56 remaining ⬜ cases:
- 30 original (25 across 6 algorithm specs + 4 invariant specs, plus 6 preset cases added to `invariant/003`)
- 26 newly added cases (2026-05-17 spec audit): 16 algorithm edge cases (AC-9–AC-12 across 001–006), 3 invariant cases (IN-7 in `invariant/001`, IN-7–IN-8 in `invariant/002`), and 7 feature cases (FT-5 in `feature/003`+`feature/004`, FT-7 in `feature/001`+`feature/005`, FT-7–FT-8 in `feature/002`, FT-7–FT-8 in `feature/005`)

One case was already implemented before this task started: `FI IN-5` (`multiple_rows_same_data_consistent_fold_point` in `tests/auto_fold_test.rs`) is ✅. Each remaining ⬜ case has a Given/When/Then body in the spec that specifies exactly what the test must assert. The test files that own these cases already exist; this task adds the missing test functions to them.

No source changes are expected. If implementing a test reveals a latent bug, document it with a `bug_reproducer(issue-NNN)` test per the bug-fix workflow and create a separate fix task; do not patch source code within this task.

## In Scope

Paths are relative to the crate root (`module/core/data_fmt/`).

**Algorithm cases (original + new):**
- `tests/multiline_cells.rs` — AC-7 (CSV escapes newlines), AC-8 (sub-row detail after sub-lines), AC-9 (truncation on last sub-line), AC-10 (3+ embedded newlines)
- `tests/word_wrap.rs` — AC-6 (Hard break at exact boundary), AC-7 (Truncate overflow), AC-8 (Ellipsis overflow), AC-9 (preserve_newlines), AC-10 (tab_width>0), AC-11 (WordThenHard fallthrough), AC-12 (no leading space after hard break)
- `tests/aligned_tree_configuration.rs` — AC-6 (max_depth), AC-7 (show_root=false), AC-8 (min_column_width floor), AC-9 (show_branches=false), AC-10 (custom separator), AC-11 (max_depth=0), AC-12 (show_root=false + max_depth combined)
- `tests/auto_wrap_test.rs` — AC-6 (CSV/TSV bypass), AC-7 (remainder to leftmost flex), AC-8 (flex budget floored), AC-9 (12-char threshold boundary), AC-10 (overhead > terminal)
- `tests/auto_fold_test.rs` — AC-6 (FoldStyle::Bare), AC-7 (per-row fold for short rows), AC-8 (single overflow column), AC-9 (FoldStyle::Stacked), AC-10 (all columns overflow), AC-11 (very narrow terminal)
- `tests/text_cli_help.rs` — AC-7 (ANSI in keys excluded from width), AC-8 (mixed-case not header), AC-9 (all-uppercase + non-empty second col), AC-10 (per-section alignment reset)

**Invariant cases (original + new):**
- `tests/auto_wrap_test.rs` — WC IN-3 through IN-10 (8 preset backward-compat cases)
- `tests/data.rs` — IN-5 (row_details parallel length), IN-6 (TableShapedView extraction), IN-7 (empty tree → empty string)
- `tests/unicode_display_width_alignment.rs` — IN-4 (reset before newline), IN-5 (per-sub-line wrapping), IN-6 (DecoratedText raw iteration), IN-7 (CJK overflow as known limitation), IN-8 (ANSI+CJK combined gap)

**Feature cases (new):**
- `tests/table_styles_presets.rs` or `tests/column_truncation.rs` — FT-7 (min_column_width floor)
- `tests/word_wrap.rs` — FT-7 (BreakStrategy::Word standalone), FT-8 (break_long_words=true)
- `tests/unified_format_trait.rs` — FT-5 (TreeFormatter direct dispatch)
- `tests/themes.rs` — FT-5 (themes feature flag compilation)
- `tests/auto_wrap_test.rs` or `tests/terminal_width_test.rs` — FT-7 (COLUMNS env var), FT-8 (Strategy 2 before Strategy 1)

## Out of Scope

- Documentation updates (already completed)
- Source code (`src/`) changes — tests must pass against the current implementation
- New test files — add to existing files only
- Benchmark additions

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Tests must be placed in the test file identified as the owner in each spec's `## Test Implementation` header
- Every new test function must follow the Given/When/Then body from the corresponding spec case verbatim for its assertions
- No mocking — use real `TableFormatter`, `WrapFormatter`, `AlignedTreeFormatter`, etc.
- Test functions must fail loudly (no `unwrap_or_default` silence, no `if let` that swallows failure)
- Test names must match the spec case ID where possible: e.g. `csv_tsv_newline_escape_ac7`, or a descriptive name that maps unambiguously to the spec case
- If a spec case references a T-number (e.g. T11), the test name should incorporate it: `t11_multiple_rows_consistent_fold`
- Code style: 2-space indentation, opening braces on new lines, `error_tools` for error handling

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note `test_organization_universal.rulebook.md` requirements for test structure, naming, and doc-comment format.
2. **Read spec cases** — for each target file, read its spec in `tests/docs/` to extract the exact Given/When/Then assertions before writing any code.
3. **Read existing test file** — understand current patterns, helper functions, and imports in the target test file before adding to it.
4. **Write test functions** — implement all ⬜ cases for that file; mark each with `// test_kind: standard` (or `bug_reproducer` if a latent bug surfaces).
5. **Run file-scoped tests** — `cargo nextest run --test <binary_name> --all-features` after each file to confirm the new tests pass before moving to the next file. The binary name is the file stem without `.rs` (e.g. `cargo nextest run --test word_wrap --all-features`).
6. **Repeat steps 2–5** for each test file in the In Scope list.
7. **Run full suite** — `w3 .test level::3`; all tests must pass with 0 failures.
8. **Update spec status** — for each newly passing case, change `⬜` to `✅` in the corresponding spec file under `tests/docs/`.
9. **Walk Validation Checklist** — check every item; every answer must be YES.

### File processing order (recommended)

Process files in this order to build familiarity incrementally:

1. `data.rs` (IN-5, IN-6) — simplest data model assertions
2. `multiline_cells.rs` (AC-7, AC-8) — CSV escape and sub-row ordering
3. `auto_wrap_test.rs` (AC-6, AC-7, AC-8, IN-3, IN-4) — budget allocation + backward compat
4. `auto_fold_test.rs` (AC-6, AC-7, AC-8, IN-5) — fold style variants and per-row behavior
5. `word_wrap.rs` (AC-6, AC-7, AC-8, AC-9) — break strategy and overflow policy
6. `aligned_tree_configuration.rs` (AC-6, AC-7, AC-8) — tree configuration options
7. `text_cli_help.rs` (AC-7, AC-8) — ANSI-in-key and header detection
8. `unicode_display_width_alignment.rs` (IN-4, IN-5, IN-6) — ANSI reset and multiline wrapping

## Test Matrix

**Legend:** MC = `001_multiline_cell_rendering`; WW = `002_word_wrapping`; TA = `003_tree_column_alignment`; BA = `004_budget_allocation`; CF = `005_column_fold_detection`; CH = `006_cli_help_alignment`; DM = `001_data_model`; AU = `002_ansi_unicode`; WC = `003_auto_wrap_backward_compat`; FI = `004_column_fold_invariants`.

| Spec Case | Test File | Input Scenario | Config Under Test | Expected Behavior |
|-----------|-----------|----------------|-------------------|-------------------|
| MC AC-7 | `multiline_cells.rs` | Cell with `"first\nsecond"` | `TableConfig::csv()` | Single output line; newline literal in field; no sub-line split |
| MC AC-8 | `multiline_cells.rs` | Row with multiline cell + sub-row detail | Default config | Sub-lines appear before detail; detail not interleaved |
| WW AC-6 | `word_wrap.rs` | `"hello world"`, `width=7`, `BreakStrategy::Hard` | Hard break | Line 1 = `"hello w"`, line 2 = `"orld"`; no leading space |
| WW AC-7 | `word_wrap.rs` | 4-line input, `width=10`, `max_lines=2` | `OverflowPolicy::Truncate` | Exactly 2 lines; no truncation indicator |
| WW AC-8 | `word_wrap.rs` | 4-line input, `width=12`, `max_lines=2` | `OverflowPolicy::Ellipsis("...")` | 2 lines; line 2 ends with `"..."`; total ≤ 12 |
| WW AC-9 | `word_wrap.rs` | `"short\na much longer line"`, `width=10` | `preserve_newlines=true` | `"short"` segment independent; second segment wraps within 10 |
| TA AC-6 | `aligned_tree_configuration.rs` | 3-level tree, `max_depth(1)` | Depth limit | Only depth-1 nodes in output; grandchildren absent |
| TA AC-7 | `aligned_tree_configuration.rs` | Root + 2 children, `show_root(false)` | Root hidden | Root absent from output; children appear as topmost entries; column data aligned |
| TA AC-8 | `aligned_tree_configuration.rs` | All col-1 values 3 chars, `min_column_width(10)` | Floor applied | Separator at ≥ 10-char position; values padded to minimum |
| BA AC-6 | `auto_wrap_test.rs` | CSV table, natural widths > terminal | `TableConfig::csv()` + `auto_wrap=true` | No wrapping; well-formed CSV; natural lengths preserved |
| BA AC-7 | `auto_wrap_test.rs` | 3 flex cols, `budget % 3 == 1` | Remainder distribution | Leftmost flex col gets +1 char; others equal |
| BA AC-8 | `auto_wrap_test.rs` | 1 fixed + 1 flex, terminal < fixed + overhead | Budget floor | No panic; flex col budget ≥ 1; output non-empty |
| WC IN-3 | `auto_wrap_test.rs` | Same table, `unicode_box()` with/without `auto_wrap=false` | unicode_box preset | Byte-identical outputs |
| WC IN-4 | `auto_wrap_test.rs` | Same table, `markdown()` with/without `auto_wrap=false` | Markdown preset | Byte-identical outputs |
| WC IN-5 | `auto_wrap_test.rs` | Same table, `minimal()` with/without `auto_wrap=false` | Minimal preset | Byte-identical outputs |
| WC IN-6 | `auto_wrap_test.rs` | Same table, `bordered()` with/without `auto_wrap=false` | Bordered preset | Byte-identical outputs |
| WC IN-7 | `auto_wrap_test.rs` | Same table, `grid()` with/without `auto_wrap=false` | Grid preset | Byte-identical outputs |
| WC IN-8 | `auto_wrap_test.rs` | Same table, `csv()` with/without `auto_wrap=false` | CSV preset (bypass guard) | Byte-identical outputs; csv auto-bypasses auto_wrap |
| WC IN-9 | `auto_wrap_test.rs` | Same table, `tsv()` with/without `auto_wrap=false` | TSV preset (bypass guard) | Byte-identical outputs; tsv auto-bypasses auto_wrap |
| WC IN-10 | `auto_wrap_test.rs` | Same table, `compact()` with/without `auto_wrap=false` | Compact preset | Byte-identical outputs |
| CF AC-6 | `auto_fold_test.rs` | 3 overflow cols, `FoldStyle::Bare` | Bare style | All overflow values on one line; no labels |
| CF AC-7 | `auto_fold_test.rs` | Mixed row lengths in same table | Per-row fold evaluation | Long rows fold; short rows do not |
| CF AC-8 | `auto_fold_test.rs` | 4-col table, col 3 overflows only | Single overflow column | Exactly one continuation line per data row |
| FI IN-5 | `auto_fold_test.rs` | 3+ identical rows, fold configured | `auto_fold=true` | All rows fold at same index (T11) |
| DM IN-5 | `data.rs` | N `add_row` calls, mixed detail presence | `RowBuilder` | `row_details.len() == rows.len()`; absent details are `None` |
| DM IN-6 | `data.rs` | Tree implementing `TableShaped` | `TableShapedView` | Extracted headers/rows match source; no missing/duplicate rows |
| AU IN-4 | `unicode_display_width_alignment.rs` | Cell with `\x1b[32m...\x1b[0m` | `TableFormatter` | Each output line with color ends in `\x1b[0m` before `\n` |
| AU IN-5 | `unicode_display_width_alignment.rs` | `"\x1b[31mred\x1b[0m\nplain"` | Multiline cell | Sub-line 0 has ANSI; sub-line 1 has no injected codes |
| AU IN-6 | `unicode_display_width_alignment.rs` | `DecoratedText` with ANSI segments | Detail iteration | Raw segments returned unmodified; no re-rendering |
| CH AC-7 | `text_cli_help.rs` | Key with ANSI codes (`\x1b[32m--verbose\x1b[0m`) | `CliHelp` alignment | Alignment uses visual width 9; ANSI preserved in output |
| CH AC-8 | `text_cli_help.rs` | Row with first col `"Options"` (mixed-case) | Header detection | Row treated as key or simple line; NOT as section header |

## Acceptance Criteria

- All 56 remaining ⬜ spec cases are now ✅ in their respective `tests/docs/` spec files (56 implemented here + 1 already done = 57 total)
- All new test functions exist in the file specified by their spec's `## Test Implementation` header
- `cargo nextest run --all-features` passes with 0 failures (no regressions)
- `cargo clippy --all-targets --all-features -- -D warnings` emits 0 warnings
- No source file under `src/` has been modified
- Each new test function asserts the exact behavioral contract stated in the spec's `**Then:**` clause
- Tests fail loudly if the system under test misbehaves (no silent swallowing of failures)

## Validation

### Checklist

Desired answer for every question is YES.

**Test existence — algorithm**
- [ ] Does every ⬜ case in `tests/docs/algorithm/001_multiline_cell_rendering.md` have a corresponding test function? (AC-7 through AC-10)
- [ ] Does every ⬜ case in `tests/docs/algorithm/002_word_wrapping.md` have a corresponding test function? (AC-6 through AC-12)
- [ ] Does every ⬜ case in `tests/docs/algorithm/003_tree_column_alignment.md` have a corresponding test function? (AC-6 through AC-12)
- [ ] Does every ⬜ case in `tests/docs/algorithm/004_budget_allocation.md` have a corresponding test function? (AC-6 through AC-10)
- [ ] Does every ⬜ case in `tests/docs/algorithm/005_column_fold_detection.md` have a corresponding test function? (AC-6 through AC-11)
- [ ] Does every ⬜ case in `tests/docs/algorithm/006_cli_help_alignment.md` have a corresponding test function? (AC-7 through AC-10)

**Test existence — invariant**
- [ ] Does every ⬜ case in `tests/docs/invariant/001_data_model.md` have a corresponding test function? (IN-5 through IN-7)
- [ ] Does every ⬜ case in `tests/docs/invariant/002_ansi_unicode.md` have a corresponding test function? (IN-4 through IN-8)
- [ ] Does every ⬜ case in `tests/docs/invariant/003_auto_wrap_backward_compat.md` have a corresponding test function? (IN-3 through IN-10: 8 preset cases)
- [ ] Does every ⬜ case in `tests/docs/invariant/004_column_fold_invariants.md` have a corresponding test function?

**Test existence — feature**
- [ ] Does every ⬜ case in `tests/docs/feature/001_table_formatting.md` have a corresponding test function? (FT-7)
- [ ] Does every ⬜ case in `tests/docs/feature/002_word_wrap.md` have a corresponding test function? (FT-7, FT-8)
- [ ] Does every ⬜ case in `tests/docs/feature/003_unified_format_interface.md` have a corresponding test function? (FT-5)
- [ ] Does every ⬜ case in `tests/docs/feature/004_color_themes.md` have a corresponding test function? (FT-5)
- [ ] Does every ⬜ case in `tests/docs/feature/005_auto_fit.md` have a corresponding test function? (FT-7, FT-8)

**Spec status updated**
- [ ] All previously-⬜ cases now show ✅ in their spec Case Index tables?

**Test quality**
- [ ] Every new test uses a real implementation (no mocks, no `unwrap_or_default` silence)?
- [ ] Every new test would fail if the system under test behaved incorrectly?
- [ ] No test was added to a wrong file (each test is in the file specified by its spec)?

**No regressions**
- [ ] Does `w3 .test level::3` pass with 0 failures?
- [ ] Does `cargo clippy --all-targets --all-features -- -D warnings` emit 0 warnings?

**Source unchanged**
- [ ] Are all files under `src/` unmodified (no accidental source edits)?

**Out of Scope confirmation**
- [ ] Are no new test files created (only additions to existing files)?
- [ ] Are no documentation files modified beyond the ⬜→✅ status changes in spec files?

### Measurements

**M1 — New test count**
Command: `cargo nextest run --all-features 2>&1 | grep -c " PASS "`
Before: passing count is N (baseline before task). Expected: passing count is N + 56. Deviation: fewer than 56 new passing tests indicates missed cases.

**M2 — No source changes**
Command: `git diff --name-only src/`
Before: empty. Expected: empty. Deviation: any output means unauthorized source modification.

**M3 — Spec ⬜ count**
Command: `grep -r "⬜" tests/docs/ | wc -l`
Before: 56. Expected: 0. Deviation: any remaining ⬜ entries indicate missing test coverage.

### Invariants

- [ ] I1 — full test suite: `w3 .test level::3` → 0 failures
- [ ] I2 — no regressions: test count after ≥ test count before + 56
- [ ] I3 — spec sync: `grep -r "⬜" tests/docs/` → empty output

### Anti-faking checks

**AF1 — Tests actually assert behavior**
Check: `grep -c "assert" tests/word_wrap.rs`
Expected: count increases by ≥ 4 (one per new AC). Why: prevents empty test bodies that pass trivially.

**AF2 — Spec status actually updated**
Check: `grep -c "✅" tests/docs/algorithm/002_word_wrapping.md`
Expected: 9 (all 9 cases ✅). Why: catches test implementation without spec update.

**AF3 — No source touched**
Check: `git diff src/ | wc -l`
Expected: 0. Why: ensures no silent source side-effects from test writing.

**AF4 — Auto-fold tests actually assert behavior**
Check: `grep -c "assert" tests/auto_fold_test.rs`
Expected: count increases by ≥ 4 (one per new AC/IN). Why: prevents empty test bodies for the 4 fold cases (AC-6, AC-7, AC-8, IN-5).

**AF5 — ANSI/unicode tests actually assert behavior**
Check: `grep -c "assert" tests/unicode_display_width_alignment.rs`
Expected: count increases by ≥ 3 (one per new IN). Why: prevents empty test bodies for the 3 ANSI invariant cases (IN-4, IN-5, IN-6).

**AF6 — Backward compat tests cover all 8 preset cases**
Check: `grep -c "auto_wrap(false)" tests/auto_wrap_test.rs`
Expected: count increases by ≥ 8 (one per new backward-compat case: IN-3 through IN-10). Why: prevents implementing only the original 2 cases (IN-3, IN-4) while omitting the 6 new preset cases added to the spec.

## Verification Record

- **Date:** 2026-06-13
- **Method:** MAAV — two independent Agent subagents (conformance + adversarial)
- **Test result:** 605/605 tests pass; 4/4 jobs clean (nextest, workspace nextest, doc tests, clippy)
- **Conformance:** all 16 checks pass — every spec case confirmed ✅ in `tests/docs/`; no ⬜ remaining
- **Adversarial:** no behavioral regressions found; FC-4 gap documented as N/A in test spec (not a missing test)
- **Verdict:** ✅ Complete

## Outcomes


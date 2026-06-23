# Fill new test coverage gaps — FT-42..FT-44, AP-14..AP-15, FT-31, FT-32

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Reopen Count:** 0
- **State:** ✅ (Completed)
- **Closes:** null
- **Blocked Reason:** null
- **Dir:** .
- **Validated By:** null
- **Validation Date:** null

## Goal

Implement test functions for 7 new ⏳ spec cases identified by the test surface audit — (Motivated: the audit identified concrete coverage gaps in `tests/output.rs` and `tests/help.rs`: Stderr+head filter combination is untested despite the complementary Stdout+head case existing as FT-36; unicode_aware=false char-based counting has no explicit test distinguishing it from byte counting; the off-by-one upper width boundary has no test while the exact boundary is tested by FT-11; merge_streams with Stdout-only and Stderr-only filters is exercised via process_output but never directly; col_gap and cmd_indent style fields have no dedicated tests despite being configurable rendering parameters; Observable: 7 new test functions exist — `stderr_filter_with_head`, `unicode_aware_false_char_not_byte`, `width_one_over_boundary`, `merge_streams_stdout_only`, `merge_streams_stderr_only` in `tests/output.rs`; `test_col_gap_custom`, `test_cmd_indent_custom` in `tests/help.rs`; all ⏳ markers in the five affected spec cases replaced with function names; `w3 .test l::3` passes; Scoped: `tests/output.rs`, `tests/help.rs`, and the five spec files to remove ⏳ markers — no source code or Cargo.toml changes; Testable: `cargo nextest run --all-features` passes with 7 additional tests; all 7 spec cases show function name instead of ⏳ in their Tests entries).

## In Scope

- `tests/output.rs` — add 5 test functions in their relevant sections:
  - `stderr_filter_with_head` — FT-42; place after `stdout_filter_with_head`
  - `unicode_aware_false_char_not_byte` — FT-43; place after `unicode_aware_truncation`
  - `width_one_over_boundary` — FT-44; place after `width_exact_boundary`
  - `merge_streams_stdout_only` — AP-14; place in StreamFilter / stream selection section near `merge_streams_ordering`
  - `merge_streams_stderr_only` — AP-15; place after `merge_streams_stdout_only`
- `tests/help.rs` — add 2 test functions:
  - `test_col_gap_custom` — FT-31; place in a suitable section
  - `test_cmd_indent_custom` — FT-32; place after `test_col_gap_custom`
- `tests/docs/feature/001_output_processing.md` — update Tests table entry for FT-42, FT-43, FT-44: replace `⏳ not yet implemented` with the function names
- `tests/docs/api/001_output_api.md` — update Tests table entry for AP-14, AP-15: replace `⏳ not yet implemented` with the function names
- `tests/docs/feature/002_cli_help_template.md` — update Tests table entry for FT-31, FT-32: replace `⏳ not yet implemented` with the function names

## Out of Scope

- `src/output.rs`, `src/help.rs` — no logic changes; all tested behaviors are already implemented
- `Cargo.toml` — no feature additions required; all new tests compile under `--all-features`
- `docs/` — no documentation changes; specs are already updated
- Other test files (`tests/output_passthrough.rs`, `tests/manual/`) — not involved

## Requirements

- 2-space indentation per codestyle rulebook
- Each test function must include `(FT-NN)` or `(AP-NN)` in assertion messages for traceability
- No mocking; all tests use real `process_output`, `merge_streams`, and `CliHelpTemplate` implementations
- `tests/output.rs` already imports `use strs_tools::string::lines::*;` — no new imports needed for output tests
- `tests/help.rs` style: match the pattern of `test_column_alignment` and `test_style_color_defaults` for CliHelpStyle construction

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read current state** — Read `tests/output.rs` around the `stdout_filter_with_head`, `unicode_aware_truncation`, and `width_exact_boundary` functions to understand placement context. Read `tests/help.rs` to understand the existing test structure for CliHelpStyle and CliHelpData construction.
2. **Add FT-42: `stderr_filter_with_head`** — After `stdout_filter_with_head`, add:
   ```rust
   #[ test ]
   fn stderr_filter_with_head()
   {
     let config = OutputConfig::default()
       .with_stream_filter( StreamFilter::Stderr )
       .with_head( 2 );
     let result = process_output( "x", "err1\nerr2\nerr3", &config );
     assert!( !result.content.contains( "x" ),      "FT-42: stdout discarded by Stderr filter" );
     assert!(  result.content.contains( "err1" ),   "FT-42: err1 retained by head(2)" );
     assert!(  result.content.contains( "err2" ),   "FT-42: err2 retained by head(2)" );
     assert!( !result.content.contains( "err3" ),   "FT-42: err3 dropped by head(2)" );
     assert_eq!( result.lines_omitted, 1,            "FT-42: one line omitted by head(2) on 3-line stderr" );
   }
   ```
3. **Add FT-43: `unicode_aware_false_char_not_byte`** — After `unicode_aware_truncation`, add:
   ```rust
   #[ test ]
   fn unicode_aware_false_char_not_byte()
   {
     // "é" is U+00E9: 1 char, 2 bytes. unicode_aware=false uses char count (1), not byte count (2).
     // At width=1 the char count equals the limit — no truncation should fire.
     let config = OutputConfig::default().with_width( 1 );
     let result = process_output( "é", "", &config );
     assert!(
       !result.width_truncated,
       "FT-43: unicode_aware=false counts chars (1), not bytes (2); width=1 matches char count — no truncation"
     );
   }
   ```
4. **Add FT-44: `width_one_over_boundary`** — After `width_exact_boundary`, add:
   ```rust
   #[ test ]
   fn width_one_over_boundary()
   {
     // 11-char input at width=10: one char over the limit — truncation must fire.
     let config = OutputConfig::default().with_width( 10 );
     let result = process_output( "01234567890", "", &config );
     assert!(
       result.width_truncated,
       "FT-44: 11 chars at width=10 exceeds limit by 1 — truncation fires at > max_width (contrast FT-11: == max_width does not)"
     );
   }
   ```
5. **Add AP-14 and AP-15: `merge_streams_stdout_only`, `merge_streams_stderr_only`** — In the StreamFilter / stream selection section near `merge_streams_ordering`, add:
   ```rust
   #[ test ]
   fn merge_streams_stdout_only()
   {
     let result = merge_streams( "hello", "world", &StreamFilter::Stdout );
     assert_eq!( result, "hello", "AP-14: Stdout filter returns only stdout; stderr discarded" );
   }

   #[ test ]
   fn merge_streams_stderr_only()
   {
     let result = merge_streams( "hello", "world", &StreamFilter::Stderr );
     assert_eq!( result, "world", "AP-15: Stderr filter returns only stderr; stdout discarded" );
   }
   ```
6. **Add FT-31 and FT-32 to `tests/help.rs`** — Add after the last existing test function:
   ```rust
   #[ test ]
   fn test_col_gap_custom()
   {
     let style = CliHelpStyle { col_gap: 4, cmd_name_width: 7, tty_detect: false, ..CliHelpStyle::default() };
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
       out.contains( "    cmd-one    do one thing" ),
       "FT-31: col_gap=4 produces 4 spaces between padded name column and description"
     );
   }

   #[ test ]
   fn test_cmd_indent_custom()
   {
     let style = CliHelpStyle { cmd_indent: 2, cmd_name_width: 3, tty_detect: false, ..CliHelpStyle::default() };
     let mut data = CliHelpData::default();
     data.groups = vec!
     [
       CommandGroup
       {
         name    : "CMDS".into(),
         entries : vec![ CommandEntry { name: "run".into(), desc: "run the app".into() } ],
       }
     ];
     let out = CliHelpTemplate::new( style, data ).render();
     assert!(
       out.contains( "  run  run the app" ),
       "FT-32: cmd_indent=2 produces 2-space leading indent instead of default 4"
     );
   }
   ```
7. **Run Level 1** — `cargo nextest run --all-features` — all 7 new functions must PASS; if any fail, read the assertion message and fix the test against the spec.
8. **Update spec ⏳ markers** — In `tests/docs/feature/001_output_processing.md` Tests table, replace `FT-42..FT-44: ⏳ not yet implemented` with `FT-42: \`stderr_filter_with_head\`; FT-43: \`unicode_aware_false_char_not_byte\`; FT-44: \`width_one_over_boundary\``. In `tests/docs/api/001_output_api.md`, replace `AP-14..AP-15: ⏳ not yet implemented` with `AP-14: \`merge_streams_stdout_only\`; AP-15: \`merge_streams_stderr_only\``. In `tests/docs/feature/002_cli_help_template.md`, replace `FT-31..FT-32: ⏳ not yet implemented` with `FT-31: \`test_col_gap_custom\`; FT-32: \`test_cmd_indent_custom\``.
9. **Run Level 3** — `w3 .test l::3` — 0 failures, 0 clippy warnings.

## Test Matrix

| # | Input / Config | Target Test Function | Expected Assertion |
|---|----------------|---------------------|-------------------|
| T01 | stdout `"x"`, stderr `"err1\nerr2\nerr3"`, Stderr filter + head(2) | `stderr_filter_with_head` | content has err1+err2, not err3, not "x"; `lines_omitted == 1` |
| T02 | input `"é"` (1 char, 2 bytes), width=1, unicode_aware=false | `unicode_aware_false_char_not_byte` | `width_truncated == false` — char count 1 equals limit 1; byte count 2 would wrongly trigger |
| T03 | input `"01234567890"` (11 chars), width=10 | `width_one_over_boundary` | `width_truncated == true` — 11 > 10; contrast FT-11 where 10 == 10 is not truncated |
| T04 | `merge_streams("hello", "world", &StreamFilter::Stdout)` | `merge_streams_stdout_only` | returns `"hello"` — only stdout |
| T05 | `merge_streams("hello", "world", &StreamFilter::Stderr)` | `merge_streams_stderr_only` | returns `"world"` — only stderr |
| T06 | col_gap=4, cmd_name_width=7, command `"cmd-one"` / `"do one thing"` | `test_col_gap_custom` | output contains `"    cmd-one    do one thing"` |
| T07 | cmd_indent=2, cmd_name_width=3, command `"run"` / `"run the app"` | `test_cmd_indent_custom` | output contains `"  run  run the app"` |

## Acceptance Criteria

- 7 test functions added: 5 in `tests/output.rs`, 2 in `tests/help.rs`
- All 7 functions pass under `cargo nextest run --all-features`
- All ⏳ markers replaced with function names in the three spec files
- `w3 .test l::3` passes with 0 failures and 0 clippy warnings

## Validation

### Checklist

Desired answer for every question is YES.

- [x] C1 — Does `tests/output.rs` have `stderr_filter_with_head`, `unicode_aware_false_char_not_byte`, `width_one_over_boundary`, `merge_streams_stdout_only`, `merge_streams_stderr_only`?
- [x] C2 — Does `tests/help.rs` have `test_col_gap_custom` and `test_cmd_indent_custom`?
- [x] C3 — Do all 7 new functions PASS under `cargo nextest run --all-features`?
- [x] C4 — Are all ⏳ markers removed from the three spec files?
- [x] C5 — Does `w3 .test l::3` pass with 0 failures?

### Measurements

- [x] M1 — `cargo nextest run --all-features 2>&1 | grep -E "stderr_filter_with_head|unicode_aware_false|width_one_over|merge_streams_stdout|merge_streams_stderr|col_gap_custom|cmd_indent_custom"` → 7 lines, all PASSED
- [x] M2 — `grep -n "⏳" tests/docs/feature/001_output_processing.md tests/docs/api/001_output_api.md tests/docs/feature/002_cli_help_template.md` → no matches

### Invariants

- [x] I1 — `w3 .test level::3` → 0 failures, 0 clippy warnings
- [x] I2 — decisions gate: `task/decisions.md` exists and Index shows no ❓ Open entries

## Related Documentation

- `tests/docs/feature/001_output_processing.md` — FT-42, FT-43, FT-44 spec cases whose ⏳ markers this task resolves
- `tests/docs/api/001_output_api.md` — AP-14, AP-15 spec cases whose ⏳ markers this task resolves
- `tests/docs/feature/002_cli_help_template.md` — FT-31, FT-32 spec cases whose ⏳ markers this task resolves
- `docs/feature/001_output_processing.md` — Feature behavioral requirements for output processing
- `docs/feature/002_cli_help_template.md` — Feature behavioral requirements for CLI help template
- `docs/api/001_output_api.md` — Output API contract (merge_streams, process_output, StreamFilter)

## History

- **[2026-06-23]** `CREATED` — Fill 7 new ⏳ spec cases from test surface audit: FT-42 (Stderr+head), FT-43 (unicode_aware=false char count), FT-44 (width+1 boundary), AP-14 (merge_streams Stdout), AP-15 (merge_streams Stderr), FT-31 (col_gap), FT-32 (cmd_indent).
- **[2026-06-23]** `COMPLETED` — All 7 functions added; all ⏳ markers replaced; tests/readme.md updated (59+34=93 integration); all checklist/measurements/invariants checked; Level 3 PASS: 93 nextest, 6 doc, 0 clippy.

## Verification Record

- **Verified By:** MAAV — 4 independent subagents (VG-1 Scope Coherence, VG-2 MOST Goal Quality, VG-3 Value/YAGNI, VG-4 Implementation Readiness)
- **Verification Date:** 2026-06-23
- **Result:** PASS — all 4 dimensions pass

**VG-1 Scope Coherence:** PASS — In Scope lists 7 specific test functions across 3 concrete files with exact function names and placement anchors; Out of Scope names 4 excluded areas; In/Out sets are fully disjoint; observable outcomes provide literal grep commands for mechanical verification.

**VG-2 MOST Goal Quality:** PASS — Motivated by prior audit identifying 7 specific untested spec cases with symmetric cases already existing; Observable names 7 function identifiers and 3 spec files where ⏳ markers are replaced; Scoped to 2 test files and 3 spec doc files with explicit exclusions; Testable via `cargo nextest run --all-features`, two grep measurements, and Level 3 invariant.

**VG-3 Value/YAGNI (Adversarial):** PASS — All 7 ⏳ markers confirmed present in spec files; each test covers a distinct failure mode not addressed by existing functions; none duplicate existing tests; col_gap and cmd_indent configurable fields have zero non-default tests making FT-31/FT-32 concretely needed; no speculative work found.

**VG-4 Implementation Readiness:** PASS — Work Procedure provides complete ready-to-paste Rust code blocks with named placement anchors; Test Matrix covers all 7 functions; all referenced APIs (merge_streams, StreamFilter variants, CliHelpStyle fields, CommandGroup/CommandEntry) confirmed present in source; assertion string arithmetic for T06 is traceable from col_gap=4/cmd_name_width=7/cmd_indent=4 defaults.

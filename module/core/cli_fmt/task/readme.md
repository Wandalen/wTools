# Tasks

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `actors/` | Actors registry (executor records) |
| `action_plan/` | Per-actor action plans |
| `unverified/` | Tasks awaiting verification before claiming |
| `cancelled/` | Abandoned tasks with cancellation rationale |
| `completed/` | Finished tasks with completion evidence |
| `bug/` | Bug reports and investigation records |
| `decisions.md` | Decision log for cli_fmt |

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | State | Executor | Dir | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|-------|----------|-----|------|---------|
| 1 | 001 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | any | . | [Implement CliHelpTemplate in cli_fmt — typed, configurable CLI help renderer](completed/001_cli_help_template.md) | Add typed template rendering three-section CLI help (commands, options, examples) from CliHelpData with configurable CliHelpStyle and TTY-conditional ANSI colors |
| 2 | 002 | 0 | 6 | 8 | 8 | 0 | ✅ (Completed) | any | . | [Fill output test coverage gaps — exact-width boundary and head lines_omitted](completed/002_test_coverage_gaps.md) | Add `width_exact_boundary` and `process_output_head_lines_omitted` tests to cover FT-11 and FT-12 |
| 3 | 003 | 0 | 8 | 6 | 8 | 0 | ✅ (Completed) | any | . | [Fill comprehensive test coverage gaps — feature guards, boundary values, untested code paths, and spec alignment](completed/003_test_coverage_comprehensive.md) | Close 12 test gaps: unicode_aware, boundary values, color defaults, spec alignment |
| 4 | 004 | 0 | 7 | 8 | 8 | 0 | ✅ (Completed) | any | . | [Fill remaining test coverage gaps — is_default tail/width, tty_detect non-TTY, data_fmt absence](completed/004_remaining_test_gaps.md) | Close 4 remaining ⏳ spec cases: FT-24, FT-25, FT-10, FT-11 |
| 5 | 005 | 0 | 9 | 6 | 9 | 0 | ✅ (Completed) | ai | . | [Extend CliHelpTemplate — multi-section options, custom usage lines, arguments section](completed/005_extend_cli_help_template_multi_section.md) | Add option_groups, usage_lines, arguments fields to CliHelpData; update render() sequence; bump to 0.9.2 |
| 6 | 006 | 0 | 7 | 7 | 9 | 0 | ✅ (Completed) | any | . | [Complete aspirational test surface — FT-36..FT-40 and FT-29..FT-30](completed/006_aspirational_test_surface.md) | Close spec gap between readme aspirational targets (FT-1..FT-40 / FT-1..FT-30) and current spec files by adding 5 output test cases and 2 help test cases |
| 7 | 007 | 0 | 4 | 8 | 9 | 0 | ✅ (Completed) | any | . | [Fix test assertion gaps found in spec audit](completed/007_fix_test_assertion_gaps.md) | Close FT-33 missing `width_truncated` assertion and FT-17 missing suffix-absence assertion in `tests/output.rs` |
| 8 | 008 | 0 | 5 | 7 | 9 | 0 | ✅ (Completed) | any | . | [Add feature-flag line filtering passthrough test — FT-41](completed/008_feature_flag_filtering_test.md) | Introduce `output_passthrough` feature and `tests/output_passthrough.rs` to make FT-41 passthrough path testable |
| 9 | 009 | 0 | 6 | 8 | 9 | 0 | ✅ (Completed) | any | . | [Fill new test coverage gaps — FT-42..FT-44, AP-14..AP-15, FT-31, FT-32](completed/009_fill_new_test_coverage_gaps.md) | Implement 7 ⏳ spec cases from surface audit: Stderr+head, unicode_aware=false, width+1 boundary, merge_streams Stdout/Stderr, col_gap, cmd_indent |

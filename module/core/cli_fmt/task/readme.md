# Tasks

### Scope

**Responsibilities:**
Tracks all work items for the cli_fmt crate through the full task lifecycle — from unverified draft through verification, execution, validation, and completion — plus actor registry and decision log.

**In Scope:**
- Task lifecycle state directories (`unverified/`, `verifying/`, `verified/`, `executing/`, `validating/`, `completed/`, `cancelled/`)
- Bug reports and investigation records (`bug/`)
- Actor registry (`actors/`) and per-actor action plans (`action_plan/`)
- Decision log (`decisions.md`) and the Tasks Index below

**Out of Scope:**
- Crate behavioral documentation (see `../docs/`)
- Test implementation and test specifications (see `../tests/`)

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `actors/` | Actors registry (executor records) |
| `action_plan/` | Per-actor action plans |
| `unverified/` | Tasks awaiting verification before claiming |
| `verifying/` | Tasks currently in verification (motion) |
| `verified/` | Verified tasks, claimable for execution |
| `executing/` | Tasks currently in execution (motion) |
| `validating/` | Tasks currently in validation (motion) |
| `cancelled/` | Abandoned tasks with cancellation rationale |
| `completed/` | Finished tasks with completion evidence |
| `bug/` | Bug reports and investigation records |
| `decisions.md` | Decision log for cli_fmt |

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | State | Executor | UnitType | Unit | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|-------|----------|----------|------|------|---------|
| 1 | 013 | 648 | 6 | 6 | 9 | 2 | 📝 (Draft) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Add 2 missing test corner cases — empty-suffix+multi-line+ANSI width truncation; exact-fit-at-max_width with head/tail](013_missing_width_boundary_corner_case_tests.md) | Close 2 confirmed-uncovered width-truncation corner cases in `tests/output.rs` |
| 2 | 090 | 576 | 4 | 8 | 9 | 2 | 📝 (Draft) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Normalize master file Overview Table column schema to ID/Name/Purpose/Status](090_master_file_overview_table_schema.md) | 4 docs/*/readme.md files use non-canonical `# \| File \| Name \| Status` schema |
| 3 | 012 | 384 | 4 | 6 | 8 | 2 | 📝 (Draft) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Switch strs_tools dependency to workspace = true](012_strs_tools_workspace_dependency.md) | `Cargo.toml:14` pins version explicitly instead of using workspace centralization |
| 4 | 011 | 378 | 3 | 7 | 9 | 2 | 📝 (Draft) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Remove Rust-specific syntax from prose in docs/](011_rust_syntax_leaks_in_prose.md) | 6 confirmed abstraction-first violations across 2 of 3 originally-audited files |
| 5 | 001 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Implement CliHelpTemplate in cli_fmt — typed, configurable CLI help renderer](completed/001_cli_help_template.md) | Add typed template rendering three-section CLI help (commands, options, examples) from CliHelpData with configurable CliHelpStyle and TTY-conditional ANSI colors |
| 6 | 002 | 0 | 6 | 8 | 8 | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Fill output test coverage gaps — exact-width boundary and head lines_omitted](completed/002_test_coverage_gaps.md) | Add `width_exact_boundary` and `process_output_head_lines_omitted` tests to cover FT-11 and FT-12 |
| 7 | 003 | 0 | 8 | 6 | 8 | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Fill comprehensive test coverage gaps — feature guards, boundary values, untested code paths, and spec alignment](completed/003_test_coverage_comprehensive.md) | Close 12 test gaps: unicode_aware, boundary values, color defaults, spec alignment |
| 8 | 004 | 0 | 7 | 8 | 8 | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Fill remaining test coverage gaps — is_default tail/width, tty_detect non-TTY, data_fmt absence](completed/004_remaining_test_gaps.md) | Close 4 remaining ⏳ spec cases: FT-24, FT-25, FT-10, FT-11 |
| 9 | 005 | 0 | 9 | 6 | 9 | 0 | ✅ (Completed) | ai | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Extend CliHelpTemplate — multi-section options, custom usage lines, arguments section](completed/005_extend_cli_help_template_multi_section.md) | Add option_groups, usage_lines, arguments fields to CliHelpData; update render() sequence; bump to 0.9.2 |
| 10 | 006 | 0 | 7 | 7 | 9 | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Complete aspirational test surface — FT-36..FT-40 and FT-29..FT-30](completed/006_aspirational_test_surface.md) | Close spec gap between readme aspirational targets (FT-1..FT-40 / FT-1..FT-30) and current spec files by adding 5 output test cases and 2 help test cases |
| 11 | 007 | 0 | 4 | 8 | 9 | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Fix test assertion gaps found in spec audit](completed/007_fix_test_assertion_gaps.md) | Close FT-33 missing `width_truncated` assertion and FT-17 missing suffix-absence assertion in `tests/output.rs` |
| 12 | 008 | 0 | 5 | 7 | 9 | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Add feature-flag line filtering passthrough test — FT-41](completed/008_feature_flag_filtering_test.md) | Introduce `output_passthrough` feature and `tests/output_passthrough.rs` to make FT-41 passthrough path testable |
| 13 | 009 | 0 | 6 | 8 | 9 | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Fill new test coverage gaps — FT-42..FT-44, AP-14..AP-15, FT-31, FT-32](completed/009_fill_new_test_coverage_gaps.md) | Implement 7 ⏳ spec cases from surface audit: Stderr+head, unicode_aware=false, width+1 boundary, merge_streams Stdout/Stderr, col_gap, cmd_indent |
| 14 | 015 | 0 | 5 | 9 | 9 | 0 | ✅ (Completed) | any | module | lib/yrd_core/wtools/dev/module/core/cli_fmt | [Fill test coverage gap — FT-33 (padded short command name contiguous with col_gap and description)](completed/015_fill_col_gap_padding_contiguity_test.md) | Add 1 test proving padding, col_gap, and description are contiguous for a short command name at default settings — closes MAAV-validated Finding 5 |

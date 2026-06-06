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

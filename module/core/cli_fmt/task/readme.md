# Tasks

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `actors/` | Actors registry (executor records) |
| `action_plan/` | Per-actor action plans |
| `backlog/` | Candidate tasks not yet prioritized |
| `cancelled/` | Abandoned tasks with cancellation rationale |
| `completed/` | Finished tasks with completion evidence |
| `decisions.md` | Decision log for cli_fmt |

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|----------|------|---------|
| 1 | 001 | 0 | 8 | 6 | 7 | 0 | ✅ (Completed) | any | [Implement CliHelpTemplate in cli_fmt — typed, configurable CLI help renderer](completed/001_cli_help_template.md) | Add typed template rendering three-section CLI help (commands, options, examples) from CliHelpData with configurable CliHelpStyle and TTY-conditional ANSI colors |
| 2 | 002 | 0 | 6 | 8 | 8 | 0 | ✅ (Completed) | any | [Fill output test coverage gaps — exact-width boundary and head lines_omitted](completed/002_test_coverage_gaps.md) | Add `width_exact_boundary` and `process_output_head_lines_omitted` tests to cover FT-11 and FT-12 |

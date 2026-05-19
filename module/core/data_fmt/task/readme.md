# Tasks

### Scope

**Responsibilities:** Track all development tasks and decisions for the data_fmt crate.
**In Scope:** Active task files (NNN_*.md), actor registries, per-actor action plans, backlog, completed and cancelled tasks, and the decision log.
**Out of Scope:** Library source code (src/), tests (tests/), behavioral documentation (docs/).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `actors/` | Actors registry (executor records) |
| `action_plan/` | Per-actor action plans |
| `backlog/` | Candidate tasks not yet prioritized |
| `cancelled/` | Abandoned tasks with cancellation rationale |
| `completed/` | Finished tasks with completion evidence |
| `decisions.md` | Decision log for data_fmt |
| `002_fill_test_coverage_gaps.md` | Active task: implement 56 remaining ⬜ test cases across all doc entity surfaces |

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|----------|------|---------|
| 1 | 001 | 0 | 8 | 4 | 1 | 0 | ✅ (Completed) | dev | [Migrate data_fmt to DecoratedText strictly](completed/001_migrate_decorated_text_strict.md) | Eliminate 42 raw-ANSI-String gaps across data model, formatters, config, and theme layers |
| 2 | 002 | 210 | 7 | 6 | 1 | 5 | ⏳ (In Progress) | any | [Fill test coverage gaps](002_fill_test_coverage_gaps.md) | Implement 56 remaining test functions covering all ⬜ cases across 6 algorithm, 4 invariant, and 5 feature specs |

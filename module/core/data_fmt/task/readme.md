# Tasks

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `backlog/` | Candidate tasks not yet prioritized |
| `cancelled/` | Abandoned tasks with cancellation rationale |
| `completed/` | Finished tasks with completion evidence |
| `001_migrate_decorated_text_strict.md` | DecoratedText strict migration task |
| `002_fix_inner_padding_between_cells.md` | Fix inner padding not applied between columns (#1626) |

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|----------|------|---------|
| 1 | [001](001_migrate_decorated_text_strict.md) | 192 | 8 | 4 | 1 | 6 | ✅ (Completed) | dev | Migrate data_fmt to DecoratedText strictly | Eliminate 42 raw-ANSI-String gaps across data model, formatters, config, and theme layers |
| 2 | [002](002_fix_inner_padding_between_cells.md) | — | — | — | — | — | ✅ (Completed) | dev | Fix inner padding between columns | Padding was only applied at row edges; fix applies it around every cell separator |

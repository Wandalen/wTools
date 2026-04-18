# CI/CD Task Management

Work items for GitHub Actions workflow and CI/CD infrastructure changes.

## Responsibility Table

| Entry | Responsibility |
|-------|----------------|
| `backlog/` | Tasks reviewed but not yet promoted to active |
| `completed/` | Tasks that passed validation and are complete |

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------|-------|----------|--------|----------|--------|----------|------|---------|
| 1 | 004 | 189 | 3 | 7 | 9 | 1 | 📥 | any | [Clean dead workflows and ghost runs](backlog/004_dead_workflow_cleanup.md) | Remove zero-caller workflow and delete ghost workflow from GitHub |
| 2 | 002 | 0 | 10 | 8 | 7 | 0 | ✅ | any | [Fix willbe nightly ICE and upgrade checkout](completed/002_fix_willbe_nightly_ice.md) | Pin willbe install to stable toolchain; upgrade checkout@v3 to @v4 |
| 3 | 003 | 0 | 6 | 5 | 5 | 0 | ✅ | any | [Fix unitore gluesql incompatibility](completed/003_fix_unitore_gluesql.md) | Bump gluesql 0.16.3 → 0.19 and fix API migration in unitore |
| 4 | 001 | 0 | 9 | 5 | 8 | 0 | ✅ | any | [Simplify CI/CD and remove beta branch](completed/001_simplify_cicd_remove_beta.md) | Replace 64 per-crate workflows with one dynamic matrix workflow; remove beta branch |

## Statistics

- **Total Tasks:** 4
- **Active:** 0
- **Completed:** 3
- **Backlog:** 1

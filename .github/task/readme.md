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
| 1 | 006 | 0 | 8 | 9 | 9 | 0 | ✅ | any | [Modernize toolchain actions](completed/006_modernize_toolchain_actions.md) | Replace wretry+actions-rs/toolchain with dtolnay/rust-toolchain |
| 2 | 005 | 0 | 7 | 9 | 9 | 0 | ✅ | any | [Fix Dependabot crash-loop](completed/005_fix_dependabot.md) | Configure dependabot.yml to workspace root; dismiss postponed-crate alerts |
| 3 | 004 | 0 | 5 | 9 | 9 | 0 | ✅ | any | [Clean dead workflows and ghost runs](completed/004_dead_workflow_cleanup.md) | Confirm ghost workflow gone; trigger runs_clean for orphaned runs |
| 4 | 003 | 0 | 6 | 5 | 5 | 0 | ✅ | any | [Fix unitore gluesql incompatibility](completed/003_fix_unitore_gluesql.md) | Bump gluesql 0.16.3 → 0.19 and fix API migration in unitore |
| 5 | 002 | 0 | 10 | 8 | 7 | 0 | ✅ | any | [Fix willbe nightly ICE and upgrade checkout](completed/002_fix_willbe_nightly_ice.md) | Pin willbe install to stable toolchain; upgrade checkout@v3 to @v4 |
| 6 | 001 | 0 | 9 | 5 | 8 | 0 | ✅ | any | [Simplify CI/CD and remove beta branch](completed/001_simplify_cicd_remove_beta.md) | Replace 64 per-crate workflows with one dynamic matrix workflow; remove beta branch |

## Statistics

- **Total Tasks:** 6
- **Active:** 0
- **Completed:** 6
- **Backlog:** 0

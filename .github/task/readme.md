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
| 1 | 009 | 336 | 6 | 8 | 7 | 1 | 🎯 (Available) | any | [Delete stale module/core/config_hierarchy stub](009_fix_core_config_hierarchy_stub.md) | Resolve workspace build conflict by deleting stale v0.4.0 core stub |
| 2 | 008 | 0 | 9 | 8 | 9 | 0 | ✅ | any | [Fix with_none_features test failures](completed/008_fix_with_none_features_test_failures.md) | Add #![cfg(feature="enabled")] guards to 7+ crates with P2 CI failures |
| 3 | 007 | 0 | 7 | 9 | 9 | 0 | ✅ | any | [Remove vulnerable postponed crates](completed/007_remove_postponed_vuln_crates.md) | Delete optimization_tools + gspread; permanently stop Dependabot rand alerts |
| 4 | 006 | 0 | 8 | 9 | 9 | 0 | ✅ | any | [Modernize toolchain actions](completed/006_modernize_toolchain_actions.md) | Replace wretry+actions-rs/toolchain with dtolnay/rust-toolchain |
| 5 | 005 | 0 | 7 | 9 | 9 | 0 | ✅ | any | [Fix Dependabot crash-loop](completed/005_fix_dependabot.md) | Configure dependabot.yml to workspace root; dismiss postponed-crate alerts |
| 6 | 004 | 0 | 5 | 9 | 9 | 0 | ✅ | any | [Clean dead workflows and ghost runs](completed/004_dead_workflow_cleanup.md) | Confirm ghost workflow gone; trigger runs_clean for orphaned runs |
| 7 | 003 | 0 | 6 | 5 | 5 | 0 | ✅ | any | [Fix unitore gluesql incompatibility](completed/003_fix_unitore_gluesql.md) | Bump gluesql 0.16.3 → 0.19 and fix API migration in unitore |
| 8 | 002 | 0 | 10 | 8 | 7 | 0 | ✅ | any | [Fix willbe nightly ICE and upgrade checkout](completed/002_fix_willbe_nightly_ice.md) | Pin willbe install to stable toolchain; upgrade checkout@v3 to @v4 |
| 9 | 001 | 0 | 9 | 5 | 8 | 0 | ✅ | any | [Simplify CI/CD and remove beta branch](completed/001_simplify_cicd_remove_beta.md) | Replace 64 per-crate workflows with one dynamic matrix workflow; remove beta branch |

## Statistics

- **Total Tasks:** 9
- **Active:** 1
- **Completed:** 8
- **Backlog:** 0

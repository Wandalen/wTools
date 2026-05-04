# Add Chrono Dependency with Feature Gating

**Date**: 2025-11-22
**Priority**: LOW (only if proceeding with formatting features)
**Category**: Feature Development - Dependencies
**Status**: ⛔️ (Blocked by foundation)
**Task ID**: 007
**Advisability**: TBD (depends on path chosen: MVP/Standard/Defer)

## Problem Statement

If implementing time formatting features, need chrono dependency. Must be feature-gated so users who only need time retrieval dont pay compilation cost.

## Dependencies

**Blockers**:
- Tasks 002-006 (foundation phase complete)
- **Path Decision**: User must choose MVP/Standard/Defer
- If Defer chosen, this task is CANCELED

## Requirements

Add chrono as optional dependency with feature gating:

```toml
[dependencies]
chrono = { workspace = true, optional = true, default-features = false, features = ["clock", "std"] }

[features]
default = ["enabled", "time_now"]
time_format = ["enabled", "dep:chrono"]  # New feature
full = ["enabled", "use_alloc", "time_now", "time_format"]
```

## Acceptance Criteria

- [ ] Cargo.toml updated
- [ ] Workspace Cargo.toml has chrono version (if needed)
- [ ] Builds without feature: `cargo build`
- [ ] Builds with feature: `cargo build --features time_format`
- [ ] No version conflicts in workspace
- [ ] Tests pass both ways

## Estimated Effort

30 minutes

## Notes

See `-comprehensive_redefinition_plan.md` lines 889-1029 for dependency management details (if needed).

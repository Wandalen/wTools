#### Tasks

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Task | Responsible |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|------|-------------|
| 1 | 001 | 720 | 10 | 3 | 6 | 4 | ✅ (Completed 2025-12-20) | [fix_tree_scoped_publication_version_conflict](./001_fix_tree_scoped_publication_version_conflict.md) | @user |
| 2 | - | - | - | - | - | High | Not Started | [remove_pth_std_feature_dependency_task](./remove_pth_std_feature_dependency_task.md) | @user |

**Note:** Task 001 follows standardized structure (Advisability = Value × Easiness × Safety × Priority = 10 × 3 × 6 × 4 = 720). Legacy tasks use simplified format.

---

### Issues Index

| ID | Title | Status | Severity | Regressions |
|----|----|--------|----------|-------------|
| 001 | Tree-scoped publication causes version conflicts | ✅ Resolved (2025-12-20) | Critical | 0 |

---

### Issues

**Issue 001: Tree-Scoped Publication Bug**
- **Status:** ✅ Resolved (2025-12-20)
- **Severity:** Critical (blocks safe workspace publication)
- **Related Tasks:** 001
- **Resolution:** Implemented workspace-scoped dependency updates (find_workspace_dependents + multi-section updates)
- **Inclusion Criteria:** Complex bug requiring significant investigation (4+ hours) and system redesign
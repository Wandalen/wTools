# Task Management

Task tracking for the `mod_interface` crate.

## File Responsibility Table

| Entry | Responsibility |
|-------|----------------|
| `001_fix_use_layer_reexports.md` | Fix `use` keyword in `mod_interface!` failing to re-export child items |
| `-problem_with_attr.md` | Temp scratch log — inner attribute error in api_openai |
| `backlog/` | Tasks reviewed but not yet promoted to active |
| `completed/` | Tasks that passed validation |

---

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Executor | Task | Purpose |
|-------|----|--------------|-------|----------|--------|----------|--------|----------|------|---------|
| 1 | 001 | 1944 | 9 | 5 | 3 | 8 | ✅ | any | [Fix use_layer re-exports](completed/001_fix_use_layer_reexports.md) | Fix `use super::layer_a` in `mod_interface!` not propagating child items to parent layers |
| 2 | 002 | 216 | 6 | 6 | 6 | 4 | 📥 | any | [Compile-fail layer tests](backlog/002_compile_fail_layer_boundary_tests.md) | Add trybuild compile-fail tests for layer boundary isolation |

---

## Issues Index

| ID | Status | Task ID | Title |
|----|--------|---------|-------|

---

## Issues

*No issues recorded.*

---

## Task System Metadata

- **Last Updated:** 2026-04-17
- **Total Tasks:** 2
- **Completed:** 1
- **Active:** 0
- **Backlog:** 1

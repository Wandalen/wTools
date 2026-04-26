# Feature Doc Entity

### Scope

- **Purpose**: Document user-facing capabilities of `fs_tools` and their design rationale.
- **Responsibility**: Registry and overview of all feature doc instances for this crate.
- **In Scope**: TempDir RAII lifecycle, glob pattern matching via re-export, path traversal utilities.
- **Out of Scope**: API signatures (see `../api/`), design patterns (see `../pattern/`), correctness invariants (see `../invariant/`).

### Files

| File | Responsibility |
|------|----------------|
| `procedure.md` | Instance creation and deprecation procedure |
| `001_temp_dir_raii.md` | TempDir RAII lifecycle feature doc |
| `002_glob_pattern_matching.md` | Glob re-export feature doc |
| `003_path_traversal.md` | Path traversal utilities feature doc |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [TempDir RAII](001_temp_dir_raii.md) | Temporary directory management with automatic cleanup | ✅ |
| 002 | [Glob Pattern Matching](002_glob_pattern_matching.md) | Unix shell-style file pattern matching via re-export | ✅ |
| 003 | [Path Traversal Utilities](003_path_traversal.md) | Upward directory-tree traversal and ancestor file collection | ✅ |

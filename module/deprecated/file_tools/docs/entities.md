# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Public contracts for TempDir and path utility functions | [api/readme.md](api/readme.md) | 2 |
| `feature/` | User-facing capability descriptions and design | [feature/readme.md](feature/readme.md) | 3 |
| `invariant/` | Compile-time correctness properties | [invariant/readme.md](invariant/readme.md) | 1 |
| `pattern/` | Reusable design decisions applied in this crate | [pattern/readme.md](pattern/readme.md) | 2 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|----|------|------|
| api | 001 | TempDir API | [api/001_temp_dir.md](api/001_temp_dir.md) |
| api | 002 | Path Utilities API | [api/002_path_utilities.md](api/002_path_utilities.md) |
| feature | 001 | TempDir RAII | [feature/001_temp_dir_raii.md](feature/001_temp_dir_raii.md) |
| feature | 002 | Glob Pattern Matching | [feature/002_glob_pattern_matching.md](feature/002_glob_pattern_matching.md) |
| feature | 003 | Path Traversal Utilities | [feature/003_path_traversal.md](feature/003_path_traversal.md) |
| invariant | 001 | Std Feature Gating | [invariant/001_std_feature_gating.md](invariant/001_std_feature_gating.md) |
| pattern | 001 | RAII Cleanup Scope | [pattern/001_raii_cleanup_scope.md](pattern/001_raii_cleanup_scope.md) |
| pattern | 002 | Three-Component Path | [pattern/002_three_component_path.md](pattern/002_three_component_path.md) |

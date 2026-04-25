# Feature: Resource Discovery

### Scope

**Purpose**: Locate project files and workspace resources via glob patterns relative to the workspace root.
**Responsibility**: Expand a glob pattern anchored at the workspace root and return the set of matching absolute file paths.
**In Scope**: `find_resources()` (gated on the `glob` feature).
**Out of Scope**: File content reading, directory creation, file watching, filtering by metadata (size, mtime), non-glob path searches.

### Design

`find_resources()` prepends the workspace root to the provided pattern before glob expansion, ensuring all results are absolute paths within the workspace. Patterns follow the standard `glob` crate syntax (`**/*.rs`, `config/*.toml`, etc.).

Results are returned as `Vec<PathBuf>`. Ordering follows the glob crate's filesystem traversal order, which is deterministic for a given directory state but not guaranteed to be sorted lexicographically.

The feature's design is intentionally minimal — a single method with a single responsibility. More complex operations (filtering, transformation, content reading) are left to callers to avoid scope creep.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/lib.rs` | Glob impl block, `find_resources()` |
| Test | `tests/comprehensive_test_suite.rs` | Full coverage including glob patterns |
| Test | `tests/feature_combination_tests.rs` | Feature flag combination correctness |
| Doc | `docs/api/001_workspace.md` | `find_resources()` method signature |

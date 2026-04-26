# Feature: Resource Discovery

### Scope

- **Purpose**: Locate project files and workspace resources via glob patterns relative to the workspace root.
- **Responsibility**: Expand a glob pattern anchored at the workspace root and return the set of matching absolute file paths.
- **In Scope**: Glob-pattern file discovery anchored at the workspace root (requires `glob` feature).
- **Out of Scope**: File content reading, directory creation, file watching, filtering by metadata (size, mtime), non-glob path searches.

### Design

The workspace root is prepended to the provided pattern before expansion, ensuring all results are absolute paths within the workspace. Patterns follow standard glob syntax (`**/*.rs`, `config/*.toml`, etc.).

Results are returned as an ordered list of absolute paths. Ordering follows the filesystem traversal order, which is deterministic for a given directory state but not guaranteed to be sorted lexicographically.

The feature's design is intentionally minimal — a single method with a single responsibility. More complex operations (filtering, transformation, content reading) are left to callers to avoid scope creep.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Glob-based resource discovery impl |
| config | `Cargo.toml` | `glob` feature flag and its optional dependency declaration |
| test | `tests/comprehensive_test_suite.rs` | Full coverage including glob patterns |
| test | `tests/feature_combination_tests.rs` | Feature flag combination correctness |
| doc | `docs/api/001_workspace.md` | `find_resources()` method signature |

# Pattern: Three-Component Path

### Scope

- **Purpose**: Enable composable, isolation-friendly temporary directory paths without string concatenation.
- **Responsibility**: Documents the `base_path / prefix_path / postfix_path` model and its test-isolation rationale.
- **In Scope**: The three public fields, `full_path()` join semantics, typical field assignments in tests.
- **Out of Scope**: Method implementations (see `../api/001_temp_dir.md`), RAII cleanup (see `001_raii_cleanup_scope.md`).

### Problem

Temporary directory paths for tests need three independent dimensions: a shared root (system temp dir), a test-suite identifier, and a per-run unique suffix. A single-string path field forces callers to concatenate these manually on each use, making it easy to produce collisions or overly-broad paths.

### Solution

Three separate `PathBuf` fields — `base_path`, `prefix_path`, `postfix_path` — composed by `full_path()` as `base / prefix / postfix`. Empty fields are skipped during joining, so callers may use any subset of the three components. All fields are public, enabling direct assignment without a builder.

```
base_path    = /tmp              (shared root)
prefix_path  = my_app            (suite identifier)
postfix_path = session_1         (run-specific suffix)
→ full_path() = /tmp/my_app/session_1
```

### Applicability

Apply when a path has 2–3 independently-variable segments that need to be composed at use-time. Prefer a single `PathBuf` field when the path is always fully specified by the caller.

### Consequences

- **Benefit**: Each dimension is independently configurable, enabling systematic test isolation.
- **Benefit**: No builder API required; direct field assignment keeps the struct minimal.
- **Limitation**: No validation that `full_path()` is a legal filesystem path before calling `create()`.
- **Limitation**: Three fields can be more verbose than a single composed path for simple cases.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/fs/fs.rs` | Field definitions and `full_path()` implementation |
| test | `../../tests/inc/tempdir_test.rs` | Tests for `full_path` joining and empty component handling |
| doc | `../feature/001_temp_dir_raii.md` | Feature context for the TempDir struct |
| doc | `../api/001_temp_dir.md` | `full_path()` contract |
| doc | `001_raii_cleanup_scope.md` | Sibling pattern governing Drop behaviour |

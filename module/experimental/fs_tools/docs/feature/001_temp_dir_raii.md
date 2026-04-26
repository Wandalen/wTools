# Feature: TempDir RAII

### Scope

- **Purpose**: Enable tests and build tools to manage temporary directories with guaranteed cleanup.
- **Responsibility**: Documents the TempDir lifecycle — creation, path composition, directory creation, and automatic removal on drop.
- **In Scope**: `TempDir` struct construction, path composition from three components, `create()`/`create_all()` directory creation, automatic RAII cleanup via `Drop`.
- **Out of Scope**: File operations within the directory, cross-platform path handling, unique path generation (no UUID/timestamp support yet).

### Design

`TempDir` is a RAII guard for a single temporary directory. It composes a path from three public fields (`base_path`, `prefix_path`, `postfix_path`) to allow flexible test isolation: the base can be the system temp dir, the prefix can identify the test suite, and the postfix can identify the specific test run.

A private `created_path` field distinguishes auto-managed directories (created via `create()`/`create_all()`) from manually-assigned paths. Only auto-managed directories are removed on `Drop`, preventing accidental deletion of user-specified directories.

| Method | Behaviour |
|--------|-----------|
| `new()` | Allocates with empty paths; no filesystem side-effect |
| `full_path()` | Joins non-empty components: `base / prefix / postfix` |
| `create()` | Creates leaf directory; parent must exist; enables Drop cleanup |
| `create_all()` | Creates full path including parents; idempotent; enables Drop cleanup |
| `Drop` | Removes `created_path` recursively; silently ignores missing-dir errors |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/fs/fs.rs` | `TempDir` struct definition and all method implementations |
| test | `../../tests/inc/basic_test.rs` | Basic construction and field-access tests (T1.1–T1.3) |
| test | `../../tests/inc/tempdir_test.rs` | Full lifecycle tests: `full_path`, `create`, `create_all`, RAII Drop (T2–T5) |
| test | `../../tests/feature_conflict_all_features_bug.rs` | Bug reproducer: TempDir availability with `--all-features` |
| doc | `../api/001_temp_dir.md` | Public method signatures and error contracts |
| doc | `../pattern/001_raii_cleanup_scope.md` | Why Drop only removes auto-created directories |
| doc | `../pattern/002_three_component_path.md` | Three-field path composition model |
| doc | `../invariant/001_std_feature_gating.md` | Feature-flag constraint on TempDir availability |

# API: TempDir

### Scope

- **Purpose**: Document the public contract of `TempDir` — construction, path composition, directory creation, and Drop behaviour.
- **Responsibility**: Authoritative reference for all public methods and fields of `TempDir`.
- **In Scope**: `TempDir::new()`, `full_path()`, `create()`, `create_all()`, `Drop`; public fields `base_path`, `prefix_path`, `postfix_path`; feature gate requirement.
- **Out of Scope**: Internal `created_path` field, RAII design rationale (see `../pattern/001_raii_cleanup_scope.md`), feature-flag correctness constraint (see `../invariant/001_std_feature_gating.md`).

### Abstract

`TempDir` is a RAII guard for a single filesystem directory. Available when `feature = "enabled"` and `not(feature = "no_std")`. Three public path fields (`base_path`, `prefix_path`, `postfix_path`) compose via `full_path()`. Calling `create()` or `create_all()` registers the directory for automatic removal on drop.

### Operations

**`TempDir::new() -> TempDir`**
Creates instance with all path fields set to empty `PathBuf`. No filesystem side-effect. No cleanup scheduled.

**`full_path(&self) -> PathBuf`**
Joins non-empty path components: `base_path.join(prefix_path).join(postfix_path)`. Empty components are skipped. Returns the composed path without creating the directory.

**`create(&mut self) -> io::Result<PathBuf>`**
Calls `fs::create_dir(full_path())`. Parent directory must already exist (use `create_all()` for nested paths). On success, stores `full_path()` in private `created_path` and schedules Drop cleanup. Returns the created path.

**`create_all(&mut self) -> io::Result<PathBuf>`**
Calls `fs::create_dir_all(full_path())`. Creates the full path and all intermediate parents. Idempotent: succeeds if the directory already exists. Stores `full_path()` in `created_path` on success.

**`Drop::drop(&mut self)`**
If `created_path` is `Some(path)`, calls `fs::remove_dir_all(path)`. Errors from removal are silently ignored (handles pre-deleted directories without panicking).

### Error Handling

- `create()` returns `Err(io::Error)` if the parent directory does not exist or if permissions are denied.
- `create_all()` returns `Err(io::Error)` only for permission errors; missing parents are created automatically.
- `Drop` never panics; removal errors are discarded.

### Compatibility Guarantees

Available under `cfg(all(feature = "enabled", not(feature = "no_std")))`. Requires `std::fs` and `std::path::PathBuf`. No compatibility guarantees at `0.x.y`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/fs/fs.rs` | Complete `TempDir` implementation |
| test | `../../tests/inc/tempdir_test.rs` | Lifecycle and RAII correctness tests |
| test | `../../tests/inc/basic_test.rs` | Field access and default value tests |
| doc | `../feature/001_temp_dir_raii.md` | User-facing capability description and design |
| doc | `../pattern/001_raii_cleanup_scope.md` | Why Drop only removes auto-created dirs |
| doc | `../pattern/002_three_component_path.md` | Three-field path composition model |
| doc | `../invariant/001_std_feature_gating.md` | Feature-flag prerequisite for this API |

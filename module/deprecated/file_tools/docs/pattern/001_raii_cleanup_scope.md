# Pattern: RAII Cleanup Scope

### Scope

- **Purpose**: Define which directories `TempDir` automatically removes and which it leaves untouched.
- **Responsibility**: Documents the decision to restrict Drop cleanup to directories created via `create()`/`create_all()`.
- **In Scope**: The private `created_path` tracking field, Drop trigger condition, manual-path safety guarantee.
- **Out of Scope**: The path composition model (see `002_three_component_path.md`), method signatures (see `../api/001_temp_dir.md`).

### Problem

A RAII guard that removes any directory whose path it holds would silently delete directories the caller set manually, causing data loss if the caller repurposed an existing path for configuration rather than creation.

### Solution

Introduce a private `created_path: Option<PathBuf>` field set only by `create()` and `create_all()`. `Drop` checks this field rather than `full_path()`. A `TempDir` whose path was set manually (via public field assignment) but never through `create()`/`create_all()` has `created_path = None` and `Drop` is a no-op for it.

### Applicability

Apply this pattern when a RAII guard must manage a resource it created but must not manage a resource it was handed. The discriminator is the provenance of the resource, not its current state.

### Consequences

- **Benefit**: Manual path assignment is safe — callers can set `base_path` to an existing directory without risking its accidental deletion.
- **Benefit**: Double-free is safe — if the directory is deleted before `TempDir` drops, `remove_dir_all` returns an error that is silently ignored.
- **Limitation**: Callers who want cleanup for manually-assigned directories must call `create_all()` even if the directory already exists (idempotent by design).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/fs/fs.rs` | `created_path` field and Drop implementation |
| test | `../../tests/inc/tempdir_test.rs` | Tests for manual-path no-op Drop and pre-deleted directory handling |
| doc | `../feature/001_temp_dir_raii.md` | User-facing description of the RAII lifecycle |
| doc | `../api/001_temp_dir.md` | Drop method contract |

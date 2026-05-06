# API Doc Entity

### Scope

- **Purpose**: Document the public API contracts exposed by `file_tools`.
- **Responsibility**: Registry and overview of all API doc instances for this crate.
- **In Scope**: `TempDir` struct methods and Drop contract; path utility function signatures, parameters, and return types.
- **Out of Scope**: Feature behavior (see `../feature/`), design decisions (see `../pattern/`), correctness invariants (see `../invariant/`).

### Files

| File | Responsibility |
|------|----------------|
| `procedure.md` | Instance creation and deprecation procedure |
| `001_temp_dir.md` | TempDir struct API doc |
| `002_path_utilities.md` | Path utility functions API doc |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [TempDir API](001_temp_dir.md) | `TempDir` struct: constructor, path composition, directory creation, RAII Drop | ✅ |
| 002 | [Path Utilities API](002_path_utilities.md) | Five path traversal functions: upward search and ancestor collection | ✅ |

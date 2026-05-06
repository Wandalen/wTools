# Pattern Doc Entity

### Scope

- **Purpose**: Document reusable design decisions applied in `file_tools`.
- **Responsibility**: Registry and overview of all pattern doc instances for this crate.
- **In Scope**: RAII cleanup scope policy, three-component path composition model.
- **Out of Scope**: Feature behavior (see `../feature/`), API contracts (see `../api/`), correctness invariants (see `../invariant/`).

### Files

| File | Responsibility |
|------|----------------|
| `procedure.md` | Instance creation and deprecation procedure |
| `001_raii_cleanup_scope.md` | RAII cleanup scope pattern doc |
| `002_three_component_path.md` | Three-component path composition pattern doc |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [RAII Cleanup Scope](001_raii_cleanup_scope.md) | Drop only removes directories created by `create()`/`create_all()` | ✅ |
| 002 | [Three-Component Path](002_three_component_path.md) | Composable `base/prefix/postfix` path model for test isolation | ✅ |

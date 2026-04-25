# Feature: Testing Support

### Scope

**Purpose**: Create isolated temporary workspace environments for testing workspace-dependent code without polluting the real workspace.
**Responsibility**: Provision a fully-structured temporary workspace (with standard sub-directories) backed by a `TempDir` that cleans up automatically when dropped.
**In Scope**: `testing::create_test_workspace_with_structure()` (gated on the `testing` feature).
**Out of Scope**: Fixture file management, mock generation, test runner integration, assertion helpers.

### Design

`create_test_workspace_with_structure()` returns a `(TempDir, Workspace)` pair. The caller holds the `TempDir` handle; dropping it removes the entire temporary directory tree. As long as the handle is in scope, the workspace is valid and writable.

The function pre-creates the standard sub-directories (`config/`, `data/`, `logs/`, `docs/`, `tests/`), so tests can immediately write files to these paths without additional setup.

Tests can run in parallel because each call produces an independent temporary directory. There is no shared global state — test isolation is structural, not mutex-based.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Source | `src/lib.rs` | `pub mod testing`, `create_test_workspace_with_structure()` |
| Test | `tests/testing_integration_examples.rs` | Testing utilities integration |
| Test | `tests/comprehensive_test_suite.rs` | Uses test workspace for isolation |
| Doc | `docs/api/001_workspace.md` | `testing` module API |

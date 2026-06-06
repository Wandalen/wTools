# Feature: Testing Support

### Scope

- **Purpose**: Create isolated temporary workspace environments for testing workspace-dependent code without polluting the real workspace.
- **Responsibility**: Provision a fully-structured temporary workspace (with standard sub-directories) backed by a handle that cleans up the directory automatically when released.
- **In Scope**: Temporary workspace creation with pre-populated standard sub-directories (requires `testing` feature).
- **Out of Scope**: Fixture file management, mock generation, test runner integration, assertion helpers.

### Design

The factory function returns a temporary directory handle paired with a workspace rooted inside it. The caller holds the handle; releasing it removes the entire temporary directory tree. As long as the handle is live, the workspace is valid and writable.

The function pre-creates the standard sub-directories (`config/`, `data/`, `logs/`, `docs/`, `tests/`), so tests can immediately write files to these paths without additional setup.

Tests can run in parallel because each call produces an independent temporary directory. There is no shared global state — test isolation is structural, not mutex-based.

### Sources

| File | Relationship |
|------|-------------|
| [src/lib.rs](../../src/lib.rs) | Testing utility module and temporary workspace factory |
| [Cargo.toml](../../Cargo.toml) | `testing` feature flag and optional dependency declaration |

### Tests

| File | Relationship |
|------|-------------|
| [tests/testing_integration_examples.rs](../../tests/testing_integration_examples.rs) | Testing utilities integration |
| [tests/comprehensive_test_suite.rs](../../tests/comprehensive_test_suite.rs) | Uses test workspace for isolation |

### APIs

| File | Relationship |
|------|-------------|
| [api/001_workspace.md](../api/001_workspace.md) | `testing` module API |

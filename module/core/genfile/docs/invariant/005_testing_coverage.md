# Invariant: Testing Coverage

### Scope

- **Purpose**: Ensures adequate test coverage across platforms with all command paths exercised.
- **Responsibility**: Documents the coverage threshold, test infrastructure requirements, and platform targets.
- **In Scope**: Line coverage ≥80%, all commands tested, cross-platform test utilities, fast total execution.
- **Out of Scope**: Library-level coverage (→ genfile_core invariant/003), manual test procedures.

### Invariant Statement

Code coverage must be ≥80% measured by `cargo tarpaulin`. All 24 commands must have integration tests. Tests must use `CARGO_MANIFEST_DIR`-based path resolution for cross-platform compatibility. Total test suite execution must complete within 30 seconds.

### Enforcement Mechanism

`tests/test_utils.rs` provides `project_dir()` using `env!("CARGO_MANIFEST_DIR")` for portable path resolution. Each command group has a corresponding test file. `repl_command()` supports both Unix (sh pipe) and Windows (temp file redirect) execution.

### Violation Consequences

Untested commands may silently regress. Platform-specific test failures on Windows block adoption on non-Linux development environments. Slow test suites discourage running tests locally.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| test | `tests/test_utils.rs` | Cross-platform test infrastructure |

### Sources

| File | Notes |
|------|-------|
| `spec.md` (deleted) | NFR5 in original spec; combined source migrated to invariant/. spec.md has been deleted — Sources entry retained as migration record. |

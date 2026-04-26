# Invariant: Testing Coverage

### Scope

- **Purpose**: Ensures adequate test coverage across platforms with all command paths exercised.
- **Responsibility**: Documents the coverage threshold, test infrastructure requirements, and platform targets.
- **In Scope**: Line coverage ≥80%, all commands tested, cross-platform test utilities, fast total execution.
- **Out of Scope**: Library-level coverage (→ genfile_core invariant/003), manual test procedures.

### Invariant Statement

Code coverage must be ≥80% measured by a coverage tool. All 24 commands must have integration tests. Tests must use manifest-directory-based path resolution for cross-platform compatibility. Total test suite execution must complete within 30 seconds.

### Enforcement Mechanism

A shared test utilities module provides portable project directory resolution for cross-platform compatibility. Each command group has a corresponding test file. REPL test helpers support both Unix and Windows execution paths.

### Violation Consequences

Untested commands may silently regress. Platform-specific test failures on Windows block adoption on non-Linux development environments. Slow test suites discourage running tests locally.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| test | `tests/test_utils.rs` | Cross-platform test infrastructure |

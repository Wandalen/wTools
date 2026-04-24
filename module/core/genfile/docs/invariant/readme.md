# Invariant Doc Entity

### Scope

- **Purpose**: Documents non-functional constraints with measurable thresholds for the genfile CLI.
- **Responsibility**: Index of all invariant doc instances migrated from NFR sections of spec.md.
- **In Scope**: Performance, usability, error handling, security, testing, and documentation constraints.
- **Out of Scope**: Functional requirements (→ `feature/`), CLI command reference (→ `cli/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Performance](001_performance.md) | Command execution and REPL startup latency bounds | ✅ |
| 002 | [Usability](002_usability.md) | CLI rulebook compliance and consistent interface conventions | ✅ |
| 003 | [Error Handling](003_error_handling.md) | Structured error format, exit codes, and no silent failures | ✅ |
| 004 | [Security](004_security.md) | Path validation and injection prevention | ✅ |
| 005 | [Testing Coverage](005_testing_coverage.md) | Minimum coverage and cross-platform test requirements | ✅ |
| 006 | [Documentation](006_documentation.md) | Complete README, doc comments, and working examples | ✅ |

# Feature Doc Entity

### Scope

- **Purpose**: Catalog user-facing capabilities of `program_tools` with their design decisions and cross-references.
- **Responsibility**: Documents each feature as a navigational hub — scope, design rationale, and pointers to source, test, API, invariant, and pattern doc instances.
- **In Scope**: Feature scope, design rationale, and cross-references for all implemented and planned capabilities.
- **Out of Scope**: API reference detail (→ `api/`); correctness guarantees (→ `invariant/`); test coverage specifics (→ `tests/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Script Execution](001_script_execution.md) | Compile and run Rust files and projects as scripts, hiding all build complexity | ✅ |
| 002 | [Output Capture and Comparison](002_output_capture.md) | Capture stdout/stderr; compare with expected values for test assertions | ✅ |
| 003 | [Artifact Management](003_artifact_management.md) | Temporary workspaces, Cargo manifest generation, artifact cache, RAII cleanup | ✅ |
| 004 | [Programmatic Test Integration](004_programmatic_test_integration.md) | Single-expression test invocation; inline assertions; fixture crate pattern | ✅ |
| 005 | [Configuration Surface](005_configuration_surface.md) | All execution parameters exposed uniformly via builder API and CLI flags | ✅ |

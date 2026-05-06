# Invariant Doc Entity

### Scope

- **Purpose**: Document the correctness guarantees that `program_tools` maintains unconditionally across all execution paths.
- **Responsibility**: Catalogs each correctness property — its statement, rationale, enforcement mechanism, and violation consequences.
- **In Scope**: Cleanup, isolation, determinism, and error propagation guarantees.
- **Out of Scope**: Feature behavior (→ `feature/`); API surface detail (→ `api/`); test methodology.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Cleanup Guarantee](001_cleanup_guarantee.md) | Temp workspaces and build artifacts are always removed after a run | ✅ |
| 002 | [Execution Isolation](002_execution_isolation.md) | Each run operates in an independent filesystem and process context | ✅ |
| 003 | [Output Determinism](003_output_determinism.md) | Captured output is reproducible for identical inputs and configuration | ✅ |
| 004 | [Error Propagation](004_error_propagation.md) | Build and runtime failures surface as structured errors — never swallowed | ✅ |

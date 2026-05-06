# Feature: Archive Analysis

### Scope

- **Purpose**: Provides commands to inspect and analyze a loaded archive's structure and readiness.
- **Responsibility**: Documents the four analysis commands.
- **In Scope**: `.analyze`, `.discover.parameters`, `.status`, `.info`.
- **Out of Scope**: Materialization execution (→ 006), parameter definition management (→ 003).

### Design

Analysis commands operate on the currently loaded archive without modifying it. `.analyze` produces a comprehensive report of files, parameters, and completeness status. `.discover.parameters` scans template content to find all `{{param}}` placeholders, whether or not they are formally defined. `.status` gives a quick readiness summary. `.info` displays archive metadata including version, description, and author.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/handlers/analysis.rs` | Handler implementations for analysis commands |
| config | `commands/analysis.yaml` | Authoritative command specs for analysis group |
| test | `tests/analysis_test.rs` | Integration tests for analysis commands |

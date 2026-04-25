# Invariant Doc Entity

### Scope

- **Purpose**: Documents non-functional constraints with measurable thresholds for genfile_core.
- **Responsibility**: Index of all invariant doc instances for genfile_core.
- **In Scope**: Performance, memory, test coverage, compilation, documentation, error message, and compatibility constraints.
- **Out of Scope**: Functional requirements (→ `feature/`), API contracts (→ `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Rendering Performance](001_rendering_performance.md) | Template rendering latency bound | ✅ |
| 002 | [Memory Efficiency](002_memory_efficiency.md) | Heap allocation ceiling for typical use | ✅ |
| 003 | [Test Coverage](003_test_coverage.md) | Minimum line coverage threshold | ✅ |
| 004 | [Compilation Impact](004_compilation_impact.md) | Max clean build time impact on dependents | ✅ |
| 005 | [Documentation Coverage](005_documentation_coverage.md) | All public items must have doc comments | ✅ |
| 006 | [Error Message Quality](006_error_message_quality.md) | Error messages include sufficient diagnostic context | ✅ |
| 007 | [Backward Compatibility](007_backward_compatibility.md) | Semver compliance for public API stability | ✅ |

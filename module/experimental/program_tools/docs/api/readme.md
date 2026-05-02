# API Doc Entity

### Scope

- **Purpose**: Document the public programmatic interfaces of `program_tools` exposed to external callers.
- **Responsibility**: Catalogs each API interface — its operations, error handling, and compatibility guarantees.
- **In Scope**: Builder API for plan construction; runner execution entry points; output capture and assertion; CLI interface surface.
- **Out of Scope**: Feature design rationale (→ `feature/`); correctness guarantees (→ `invariant/`); test helpers (→ `tests/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Builder API](001_builder_api.md) | Fluent builder chain for constructing execution plans | ✅ |
| 002 | [Runner API](002_runner_api.md) | Execution entry points: run, capture, and convenience constructors | ✅ |
| 003 | [Output API](003_output_api.md) | Captured output type with assertion and predicate methods | ✅ |
| 004 | [CLI Interface](004_cli_interface.md) | Command-line interface: flags, exit codes, and output behavior | ✅ |

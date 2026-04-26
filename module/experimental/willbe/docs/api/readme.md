# API Doc Entity

### Scope

- **Purpose**: Document the public API surfaces exposed by willbe.
- **Responsibility**: Registry and overview of all API doc instances.
- **In Scope**: CLI binary interface (commands, parameters, exit codes), Rust library API (public functions, types, modules).
- **Out of Scope**: Internal implementation details, architectural patterns (see `../pattern/`), feature behavior (see `../feature/`). Instance lifecycle governed by `procedure.md`.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [CLI Interface](001_cli_interface.md) | CLI commands, parameters, binary entry points | ✅ |

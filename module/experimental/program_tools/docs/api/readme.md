# API Doc Entity

### Scope

- **Purpose**: Document the public programmatic interfaces of `program_tools` exposed to external callers.
- **Responsibility**: Catalogs each API interface — its operations, error handling, and compatibility guarantees.
- **In Scope**: Public builder API for Source, Program, and Plan types; field semantics; construction entry points.
- **Out of Scope**: Internal Former derive mechanics; feature design rationale (→ `feature/`); test helpers (→ `tests/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Source, Program, and Plan](001_program_api.md) | Builder API for program representation construction | ✅ |

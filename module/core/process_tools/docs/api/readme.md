# API Doc Entity

### Scope

- **Purpose:** Document the public API surface of `process_tools` — type signatures, function contracts, and return type semantics.
- **Responsibility:** Collect one doc instance per public type or function group; specify operations, error handling, and compatibility guarantees.
- **In Scope:** Function signatures, parameter contracts, error variants, and platform compatibility for public symbols.
- **Out of Scope:** Design rationale and usage intent (→ `feature/`); behavioral contracts (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Run Builder](001_run_api.md) | `Run` builder and `RunFormer` executor API | ✅ |
| 002 | [Report](002_report_api.md) | `Report` struct fields, `Display`, and `Clone` contract | ✅ |
| 003 | [Exit Status Synthesis](003_exit_status_api.md) | `synthetic_exit_status` and convenience wrappers | ✅ |
| 004 | [Lifecycle Management](004_lifecycle_api.md) | `check`, `signal`, and `daemon` sub-module functions | ✅ |

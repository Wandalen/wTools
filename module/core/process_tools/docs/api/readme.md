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
| 004 | [Signal Lookup](004_signal_api.md) | `signal_name()`, `signal_number()`, `all_signals()` | ✅ |
| 005 | [Process Liveness Check](005_check_api.md) | `is_process_alive()`, `wait_for_exit()`, `is_pidfile_alive()` | ✅ |
| 006 | [Unix Daemonization](006_daemon_api.md) | `DaemonizeOptions`, `daemonize()`, PID file management | ✅ |
| 007 | [Environment Detection](007_environment_api.md) | `is_cicd()` feature-gated CI/CD detection | ✅ |

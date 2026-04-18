# Guide Doc Entity

### Scope

- **Purpose:** Provide end-to-end usage guides for cross-module workflows that span multiple `process_tools` capabilities.
- **Responsibility:** Collect one guide per coherent usage scenario, showing how to combine features from two or more modules to accomplish a real task.
- **In Scope:** Step-by-step workflows, integration examples, common patterns, and pitfall avoidance for multi-module use cases.
- **Out of Scope:** Single-function API documentation (→ `api/`); feature design rationale (→ `feature/`); behavioral contracts (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Daemon Monitoring](001_daemon_monitoring.md) | Monitor a running daemon using PID files and liveness checks | ✅ |
| 002 | [Test Exit Status Fixtures](002_test_exit_status.md) | Construct `ExitStatus` values in tests without spawning processes | ✅ |

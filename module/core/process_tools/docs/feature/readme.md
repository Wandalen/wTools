# Feature Doc Entity

### Scope

- **Purpose:** Document functional capabilities of `process_tools` — what each feature does and why it exists.
- **Responsibility:** Collect one doc instance per discrete user-visible capability; link to related api/ and invariant/ instances.
- **In Scope:** Behavioral intent, design rationale, usage examples, and feature-to-API cross-references.
- **Out of Scope:** Function signatures and type contracts (→ `api/`); behavioral guarantees (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Process Execution](001_process_execution.md) | Subprocess execution via builder pattern | ✅ |
| 002 | [Output Capture](002_output_capture.md) | Stdout/stderr capture and display via `Report` | ✅ |
| 003 | [CI/CD Environment Detection](003_environment_detection.md) | Detect CI/CD pipeline via environment variables | ✅ |
| 004 | [Exit Status Synthesis](004_exit_status_synthesis.md) | Platform-agnostic `ExitStatus` construction | ✅ |
| 005 | [Process Lifecycle Management](005_lifecycle_management.md) | PID-based alive checks, signal mapping, daemonization | ✅ |

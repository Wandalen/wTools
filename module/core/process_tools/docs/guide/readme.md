# Guide Doc Entity

### Scope

- **Purpose:** Provide end-to-end usage guides for cross-module workflows that span multiple `process_tools` capabilities.
- **Responsibility:** Collect one guide per coherent usage scenario, showing how to combine features from two or more modules to accomplish a real task.
- **In Scope:** Step-by-step workflows, integration examples, common patterns, and pitfall avoidance for multi-module use cases.
- **Out of Scope:** Single-function API documentation (→ `api/`); feature design rationale (→ `feature/`); behavioral contracts (→ `invariant/`).

### Required Sections

Each guide instance must contain these H3 sections in order:

| Section | Requirement |
|---------|-------------|
| `### Scope` | Four tight bullets: Purpose, Responsibility, In Scope, Out of Scope |
| `### Abstract` | One-paragraph summary of what the guide teaches and when to use it |
| Workflow section(s) | One or more H3 sections showing the step-by-step usage pattern with code examples |
| `### Notes` | Pitfalls, edge cases, and behavioral nuances discovered through usage |
| `### Cross-References` | Table linking to all api/, feature/, and invariant/ docs that the guide composes |

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Daemon Monitoring](001_daemon_monitoring.md) | Monitor a running daemon using PID files and liveness checks | ✅ |
| 002 | [Test Exit Status Fixtures](002_test_exit_status.md) | Construct `ExitStatus` values in tests without spawning processes | ✅ |

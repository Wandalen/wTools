# Invariant: Execution Isolation

### Scope

- **Purpose**: Guarantee that each run operates in an independent filesystem and process context.
- **Responsibility**: Documents the isolation contract — what is isolated, the scope of isolation, and the consequences of isolation failures.
- **In Scope**: Filesystem isolation (separate temp directories per run); process isolation (separate child processes per run).
- **Out of Scope**: Artifact cache sharing policy (→ `feature/003`); network isolation; memory isolation between caller and subprocess.

### Statement

Each invocation of the runner receives its own isolated workspace directory and subprocess, ensuring no run reads, writes, or interferes with another concurrent run.

### Rationale

Test suites frequently run invocations concurrently. Without isolation, two concurrent runs writing to the same workspace directory would corrupt each other's source files and build artifacts, producing non-deterministic failures that are difficult to diagnose and impossible to reproduce reliably.

### Enforcement

Each run allocates a uniquely-named temporary directory within the system temp root, identified by process ID and a subsecond timestamp. The generated Cargo manifest references only the sources from that specific run. Cargo is invoked with `--manifest-path` pointing to the run's isolated manifest and `--target-dir` pointing to either the run's own target directory or a caller-configured shared cache. No shared mutable state exists between concurrent runs, except the optional shared artifact cache — which Cargo itself manages safely via its file-locking protocol.

### Violations

A shared artifact cache (a common `target_dir` across concurrent runs) introduces lock contention but not corruption. Cargo's file locks prevent artifact interleaving. Callers experiencing high lock wait times should either tolerate the wait or configure per-run target directories.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `docs/feature/003_artifact_management.md` | Workspace creation and artifact cache configuration |
| doc | `docs/feature/001_script_execution.md` | Script execution feature that isolation governs |
| doc | `docs/invariant/001_cleanup_guarantee.md` | Cleanup of isolated workspaces after each run |
| doc | `docs/invariant/003_output_determinism.md` | Determinism guarantee that isolation enables |
| test | `tests/inc/runner_test.rs` | Integration tests verifying per-run workspace uniqueness |

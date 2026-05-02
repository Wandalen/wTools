# Invariant: Cleanup Guarantee

### Scope

- **Purpose**: Guarantee that no temporary workspace or build artifact outlives its associated run.
- **Responsibility**: Documents the cleanup contract — when cleanup occurs, what is removed, and the conditions under which cleanup may be deferred.
- **In Scope**: Temporary workspace directories; generated Cargo manifests; compilation artifacts when not using a persistent cache.
- **Out of Scope**: Persistent artifact cache management (→ `feature/003`); process-level cleanup (→ `invariant/002`).

### Statement

Every temporary workspace created for a run is removed before the runner returns control to the caller, unless the caller explicitly defers cleanup via configuration.

### Rationale

Script execution creates temporary directories containing Cargo manifests, source files, and compiled binaries. Without guaranteed cleanup, repeated invocations accumulate dead artifacts and can exhaust disk space — especially in CI environments where tests run thousands of times.

### Enforcement

Cleanup is implemented as an explicit conditional call after the run completes. `execute_in_workspace` is called without propagating its result immediately; the workspace is removed via `std::fs::remove_dir_all` before the result is returned. This ensures cleanup executes regardless of whether the run succeeded or returned an infrastructure error. If the caller sets `run_options.cleanup = false`, cleanup is skipped and the workspace is left for the caller to inspect. If the caller configures a persistent target directory, that directory is not removed and is the caller's responsibility.

### Violations

If cleanup fails — for example, because the OS denies directory removal — the error is silently discarded and disk space leaks. Callers running in disk-constrained environments should monitor for accumulated orphaned directories under the system temp root with names matching `program_tools_<pid>_<nanos>` and sweep them periodically.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `docs/feature/003_artifact_management.md` | Artifact management feature: workspace creation and cleanup policy |
| doc | `docs/feature/001_script_execution.md` | Script execution feature that this invariant governs |
| doc | `docs/invariant/002_execution_isolation.md` | Isolation guarantee that per-run cleanup supports |
| doc | `docs/api/002_runner_api.md` | Runner API that implements cleanup through its execution lifecycle |
| doc | `docs/pattern/001_builder_hierarchy.md` | Builder configuration controlling cleanup mode |
| test | `tests/inc/runner_test.rs` | Integration tests verifying workspace removal on all exit paths |

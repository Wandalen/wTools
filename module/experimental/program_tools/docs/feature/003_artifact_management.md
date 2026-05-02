# Feature: Artifact Management

### Scope

- **Purpose**: Manage the lifecycle of temporary workspaces and build artifacts produced during script execution.
- **Responsibility**: Documents workspace allocation, source materialization, Cargo manifest generation and injection, artifact cache configuration, and cleanup.
- **In Scope**: Temporary workspace allocation; source file writing; Cargo manifest generation; build artifact directory configuration; conditional post-run cleanup.
- **Out of Scope**: Executing the materialized workspace (→ `feature/001`); capturing output from the execution (→ `feature/002`).

### Design

Every script execution requires a materialized filesystem context: source files written to disk at the correct paths, a valid Cargo manifest, and a directory for build artifacts. Artifact management governs this lifecycle from allocation through cleanup.

**Workspace allocation**: Each run allocates a fresh directory within the system temp root. The directory name incorporates the process ID and a timestamp-derived suffix to prevent collisions when multiple runs execute concurrently in the same process.

**Source materialization**: Source files from the Plan are written into the workspace at the relative paths specified in each Source entry. Intermediate directories are created as needed.

**Manifest generation**: When the Program builder carries no manifest, the runner generates a minimal Cargo.toml with:
- Package name: from `run_options.package_name` (default: `"script"`)
- Rust edition: from `run_options.edition` (default: `"2021"`)
- Dependencies: none
- Binary entry point: `src/main.rs` by convention

Programs that require external crate dependencies must supply an explicit manifest through the Program builder's manifest field. The generated manifest only covers programs using the standard library.

**Artifact caching**: By default, each run uses a fresh target directory inside the workspace, removed during cleanup. Callers can configure a persistent shared target directory via `run_options.target_dir` to cache compiled artifacts across runs. This avoids recompiling unchanged dependencies on repeated invocations and is the recommended configuration for test suites executing many runs. Cargo's file locking ensures the shared cache is safe under concurrent access.

**Cleanup**: The temporary workspace directory is removed after the run completes, regardless of exit status. Cleanup executes after `execute_in_workspace` returns, including when the run produces a non-zero exit code. It is skipped only when the caller explicitly sets `run_options.cleanup = false`. When a persistent target directory is configured, only the workspace directory is removed; the artifact cache is the caller's responsibility.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/runner.rs` | Workspace allocation, source write, manifest generation, cleanup |
| doc | `docs/invariant/001_cleanup_guarantee.md` | Cleanup guarantee that this feature must uphold |
| doc | `docs/invariant/002_execution_isolation.md` | Isolation guarantee enabled by per-run workspaces |
| doc | `docs/feature/001_script_execution.md` | Script execution that consumes the materialized workspace |
| doc | `docs/feature/005_configuration_surface.md` | Configuration parameters for workspace and cache behavior |

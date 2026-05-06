# API: Runner API

### Scope

- **Purpose**: Provide programmatic entry points for executing a plan and retrieving structured output.
- **Responsibility**: Documents the runner execution interface — execution functions, their parameters, return types, error conditions, and the execution lifecycle.
- **In Scope**: Plan execution functions; the captured output return type; error variants; execution lifecycle stages; plan convenience constructors.
- **Out of Scope**: Plan construction (→ `api/001`); output assertion methods (→ `api/003`); CLI invocation (→ `api/004`).

### Abstract

The runner accepts a completed plan and executes it: allocating a temporary workspace, writing source files, generating a Cargo manifest when absent, invoking Cargo, capturing output, and cleaning up. Four entry points cover the primary use cases — full plan execution and three convenience constructors — all returning a `Result<CapturedOutput>`.

### Operations

**Execution entry point**:

- `run(plan)` — execute a completed plan and return `Ok(CapturedOutput)`. Returns `Err` only for infrastructure failures (workspace allocation, file write, Cargo binary not found). Compilation failures and target program non-zero exits are represented as `exit_status != 0` in the returned `CapturedOutput`, not as `Err`. Callers that need to assert on exit status use `assert_exit_ok()` on the returned value.

**Convenience constructors** (build a Plan and execute in one call):

- `run_file(path)` — read a Rust source file from disk, wrap it in a generated Plan, and execute. Returns `Err` if the file cannot be read or for any infrastructure failure.
- `run_source(code)` — wrap a Rust source string in a generated Plan and execute.
- `run_project(dir, opts)` — execute an existing Cargo project directory with the given `RunOptions`. Skips manifest generation and executes the project as-is. All `RunOptions` fields apply (`cargo_path`, `build_profile`, `features`, `env_vars`, `capture`, `timeout_ms`). Returns `Err` if no `Cargo.toml` is found in `dir`.

**Execution lifecycle** (for `run`, `run_file`, `run_source`):
1. Allocate a temporary workspace directory.
2. Write source files into the workspace at their specified paths.
3. Write or generate the Cargo manifest.
4. Invoke Cargo with the configured build profile, features, and environment.
5. Capture stdout, stderr, and exit status into memory.
6. Remove the workspace (unless cleanup is disabled via `run_options.cleanup = false`).
7. Return the captured output.

### Error Handling

Infrastructure failures return a structured error: workspace allocation failure, file write failure, Cargo binary not found, manifest generation failure. Cargo compilation errors are NOT infrastructure failures — they produce non-zero exit status and compiler output in the stderr buffer of the captured output. This distinction enables callers to assert on expected compilation failures (e.g., testing that invalid code is rejected).

### Known Limitations

**Capture mode with timeout — child process not killed on expiry**: When `capture = true` and `timeout_ms` is set, the runner spawns a background thread to read the child's output pipes. If the timeout expires, the background thread continues holding the pipe; the child process is not killed and runs to natural completion. In forwarding mode (`capture = false`) with a timeout, the child is killed correctly on expiry. Callers that require hard process termination on timeout must use forwarding mode.

### Compatibility Guarantees

Version 0.1.0, marked experimental. The execution function names and signatures are expected to evolve as the feature set grows. The captured output type fields are intended to be stable but may gain additional fields before stabilization.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/runner.rs` | Runner function implementations |
| test | `tests/inc/runner_test.rs` | Integration tests for all four entry points |
| doc | `docs/api/001_builder_api.md` | Plan construction: input to the runner |
| doc | `docs/api/003_output_api.md` | Captured output type returned by runner execution |
| doc | `docs/api/004_cli_interface.md` | CLI interface that delegates to the runner |
| doc | `docs/feature/001_script_execution.md` | Script execution feature that this API implements |
| doc | `docs/feature/004_programmatic_test_integration.md` | Test integration using runner convenience constructors |
| doc | `docs/invariant/001_cleanup_guarantee.md` | Cleanup contract implemented in the execution lifecycle |
| doc | `docs/invariant/004_error_propagation.md` | Error propagation contract for infrastructure failures |

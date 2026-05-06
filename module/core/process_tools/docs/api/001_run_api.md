# API: Run Builder

### Scope

- **Purpose**: Define the complete public surface of `Run` and `RunFormer` so callers know every available configuration option and execution entry point.
- **Responsibility**: Documents all builder fields, executor methods, error return contracts, and platform compatibility guarantees for subprocess invocation.
- **In Scope**: `Run::former()`, all builder setter methods, `run()`, `run_with_shell()`, and their parameter and return type contracts.
- **Out of Scope**: `Report` type details (→ `api/002`); exit status synthesis (→ `api/003`); lifecycle APIs (→ `api/004`).

### Abstract

`Run` is the configuration struct for a subprocess invocation. It is built via the `former` derive pattern: `Run::former()` returns a `RunFormer` builder that accepts fields via method chaining and exposes `.run()` and `.run_with_shell()` for execution. `RunFormer` is the primary entry point for all subprocess invocations in this crate.

### Operations

| Symbol | Kind | Notes |
|--------|------|-------|
| `Run::former()` | constructor | Entry point for all subprocess invocations |
| `RunFormer::bin_path( path )` | builder | Path to the executable |
| `RunFormer::current_path( path )` | builder | Working directory for the spawned process |
| `RunFormer::args( args )` | builder | Command-line arguments |
| `RunFormer::joining_streams( flag )` | builder | `true` = merge stderr into stdout via duct |
| `RunFormer::env_variable( vars )` | builder | Extra env vars merged over current environment |
| `RunFormer::run()` | executor | Spawns process; returns full report on both branches |
| `RunFormer::run_with_shell( cmd )` | executor | Wraps command in `sh -c` (Unix) or `cmd /C` (Windows) |
| `run( config )` (free fn) | executor | Consumes a formed `Run`; internal dispatch |

### Error Handling

Both executors return either a success report or a failure report — the failure variant is never a bare error, always a fully-populated `Report`. Failures originate from:

- Binary not found or not executable — report is fully populated and returned as failure
- Non-zero exit code — report carries the exit error message and is returned as failure
- Non-UTF-8 stdout or stderr — report carries the encoding error and is returned as failure

Callers apply identical display logic to both outcomes. See invariant `001_result_contract.md`.

### Compatibility Guarantees

- **Platform:** cross-platform. Direct execution works on all targets via the standard process spawning API. Shell execution (`run_with_shell`) selects `sh -c` on Unix and `cmd /C` on Windows at compile time.
- **Stability:** stable since 0.1.0. Field names and builder method names are stable.
- **`joining_streams` default:** `false`. This default will not change without a semver bump.
- **Environment inheritance:** the current process environment is always merged before `env_variable`. This behavior is stable.

### Example

```rust
use process_tools::process::Run;

let report = Run::former()
  .bin_path( "echo" )
  .args( vec![ "hello".into() ] )
  .current_path( "." )
  .run()
  .unwrap();

assert!( report.out.contains( "hello" ) );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/process.rs](../../src/process.rs) | `Run` struct, `RunFormer` builder, and `run()` implementation |
| doc | [feature/001_process_execution.md](../feature/001_process_execution.md) | High-level design rationale for the execution layer |
| doc | [api/002_report_api.md](002_report_api.md) | `Report` type returned by `run()` |
| doc | [invariant/001_result_contract.md](../invariant/001_result_contract.md) | Uniform return type contract for subprocess invocations |
| doc | [invariant/002_cross_platform_shell.md](../invariant/002_cross_platform_shell.md) | Shell selection invariant for `run_with_shell()` |

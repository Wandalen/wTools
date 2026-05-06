# Feature: Process Execution

### Scope

- **Purpose**: Provide ergonomic subprocess execution without exposing platform-specific APIs to callers.
- **Responsibility**: Owns the `Run` builder, `RunFormer` builder, and `run()` free function as the sole subprocess execution entry points.
- **In Scope**: Builder construction, argument and environment assembly, backend dispatch (joining vs separate stream capture), and shell-wrapped execution.
- **Out of Scope**: Output capture design (→ `feature/002`); exit code synthesis (→ `feature/004`); post-spawn process monitoring (→ `feature/005`).

### Status

- **Version introduced:** 0.1.0
- **Stability:** stable
- **Module path:** `process_tools::process`

### Design

Two execution backends coexist behind `Run::former()`, selected via the `joining_streams` flag:

- **Joining backend** (`joining_streams = true`) — merges stderr into stdout in a single captured stream using the `duct` library. Use when downstream consumers expect a single interleaved output string (e.g., build-tool output where ordering of stdout/stderr matters).
- **Separate backend** (`joining_streams = false`, default) — captures stdout and stderr into separate string fields. Use when callers need to distinguish diagnostic output (stderr) from result output (stdout).

Both backends inherit the current process environment automatically; `env_variable` entries are merged on top. Callers never select the backend by name — they set `joining_streams` and the dispatch is encapsulated inside `process.rs`.

`RunFormer::run_with_shell()` performs compile-time platform detection and injects the platform-native shell as the binary path. This keeps the platform-detection logic in a single location and eliminates platform guards at every call site.

### Example

```rust
use process_tools::process::Run;

// Direct execution (no shell)
let report = Run::former()
  .bin_path( "echo" )
  .args( vec![ "hello".into() ] )
  .current_path( "." )
  .run()
  .expect( "echo should succeed" );
assert!( report.out.contains( "hello" ) );

// Shell execution (pipes, redirections)
let report = Run::former()
  .current_path( "." )
  .run_with_shell( "echo hello | grep hello" )
  .expect( "piped command should succeed" );
assert!( report.out.contains( "hello" ) );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/process.rs](../../src/process.rs) | `Run` struct, `RunFormer` builder, `run()` dispatch |
| test | [tests/inc/process_run.rs](../../tests/inc/process_run.rs) | Subprocess execution tests |
| test | [tests/smoke_test.rs](../../tests/smoke_test.rs) | Smoke-level execution check |
| doc | [api/001_run_api.md](../api/001_run_api.md) | `Run` and `RunFormer` type surface |
| doc | [api/002_report_api.md](../api/002_report_api.md) | `Report` return type produced by every invocation |
| doc | [invariant/001_result_contract.md](../invariant/001_result_contract.md) | Uniform return type guarantees full context on both branches |
| doc | [invariant/002_cross_platform_shell.md](../invariant/002_cross_platform_shell.md) | Shell selection is opaque to callers |
| doc | [feature/002_output_capture.md](002_output_capture.md) | Every execution produces a captured `Report` |

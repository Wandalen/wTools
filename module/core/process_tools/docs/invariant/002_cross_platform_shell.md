# Invariant: Cross-Platform Shell Abstraction

### Scope

- **Purpose**: Ensure platform-specific shell selection never leaks into caller code so automation scripts require no platform guards.
- **Responsibility**: Enforces that `RunFormer::run_with_shell()` is the sole location of the Unix/Windows shell branch in this crate.
- **In Scope**: Shell selection logic in `run_with_shell()`; guarantee that no call sites contain `cfg!(target_os)` for shell choice.
- **Out of Scope**: Shell command syntax differences; environment variable handling; output capture.

### Invariant Statement

`run_with_shell(exec_path)` always invokes the platform-native shell: `sh -c` on Unix and `cmd /C` on Windows, determined via `cfg!(target_os)` at compile time. Callers pass a shell command string and receive a `Report` identical in structure to direct execution. No platform-detection logic leaks into call sites — the abstraction is complete.

### Enforcement Mechanism

The platform branch lives exclusively in `RunFormer::run_with_shell()` in `process.rs`:

```rust
let ( program, args ) =
if cfg!( target_os = "windows" )
{
  ( "cmd", [ "/C", exec_path ] )
}
else
{
  ( "sh", [ "-c", exec_path ] )
};
```

No other function performs this selection. Call sites use only `run_with_shell(cmd)` without any `cfg!` guard.

Verification command:

```bash
grep -rn "run_with_shell" src/
# Must show no cfg!( target_os ) at call sites
# Only src/process.rs contains the platform branch
```

### Violation Consequences

If callers had to choose the shell themselves, cross-platform automation code would need a `cfg!(target_os)` guard at every invocation. Shell feature usage (pipes, redirections, environment variable expansion) would diverge silently across platforms when callers forget the guard. The invariant keeps that complexity at one location.

### Example

```rust
use process_tools::process::Run;

// No platform guard needed — run_with_shell handles it
let report = Run::former()
  .current_path( "." )
  .run_with_shell( "echo hello" )
  .expect( "shell echo should succeed" );

assert!( report.out.contains( "hello" ) );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/process.rs](../../src/process.rs) | `RunFormer::run_with_shell()` platform branch implementation |
| doc | [api/001_run_api.md](../api/001_run_api.md) | `RunFormer::run_with_shell()` definition |
| doc | [feature/001_process_execution.md](../feature/001_process_execution.md) | Design rationale for the unified execution API |

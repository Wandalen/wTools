# Feature: Process Lifecycle Management

### Scope

- **Purpose**: Enable monitoring and control of long-running processes using only their PIDs, without requiring a child-process handle.
- **Responsibility**: Owns the `lifecycle` module with `check`, `signal`, and `daemon` sub-modules for post-spawn process management.
- **In Scope**: PID liveness probing, poll-based exit waiting, POSIX signal name/number lookup, Unix double-fork daemonization, and PID file management.
- **Out of Scope**: Process spawning (→ `feature/001`); output capture (→ `feature/002`); Windows process management.

### Status

- **Version introduced:** 0.30.0
- **Stability:** experimental
- **Module path:** `process_tools::lifecycle`
- **Platform:** `check` and `signal` are Unix-only (`#[cfg(unix)]`); `daemon` is Unix-only

### Design

The `lifecycle` layer is organized into three cohesive sub-modules, each with a single responsibility:

**`check`** — PID-based process probing. Uses POSIX `kill(pid, 0)` (signal 0, null signal) to test process existence without sending a real signal. `EPERM` is treated as alive (the process exists but the caller lacks send permission). `wait_for_exit()` polls at 50 ms intervals rather than using `waitpid()` because the lifecycle layer owns no child process handle — it monitors arbitrary PIDs. `is_pidfile_alive()` composes file reading with the PID probe for daemon monitoring workflows.

**`signal`** — Bidirectional POSIX signal table. A single `const SIGNALS: &[(i32, &str)]` slice is the single source of truth for both the name→number and number→name lookups. This prevents the two lookup directions from drifting out of sync. Signal numbers are Linux-specific; macOS/BSD differ for some user signals.

**`daemon`** — Unix process daemonization. Covers the standard double-fork daemonization sequence and PID file management. Marked experimental because daemonization is complex, platform-specific, and interacts poorly with multi-threaded code.

All `check` and `signal` functions compile only on Unix; the non-Unix stubs return `Err(Unsupported)` to preserve a callable API without silent no-ops.

### Example

```rust
#[ cfg( unix ) ]
{
  use process_tools::lifecycle::{ check, signal };
  use std::time::Duration;

  // Probe the current process itself
  let pid = i32::try_from( std::process::id() ).unwrap();
  assert!( check::is_process_alive( pid ).unwrap() );

  // Signal name lookup
  assert_eq!( signal::signal_name( 9 ), "SIGKILL" );
  assert_eq!( signal::signal_number( "SIGTERM" ), Some( 15 ) );

  // Wait up to 1 second for a PID to exit (example only)
  // check::wait_for_exit( pid, Duration::from_secs( 1 ) ).ok();
}
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/lifecycle/mod.rs](../../src/lifecycle/mod.rs) | Lifecycle module re-exports and sub-module declarations |
| source | [src/lifecycle/check.rs](../../src/lifecycle/check.rs) | PID liveness probing and poll-based exit waiting |
| source | [src/lifecycle/signal.rs](../../src/lifecycle/signal.rs) | Bidirectional POSIX signal name/number table |
| source | [src/lifecycle/daemon.rs](../../src/lifecycle/daemon.rs) | Unix double-fork daemonization and PID file management |
| test | [tests/lifecycle_check_test.rs](../../tests/lifecycle_check_test.rs) | PID probe and wait_for_exit tests |
| test | [tests/lifecycle_signal_test.rs](../../tests/lifecycle_signal_test.rs) | Signal name/number lookup tests |
| test | [tests/lifecycle_daemon_test.rs](../../tests/lifecycle_daemon_test.rs) | Daemonization and PID file tests |
| doc | [api/004_signal_api.md](../api/004_signal_api.md) | Signal name/number lookup function signatures |
| doc | [api/005_check_api.md](../api/005_check_api.md) | Process liveness check function signatures |
| doc | [api/006_daemon_api.md](../api/006_daemon_api.md) | Daemonization and PID file management API |
| doc | [feature/004_exit_status_synthesis.md](004_exit_status_synthesis.md) | Lifecycle outcomes can be represented as synthetic exit statuses |
| doc | [feature/001_process_execution.md](001_process_execution.md) | Execution layer; lifecycle covers the post-spawn monitoring gap |

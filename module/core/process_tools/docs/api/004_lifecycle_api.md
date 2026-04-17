# API: Lifecycle Management

### Scope

- **Purpose**: Define the full function surface of the `lifecycle` module's three sub-modules for process monitoring and control.
- **Responsibility**: Documents all `check`, `signal`, and `daemon` function signatures, error contracts, and platform restrictions.
- **In Scope**: All `check` functions, all `signal` functions, `daemon` sub-module overview, Unix-only platform restrictions, and non-Unix fallback behavior.
- **Out of Scope**: Exit status synthesis (→ `api/003`); process spawning API (→ `api/001`).

### Abstract

`process_tools::lifecycle` exports three sub-modules covering process monitoring and control without a child-process handle. `check` provides PID-based liveness probing and exit waiting. `signal` provides bidirectional POSIX signal name/number lookup. `daemon` (Unix only) provides process daemonization and PID file management. All `check` and `signal` functions are Unix-only.

### Operations

**`lifecycle::check` sub-module** (Unix only):

| Symbol | Kind | Signature | Notes |
|--------|------|-----------|-------|
| `check::is_process_alive()` | free fn | `(pid: i32) -> io::Result<bool>` | `kill(pid,0)` probe; EPERM → alive |
| `check::wait_for_exit()` | free fn | `(pid: i32, timeout: Duration) -> io::Result<()>` | Polls at 50 ms; `Err(TimedOut)` on timeout |
| `check::is_pidfile_alive()` | free fn | `(path: &Path) -> io::Result<bool>` | Reads PID from file, then calls `is_process_alive()` |

**`lifecycle::signal` sub-module** (Unix only):

| Symbol | Kind | Signature | Notes |
|--------|------|-----------|-------|
| `signal::signal_name()` | free fn | `(signal: i32) -> &'static str` | Returns `"UNKNOWN"` for unmapped numbers |
| `signal::signal_number()` | free fn | `(name: &str) -> Option<i32>` | Case-sensitive; returns `None` for unknown names |
| `signal::all_signals()` | free fn | `() -> &'static [(i32, &'static str)]` | Full signal table; 25 entries (Linux numbers) |

**`lifecycle::daemon` sub-module** (Unix only):

| Symbol | Kind | Notes |
|--------|------|-------|
| (see source) | daemonization | Double-fork daemonization; PID file write/read/remove |

### Error Handling

**`check` functions** return `io::Result<T>`:

| Function | `Ok` meaning | `Err` meaning |
|----------|-------------|---------------|
| `is_process_alive(pid)` | `true` = alive, `false` = dead | pid ≤ 0 (`InvalidInput`), or unexpected errno |
| `wait_for_exit(pid, timeout)` | Process exited within timeout | `TimedOut` if still alive; propagates `is_process_alive` errors |
| `is_pidfile_alive(path)` | PID in file is alive (`true`) or dead (`false`) | File not found, not a valid integer, or `is_process_alive` error |

**`signal` functions** are infallible (return `&'static str` or `Option<i32>`). Unknown signals return `"UNKNOWN"` or `None` rather than errors.

**Non-Unix fallback:** on non-Unix targets, `is_process_alive()` returns `Err(Unsupported)` rather than failing to compile. This allows mixed-target code to compile, with graceful degradation at runtime.

### Compatibility Guarantees

- **Platform:** all `check` and `signal` functions require Unix. Non-Unix builds receive an `Unsupported` error at runtime, not a compile-time failure.
- **Signal table:** Linux signal numbers. macOS/BSD differ for some signals (e.g., `SIGUSR1` = 10 on Linux, 30 on macOS). The table will not change without a major version bump.
- **Poll interval:** `wait_for_exit()` polls at 50 ms. This is not configurable and may change in future versions.
- **Stability:** experimental since 0.30.0 (daemonization is complex and platform-specific).

### Example

```rust
#[ cfg( unix ) ]
{
  use process_tools::lifecycle::{ check, signal };
  use std::time::Duration;

  let pid = i32::try_from( std::process::id() ).unwrap();

  // Probe liveness
  assert!( check::is_process_alive( pid ).unwrap() );

  // Signal name/number lookup
  assert_eq!( signal::signal_name( 9 ), "SIGKILL" );
  assert_eq!( signal::signal_number( "SIGTERM" ), Some( 15 ) );
}
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/lifecycle/check.rs](../../src/lifecycle/check.rs) | PID liveness probing and poll-based exit waiting |
| source | [src/lifecycle/signal.rs](../../src/lifecycle/signal.rs) | Bidirectional POSIX signal name/number table |
| source | [src/lifecycle/daemon.rs](../../src/lifecycle/daemon.rs) | Unix double-fork daemonization and PID file management |
| feature | [feature/005_lifecycle_management.md](../feature/005_lifecycle_management.md) | Design rationale for sub-module separation |
| api | [api/003_exit_status_api.md](003_exit_status_api.md) | Exit status synthesis used alongside lifecycle monitoring |

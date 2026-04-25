# API: Process Liveness Check

### Scope

- **Purpose**: Define the PID-based process monitoring API for checking liveness, waiting for exit, and reading PID files.
- **Responsibility**: Documents `is_process_alive()`, `wait_for_exit()`, and `is_pidfile_alive()` signatures, EPERM semantics, polling contract, and non-Unix fallback.
- **In Scope**: All three `check` sub-module functions, EPERM-means-alive behavioral contract, 50 ms polling interval, PID file read format, and non-Unix runtime fallback.
- **Out of Scope**: Signal name/number lookup (→ `api/004`); daemonization and PID file writes (→ `api/006`).

### Abstract

Three free functions in `process_tools::lifecycle::check` probe process liveness using only a PID — no child-process handle required. The core primitive is POSIX `kill(pid, 0)` (null signal), which tests process existence without delivering a signal. `wait_for_exit()` polls until exit or timeout. `is_pidfile_alive()` reads a PID from a file and then probes it — composing daemon PID file management with liveness checking.

### Operations

| Symbol | Kind | Signature | Notes |
|--------|------|-----------|-------|
| `is_process_alive()` | free fn | `(pid: i32) -> io::Result<bool>` | `kill(pid,0)` probe; `EPERM` → `Ok(true)` |
| `wait_for_exit()` | free fn | `(pid: i32, timeout: Duration) -> io::Result<()>` | Polls at 50 ms; `Err(TimedOut)` on timeout |
| `is_pidfile_alive()` | free fn | `(path: &Path) -> io::Result<bool>` | Reads PID from file, then calls `is_process_alive()` |

### Error Handling

| Function | `Ok` meaning | `Err` meaning |
|----------|-------------|---------------|
| `is_process_alive(pid)` | `true` = alive, `false` = dead | `pid ≤ 0` (`InvalidInput`), or unexpected errno |
| `wait_for_exit(pid, timeout)` | Process exited within timeout | `TimedOut` if still alive; propagates `is_process_alive` errors |
| `is_pidfile_alive(path)` | PID in file is alive (`true`) or dead (`false`) | File not found, content not a valid integer, or `is_process_alive` error |

**Critical:** `EPERM` from `kill(pid, 0)` means the process IS alive — the caller lacks send permission. This returns `Ok(true)`, not `Err`. See invariant `004_eperm_means_alive.md`. Treating `EPERM` as "not alive" is the most common misuse of this function.

**Non-Unix fallback:** on non-Unix targets, `is_process_alive()` returns `Err(Unsupported)` rather than failing to compile. This allows mixed-target code to compile, with graceful degradation at runtime.

### Compatibility Guarantees

- **Platform:** Unix-only (uses `libc::kill`). Non-Unix builds compile but return `Err(Unsupported)` at runtime.
- **PID file format:** expects a decimal integer optionally surrounded by whitespace. See invariant `003_pidfile_format.md`.
- **Poll interval:** `wait_for_exit()` polls at 50 ms. Not currently configurable; may change in future versions.
- **`#[must_use]`:** `wait_for_exit()` is `#[must_use]` — ignoring its `Err` on timeout silently hides alive processes.
- **Stability:** experimental since 0.30.0.

### Example

```rust
# #[ cfg( unix ) ]
# {
use process_tools::lifecycle::check;
use std::time::Duration;

let pid = i32::try_from( std::process::id() ).unwrap();
let alive = check::is_process_alive( pid ).unwrap();
assert!( alive );

// Wait up to 5 seconds for a process to exit (example only — don't wait on self)
// check::wait_for_exit( pid, Duration::from_secs( 5 ) ).ok();
# }
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/lifecycle/check.rs](../../src/lifecycle/check.rs) | PID liveness probing and poll-based exit waiting |
| doc | [feature/005_lifecycle_management.md](../feature/005_lifecycle_management.md) | Design rationale for PID-only process monitoring |
| doc | [invariant/004_eperm_means_alive.md](../invariant/004_eperm_means_alive.md) | `EPERM` from `kill(pid,0)` means process is alive |
| doc | [invariant/003_pidfile_format.md](../invariant/003_pidfile_format.md) | PID file decimal format shared with the daemon module |
| doc | [api/006_daemon_api.md](006_daemon_api.md) | Daemon PID file writes that `is_pidfile_alive()` reads |
| doc | [guide/001_daemon_monitoring.md](../guide/001_daemon_monitoring.md) | End-to-end daemon monitoring workflow using these functions |

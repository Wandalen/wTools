# API: Unix Process Daemonization

### Scope

- **Purpose**: Define the full public surface of the `lifecycle::daemon` sub-module: PID file management functions and the daemonization entry point.
- **Responsibility**: Documents all five exported symbols — three PID file I/O functions, `DaemonizeOptions` builder, and `daemonize()` — with error contracts and the five implementation pitfalls addressed.
- **In Scope**: `write_pidfile()`, `read_pidfile()`, `remove_pidfile()`, `DaemonizeOptions`, `daemonize()`, Unix-only restriction, and singleton lock semantics.
- **Out of Scope**: PID-based process probing (→ `api/005`); signal name lookup (→ `api/004`); process spawning (→ `api/001`).

### Abstract

`process_tools::lifecycle::daemon` (Unix only) provides two capabilities: PID file management (write/read/remove) and POSIX double-fork daemonization via `daemonize()`. `DaemonizeOptions` follows the same `Former`-derived builder pattern as `Run`. After a successful `daemonize()` call, the calling code runs in a fully detached daemon process; the original parent has already exited with a clean POSIX exit.

### Operations

**PID file management:**

| Symbol | Kind | Notes |
|--------|------|-------|
| `write_pidfile( path, pid )` | free fn | Writes PID as decimal string; no newline |
| `read_pidfile( path )` | free fn | Reads and parses PID; trims whitespace |
| `remove_pidfile( path )` | free fn | Deletes PID file |

**Daemonization:**

| Symbol | Kind | Notes |
|--------|------|-------|
| `DaemonizeOptions` | struct | Former-derived builder; holds optional PID file path and working directory |
| `DaemonizeOptions::former()` | constructor | Entry point for building options |
| `daemonize( options )` | free fn | POSIX double-fork; irreversible — caller becomes the daemon |

`DaemonizeOptions` takes an optional PID file path (skipped when absent, written with exclusive-lock singleton protection when set) and a working directory (defaults to the filesystem root).

### Error Handling

| Function | Success | Failure |
|----------|---------|---------|
| `write_pidfile( path, pid )` | PID written | File cannot be created or written |
| `read_pidfile( path )` | PID read and parsed | File not found, or content not a valid integer |
| `remove_pidfile( path )` | File deleted | File not found or cannot be removed |
| `daemonize( opts )` | Caller is now the daemon process | `fork` or `setsid` failed; FD operations failed; another daemon holds the PID file lock (exclusive lock error) |

### Known Pitfalls Addressed

Five pitfalls from the double-fork daemonization pattern are addressed in the implementation:

1. **TOCTOU race** — `flock(LOCK_EX | LOCK_NB)` before any file mutation prevents concurrent instances from both seeing an empty PID file.
2. **Truncate-before-lock** — lock is acquired before truncation; not after.
3. **PID verification after IPC** — callers must verify the PID file contains the expected child PID after observing readiness signals (not preventable at API level; caller responsibility).
4. **FD closure vs redirect** — stdin/stdout/stderr are redirected to `/dev/null` instead of closed, preventing socket fd reuse by `eprintln!()`.
5. **Inherited FD leak** — all FDs from 3 to `sysconf(_SC_OPEN_MAX)` are closed after fork to prevent the parent hanging in test runners.

### Compatibility Guarantees

- **Platform:** Unix only. The entire `daemon` sub-module is absent on non-Unix targets — no fallback stub, unlike `check`.
- **PID file format:** decimal integer, no newline. See invariant `003_pidfile_format.md`.
- **`daemonize()` irreversibility:** after successful return, the parent process has exited. This cannot be undone.
- **Singleton lock lifetime:** the `flock` held after `daemonize()` is intentionally leaked for the daemon's lifetime; the OS releases it on process exit.
- **Stability:** experimental since 0.30.0.

### Example

```rust,no_run
# #[ cfg( unix ) ]
# {
use process_tools::lifecycle::daemon;
use std::path::Path;

// PID file round-trip
daemon::write_pidfile( Path::new( "/tmp/mypid.pid" ), 1234 ).unwrap();
let pid = daemon::read_pidfile( Path::new( "/tmp/mypid.pid" ) ).unwrap();
assert_eq!( pid, 1234 );
daemon::remove_pidfile( Path::new( "/tmp/mypid.pid" ) ).unwrap();

// Daemonize (no_run: caller becomes daemon process after this call)
let opts = daemon::DaemonizeOptions::former()
  .pid_file( "/var/run/mydaemon.pid" )
  .form();
daemon::daemonize( &opts ).expect( "daemonization failed" );
// — running in daemon process now —
# }
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/lifecycle/daemon.rs](../../src/lifecycle/daemon.rs) | Double-fork daemonization and PID file management |
| doc | [feature/005_lifecycle_management.md](../feature/005_lifecycle_management.md) | Design rationale for the daemon sub-module |
| doc | [invariant/003_pidfile_format.md](../invariant/003_pidfile_format.md) | PID file decimal format shared with the check module |
| doc | [api/005_check_api.md](005_check_api.md) | `is_pidfile_alive()` reads PID files written by this module |
| doc | [guide/001_daemon_monitoring.md](../guide/001_daemon_monitoring.md) | End-to-end daemon monitoring workflow |

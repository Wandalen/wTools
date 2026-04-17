# API: Lifecycle Management

### Statement

`process_tools::lifecycle` exports three sub-modules. `check` provides: `is_process_alive(pid: i32) -> io::Result<bool>` (POSIX `kill(pid,0)` probe, EPERM means alive), `wait_for_exit(pid: i32, timeout: Duration) -> io::Result<()>` (polls at 50 ms until exit or timeout), `is_pidfile_alive(path: &Path) -> io::Result<bool>` (reads PID from file then probes). `signal` provides bidirectional POSIX signal name/number mapping. `daemon` (Unix only) provides Unix process daemonization and PID file management. All `check` and `signal` functions are Unix-only via `#[cfg(unix)]`.

### Status

- **Version:** 0.30.0
- **Module path:** `process_tools::lifecycle`
- **Platform:** Unix only (`#[cfg(unix)]`)

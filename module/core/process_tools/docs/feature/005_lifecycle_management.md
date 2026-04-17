# Feature: Process Lifecycle Management

### Statement

The `lifecycle` layer provides three sub-layers for managing process lifetime without requiring a child-process handle: `check` for probing whether a PID is alive and waiting for exit, `signal` for POSIX signal name/number bidirectional mapping, and `daemon` (Unix only) for daemonizing a process and managing PID files. Together they let automation tools monitor and control long-running processes using only their PIDs, covering the gap between "spawn and wait" (covered by the `process` layer) and full process supervision.

### Status

- **Version introduced:** 0.30.0
- **Stability:** experimental
- **Module path:** `process_tools::lifecycle`
- **Platform:** `check` and `signal` are Unix-only (`#[cfg(unix)]`); `daemon` is Unix-only

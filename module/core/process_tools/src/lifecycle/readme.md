# Lifecycle Module

Process lifecycle management: signal mapping, process-alive detection, and Unix daemonization.

## File Responsibility Table

| File | Responsibility |
|------|---------------|
| `mod.rs` | Module aggregator registering signal, check, and daemon layers via `mod_interface!` |
| `signal.rs` | POSIX signal name/number bidirectional mapping (25 Linux signals) |
| `check.rs` | Process existence detection via `kill(pid, 0)` with ESRCH/EPERM handling |
| `daemon.rs` | Unix-only daemonization (double-fork, PID file utilities, 5 pitfall fixes) |

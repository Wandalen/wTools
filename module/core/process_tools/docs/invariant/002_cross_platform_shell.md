# Invariant: Cross-Platform Shell Abstraction

### Statement

`run_with_shell(exec_path)` always invokes the platform-native shell: `sh -c` on Unix and `cmd /C` on Windows, determined via `cfg!(target_os)` at compile time. Callers pass a shell command string and receive a `Report` identical in structure to direct execution. No platform-detection logic leaks into call sites — the abstraction is complete.

### Violation Consequence

If callers had to choose the shell themselves, cross-platform automation code would need conditional compilation at every call site. The invariant keeps that logic in one place.

### Measurement

`grep -rn "run_with_shell" src/` must show no platform-conditional logic outside of `process.rs`. Call sites use `run_with_shell` without `cfg!` guards.

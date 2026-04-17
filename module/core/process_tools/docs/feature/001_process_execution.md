# Feature: Process Execution

### Statement

The `process` layer provides ergonomic subprocess execution via the `Run` builder and `run()` function. It wraps both `duct` (for stream-joining mode) and `std::process::Command` (for separate-stream mode) behind a unified fluent API, so callers configure binary path, arguments, working directory, and environment variables without touching platform-specific types. A cross-platform `run_with_shell()` helper selects `sh -c` on Unix or `cmd /C` on Windows automatically.

### Status

- **Version introduced:** 0.1.0
- **Stability:** stable
- **Module path:** `process_tools::process`

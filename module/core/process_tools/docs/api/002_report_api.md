# API: Report

### Statement

`Report` captures the complete result of a subprocess invocation. Fields: `command: String` (the command as executed), `current_path: PathBuf` (working directory), `out: String` (captured stdout), `err: String` (captured stderr, empty when `joining_streams` is true), `error: Result<(), Error>` (execution result, `Ok(())` on zero exit code). Returned as both the `Ok` and `Err` variant of `run()`'s `Result<Report, Report>` so callers have full context on both success and failure paths.

### Status

- **Version:** 0.1.0+
- **Module path:** `process_tools::process::Report`
- **Derives:** `Debug`, `Clone`
- **Implements:** `Display`

# Feature: Output Capture

### Statement

Every subprocess invocation returns a `Report` struct that captures the executed command string, working directory, stdout, stderr, and the process result in one value. Both success and failure return a `Report` (via `Result<Report, Report>`), ensuring callers always have full diagnostic context regardless of exit code. The `Display` implementation renders the report with indented output blocks suitable for CLI tools and log output.

### Status

- **Version introduced:** 0.1.0
- **Stability:** stable
- **Module path:** `process_tools::process::Report`

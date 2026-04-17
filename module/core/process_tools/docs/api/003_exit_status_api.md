# API: Exit Status Synthesis

### Statement

Three free functions in `process_tools::exit_status` construct a `std::process::ExitStatus` without spawning a process: `synthetic_exit_status(code: i32) -> ExitStatus` (encodes `code` using the platform-appropriate format), `synthetic_success_status() -> ExitStatus` (equivalent to `synthetic_exit_status(0)`), and `synthetic_failure_status() -> ExitStatus` (equivalent to `synthetic_exit_status(1)`). All three are `#[must_use]`. Valid code range is 0–255.

### Status

- **Version:** 0.30.0
- **Module path:** `process_tools::exit_status`

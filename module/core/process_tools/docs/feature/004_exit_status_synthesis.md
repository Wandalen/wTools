# Feature: Exit Status Synthesis

### Statement

The `exit_status` layer provides `synthetic_exit_status(code)` and two convenience wrappers (`synthetic_success_status`, `synthetic_failure_status`) that construct a `std::process::ExitStatus` from an integer exit code without spawning a real process. The implementation hides the platform encoding difference: on Unix, the POSIX `waitpid` status word format requires `code << 8`; on Windows, the code is used directly as a `u32`. Callers working with testing, simulation, or status propagation use this to create `ExitStatus` values without the subprocess overhead.

### Status

- **Version introduced:** 0.30.0
- **Stability:** stable
- **Module path:** `process_tools::exit_status`
- **Pitfall:** valid range is 0–255; on Unix, codes outside this range produce an `ExitStatus` with inconsistent `code()` semantics

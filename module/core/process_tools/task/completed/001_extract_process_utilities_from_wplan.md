# Extract process management utilities from wplan

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Done)
- **Validated By:** Level 3 verification (nextest + doc tests + clippy)
- **Validation Date:** 2026-04-17

## Goal

Three sites in the wplan ecosystem independently implement process lifecycle checks, signal name mapping, and daemonization — all platform-specific code that is duplicated and tested per-project rather than centralized (Motivated: duplicated platform knowledge is a maintenance hazard — a fix in one site is missed in others, and new wTools consumers must re-derive the same platform abstractions; Observable: `process_tools::lifecycle` module exports `is_process_alive`, `signal_name`, and `daemonize` with cross-platform implementations; Scoped: add one new `lifecycle` module tree to `process_tools/src/` with three submodules `check`, `signal`, `daemon`, plus tests in `process_tools/tests/`; Testable: `is_process_alive(std::process::id() as i32)` returns `Ok(true)`, `signal_name(9)` returns `"SIGKILL"`, all tests pass on Unix).

## In Scope

- New `lifecycle` module tree in `process_tools/src/lifecycle/`:
  - `check.rs` — `is_process_alive(pid) -> io::Result<bool>`, `wait_for_exit(pid, timeout) -> io::Result<()>`, `is_pidfile_alive(path) -> io::Result<bool>`
  - `signal.rs` — `signal_name(i32) -> &'static str`, `signal_number(&str) -> Option<i32>`, `all_signals() -> Vec<(i32, &str, &str)>`
  - `daemon.rs` — `DaemonizeOptions`, `daemonize(&opts) -> io::Result<()>`, `write_pidfile(path)`, `read_pidfile(path)`, `remove_pidfile(path)` (Unix only via `#[cfg(unix)]`)
- Register module via `layer lifecycle;` in `process_tools/src/lib.rs` mod_interface block
- Platform-specific implementations: Unix (`kill(pid, 0)`, fork/setsid) and Windows (`OpenProcess`) where applicable
- Tests in `process_tools/tests/lifecycle_test.rs`

## Out of Scope

- Consumer migration (wplan, wplan_client) — covered by follow-up adoption tasks
- Async process management
- Interactive stdin handling
- Signal sending (only mapping names ↔ numbers)
- Windows daemonization (Windows Services API is a different paradigm)
- PTY spawning or terminal management

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Module follows existing `process_tools` patterns: `mod_interface` with `layer` keyword, `mod private {}` block
- Custom codestyle per `code_style.rulebook.md` — 2-space indents, no `cargo fmt`
- Tests in `process_tools/tests/` directory — no `#[cfg(test)]` in src
- No mocking — test real process behavior
- New dependency: `libc` (workspace) for Unix syscalls
- Conditional dependency: `windows` (workspace, `Win32_System_Threading` feature) for Windows process checks
- **CRITICAL**: This task is INCOMPLETE without follow-up adoption — extraction without migration leaves duplication worse (two copies instead of one)

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note code style, mod_interface patterns, test organization constraints
2. **Add dependencies** — add `libc` (workspace) and conditional `windows` dependency to `process_tools/Cargo.toml`
3. **Create check submodule** — `process_tools/src/lifecycle/check.rs` with `mod private {}` containing `is_process_alive`, `wait_for_exit`, `is_pidfile_alive`; use `#[cfg(unix)]` and `#[cfg(windows)]` inside function bodies only
4. **Create signal submodule** — `process_tools/src/lifecycle/signal.rs` with `mod private {}` containing `signal_name`, `signal_number`, `all_signals`
5. **Create daemon submodule** — `process_tools/src/lifecycle/daemon.rs` with `mod private {}` containing `DaemonizeOptions`, `daemonize`, PID file utilities; entire module gated with `#[cfg(unix)]`
6. **Create lifecycle module** — `process_tools/src/lifecycle/mod.rs` using `mod_interface!` to aggregate check, signal, daemon submodules
7. **Register module** — add `layer lifecycle;` to the `mod_interface!` block in `process_tools/src/lib.rs`
8. **Write tests** — create `process_tools/tests/lifecycle_test.rs` covering T01–T12 from Test Matrix
9. **Verify** — `cargo test -p process_tools --all-features` passes in wtools workspace
10. **Walk Validation Checklist** — verify every item answers YES

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | `is_process_alive(std::process::id() as i32)` | Unix and Windows | `Ok(true)` — current process is alive |
| T02 | `is_process_alive(-1)` | Unix and Windows | `Ok(false)` or `Err(_)` — invalid PID |
| T03 | `is_process_alive(999999)` | Unix | `Ok(false)` — nonexistent PID |
| T04 | `signal_name(9)` | Unix | `"SIGKILL"` |
| T05 | `signal_name(15)` | Unix | `"SIGTERM"` |
| T06 | `signal_name(2)` | Unix | `"SIGINT"` |
| T07 | `signal_name(999)` | Unix | `"UNKNOWN"` |
| T08 | `signal_number("SIGKILL")` | Unix | `Some(9)` |
| T09 | `signal_number("UNKNOWN")` | Unix | `None` |
| T10 | `all_signals()` | Unix | Non-empty vec with at least SIGHUP, SIGINT, SIGTERM, SIGKILL |
| T11 | `write_pidfile` + `read_pidfile` + `remove_pidfile` | Unix | Round-trip PID file lifecycle succeeds |
| T12 | `wait_for_exit` with already-dead PID | Unix | `Ok(())` — returns immediately |

## Acceptance Criteria

- `process_tools` crate exports `lifecycle` module with `is_process_alive`, `wait_for_exit`, `signal_name`, `signal_number`, `all_signals`
- `is_process_alive(std::process::id() as i32)` returns `Ok(true)` on current platform
- `signal_name(9)` returns `"SIGKILL"` on Unix
- PID file round-trip (write/read/remove) works correctly
- Daemon submodule compiles and is gated to Unix only
- `cargo test -p process_tools --all-features` passes with zero failures

## Validation

### Checklist

Desired answer for every question is YES.

**API Surface**
- [x] C1 — Does `process_tools/src/lifecycle/` directory exist with `check.rs`, `signal.rs`, `daemon.rs`, `mod.rs`?
- [x] C2 — Does `is_process_alive(pid: i32) -> io::Result<bool>` exist with cfg-gated internals?
- [x] C3 — Does `wait_for_exit(pid, timeout)` exist with polling implementation?
- [x] C4 — Do `signal_name` and `signal_number` exist with bidirectional mapping?
- [x] C5 — Does `daemonize(&DaemonizeOptions) -> io::Result<()>` exist gated to `#[cfg(unix)]`?
- [x] C6 — Do PID file utilities (`write_pidfile`, `read_pidfile`, `remove_pidfile`) exist?
- [x] C7 — Is the module registered as `layer lifecycle;` in `lib.rs` mod_interface block?

**Tests**
- [x] C8 — Does `process_tools/tests/lifecycle_check_test.rs` (and sibling files) exist?
- [x] C9 — Does it cover all 12 test matrix scenarios (T01–T12) plus 45 corner cases?
- [x] C10 — Does `cargo test -p process_tools --all-features` pass with zero failures?

**Code Quality**
- [x] C11 — Does the module use 2-space indents (custom codestyle)?
- [x] C12 — Are there zero `#[cfg(test)]` blocks in `src/lifecycle/*.rs`?
- [x] C13 — Is platform-specific code confined to `#[cfg]` blocks inside function bodies (check, signal) or at module level (daemon)?

### Measurements

- Test count: 102 nextest tests + 17 doc tests (far exceeds minimum 12)
- Platform coverage: Unix fully tested, Windows compilation verified
- API surface: 3 submodules, 11 public functions + 1 struct

### Invariants

- `is_process_alive(std::process::id() as i32)` always returns `Ok(true)` regardless of platform
- `signal_name(signal_number(name).unwrap())` round-trips for all standard signal names
- PID file write + read returns the same PID value

### Anti-Faking

- Tests must call real OS syscalls (no mocking `kill` or `OpenProcess`)
- `is_process_alive` test uses actual current process PID, not a hardcoded value
- Signal mapping tests verify specific known POSIX signal numbers, not just "returns something"
- PID file tests use real filesystem operations with temp directories

## Outcomes

- Created `src/lifecycle/` module tree with three submodules: `signal.rs`, `check.rs`, `daemon.rs`
- `signal.rs`: 25 POSIX signals bidirectional mapping via single `SIGNALS` const array (3 functions)
- `check.rs`: `is_process_alive` via `libc::kill(pid, 0)` with correct ESRCH/EPERM handling, `wait_for_exit` with 50ms polling, `is_pidfile_alive` combining file read + alive check (3 functions)
- `daemon.rs`: double-fork daemonization with all 5 wplan pitfalls addressed (flock singleton, lock-before-truncate, fd redirect not close, inherited fd cleanup), `DaemonizeOptions` with `#[derive(Former)]`, PID file utilities (5 functions + 1 struct, Unix-only)
- 16 automated tests covering T01-T12 matrix + 45 additional corner case tests (total 102 nextest + 17 doc)
- 8 manual test scenarios documented for daemon and process check verification
- `unsafe-code` override via per-function `#[allow(unsafe_code)]` (Cargo.toml `[lints.rust]` override incompatible with `workspace = true`)
- Level 3 verification: 102 nextest + 17 doc tests + clippy clean (0 errors)
- `/test_manual` completed 2026-04-17: 45 corner cases added, 1 doc issue fixed (exit_status range pitfall)
- `/test_clean` completed 2026-04-17: all 3 impacted crates verified clean (process_tools, test_tools, willbe)

## Source Analysis

### Process Alive Check

**Current Location**: `wplan/src/daemon_routines.rs:99-107`

```rust
pub fn is_process_alive( pid : i32 ) -> bool
{
  unsafe
  {
    // kill( pid, 0 ) doesn't send a signal, just checks if process exists
    libc::kill( pid, 0 ) == 0
  }
}
```

**Why This Works**:
- `kill(pid, 0)` is a Unix idiom for "check if process exists"
- Returns 0 if process alive and we have permission
- Returns -1 with `ESRCH` if process doesn't exist
- Returns -1 with `EPERM` if process exists but we lack permission

**Use Cases**: Daemon management, test teardown, PID file validation, service health checks

### Signal Name Mapping

**Current Location**: `wplan_client/src/cli/formatting.rs:981-1011`

```rust
pub fn signal_name( signal : i32 ) -> &'static str
{
  match signal
  {
    1 => "SIGHUP",
    2 => "SIGINT",
    3 => "SIGQUIT",
    6 => "SIGABRT",
    9 => "SIGKILL",
    14 => "SIGALRM",
    15 => "SIGTERM",
    _ => "UNKNOWN",
  }
}
```

**Use Cases**: Displaying process exit status, log messages about killed processes, error reporting, test output

### Daemon Management (Fork/Setsid)

**Current Location**: `wplan/src/daemon_routines.rs:150-200`

**Functionality**: Fork to background, call `setsid()` for new session, redirect stdio to /dev/null, write PID file

**Use Cases**: Server/daemon applications, background task runners, service management

## Platform Considerations

### Unix
- `kill(pid, 0)` for process checks
- Standard POSIX signals (full mapping)
- Full daemonization support (fork + setsid + chdir + stdio redirect)

### Windows
- `OpenProcess` + `CloseHandle` for process checks
- Limited signal support (SIGINT, SIGTERM via Ctrl+C/Break)
- No daemonization (Windows Services API is a different paradigm)

### Cross-Platform Strategy
- Use `#[cfg(unix)]` and `#[cfg(windows)]` inside function bodies for check/signal modules
- Gate entire daemon module with `#[cfg(unix)]`
- Document platform limitations clearly in function-level docs

## Dependencies

```toml
# process_tools/Cargo.toml additions
[dependencies]
libc = { workspace = true }

[target.'cfg(windows)'.dependencies]
windows = { workspace = true, features = ["Win32_System_Threading"] }
```

## Cross-References

- **Follow-up adoption** (CRITICAL — extraction without migration leaves duplication):
  - `wplan_client/task/006_adopt_process_utilities_from_process_tools.md`
  - `wplan/task/086_adopt_process_utilities_from_process_tools.md`
- **Source files**:
  - `/home/user1/pro/lib/willbe/module/wplan/src/daemon_routines.rs:99-107` (is_process_alive)
  - `/home/user1/pro/lib/willbe/module/wplan/src/daemon_routines.rs:150-200` (daemonize)
  - `/home/user1/pro/lib/willbe/module/wplan_client/src/cli/formatting.rs:981-1011` (signal_name)
- **Related consumers**: wtest (test harness), benchkit (benchmark isolation), willbe (background builds)

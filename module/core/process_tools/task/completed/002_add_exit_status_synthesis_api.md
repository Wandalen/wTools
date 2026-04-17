# Add platform-agnostic ExitStatus synthesis API

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Done)
- **Validated By:** Level 3 verification (nextest + doc tests + clippy)
- **Validation Date:** 2026-04-17

## Goal

Four sites across the willbe workspace independently duplicate `ExitStatus::from_raw` with platform-specific cfg-gating — Unix uses POSIX waitpid encoding (`code << 8`), Windows uses direct code (`code as u32`). The `process_tools` crate already provides process execution utilities but lacks a platform-agnostic way to synthesize `ExitStatus` from an integer exit code (Motivated: duplicated platform knowledge is a maintenance hazard — a fix in one site is missed in others; Observable: `process_tools::exit_status::synthetic_exit_status(code)` exists and returns correct `ExitStatus` on both platforms; Scoped: add one new module `exit_status` to `process_tools/src/`, with tests in `process_tools/tests/`; Testable: `synthetic_exit_status(0).success() == true`, `synthetic_exit_status(1).code() == Some(1)` on both Unix and Windows).

## In Scope

- New `exit_status` module in `process_tools/src/exit_status.rs`
- `synthetic_exit_status(code: i32) -> ExitStatus` — hides Unix (`from_raw(code << 8)`) vs Windows (`from_raw(code as u32)`)
- `synthetic_success_status() -> ExitStatus` convenience (equivalent to `synthetic_exit_status(0)`)
- `synthetic_failure_status() -> ExitStatus` convenience (equivalent to `synthetic_exit_status(1)`)
- Register module via `layer exit_status;` in `process_tools/src/lib.rs` mod_interface block
- Tests in `process_tools/tests/exit_status_test.rs`

## Out of Scope

- Consumer migration (wrun_core, will_test, wflow) — covered by wrun_core task 401
- Adding `process_tools` as cross-workspace dependency in willbe — covered by wrun_core task 401
- Modifying existing `process` or `environment` modules
- Signal handling or PTY spawning extraction
- Async process management

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Module follows existing `process_tools` patterns: `mod_interface` with `layer` keyword, `mod private {}` block
- Custom codestyle per `code_style.rulebook.md` — 2-space indents, no `cargo fmt`
- Tests in `process_tools/tests/` directory — no `#[cfg(test)]` in src
- No mocking — test real `ExitStatus` behavior

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note code style, mod_interface patterns, test organization constraints
2. **Create exit_status module** — `process_tools/src/exit_status.rs` with `mod private {}` containing the API; use `#[cfg(unix)]` and `#[cfg(windows)]` inside `synthetic_exit_status()` body only
3. **Register module** — add `layer exit_status;` to the `mod_interface!` block in `process_tools/src/lib.rs`
4. **Write tests** — create `process_tools/tests/exit_status_test.rs` covering T01–T05 from Test Matrix
5. **Verify** — `cargo test -p process_tools --all-features` passes in wtools workspace
6. **Walk Validation Checklist** — verify every item answers YES

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | `synthetic_exit_status(0)` | Unix and Windows | `.success() == true` and `.code() == Some(0)` |
| T02 | `synthetic_exit_status(1)` | Unix and Windows | `.success() == false` and `.code() == Some(1)` |
| T03 | `synthetic_exit_status(42)` | Unix and Windows | `.code() == Some(42)` |
| T04 | `synthetic_success_status()` | Unix and Windows | Equivalent to `synthetic_exit_status(0)` |
| T05 | `synthetic_failure_status()` | Unix and Windows | Equivalent to `synthetic_exit_status(1)` |

## Acceptance Criteria

- `process_tools` crate exports `exit_status` module with `synthetic_exit_status`, `synthetic_success_status`, `synthetic_failure_status`
- `synthetic_exit_status(0).success() == true` on current platform
- `synthetic_exit_status(1).success() == false` and `.code() == Some(1)` on current platform
- `cargo test -p process_tools --all-features` passes with zero failures

## Validation

### Checklist

Desired answer for every question is YES.

**API Surface**
- [x] C1 — Does `process_tools/src/exit_status.rs` exist with `mod private {}` block?
- [x] C2 — Does `synthetic_exit_status(code: i32) -> ExitStatus` exist and use cfg-gated internals?
- [x] C3 — Do `synthetic_success_status()` and `synthetic_failure_status()` convenience functions exist?
- [x] C4 — Is the module registered as `layer exit_status;` in `lib.rs` mod_interface block?

**Tests**
- [x] C5 — Does `process_tools/tests/exit_status_test.rs` exist?
- [x] C6 — Does it cover all 5 test matrix scenarios (T01–T05)?
- [x] C7 — Does `cargo test -p process_tools --all-features` pass with zero failures?

**Code Quality**
- [x] C8 — Does the module use 2-space indents (custom codestyle)?
- [x] C9 — Are there zero `#[cfg(test)]` blocks in `src/exit_status.rs`?
- [x] C10 — Is platform-specific code confined to `#[cfg]` blocks inside function bodies only?

### Measurements

- Test count: 13 nextest tests + 3 doc tests (exceeds minimum 5)
- Platform coverage: Unix fully tested including out-of-range wrapping behavior
- API surface: 1 module, 3 public functions

### Invariants

- `synthetic_exit_status(0).success() == true` on all platforms
- `synthetic_exit_status(1).success() == false` on all platforms
- Valid range 0–255: codes outside this range produce inconsistent `ExitStatus` semantics on Unix

### Anti-Faking

- Tests call real `ExitStatus` API — no mocking
- Out-of-range wrapping tests confirm actual POSIX behavior (256 wraps inconsistently)
- All tests use real platform encoding paths

## Outcomes

- Created `src/exit_status.rs` with `mod private {}` and 3 public functions
- `synthetic_exit_status(code)`: cfg-gated — Unix `from_raw(code << 8)` (POSIX waitpid encoding), Windows `from_raw(code as u32)` (direct)
- `synthetic_success_status()` and `synthetic_failure_status()` convenience wrappers
- All functions marked `#[must_use]` with doc examples
- `# Pitfalls` section added to `synthetic_exit_status` docs: documents valid range 0–255 and the broken invariant at code 256 (`code()==Some(0)` yet `success()==false`)
- 5 matrix tests + 8 corner case tests in `tests/exit_status_test.rs` (13 total nextest)
- 3 doc tests passing
- Corner cases discovered and documented during `/test_manual`:
  - `synthetic_exit_status(-1)` wraps to `code()==Some(255)` on Unix
  - `synthetic_exit_status(256)` produces `code()==Some(0)` yet `success()==false` (broken invariant)
  - `synthetic_exit_status(i32::MAX)` wraps to `code()==Some(255)` on Unix
  - `synthetic_exit_status(i32::MIN)` wraps to raw 0 — accidental success
- Level 3 verification: 13 nextest + 3 doc tests + clippy clean (0 errors)
- `/test_manual` completed 2026-04-17: 8 corner cases added, exit_status range pitfall documented in src
- `/test_clean` completed 2026-04-17: all 3 impacted crates verified clean (process_tools, test_tools, willbe)

## Cross-References

- **Upstream consumer:** wrun_core task 401 (`willbe/will_test/module/wrun_core/task/backlog/401_extract_platform_abstractions_to_process_tools.md`) — depends on this task completing first
- **Duplication sites** (to be migrated by task 401):
  - `wrun_core/src/orchestration/process.rs:405-422`
  - `will_test/src/commands/command_builder.rs:248-257`
  - `wflow/tests/clipboard_test_utils.rs:108-111`
  - `wrun_core/tests/pty_exit_status_fix_test.rs` (test helpers)

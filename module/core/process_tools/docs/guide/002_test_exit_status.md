# Guide: Test Exit Status Fixtures

### Scope

- **Purpose:** Show how to construct `ExitStatus` values in tests without spawning real processes.
- **Responsibility:** Documents the pattern for using `synthetic_exit_status()` and its wrappers to simulate process success/failure outcomes in test code.
- **In Scope:** Using `synthetic_success_status()`, `synthetic_failure_status()`, and `synthetic_exit_status()` as test fixtures; valid code range; `#[must_use]` behavior.
- **Out of Scope:** Process spawning (→ `api/001`); PID-based monitoring (→ `api/005`); actual exit code propagation from real processes.

### Abstract

`std::process::ExitStatus` has no public constructor. Code that works with `ExitStatus` values — for example, test assertions against exit code semantics — normally requires spawning a real process. `synthetic_exit_status()` and its convenience wrappers provide direct construction without spawning, enabling isolated unit tests.

### Pattern

```rust
use process_tools::exit_status::{
  synthetic_exit_status,
  synthetic_success_status,
  synthetic_failure_status,
};

// Convenience wrappers for the common cases
let ok = synthetic_success_status();
assert!( ok.success() );
assert_eq!( ok.code(), Some( 0 ) );

let fail = synthetic_failure_status();
assert!( !fail.success() );
assert_eq!( fail.code(), Some( 1 ) );

// General form for a specific exit code
let custom = synthetic_exit_status( 42 );
assert_eq!( custom.code(), Some( 42 ) );
assert!( !custom.success() );
```

### Notes

- **Valid range is 0–255.** On Unix, only the low 8 bits are preserved in the POSIX `waitpid` status word. `synthetic_exit_status(256)` produces `code() == Some(0)` yet `success() == false` — inconsistent semantics. Validate input range before calling if the code comes from external input.
- **All three functions are `#[must_use]`.** Constructing an `ExitStatus` and not using it is a compile-time warning.
- **Platform encoding is hidden.** Unix uses `code << 8` (POSIX waitpid format); Windows uses `code as u32` directly. Callers pass the same `i32` on all platforms and get consistent `code()` and `success()` behavior.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/003_exit_status_api.md](../api/003_exit_status_api.md) | Full function signatures and platform encoding detail |
| doc | [feature/004_exit_status_synthesis.md](../feature/004_exit_status_synthesis.md) | Design rationale for synthetic status construction |

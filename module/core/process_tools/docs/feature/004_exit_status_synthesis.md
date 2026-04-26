# Feature: Exit Status Synthesis

### Scope

- **Purpose**: Allow test and simulation code to construct `ExitStatus` values without spawning a real process.
- **Responsibility**: Owns `synthetic_exit_status()`, `synthetic_success_status()`, and `synthetic_failure_status()` as the complete API for platform-agnostic status construction.
- **In Scope**: Platform encoding abstraction (Unix exit code shifted by 8 positions vs Windows direct), must-use enforcement, and valid code range documentation.
- **Out of Scope**: Actual process spawning (→ `feature/001`); PID-based process monitoring (→ `feature/005`).

### Status

- **Version introduced:** 0.30.0
- **Stability:** stable
- **Module path:** `process_tools::exit_status`
- **Pitfall:** valid range is 0–255; on Unix, codes outside this range produce an exit status with inconsistent semantics

### Design

`ExitStatus` has no public constructor. The only way to create one without spawning a process is through platform extension traits, each with a different encoding convention.

On Unix, POSIX `waitpid()` encodes the exit code in bits 8–15 of the raw status word (shifted by 8 positions). On Windows, the raw value is the exit code used directly. This difference is invisible to callers — they pass an integer code and receive a standard exit status.

The three functions follow a convenience hierarchy: `synthetic_success_status()` and `synthetic_failure_status()` delegate to `synthetic_exit_status(0)` and `synthetic_exit_status(1)` respectively. This eliminates magic numbers and documents intent at the call site.

All three functions are must-use to prevent callers from accidentally constructing an exit status they never use.

### Example

```rust
use process_tools::exit_status::{
  synthetic_exit_status,
  synthetic_success_status,
  synthetic_failure_status,
};

let ok = synthetic_success_status();
assert!( ok.success() );
assert_eq!( ok.code(), Some( 0 ) );

let fail = synthetic_failure_status();
assert!( !fail.success() );
assert_eq!( fail.code(), Some( 1 ) );

let custom = synthetic_exit_status( 42 );
assert_eq!( custom.code(), Some( 42 ) );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/exit_status.rs](../../src/exit_status.rs) | `synthetic_exit_status` and convenience wrapper implementations |
| test | [tests/exit_status_test.rs](../../tests/exit_status_test.rs) | Synthesis correctness and platform encoding tests |
| doc | [api/003_exit_status_api.md](../api/003_exit_status_api.md) | Full function signatures and parameter contracts |
| doc | [feature/005_lifecycle_management.md](005_lifecycle_management.md) | Lifecycle check results can be propagated as synthetic statuses |
| doc | [guide/002_test_exit_status.md](../guide/002_test_exit_status.md) | How to use these functions as test fixtures |
| task | [task/completed/002_add_exit_status_synthesis_api.md](../../task/completed/002_add_exit_status_synthesis_api.md) | Task that introduced the synthesis functions |

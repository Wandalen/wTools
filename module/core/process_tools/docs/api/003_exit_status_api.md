# API: Exit Status Synthesis

### Scope

- **Purpose**: Define the three free functions that construct `ExitStatus` values from integer codes without spawning a process.
- **Responsibility**: Documents function signatures, platform encoding details, valid code range, and `#[must_use]` enforcement.
- **In Scope**: `synthetic_exit_status()`, `synthetic_success_status()`, `synthetic_failure_status()`, and the Unix/Windows encoding difference.
- **Out of Scope**: Actual process exit handling; lifecycle process monitoring (→ `api/004`).

### Abstract

Three free functions in `process_tools::exit_status` construct a `std::process::ExitStatus` from an integer code without spawning a process. They hide the Unix/Windows platform encoding difference behind a single `i32` parameter. All three are `#[must_use]`.

### Operations

| Symbol | Kind | Notes |
|--------|------|-------|
| `synthetic_exit_status( code )` | free fn | Platform-encodes the code; valid range 0–255 |
| `synthetic_success_status()` | free fn | Equivalent to `synthetic_exit_status(0)` |
| `synthetic_failure_status()` | free fn | Equivalent to `synthetic_exit_status(1)` |

**Platform encoding detail:**

| Platform | Encoding | Raw value for code `N` |
|----------|----------|----------------------|
| Unix | POSIX `waitpid` status word | `N << 8` |
| Windows | Direct exit code | `N as u32` |

### Error Handling

None of the three functions return `Result`. They are infallible at the Rust type level. However, passing a code outside `0–255` on Unix produces an `ExitStatus` with inconsistent semantics:

- `synthetic_exit_status(256)` → `from_raw(256 << 8)` = `from_raw(0)` → `code() == Some(0)` yet `success() == false`

Callers must validate that codes are in range before calling if they intend to use `code()` or `success()` predictably.

### Compatibility Guarantees

- **Stability:** stable since 0.30.0. Function signatures will not change.
- **Platform:** compiles on all targets. Unix uses `std::os::unix::process::ExitStatusExt`; Windows uses `std::os::windows::process::ExitStatusExt`. On other platforms, compilation will fail (not currently gated for other targets).
- **Valid range:** 0–255 guaranteed correct. Out-of-range behavior is documented but not guaranteed to remain stable across Rust `std` changes.
- **`#[must_use]`:** all three functions. Unused `ExitStatus` values are a compile-time warning.

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
| doc | [feature/004_exit_status_synthesis.md](../feature/004_exit_status_synthesis.md) | Design rationale and platform encoding explanation |
| doc | [guide/002_test_exit_status.md](../guide/002_test_exit_status.md) | Practical guide for using these functions as test fixtures |

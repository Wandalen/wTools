# Invariant: EPERM Means Process Is Alive

### Scope

- **Purpose**: Guarantee that `check::is_process_alive()` never misclassifies a running process as dead due to a permission error.
- **Responsibility**: Enforces the correct interpretation of `EPERM` from `libc::kill(pid, 0)`: the process exists but the caller lacks permission to signal it.
- **In Scope**: `check::is_process_alive()` behavior when `errno == EPERM`; `check::wait_for_exit()` and `check::is_pidfile_alive()` by composition.
- **Out of Scope**: Processes that exit between the `kill()` call and the caller acting on the result (inherent PID reuse — not preventable at the application level).

### Invariant Statement

When `libc::kill(pid, 0)` returns `-1` with `errno == EPERM`, `check::is_process_alive()` returns `Ok(true)`. It does NOT return `Ok(false)` (dead) or `Err(PermissionDenied)` (error). `EPERM` from the null signal means the process exists in the kernel process table but the caller lacks permission to send real signals to it. The process is alive.

### Enforcement Mechanism

Within `is_process_alive()`, the OS error code is matched against three named cases: `ESRCH` ("no such process") maps to dead, `EPERM` ("operation not permitted") maps to alive, and any other error is propagated as an unexpected failure. `ESRCH` is the only path that means dead. `EPERM` is an explicit alive path, not a fallthrough.

Verification:

```bash
grep -n "EPERM" src/lifecycle/check.rs
# Must show: Some( libc::EPERM ) => Ok( true )
# Must NOT show EPERM mapped to Ok(false) or Err
```

### Violation Consequences

If `EPERM` were mapped to `Ok(false)`, `is_process_alive()` would incorrectly report root-owned or differently-privileged processes as dead. `wait_for_exit()` would return `Ok(())` immediately for any process the caller cannot signal — a silent false positive. `is_pidfile_alive()` would report any privileged daemon as dead. This bug class only manifests in production environments where the monitoring process has lower privilege than the monitored process, making it easy to miss in developer testing.

### Example

```rust
# #[ cfg( unix ) ]
# {
use process_tools::lifecycle::check;

// PID 1 (init/systemd) is always running.
// Non-root callers get EPERM — which is correctly reported as alive.
let result = check::is_process_alive( 1 );
assert!( result.unwrap_or( false ) );
# }
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/lifecycle/check.rs](../../src/lifecycle/check.rs) | `EPERM` match arm in `is_process_alive()` |
| doc | [api/005_check_api.md](../api/005_check_api.md) | `is_process_alive()` function contract |
| doc | [feature/005_lifecycle_management.md](../feature/005_lifecycle_management.md) | Rationale for `kill(pid,0)` probe approach |

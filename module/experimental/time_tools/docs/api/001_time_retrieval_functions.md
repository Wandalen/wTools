# API: Time Retrieval Functions

### Scope

- **Purpose**: Define the public contract for all time retrieval entry points.
- **Responsibility**: Document operations, error behavior, and compatibility for now, s::now, ms::now, and ns::now.
- **In Scope**: Function signatures, return semantics, panic conditions, thread safety.
- **Out of Scope**: Internal duration_since_epoch helper (private), feature flag wiring (see feature/001).

### Abstract

Four functions expose current wall-clock time as signed 64-bit integer milliseconds from UNIX epoch. Each resides in a sub-module named for its unit. The root-level now is an alias for milliseconds. All functions are zero-allocation, thread-safe, and panic only on misconfigured system clocks.

### Operations

- **now**: Return current time in milliseconds since UNIX epoch. Default entry point.
- **s::now**: Return current time in seconds since UNIX epoch. Coarse-grained timing.
- **ms::now**: Return current time in milliseconds since UNIX epoch. Identical to now.
- **ns::now**: Return current time in nanoseconds since UNIX epoch. High-precision; subject to signed 64-bit integer overflow around year 2262.

All operations require the standard library and are disabled under the `no_std` feature flag.

### Error Handling

All four functions panic if the system clock is set before UNIX epoch (1970-01-01 00:00:00 UTC). This indicates a misconfigured system clock. No recoverable error path is provided — callers requiring graceful handling should query the system clock directly.

### Compatibility Guarantees

- Thread-safe: no shared mutable state.
- Zero heap allocations.
- Wall clock (not monotonic): values may jump backwards due to NTP adjustments.
- Typical call latency under 100 nanoseconds.
- Available on all platforms with standard library time support.

### Cross-References

| Type | File | Responsibility |
|--------|-----------------------------------|--------------------------------------------------|
| source | `src/now.rs` | Implementation of all four functions |
| test | `tests/time_tests.rs` | Conversion accuracy, monotonicity, rapid polling |
| test | `tests/smoke_test.rs` | API reachability smoke test |
| doc | [feature/001_time_now.md](../feature/001_time_now.md) | Feature hub for time retrieval capability |
| doc | [invariant/001_epoch_base_timestamp.md](../invariant/001_epoch_base_timestamp.md) | Epoch base and overflow constraints |

# Feature: Time Now

### Scope

- **Purpose**: Provide current UNIX epoch timestamps in seconds, milliseconds, and nanoseconds.
- **Responsibility**: Document the time retrieval capability, its design, and all related artifacts.
- **In Scope**: Module-based API design, unit variants, feature gating, no_std behavior.
- **Out of Scope**: Chrono integration (deferred), duration formatting (deferred).

### Design

The crate exposes current time via a module-based API where each sub-module selects a time unit. The root function `now` defaults to milliseconds. Three explicit sub-modules (`s`, `ms`, `ns`) allow unit-specific access.

All functions return signed 64-bit integers (not unsigned) to allow signed arithmetic on timestamp differences without wrapping. A shared `duration_since_epoch` helper centralizes the system clock query and panic behavior.

Feature gating: the `time_now` feature activates the `now` module. The `enabled` feature is the master switch for the entire crate's public surface. Under `no_std`, all time functions are disabled because they depend on standard library time support.

### Cross-References

| Type | File | Responsibility |
|--------|-----------------------------------|--------------------------------------------------|
| source | `src/now.rs` | Core implementation of all time retrieval functions |
| source | `src/lib.rs` | Module wiring and namespace re-exports |
| test | `tests/time_tests.rs` | Unit conversion, monotonicity, edge case tests |
| test | `tests/smoke_test.rs` | Basic API reachability verification |
| config | `Cargo.toml` | Feature flag definitions and dependency wiring |
| doc | [api/001_time_retrieval_functions.md](../api/001_time_retrieval_functions.md) | API contracts for the four public functions |
| doc | [invariant/001_epoch_base_timestamp.md](../invariant/001_epoch_base_timestamp.md) | Epoch base and overflow invariants |

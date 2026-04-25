# API: Signal Name/Number Lookup

### Scope

- **Purpose**: Define the bidirectional POSIX signal lookup API so callers can translate between signal names and numbers without maintaining their own table.
- **Responsibility**: Documents `signal_name()`, `signal_number()`, and `all_signals()` signatures, lookup semantics, and Linux-specific scope limitation.
- **In Scope**: All three `signal` sub-module functions, return values for unknown inputs, and the Linux signal number scope.
- **Out of Scope**: Sending signals to processes (not provided by this crate); process liveness checking (→ `api/005`); daemonization (→ `api/006`).

### Abstract

Three free functions in `process_tools::lifecycle::signal` provide bidirectional lookup over a static Linux POSIX signal table. A single `const SIGNALS: &[(i32, &str)]` slice is the sole source of truth for both directions, preventing name-number drift. All three functions are `#[must_use]` and infallible.

### Operations

| Symbol | Kind | Signature | Notes |
|--------|------|-----------|-------|
| `signal_name()` | free fn | `(signal: i32) -> &'static str` | Returns `"UNKNOWN"` for unmapped numbers |
| `signal_number()` | free fn | `(name: &str) -> Option<i32>` | Case-sensitive; `None` for unrecognized names |
| `all_signals()` | free fn | `() -> &'static [(i32, &'static str)]` | Full table as `(number, name)` pairs |

### Error Handling

All three functions are infallible at the Rust type level:

- `signal_name()` returns `"UNKNOWN"` rather than `Err` for unmapped numbers.
- `signal_number()` returns `None` rather than `Err` for unrecognized names.
- `all_signals()` always returns a non-empty slice.

No panics, no allocations, no I/O.

### Compatibility Guarantees

- **Platform:** all targets — signal lookup is pure data, not gated on `#[cfg(unix)]`.
- **Signal table scope:** Linux signal numbers only. macOS/BSD differ for some user signals (e.g., `SIGUSR1` = 10 on Linux, 30 on macOS). The table is authoritative for Linux; use with care on macOS/BSD.
- **Stability:** stable since 0.30.0. Table contents and ordering will not change without a major version bump.
- **`#[must_use]`:** all three functions. Unused return values are a compile-time warning.

### Example

```rust
use process_tools::lifecycle::signal;

assert_eq!( signal::signal_name( 9 ),   "SIGKILL" );
assert_eq!( signal::signal_name( 15 ),  "SIGTERM" );
assert_eq!( signal::signal_name( 999 ), "UNKNOWN" );

assert_eq!( signal::signal_number( "SIGKILL" ),     Some( 9 ) );
assert_eq!( signal::signal_number( "NONEXISTENT" ), None );

let table = signal::all_signals();
assert!( table.len() >= 25 );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/lifecycle/signal.rs](../../src/lifecycle/signal.rs) | Signal table and bidirectional lookup implementation |
| doc | [feature/005_lifecycle_management.md](../feature/005_lifecycle_management.md) | Design rationale for the single-source-of-truth signal table |
| doc | [api/005_check_api.md](005_check_api.md) | PID liveness checking (uses signal concepts but not this table) |

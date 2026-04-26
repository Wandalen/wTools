# Invariant: Epoch Base Timestamp

### Scope

- **Purpose**: Guarantee that all timestamps use a consistent base and document the overflow boundary.
- **Responsibility**: Define the UNIX epoch invariant and signed 64-bit integer nanosecond overflow characteristics.
- **In Scope**: Epoch base, consistent unit conversion, signed 64-bit integer overflow at nanosecond precision.
- **Out of Scope**: Monotonicity guarantees (wall clock, not monotonic by design).

### Invariant Statement

All time retrieval functions return signed 64-bit integers representing elapsed time since UNIX epoch (1970-01-01 00:00:00 UTC). For any pair of unit functions called in close succession: ms / 1000 == s and ns / 1_000_000 == ms within measurement tolerance.

The signed 64-bit integer nanosecond representation overflows at approximately 2^63 nanoseconds from epoch, which corresponds to approximately year 2262. After overflow, nanosecond values wrap and produce incorrect timestamps.

### Enforcement Mechanism

The shared `duration_since_epoch` helper guarantees all functions derive from the same system clock query and epoch-relative computation. Unit conversion is performed using standard seconds, milliseconds, and nanoseconds extraction on the same duration value, ensuring mathematical consistency.

Tests verify cross-unit consistency with tolerance for execution time between successive calls (10ms for ms/ns, 1s for s/ms).

### Violation Consequences

If the epoch invariant is violated (different base or inconsistent conversion), all downstream consumers computing time differences, cache expiry, or rate limiting will produce incorrect results. The signed 64-bit integer overflow at nanosecond precision will silently produce negative or wrapped values after year 2262 — no panic, no warning.

### Cross-References

| Type | File | Responsibility |
|--------|-----------------------------------|--------------------------------------------------|
| source | `src/now.rs` | duration_since_epoch helper enforcing consistent base |
| test | `tests/time_tests.rs` | Cross-unit consistency and epoch boundary tests |
| doc | [api/001_time_retrieval_functions.md](../api/001_time_retrieval_functions.md) | API contracts referencing this invariant |
| doc | [feature/001_time_now.md](../feature/001_time_now.md) | Feature hub for time retrieval capability |

# Invariant: Send Bounds on Async Conversions

### Scope

- **Purpose**: Guarantee that async conversions are safe to execute across thread boundaries in multi-threaded async runtimes.
- **Responsibility**: States the thread-safety requirement and explains its enforcement in the AsyncInto and AsyncTryInto blanket implementations.
- **In Scope**: Thread-safety constraints on AsyncInto and AsyncTryInto blanket impls, compile-time enforcement, and violation behaviour.
- **Out of Scope**: Specific runtime configurations, single-thread workarounds, and async_trait future boxing internals.

### Invariant Statement

For all blanket implementations of AsyncInto and AsyncTryInto, both the source type and the target type must be thread-safe. This holds for every concrete instantiation of these blanket impls — no type that cannot safely cross thread boundaries may use them.

### Enforcement Mechanism

The blanket implementations carry compile-time constraints requiring that both the source and target types be thread-safe, alongside the conversion capability bound. The Rust type system enforces this statically at instantiation — any attempt to use a non-thread-safe type with these blanket impls fails at compile time with a trait bound error.

### Violation Consequences

Without thread-safety constraints, futures produced by async conversions could be sent across threads in a multi-threaded executor, causing a data race on non-thread-safe data. Because enforcement is static, there is no runtime failure mode — a violation is a compile error, never a panic or undefined behaviour.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Thread-safety where clauses on AsyncInto and AsyncTryInto blanket impls |
| test | `tests/send_bounds_validation_test.rs` | Multi-thread runtime tests exercising thread-safety constraints |
| doc | `docs/api/002_async_into.md` | AsyncInto blanket impl subject to this invariant |
| doc | `docs/api/004_async_try_into.md` | AsyncTryInto blanket impl subject to this invariant |

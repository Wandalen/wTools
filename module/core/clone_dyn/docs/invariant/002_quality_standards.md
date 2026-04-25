# Invariant: Quality Standards

### Scope

- **Purpose**: Define the minimum quality bar that all three crates in the clone_dyn ecosystem must maintain.
- **Responsibility**: Specify clippy compliance, test coverage, and documentation requirements.
- **In Scope**: Compile-time lint compliance, test feature combinations, public API documentation.
- **Out of Scope**: Performance benchmarks, fuzzing, MSRV policy (workspace-level concern).

### Invariant Statement

All three crates (`clone_dyn`, `clone_dyn_meta`, `clone_dyn_types`) must at all times:

1. Compile without warnings under `cargo clippy -- -D warnings`.
2. Have passing tests for all feature combinations listed in `tests/manual/readme.md § Feature Flag Combinations`.
3. Have all public API items documented with doc comments and examples.

### Enforcement Mechanism

- CI runs `RUSTFLAGS="-D warnings" cargo nextest run --all-features` and `cargo clippy --all-targets --all-features -- -D warnings` on every push.
- Manual test procedure verifies feature combinations: see `../../tests/manual/readme.md`.

### Violation Consequences

A lint warning or undocumented public item is a blocking PR failure. Test failures block merge.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../api/001_facade_api.md` | All public items subject to this invariant |

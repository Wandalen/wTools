# Invariant: Test Coverage

### Scope

- **Purpose**: Ensures adequate test coverage of core library logic.
- **Responsibility**: Documents the minimum coverage threshold and measurement method.
- **In Scope**: Line coverage for all non-trivial source code in `src/`.
- **Out of Scope**: Coverage of example binaries, benchmark code.

### Invariant Statement

Core library code must maintain a minimum of 80% line coverage, measured by `cargo tarpaulin` or equivalent coverage tool.

### Enforcement Mechanism

Run `cargo tarpaulin --all-features` and verify reported line coverage is ≥ 80%. Formal CI integration of coverage measurement is pending; currently verified on demand.

### Violation Consequences

Coverage below 80% indicates under-tested paths that may contain latent bugs. Template generation, error handling, and serialization paths are the highest-risk areas; they must be thoroughly covered.

### Tests

| File | Relationship |
|------|--------------|
| `tests/tests.rs` | Primary test suite that drives coverage measurement |

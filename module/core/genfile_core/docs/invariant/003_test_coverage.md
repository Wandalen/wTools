# Invariant: Test Coverage

### Scope

- **Purpose**: Ensures adequate test coverage of core library logic.
- **Responsibility**: Documents the minimum coverage threshold and measurement method.
- **In Scope**: Line coverage for all non-trivial source code in `src/`.
- **Out of Scope**: Coverage of example binaries, benchmark code.

### Invariant Statement

Core library code must maintain a minimum of 80% line coverage, measured by `cargo tarpaulin` or equivalent coverage tool.

### Enforcement Mechanism

Run `cargo tarpaulin --all-features` and verify reported line coverage is ≥ 80%. The crate currently has 188+ tests providing high coverage; formal measurement is pending. This invariant is enforced in CI on coverage-enabled runs.

### Violation Consequences

Coverage below 80% indicates under-tested paths that may contain latent bugs. Template generation, error handling, and serialization paths are the highest-risk areas; they must be thoroughly covered.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| test | `tests/` | Primary test suite that drives coverage |

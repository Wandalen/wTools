# Invariant: Child Index Determinism

### Scope

- **Purpose**: Guarantee that the sequence of values produced by a child generator is fully determined by its batch ID and the parent seed, regardless of when or on which thread the child is created.
- **Responsibility**: Define the child identity contract and the conditions under which it holds.
- **In Scope**: Child spawning by batch ID, independence of spawn timing from child output, deterministic mode only.
- **Out of Scope**: Non-deterministic mode behavior, the seed format (→ `invariant/002`), API signatures (→ `api/001`).

### Invariant Statement

Given a parent generator initialized with a fixed seed, spawning a child with a given batch ID always yields a child whose output sequence is identical, regardless of:

- The order in which children are spawned
- The timing of spawning relative to parent draws
- The thread that performs the spawning

This invariant holds only when compiled with the deterministic backend.

### Enforcement Mechanism

The parent maintains a dedicated internal offspring generator that is advanced only by child spawning operations. This generator is never advanced by value draws on the parent, so the seed of any child is unaffected by how many values the parent has produced.

The child seed is computed as a deterministic function of the offspring generator's current state and the batch ID. Because the offspring generator's state at any spawn call depends only on the number of prior spawns, and because each spawn consumes exactly one step, the child seed is a function of (initial seed, number of prior spawns, batch ID) — fully reproducible.

### Violation Consequences

If this invariant is broken:

- Parallel simulations produce different results when run with different thread schedules.
- Reproducibility claims cannot be verified: re-running with the same seed gives different output.
- Tests that execute only in deterministic mode become fragile and may fail non-deterministically.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_hierarchical_rng.md](../feature/001_hierarchical_rng.md) | Feature whose parent-child model depends on this invariant |
| doc | [api/001_public_api.md](../api/001_public_api.md) | Public interface that carries the child spawning contract |
| source | [src/hrng_deterministic.rs](../../src/hrng_deterministic.rs) | Deterministic backend; enforces offspring generator separation |
| test | [tests/basic_test.rs](../../tests/basic_test.rs) | Monte Carlo tests that would fail if this invariant breaks |

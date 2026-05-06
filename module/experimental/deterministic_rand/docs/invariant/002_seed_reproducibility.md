# Invariant: Seed Reproducibility

### Scope

- **Purpose**: Guarantee that a master generator initialized with the same seed always produces the same sequence of values across process restarts and library versions within a stable series.
- **Responsibility**: Define the seed-to-output reproducibility contract and the stability boundaries.
- **In Scope**: Master seed format, seed-to-sequence mapping, stability across process restarts, deterministic mode only.
- **Out of Scope**: Non-deterministic mode behavior, child spawning identity (→ `invariant/001`), API signatures (→ `api/001`).

### Invariant Statement

A master generator initialized with seed S always produces the same sequence of values V₁, V₂, ... Vₙ when:

- The deterministic backend is active
- The same sequence of operations is performed (same draws and same child spawns in the same order)
- The crate version is within the same stable series

This invariant enables test assertions that hard-code expected values, and enables simulation checkpointing by seed alone.

### Enforcement Mechanism

The deterministic backend uses a stream cipher whose output is fully specified by its key and nonce. The master seed is mapped to the cipher key in a stable, documented way. No platform-dependent entropy (system time, memory addresses, thread IDs) enters the cipher initialization path.

Text seeds are converted to byte sequences via a stable hash. Byte sequence seeds are used directly. The mapping from seed value to cipher state is part of the public contract and does not change across patch versions.

### Violation Consequences

If this invariant is broken:

- Recorded expected values in tests become invalid and must be regenerated.
- Simulation checkpoints saved as seeds cannot be replayed after a library update.
- Users who depend on seed-based reproducibility for auditing or debugging lose that capability without notice.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_hierarchical_rng.md](../feature/001_hierarchical_rng.md) | Feature whose reproducibility guarantee this invariant governs |
| doc | [api/001_public_api.md](../api/001_public_api.md) | Public interface that carries the seed reproducibility contract |
| source | [src/seed.rs](../../src/seed.rs) | Master seed type and seed-to-bytes conversion |
| source | [src/hrng_deterministic.rs](../../src/hrng_deterministic.rs) | Deterministic backend; maps seed to cipher state |
| test | [tests/basic_test.rs](../../tests/basic_test.rs) | Tests with hard-coded expected values that validate reproducibility |

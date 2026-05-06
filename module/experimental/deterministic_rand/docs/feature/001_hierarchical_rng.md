# Feature: Hierarchical RNG

### Scope

- **Purpose**: Provide a tree-structured random number generator where a parent spawns deterministic children by batch ID.
- **Responsibility**: Define the hierarchical parent-child generator model and its determinism guarantees for parallel workloads.
- **In Scope**: Parent generator, child spawning by batch ID, independent sequences per child, parallel-safe access via shared references.
- **Out of Scope**: The mechanism for switching backends (→ `feature/002`), iterator ordering (→ `feature/003`), concrete API signatures (→ `api/001`).

### Design

A hierarchical random number generator maintains two internal sequences: one for generating values on behalf of callers, and a separate sequence dedicated exclusively to producing offspring. This separation ensures that spawning a child at any point in time does not perturb the parent's value sequence.

Children are identified by a batch ID rather than a thread ID. The generator for a given batch ID is derived solely from the batch ID and the parent's offspring sequence, making the child sequence independent of scheduling order. Any thread may process the batch and will produce identical values.

The shared-reference model wraps each generator in an exclusively-lockable shared container, allowing multiple owners to hold a reference and a single owner to lock and use it at a time. This design avoids duplicating the generator state and keeps concurrent access safe without requiring external synchronization.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [invariant/001_child_index_determinism.md](../invariant/001_child_index_determinism.md) | Child sequence is determined solely by batch ID |
| doc | [invariant/002_seed_reproducibility.md](../invariant/002_seed_reproducibility.md) | Master seed fully reproduces generator output |
| doc | [api/001_public_api.md](../api/001_public_api.md) | Public surface for the hierarchical generator, shared reference, and seed |
| source | [src/hrng_deterministic.rs](../../src/hrng_deterministic.rs) | Deterministic backend implementation |
| source | [src/hrng_non_deterministic.rs](../../src/hrng_non_deterministic.rs) | Non-deterministic backend implementation |
| test | [tests/basic_test.rs](../../tests/basic_test.rs) | Monte Carlo tests proving determinism under parallelism |

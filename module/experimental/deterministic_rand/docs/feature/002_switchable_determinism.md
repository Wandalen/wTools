# Feature: Switchable Determinism

### Scope

- **Purpose**: Allow callers to select deterministic or non-deterministic RNG backends at compile time via a compile-time feature flag.
- **Responsibility**: Define the mode-switching mechanism and the behavioral difference between the two backends.
- **In Scope**: The compile-time determinism feature flag, backend selection, behavioral contract in each mode.
- **Out of Scope**: The hierarchical generator model itself (→ `feature/001`), iterator ordering (→ `feature/003`), API signatures (→ `api/001`).

### Design

The compile-time determinism feature flag acts as a compile-time selector. When the flag is active, the hierarchical generator uses a stream cipher-based backend whose output sequence is fully determined by the initial seed. When the flag is inactive, the generator uses a thread-local backend that draws entropy from the operating system and produces non-reproducible output.

The external interface of the generator does not change between modes. Callers interact with the same types and methods regardless of which backend is compiled in. This means test suites can assert exact expected values when the determinism feature is enabled, and the same test suite structure runs without assertions when the feature is disabled.

The non-deterministic backend has lower latency per draw because it avoids the cipher state management required by the deterministic backend. Production deployments that do not require reproducibility can omit the feature to reduce per-call overhead.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_hierarchical_rng.md](001_hierarchical_rng.md) | The hierarchical generator this feature controls |
| doc | [invariant/001_child_index_determinism.md](../invariant/001_child_index_determinism.md) | Determinism contract enforced when this feature is active |
| doc | [api/001_public_api.md](../api/001_public_api.md) | Public interface that is identical in both modes |
| source | [src/lib.rs](../../src/lib.rs) | Feature-conditional backend selection and exports |
| test | [tests/basic_test.rs](../../tests/basic_test.rs) | End-to-end tests that exercise both deterministic and non-deterministic backends |

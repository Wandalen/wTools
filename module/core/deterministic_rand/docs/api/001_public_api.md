# API: Public API

### Scope

- **Purpose**: Document the complete public interface of the deterministic_rand crate.
- **Responsibility**: Specify all exported types, their operations, error conditions, and compatibility guarantees.
- **In Scope**: Hierarchical generator, shared generator reference, master seed, deterministic iterator extension.
- **Out of Scope**: Implementation internals, feature design rationale (→ `feature/`), correctness proofs (→ `invariant/`).

### Abstract

The crate exports four public abstractions: a hierarchical generator that spawns child generators by ID, a shared reference type for safe concurrent access to a generator, a master seed type for initializing reproducible generator trees, and an iterator extension trait for conditional deterministic sorting.

The public interface is identical in both deterministic and non-deterministic compile modes. Callers do not branch on the active mode; behavioral differences are expressed through conditional assertions in test code.

### Operations

**Hierarchical Generator**

- Create a generator without a seed, drawing entropy from the environment.
- Create a generator with an explicit master seed, producing a reproducible sequence.
- Spawn a child generator identified by a numeric batch ID. The child's sequence is independent of the parent's draw sequence and depends only on the batch ID and the parent seed.
- Obtain a shared reference to the generator for use in concurrent contexts.

**Shared Generator Reference**

- Lock the shared reference to obtain exclusive access for a series of draws.
- The reference is cloneable; multiple owners may hold a reference and take turns locking.

**Master Seed**

- Construct a seed from a text string.
- Construct a seed from a byte sequence.
- The seed type is the sole entry point for deterministic initialization; passing a seed to the generator constructor activates the reproducibility contract.

**Deterministic Iterator Extension**

- Sort all elements of an iterator by natural order. This is a no-op in non-deterministic mode.
- Sort all elements of an iterator by a caller-provided comparator. This is a no-op in non-deterministic mode.
- The sorted iterator is lazy in the same way as standard iterator adapters; elements are materialized when consumed.

### Error Handling

- Locking the shared generator reference may fail if the lock is poisoned (a thread panicked while holding it). The caller receives a standard lock-poisoning error and must decide whether to recover or propagate.
- No other public operations produce errors under normal usage. Construction and child spawning are infallible.

### Compatibility Guarantees

- The type signatures of all public items are stable within a minor version series.
- When the `determinism` feature is active, the numeric output sequence for a given seed is stable within a patch version series. A minor version bump may change the sequence and will be documented in the changelog.
- When the `determinism` feature is inactive, no output stability guarantees apply.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_hierarchical_rng.md](../feature/001_hierarchical_rng.md) | Design rationale for the hierarchical generator model |
| doc | [invariant/001_child_index_determinism.md](../invariant/001_child_index_determinism.md) | Correctness contract for child spawning |
| doc | [invariant/002_seed_reproducibility.md](../invariant/002_seed_reproducibility.md) | Correctness contract for seed-to-sequence mapping |
| source | [src/lib.rs](../../src/lib.rs) | Crate entry point and re-exports |
| source | [src/hrng_deterministic.rs](../../src/hrng_deterministic.rs) | Deterministic generator implementation |
| source | [src/hrng_non_deterministic.rs](../../src/hrng_non_deterministic.rs) | Non-deterministic generator implementation |
| source | [src/seed.rs](../../src/seed.rs) | Master seed type |
| source | [src/iter.rs](../../src/iter.rs) | Deterministic iterator extension |
| test | [tests/basic_test.rs](../../tests/basic_test.rs) | End-to-end API usage under parallelism |

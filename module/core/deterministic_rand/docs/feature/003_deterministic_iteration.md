# Feature: Deterministic Iteration

### Scope

- **Purpose**: Provide an iterator extension that sorts elements only when the determinism feature is active, eliminating HashMap/HashSet ordering as a source of non-determinism.
- **Responsibility**: Define the conditional-sort extension trait and its no-op contract in non-deterministic mode.
- **In Scope**: The extension trait, conditional sort behavior, HashMap/HashSet key ordering use case.
- **Out of Scope**: The backend selection mechanism (→ `feature/002`), the hierarchical generator model (→ `feature/001`), API signatures (→ `api/001`).

### Design

Collections such as hash maps and hash sets iterate their elements in unspecified order. When a simulation must reproduce a sequence of decisions, any step that processes these elements in an unspecified order introduces non-determinism even if the random number generator itself is seeded consistently.

The iterator extension provides two methods for conditional sorting. Both methods accept the same arguments as a standard sort operation. When the determinism feature is active, the methods perform a full sort before the caller sees any elements. When the feature is inactive, the methods pass all elements through without sorting and without allocating any intermediate storage.

This design lets callers write a single code path that is correct in both modes. The sorting call is present in the source at all times; the compiler eliminates it entirely in non-deterministic builds. No branching or runtime flag is involved.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/002_switchable_determinism.md](002_switchable_determinism.md) | Controls whether sorting is active or a no-op |
| source | [src/iter.rs](../../src/iter.rs) | Extension trait implementation |
| test | [tests/assumption_test.rs](../../tests/assumption_test.rs) | Validates iterator extension behavior and assumptions |

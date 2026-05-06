# Feature Doc Entity

### Scope

- **Purpose**: Document the behavioral capabilities of the deterministic_rand crate.
- **Responsibility**: One instance per distinct user-facing capability; no API signatures or output examples.
- **In Scope**: Feature statement, design rationale, scope boundaries, cross-references to related entities.
- **Out of Scope**: API signatures (→ `api/`), correctness contracts (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Hierarchical RNG](001_hierarchical_rng.md) | Parent generator spawns deterministic children by batch ID | ✅ |
| 002 | [Switchable Determinism](002_switchable_determinism.md) | Compile-time mode switch between deterministic and non-deterministic backends | ✅ |
| 003 | [Deterministic Iteration](003_deterministic_iteration.md) | Iterator extension that sorts only when determinism feature is active | ✅ |

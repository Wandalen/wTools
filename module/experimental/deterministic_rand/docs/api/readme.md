# API Doc Entity

### Scope

- **Purpose**: Document the public interface of the deterministic_rand crate.
- **Responsibility**: One instance per coherent API surface group; no feature rationale or invariant proofs.
- **In Scope**: Public types, operations, error handling, compatibility guarantees.
- **Out of Scope**: Feature design rationale (→ `feature/`), correctness contracts (→ `invariant/`), usage examples (→ `readme.md`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Public API](001_public_api.md) | Complete public surface: hierarchical generator, shared reference, seed, and iterator extension | ✅ |

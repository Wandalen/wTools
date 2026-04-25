# Invariant Doc Entity

### Scope

- **Purpose**: Document correctness properties that must hold at all times in deterministic mode.
- **Responsibility**: One instance per distinct invariant; no API signatures or feature descriptions.
- **In Scope**: Invariant statement, enforcement mechanism, violation consequences, cross-references.
- **Out of Scope**: Feature descriptions (→ `feature/`), API signatures (→ `api/`), implementation details.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Child Index Determinism](001_child_index_determinism.md) | Child generator sequence is determined solely by batch ID | ✅ |
| 002 | [Seed Reproducibility](002_seed_reproducibility.md) | Identical seed always reproduces identical generator output | ✅ |

# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | One instance per coherent API surface group; no feature rationale or invariant proofs. | [api/readme.md](api/readme.md) | 1 |
| `feature/` | One instance per distinct user-facing capability; no API signatures or output examples. | [feature/readme.md](feature/readme.md) | 3 |
| `invariant/` | One instance per distinct invariant; no API signatures or feature descriptions. | [invariant/readme.md](invariant/readme.md) | 2 |

## Master Doc Instances Table

| Entity | ID | Name | File |
|--------|-----|------|------|
| api | 001 | Public API | [api/001_public_api.md](api/001_public_api.md) |
| feature | 001 | Hierarchical RNG | [feature/001_hierarchical_rng.md](feature/001_hierarchical_rng.md) |
| feature | 002 | Switchable Determinism | [feature/002_switchable_determinism.md](feature/002_switchable_determinism.md) |
| feature | 003 | Deterministic Iteration | [feature/003_deterministic_iteration.md](feature/003_deterministic_iteration.md) |
| invariant | 001 | Child Index Determinism | [invariant/001_child_index_determinism.md](invariant/001_child_index_determinism.md) |
| invariant | 002 | Seed Reproducibility | [invariant/002_seed_reproducibility.md](invariant/002_seed_reproducibility.md) |

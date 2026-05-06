# config_hierarchy Docs

### Scope

- **What**: Behavioral specifications and design documentation for config_hierarchy
- **Who**: Developers implementing, extending, or reviewing crate behavior
- **When**: Before and during feature implementation; when auditing behavioral requirements
- **Out of scope**: Test specifications (→ tests/docs/), implementation code (→ src/)

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `algorithm/` | Detection and resolution algorithms |
| `api/` | Public trait and method contracts |
| `doc_graph.yml` | Cross-reference graph of all doc instances |
| `entities.md` | Cross-entity navigation index |
| `feature/` | Feature behavioral specifications |
| `format/` | Configuration file format specifications |
| `invariant/` | System invariants and constraints |
| `pattern/` | Reusable design patterns in the implementation |

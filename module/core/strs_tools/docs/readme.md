# docs

### Scope

- **Purpose**: Organize behavioral requirements, API contracts, correctness invariants, and algorithmic design documentation for the strs_tools crate.
- **Responsibility**: Root documentation directory linking to all doc entity directories.
- **In Scope**: Feature capabilities, public API contracts, correctness guarantees, algorithm documentation.
- **Out of Scope**: Source code, test code, build configuration.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `entities.md` | Cross-entity index of all doc entities and doc instances |
| `doc_graph.yml` | Machine-readable cross-reference graph of doc instances |
| `feature/` | Feature doc entity — user-facing capability navigation hubs |
| `api/` | API doc entity — public programmatic interface contracts |
| `invariant/` | Invariant doc entity — correctness properties and guarantees |
| `algorithm/` | Algorithm doc entity — internal algorithmic design |

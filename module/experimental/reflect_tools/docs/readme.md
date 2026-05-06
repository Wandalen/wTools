# Docs

### Scope

- **Purpose**: Provide structured design documentation for the `reflect_tools` crate.
- **Responsibility**: Contains all doc entity types for entity reflection and fields iteration subsystems.
- **In Scope**: Design decisions, API contracts, correctness invariants, data structure descriptions.
- **Out of Scope**: Source code comments (→ `src/`); test documentation (→ `tests/readme.md`); user onboarding (→ `readme.md`).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| [entities.md](entities.md) | Cross-entity index of all doc types and instances |
| [doc_graph.yml](doc_graph.yml) | Machine-readable cross-reference graph |
| [api/](api/readme.md) | Public interface contracts for reflection and fields APIs |
| [data_structure/](data_structure/readme.md) | In-memory data type descriptions |
| [feature/](feature/readme.md) | Navigational hubs for major subsystem features |
| [invariant/](invariant/readme.md) | Correctness properties and enforcement mechanisms |

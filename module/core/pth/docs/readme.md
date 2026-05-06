# Docs

### Scope

- **Purpose**: Host design and API documentation for the `pth` crate.
- **Responsibility**: Index doc entity collections covering invariants, public API, and feature navigation.
- **In Scope**: Correctness invariants, API contracts, feature navigation, cross-entity index, and doc graph.
- **Out of Scope**: Source code comments (see `src/`), user onboarding (see `readme.md`).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `api/` | Document public API — free functions, type wrappers, and conversion traits |
| `doc_graph.yml` | Cross-reference graph for doc instance navigation |
| `entities.md` | Index all active doc entity types and instances |
| `feature/` | Navigate feature artifacts across source, tests, and docs |
| `invariant/` | Document correctness invariants — always-hold properties |

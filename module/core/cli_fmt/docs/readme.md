# Docs

### Scope

**Responsibilities:**
Design and API documentation for `cli_fmt` — organized as typed doc entity instances: features describe what the crate does, invariants define behavioral contracts, and API docs reference the public interface.

**In Scope:**
- Feature, API, invariant, and pitfall doc entity collections
- Cross-entity navigation index (`entity.md`) and cross-reference graph (`doc_graph.yml`)

**Out of Scope:**
- Test specification documents (see `../tests/docs/`)
- Implementation code (see `../src/`)

### Responsibility Table

| File | Responsibility |
|------|----------------|
| [feature/](feature/readme.md) | Feature documentation — what the crate does and why |
| [api/](api/readme.md) | API reference — public types and processing interface |
| [invariant/](invariant/readme.md) | Invariant documentation — architectural boundary contracts |
| [pitfall/](pitfall/readme.md) | Pitfall documentation — confirmed design traps and mitigations |
| [entity.md](entity.md) | Cross-entity navigation index — all doc entities and instances |
| [doc_graph.yml](doc_graph.yml) | Cross-reference graph linking all doc instances |

# Docs

## Scope

Design and API documentation for `clone_dyn_types`. Organized as typed doc entity instances: features describe what the crate does, invariants define safety contracts, API docs reference the public interface, algorithm docs describe DST cloning logic, and pattern docs capture architectural decisions.

See [entities.md](entities.md) for the complete registry of all doc instances.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| [entities.md](entities.md) | Master registry of all doc entity types and instances |
| [doc_graph.yml](doc_graph.yml) | Cross-reference graph linking all doc instances |
| [feature/](feature/readme.md) | Feature documentation — what the crate does and why |
| [invariant/](invariant/readme.md) | Invariant documentation — safety contracts that must always hold |
| [api/](api/readme.md) | API reference — public traits and functions |
| [algorithm/](algorithm/readme.md) | Algorithm documentation — DST cloning implementation |
| [pattern/](pattern/readme.md) | Pattern documentation — architectural design patterns |

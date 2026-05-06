# Docs

### Scope

- **Purpose**: Provide structured design documentation for the `reflect_tools_meta` proc-macro crate.
- **Responsibility**: Contains doc entity types for the Reflect derive macro system.
- **In Scope**: Derive macro API contracts, feature scope, behavioral invariants.
- **Out of Scope**: Runtime reflection behavior (→ `reflect_tools/docs/`); source code details (→ `src/`).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| [entities.md](entities.md) | Master index of all doc entities and instances |
| [api/](api/readme.md) | Reflect derive proc-macro API contracts |
| [feature/](feature/readme.md) | Reflect derive feature scope and artifact navigation |
| [invariant/](invariant/readme.md) | Behavioral constraints on the derive macro |

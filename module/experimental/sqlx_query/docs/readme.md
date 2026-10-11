# Docs

### Scope

Design and API documentation for `sqlx_query`: what it decides for a build (`feature/`), how it is called (`api/`), and the properties those decisions rely on (`invariant/`). Start at `definition.md`.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `definition.md` | Index of doc definitions and their doc instances |
| `doc_graph.yml` | Typed-reference edges between doc instances |
| `api/` | Public operations, settings, errors and compatibility |
| `feature/` | Offline enforcement and opt-in automatic regeneration |
| `invariant/` | Hash parity, isolated regeneration, sqlx parity, rerun inputs |

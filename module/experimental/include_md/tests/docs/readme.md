# tests/docs

Test spec files for the `include_md` crate, organized by doc entity type.

## Scope

Covers all behavioral requirements, API contracts, and invariants defined under `docs/`.

| Directory | Responsibility |
|-----------|----------------|
| `feature/` | Feature spec files — FT- cases for file inclusion and section extraction |
| `invariant/` | Invariant spec files — IN- cases for path resolution, compile-time errors, size limit, extraction rules |
| `api/` | API spec files — AP- cases for include_md! and include_md_section! macro contracts |
| `readme.md` | Responsibility table for this directory |

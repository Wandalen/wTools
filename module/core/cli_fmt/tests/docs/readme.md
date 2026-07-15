# Test Surface Docs

Test specification documents for all doc entity instances in `cli_fmt`. Organized by doc entity type, mirroring the `docs/` structure per `test_surface.rulebook.md § Inventory : Surface Mapping`.

### Scope

**Responsibilities:**
Documents the test specification surface for cli_fmt — one spec file per doc entity instance in `docs/`, mapping documented capabilities to their test cases per `test_surface.rulebook.md`.

**In Scope:**
- Test spec documents organized by doc entity type (`feature/`, `invariant/`, `api/`, `pitfall/`)
- Spec-case-to-test-function mappings (FT-, IN-, AP-, PF- identifiers)

**Out of Scope:**
- Test implementation code (see `../output.rs`, `../help.rs`)
- Doc entity source content itself (see `../../docs/`)

### Conventions

- **File naming:** Spec files use 3-digit NNN prefix (e.g. `001_foo.md`) to mirror the `docs/` NNN convention — workspace override of `test_surface.rulebook.md`'s 2-digit NN default.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Root index for tests/docs/ subdirectories |
| `feature/` | FT- test specs for docs/feature/ instances |
| `invariant/` | IN- test specs for docs/invariant/ instances |
| `api/` | AP- test specs for docs/api/ instances |
| `pitfall/` | PF- test specs for docs/pitfall/ instances |

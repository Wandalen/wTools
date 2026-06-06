# Test Surface Docs

Test specification documents for all doc entity instances in `cli_fmt`. Organized by doc entity type, mirroring the `docs/` structure per `test_surface.rulebook.md § Inventory : Surface Mapping`.

### Conventions

- **File naming:** Spec files use 3-digit NNN prefix (e.g. `001_foo.md`) to mirror the `docs/` NNN convention — workspace override of `test_surface.rulebook.md`'s 2-digit NN default.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Root index for tests/docs/ subdirectories |
| `feature/` | FT- test specs for docs/feature/ instances |
| `invariant/` | IN- test specs for docs/invariant/ instances |
| `api/` | AP- test specs for docs/api/ instances |

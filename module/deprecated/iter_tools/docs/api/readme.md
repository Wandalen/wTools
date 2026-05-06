# API Doc Entity

### Scope

- **Purpose**: Define the formal public contracts exposed by `iter_tools` to consuming crates.
- **Responsibility**: Document trait bounds, method signatures, error semantics, and compatibility guarantees.
- **In Scope**: Iterator trait hierarchy (`_IterTrait`, `IterTrait`, `BoxedIter`); extension method (`map_result`); re-exported function catalogue.
- **Out of Scope**: Internal implementation details, benchmark data, and architectural rationale — those live in `feature/` and `src/`.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `001_iter_traits.md` | Formal contract for the iterator trait hierarchy and `BoxedIter`. |
| `002_iter_ext.md` | Formal contract for the `IterExt::map_result` extension method. |
| `003_reexports.md` | Catalogue of all re-exported symbols grouped by purpose. |

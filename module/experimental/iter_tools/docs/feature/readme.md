# Feature Doc Entity

### Scope

- **Purpose**: Catalog user-facing iterator capabilities provided by `iter_tools`.
- **Responsibility**: Document scope, design rationale, and cross-references for each user-facing capability.
- **In Scope**: Iterator re-export facade, clonable boxed iterator system, iterator extension methods.
- **Out of Scope**: Implementation details and type signatures — those live in `api/` and `src/`.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `001_itertools_reexports.md` | Selective itertools facade for wTools ecosystem. |
| `002_clonable_boxed_iterators.md` | Enable Clone on boxed iterator trait objects. |
| `003_iter_ext.md` | Result-oriented iterator processing methods. |

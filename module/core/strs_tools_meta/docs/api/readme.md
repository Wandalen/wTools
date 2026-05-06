# API Doc Entity

## Scope

- **In Scope**: Public API doc instances for the two exported procedural macros.
- **Out of Scope**: Internal helpers and private functions in `src/lib.rs`.
- **Boundary**: API instances cover call interface and generated code contract; behavioral design is in `feature/`.
- **Status**: Active.

### Overview Table

| # | File | Responsibility |
|---|------|----------------|
| 1 | `001_optimize_split_api.md` | `optimize_split!` macro public interface |
| 2 | `002_optimize_match_api.md` | `optimize_match!` macro public interface |

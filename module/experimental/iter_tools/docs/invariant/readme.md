# Invariant Doc Entity

### Scope

- **Purpose**: Specify non-negotiable behavioral contracts that `iter_tools` must uphold across all configurations and versions.
- **Responsibility**: Document each invariant's statement, how it is enforced in the implementation, and the consequences of violation.
- **In Scope**: Clone guarantee for boxed iterators, module pattern constraint, `no_std` core compatibility requirement.
- **Out of Scope**: API contracts and feature design rationale — those live in `api/` and `feature/`.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `001_clone_contract.md` | Clone guarantee for all four `BoxedIter` marker combinations. |
| `002_module_pattern.md` | Prohibition on `mod_interface` dependency; manual namespace chain. |
| `003_no_std_compatibility.md` | Core iterator operations available in all feature configurations. |

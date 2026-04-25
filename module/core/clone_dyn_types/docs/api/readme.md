# API Doc Entity

### Scope

- **Purpose**: Document the public trait and function API of `clone_dyn_types`.
- **Responsibility**: Specify operation semantics, error handling, and compatibility guarantees.
- **In Scope**: `CloneDyn` trait, `clone_into_box` function, all public items exported by this crate.
- **Out of Scope**: Internal algorithm implementation (`algorithm/`), feature rationale (`feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [CloneDyn Trait](001_clone_dyn_trait.md) | Object-safe trait enabling type-erased cloning | ✅ |
| 002 | [clone_into_box and clone](002_clone_into_box.md) | Public functions for cloning DSTs and sized types | ✅ |

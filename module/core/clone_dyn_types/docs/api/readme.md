# API Doc Entity

### Scope

- **Purpose**: Document the public trait and function API of `clone_dyn_types`.
- **Responsibility**: Specify operation semantics, error handling, and compatibility guarantees.
- **In Scope**: `CloneDyn` trait, `clone_into_box` function, all public items exported by this crate.
- **Out of Scope**: Internal algorithm implementation (`algorithm/`), feature rationale (`feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | CloneDyn Trait | Object-safe trait enabling type-erased cloning | ✅ |
| 002 | Clone Into Box | Public function for cloning DSTs into Box | ✅ |

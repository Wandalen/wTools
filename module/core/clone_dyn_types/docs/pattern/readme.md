# Pattern Doc Entity

### Scope

- **Purpose**: Document architectural design patterns employed in `clone_dyn_types`.
- **Responsibility**: Explain the problem, solution, applicability, and consequences of each pattern.
- **In Scope**: Sealed trait pattern for `CloneDyn`.
- **Out of Scope**: Feature behavioral specs (`feature/`), invariants (`invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Sealed Trait](001_sealed_trait.md) | Prevent external CloneDyn implementations | ✅ |

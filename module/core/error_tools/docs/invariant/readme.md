# Invariant Doc Entity

### Scope

- **Purpose**: Define behavioral contracts that must always hold for `error_tools` regardless of code path.
- **Responsibility**: Master index for all invariant doc instances in this crate.
- **In Scope**: Instances covering one constraint each — exclusive dependency, zero-cost facade, alloc requirement.
- **Out of Scope**: Desired capabilities and behaviors — see `feature/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Exclusive Error Dependency](001_exclusive_dependency.md) | Sole error framework mandate for consumers | ✅ |
| 002 | [Zero-Cost Facade](002_zero_cost_facade.md) | Pure pass-through with no wrapper overhead | ✅ |
| 003 | [Alloc Feature Requires No-Std](003_alloc_requirement.md) | use_alloc depends on no_std invariant | ✅ |

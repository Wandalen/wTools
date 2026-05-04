# Invariant Doc Entity

### Scope

- **Purpose**: Document the behavioral contracts and constraints all collection macros must uphold.
- **Responsibility**: Registry and overview of all invariant doc instances.
- **In Scope**: No-std allocation selection; compile-time capacity pre-allocation.
- **Out of Scope**: API signatures (see `api/`), feature guides (see `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [No-std Allocation Selection](001_no_std_alloc.md) | HashMap/HashSet source switches between hashbrown and std based on feature flags | ✅ |
| 002 | [Capacity Pre-allocation](002_capacity_preallocated.md) | The 10 macros for capacity-supporting types pre-allocate exact capacity at compile time | ✅ |

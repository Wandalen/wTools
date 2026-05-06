# API Doc Entity

### Scope

- **Purpose**: Document the public API of `component_model_types` — the Assign trait system for type-safe component assignment.
- **Responsibility**: Collect one doc instance per significant API surface; each instance owns operations, usage semantics, and compatibility guarantees.
- **In Scope**: Public traits, their methods, blanket implementations, and semver stability.
- **Out of Scope**: Feature design rationale (→ `feature/`); behavioral constraints (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Assign Trait System](001_assign_trait.md) | Core generic assignment traits: Assign, OptionExt, AssignWithType, PopularType | ✅ |

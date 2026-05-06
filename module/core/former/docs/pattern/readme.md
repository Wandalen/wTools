# Pattern Doc Entity

### Scope

- **Purpose**: Documents the design patterns that the former macro implements or enables, capturing the problem each solves and the structural solution it provides.
- **Responsibility**: Master index for pattern doc instances — their identifiers, names, and status.
- **In Scope**: Reusable design solutions applied or made available by the macro.
- **Out of Scope**: Feature behavioral descriptions (→ feature/), attribute contracts (→ api/), algorithmic procedures (→ algorithm/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Builder Pattern](001_builder_pattern.md) | Separates object construction from representation via an intermediate builder | ✅ |
| 002 | [Subformer Composition](002_subformer_composition.md) | Hierarchical builder delegation for nested type construction | ✅ |

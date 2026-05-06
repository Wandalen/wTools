# Pattern Doc Entity

### Scope

- **Purpose**: Provide documentation of reusable design patterns employed in the implementation.
- **Responsibility**: One doc instance per identified pattern; captures motivation, structure, and applicability.
- **In Scope**: Patterns that inform the library's zero-cost abstraction and trait composition approach.
- **Out of Scope**: Trait API contracts (→ api/), behavioral requirements (→ feature/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Zero-Cost Composition](001_zero_cost_composition.md) | Phantom type parameter composition without runtime cost | ✅ |

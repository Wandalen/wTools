# Algorithm Doc Entity

### Scope

- **Purpose**: Documents the computational procedures used during macro expansion to determine builder generation strategies.
- **Responsibility**: Master index for algorithm doc instances — their identifiers, names, and status.
- **In Scope**: Decision algorithms and rule tables that govern macro behavior at expansion time.
- **Out of Scope**: Generated builder behavior at runtime (→ feature/), design patterns (→ pattern/), attribute contracts (→ api/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Variant Constructor Logic](001_variant_constructor_logic.md) | 14-rule decision table for enum variant constructor type selection | ✅ |

# API Doc Entity

### Scope

- **Purpose**: Documents the public attribute contracts of the former macro — the stable interface through which users configure builder generation behavior.
- **Responsibility**: Master index for API doc instances — their identifiers, names, and status.
- **In Scope**: All user-facing configuration attributes, their operations, error conditions, and compatibility guarantees.
- **Out of Scope**: Internal generated types and traits (→ algorithm/), feature behavioral descriptions (→ feature/), design patterns (→ pattern/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Item Attributes](001_item_attributes.md) | Attributes applied at the type level controlling global builder behavior | ✅ |
| 002 | [Field Attributes](002_field_attributes.md) | Attributes applied at the field or variant level controlling individual setter generation | ✅ |

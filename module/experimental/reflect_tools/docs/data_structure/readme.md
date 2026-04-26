# Data Structure Doc Entity

### Scope

- **Purpose**: Document in-memory data types with non-trivial design decisions in `reflect_tools`.
- **Responsibility**: Describe structure, layout choices, and operations for key data types.
- **In Scope**: OptionalCow wrapper design, Primitive enum variant catalog.
- **Out of Scope**: API usage patterns (→ `api/`); feature-level navigation (→ `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [OptionalCow](001_optional_cow.md) | Transparent wrapper for optional borrowed-or-owned field values | ✅ |
| 002 | [Primitive](002_primitive.md) | Discriminated union of 16 primitive value variants | ✅ |

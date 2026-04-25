# Invariant Doc Entity

### Scope

- **Purpose**: Documents the hard constraints imposed on types deriving a builder — properties that must always hold and whose violation causes compile-time failure.
- **Responsibility**: Master index for invariant doc instances — their identifiers, names, and status.
- **In Scope**: Fundamental, non-negotiable constraints on types and their fields that derive the builder macro.
- **Out of Scope**: Soft guidelines and recommendations, behavioral feature descriptions (→ feature/), algorithmic procedures (→ algorithm/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Owned Types Only](001_owned_types_only.md) | All fields must use owned data — no non-static borrowed types | ✅ |
| 002 | [No Generic Enums](002_no_generic_enums.md) | Enum types with generic type parameters cannot derive the builder | ✅ |
| 003 | [Single Variant Enum](003_single_variant_enum.md) | Multi-variant enums may produce conflicting trait implementations | ✅ |

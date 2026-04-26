# Algorithm: Variant Constructor Logic

### Scope

- **Purpose**: Determines which type of constructor to generate for each enum variant by mapping variant structural form and applied attribute to a constructor strategy.
- **Responsibility**: Documents the decision algorithm for enum variant constructor generation — the classification inputs, the rule table, and the four possible outcomes.
- **In Scope**: The 14-rule decision table, its two inputs (structural form and attribute), and its four outputs (compile error, direct, scalar, subformer).
- **Out of Scope**: Struct builder generation (→ feature/001_struct_former.md), attribute contract details (→ api/002_field_attributes.md), behavioral categories overview (→ feature/002_enum_former.md).

### Abstract

When a builder is derived for an enum type, each variant requires a constructor method. The type of constructor — whether it constructs directly with no arguments, accepts all fields as scalar arguments, or returns a nested builder — is determined at compile time by a 14-rule decision table. The table maps each combination of variant structural form and applied attribute to exactly one of four outcomes. The algorithm runs during macro expansion, before any code is compiled.

### Algorithm

The algorithm classifies each enum variant along two axes:

**Axis 1 — Structural form** (determined by variant syntax):
- Unit variant: no associated data
- Tuple variant with zero fields
- Tuple variant with one field
- Tuple variant with two or more fields
- Struct variant with zero fields
- Struct variant with one named field
- Struct variant with two or more named fields

**Axis 2 — Applied attribute** (from field or variant annotation):
- Default (no attribute specified)
- Scalar attribute
- Scalar subformer attribute

The intersection of these two axes produces one of four outcomes:

- **Compile error**: The combination is structurally invalid and rejected before generation.
- **Direct constructor**: A zero-argument static method that constructs the variant with no field input.
- **Scalar constructor**: A static method accepting all fields as positional arguments, producing the variant directly.
- **Subformer**: A static method returning a nested builder — either a builder for the variant itself or, for single-field tuple variants, a builder for the inner field type.

**Default resolution** (when no attribute is specified):
- Unit and zero-field tuple variants → direct constructor
- Single-field tuple variants → subformer for the inner type
- Multi-field tuple and multi-field struct variants → subformer for the variant
- Zero-field struct variants → compile error (explicit scalar attribute required)

The full 14-rule table covers all valid combinations. Rules 1a–1g cover scalar attribute behavior. Rules 2a–2g cover scalar subformer attribute behavior. Rules 3c and 3f cover notable default-resolution cases. Invalid combinations are assigned a compile error outcome and documented in the table.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/002_enum_former.md](../feature/002_enum_former.md) | Enum builder feature that applies this algorithm |
| doc | [api/002_field_attributes.md](../api/002_field_attributes.md) | Scalar and subformer attributes that provide Axis 2 input |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — section 2.2 Enum Variant Constructor Logic: 14-rule decision table, axis definitions, and four constructor outcomes; also produced 12 sibling instances — see entities.md |

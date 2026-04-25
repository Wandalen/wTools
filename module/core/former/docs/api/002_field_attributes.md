# API: Field Attributes

### Scope

- **Purpose**: Defines the set of attributes that appear on individual fields or enum variants and control how setter methods, subformers, defaults, and exclusions are generated for each field.
- **Responsibility**: Documents field-level attribute names, their effects, precedence rules, and stability guarantees.
- **In Scope**: All attributes applied to individual struct fields or enum variant fields, their semantics, and precedence when combined.
- **Out of Scope**: Item-level type-wide attributes (→ api/001_item_attributes.md), generated setter method signatures (→ feature/001_struct_former.md).

### Abstract

Field-level attributes modify per-field builder generation: whether a field uses a direct setter, delegates to a nested builder, works through a collection aggregate, accepts per-entry construction, carries a fallback default, or is excluded from standalone constructors. These attributes are the primary customization surface for individual fields within a type.

### Operations

Six field-level attributes are available:

- **Default value**: Declares a fallback value for the field used when its setter is never called during a build session. Compatible with any setter type.
- **Scalar**: Forces generation of a direct setter for this field regardless of the field's type. Accepts any value convertible to the field type. Use when the field type does not derive a builder or when nested building is not desired.
- **Scalar subformer**: Generates a method that returns a nested builder for this field's type. The field type must derive a builder. Completion of the nested builder automatically sets this field and returns to the parent. See feature/003_subform_scalar.md.
- **Collection subformer**: Generates a method that returns an aggregate builder for collection-typed fields. Entries are added to the collection through the aggregate builder. See feature/004_subform_collection.md.
- **Entry subformer**: Generates a method returning a per-entry builder for collection fields whose entry type derives a builder. Each completed entry is appended to the collection. See feature/005_subform_entry.md.
- **Ignore**: Excludes the field from standalone constructor parameters. The field receives its declared default or remains unset. Triggers SC-2 partial construction when standalone constructors are active. See feature/006_standalone_constructors.md.

### Error Handling

Conflicting field-level attributes on the same field produce compile-time errors. Precedence rules: the subformer family (scalar subformer, collection subformer, entry subformer) takes precedence over the scalar attribute when both appear on the same field. Applying a subformer attribute to a field whose type does not support the required interface produces a compile-time error.

### Compatibility Guarantees

Same stability guarantees as item-level attributes: stable within major versions, additions allowed in minor releases, removals follow a deprecation cycle.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/001_item_attributes.md](001_item_attributes.md) | Complementary item-level attribute reference |
| doc | [feature/003_subform_scalar.md](../feature/003_subform_scalar.md) | Feature using the scalar subformer attribute |
| doc | [feature/004_subform_collection.md](../feature/004_subform_collection.md) | Feature using the collection subformer attribute |
| doc | [feature/005_subform_entry.md](../feature/005_subform_entry.md) | Feature using the entry subformer attribute |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — section 2.4.2 Field-Level/Variant-Level Attributes table and section 2.4.3 Attribute Precedence and Interaction Rules |

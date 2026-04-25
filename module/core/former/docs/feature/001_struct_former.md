# Feature: Struct Former

### Scope

- **Purpose**: Enables fluent, ergonomic construction of struct values by generating a builder implementation from a type annotation.
- **Responsibility**: Documents the struct builder derivation feature — its behavior, generated interface, field defaults, and configuration surface.
- **In Scope**: Derive macro behavior on struct types, setter methods, field defaults, storage lifecycle, and form invocation.
- **Out of Scope**: Enum builder derivation (→ feature/002_enum_former.md), subformer composition (→ pattern/002_subformer_composition.md), attribute contracts (→ api/).

### Design

Builder derivation for structs is the foundational capability. Annotating a struct type activates generation of a companion builder that:

- Holds all field values as optional slots during a build session
- Provides one setter method per field, accepting any value convertible to the field type
- Chains setters by returning the builder on each call
- Produces the final struct via a form call that resolves declared defaults for any unset fields

The generated builder is named after the source type. Fields may carry a declared default that is used when no setter is called during a build session. Storage fields may be declared separately to hold intermediate bookkeeping state that does not appear in the final type. The mutator step, between storage population and form, can be customized or disabled.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Crate entry point re-exporting former_meta and former_types |
| doc | [api/001_item_attributes.md](../api/001_item_attributes.md) | Item-level attribute reference for builder customization |
| doc | [api/002_field_attributes.md](../api/002_field_attributes.md) | Field-level attribute reference for setter control |
| doc | [pattern/001_builder_pattern.md](../pattern/001_builder_pattern.md) | Builder pattern structure and participants |
| doc | [invariant/001_owned_types_only.md](../invariant/001_owned_types_only.md) | Constraint: all fields must use owned types |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — sections 1–3: struct former behavior, generated code architecture, setter design |
| [../../advanced.md](../../advanced.md) | Combined source covering all feature types; Former/Storage/Definition concepts in depth |

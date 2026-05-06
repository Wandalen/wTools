# Feature: Standalone Constructors

### Scope

- **Purpose**: Provides a function-call alternative to the builder chain for constructing values, accepting fields directly as arguments without requiring explicit builder interaction.
- **Responsibility**: Documents the standalone constructor generation feature — activation, return type rules, and field exclusion behavior.
- **In Scope**: The standalone constructors item-level attribute, the SC-1 full construction rule, the SC-2 partial construction rule, and the field ignore attribute interaction.
- **Out of Scope**: The builder chain API (→ feature/001_struct_former.md), field-level attribute details (→ api/002_field_attributes.md).

### Design

Standalone constructors generate top-level functions as an alternative entry point to the builder. The return type depends on whether any fields are excluded:

- **Full construction (SC-1)**: When no fields are excluded, the generated function takes all fields as positional arguments and returns a fully constructed value. No builder interaction is required.
- **Partial construction (SC-2)**: When one or more fields are marked for exclusion, the generated function takes only the included fields as arguments and returns a pre-initialized builder with those fields already set. The caller may then set the excluded fields through the builder interface before calling form.

The exclude attribute on a field causes that field to be omitted from the standalone constructor's parameter list. The field receives its declared default or remains unset in the returned builder, depending on whether a default is declared.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Crate entry point |
| doc | [api/001_item_attributes.md](../api/001_item_attributes.md) | Item-level attribute that activates standalone constructor generation |
| doc | [api/002_field_attributes.md](../api/002_field_attributes.md) | Field-level ignore attribute that triggers SC-2 partial construction |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — section 2.5 Standalone Constructor Generation: SC-1 full construction rule, SC-2 partial construction rule, and field exclusion model; also produced 12 sibling instances — see entities.md |

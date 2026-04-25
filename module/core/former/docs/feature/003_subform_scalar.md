# Feature: Scalar Subformer

### Scope

- **Purpose**: Enables inline construction of a nested value within the parent builder chain by delegating a single field's construction to its own builder.
- **Responsibility**: Documents the scalar subformer feature — activation, behavior, and completion model.
- **In Scope**: The scalar subformer attribute applied to a field whose type supports builder derivation, the delegation and return mechanism.
- **Out of Scope**: Collection subformers (→ feature/004_subform_collection.md), entry subformers (→ feature/005_subform_entry.md), the subformer composition pattern (→ pattern/002_subformer_composition.md).

### Design

When a field's type itself derives a builder, the scalar subformer attribute activates generation of a nested builder method in place of a plain setter. The caller:

1. Invokes the nested builder method on the parent builder
2. Receives the field type's own builder with a completion callback bound to the parent
3. Sets the nested type's fields via its builder interface
4. Calls form on the nested builder, which completes the nested value and returns control to the parent builder automatically

This preserves uninterrupted chaining — the caller never explicitly handles the nested value or re-references the parent builder. The completion callback is injected at the point the nested builder is created.

Note: for fields whose type is a primitive or does not derive a builder, the plain scalar attribute should be used instead. The scalar subformer requires the field type to support the builder interface.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Crate entry point |
| doc | [feature/001_struct_former.md](001_struct_former.md) | Foundational struct builder on which subformer composition builds |
| doc | [api/002_field_attributes.md](../api/002_field_attributes.md) | Field-level attribute that activates this feature |
| doc | [pattern/002_subformer_composition.md](../pattern/002_subformer_composition.md) | Design pattern governing parent-child builder delegation |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — section 2.4.2 field attribute table, subform_scalar entry |
| [../../advanced.md](../../advanced.md) | Combined source; subformer delegation mechanics described in FormingEnd sections |

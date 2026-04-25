# Feature: Collection Subformer

### Scope

- **Purpose**: Enables element-by-element assembly of collection-typed fields within the parent builder chain using a specialized aggregate builder.
- **Responsibility**: Documents the collection subformer feature — activation, the aggregate builder interface, and completion model.
- **In Scope**: The collection subformer attribute, the aggregate builder it generates, and how it integrates with the parent builder.
- **Out of Scope**: Scalar subformers for non-collection types (→ feature/003_subform_scalar.md), per-entry subformers for individually constructed collection entries (→ feature/005_subform_entry.md).

### Design

Collection fields — lists, sets, maps, and similar aggregates — benefit from a dedicated builder that accumulates entries before producing the complete collection. When the collection subformer attribute is applied to a field, an aggregate builder method is generated on the parent builder. The caller:

1. Invokes the aggregate builder method on the parent builder
2. Receives a collection-specific builder with insertion methods suited to the collection type
3. Adds entries to the collection one at a time via the aggregate builder's interface
4. Calls form on the aggregate builder, which finalizes the collection and stores it in the parent builder's field slot automatically

The aggregate builder handles collection construction semantics (ordering, uniqueness, type compatibility) appropriate to the collection type. On completion it returns control to the parent builder, preserving the chaining model.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Crate entry point |
| doc | [feature/001_struct_former.md](001_struct_former.md) | Foundational struct builder |
| doc | [api/002_field_attributes.md](../api/002_field_attributes.md) | Field-level attribute that activates this feature |
| doc | [pattern/002_subformer_composition.md](../pattern/002_subformer_composition.md) | Parent-child builder delegation pattern |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — section 2.4.2 field attribute table, subform_collection entry |
| [../../advanced.md](../../advanced.md) | Combined source; collection subformer mechanics and VectorFormer usage described |

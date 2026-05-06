# Feature: Entry Subformer

### Scope

- **Purpose**: Enables construction of individual collection entries inline within the parent builder chain when each entry is itself a structured type that supports builder derivation.
- **Responsibility**: Documents the entry subformer feature — activation, per-entry builder interface, and entry registration model.
- **In Scope**: The entry subformer attribute applied to a collection field whose entry type supports builder derivation.
- **Out of Scope**: Aggregate collection building (→ feature/004_subform_collection.md), scalar subformers for non-collection types (→ feature/003_subform_scalar.md).

### Design

When collection entries are themselves structured types with builder support, the entry subformer attribute generates a per-entry builder method. Each invocation constructs one entry and registers it in the collection. The caller:

1. Invokes the entry builder method on the parent builder
2. Receives a builder scoped to the entry type, with a completion callback that adds the finished entry to the collection slot
3. Builds the entry using the entry type's builder interface
4. Calls form on the entry builder; the finished entry is appended to the collection and control returns to the parent builder

Multiple successive entry builder invocations accumulate entries into the collection. The collection slot is populated incrementally across multiple entry builder sessions during the same parent build.

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
| [../../spec.md](../../spec.md) | Primary source — section 2.4.2 field attribute table, subform_entry entry; also produced 12 sibling instances — see entities.md |
| [../../advanced.md](../../advanced.md) | Combined source; entry subformer mechanics and collection accumulation described; also produced 5 sibling instances: feature/001–004, pattern/002 |

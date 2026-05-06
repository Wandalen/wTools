# Algorithm: Type Deduplication

### Scope

- **Purpose**: Document how conflicting trait impl errors are prevented when a struct has multiple fields of the same type.
- **Responsibility**: Explains the HashSet-based deduplication used by Assign and ComponentFrom derives to emit only one impl per distinct field type.
- **In Scope**: The deduplication mechanism, the string-key used for identity, and which derives apply it.
- **Out of Scope**: Recognition of standard library types (→ `algorithm/001_popular_type_generation.md`); trait semantics (→ `component_model_types/docs/api/` — to create).

### Abstract

Rust disallows two trait impls with the same self-type and trait parameters in the same crate (E0119 — conflicting implementations). When a struct has two fields of the same type, naively generating one impl per field would produce this error.

Both the Assign derive and the ComponentFrom derive apply deduplication: before emitting an impl for a field's type, they check whether an impl for that type has already been emitted. Only the first field of each distinct type produces an impl; subsequent fields of the same type are silently skipped. This is intentional and documented behavior — struct semantics are preserved because the single impl covers all fields of that type.

### Algorithm

1. Initialize an empty set of seen type strings.
2. Iterate over struct fields in declaration order.
3. For each field, render its type to a token string via the token rendering facility.
4. Attempt to insert the token string into the seen set.
   - If the insertion succeeds (type is new): emit the impl.
   - If the insertion fails (type already seen): skip the field — emit nothing.

**Applies to:**

| Derive | Source file |
|--------|------------|
| Assign | `src/component/component_assign.rs` |
| ComponentFrom | `src/component/component_from.rs` |

**Token-string identity:** Two types are considered identical if their rendered token string is identical. This is structural, not semantic — type aliases that expand to the same concrete type are treated as distinct. Generic parameters are included in the rendered string, so a vector of integers and a vector of bytes are distinct keys.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/component/component_assign.rs` | Assign derive — applies deduplication per field |
| source | `src/component/component_from.rs` | ComponentFrom derive — applies deduplication per field |
| test | `tests/assign_duplicate_types_bug.rs` | Verifies deduplication in Assign |
| test | `tests/component_from_duplicate_types_bug.rs` | Verifies deduplication in ComponentFrom |
| doc | [api/001_derive_macros.md](../api/001_derive_macros.md) | Derive macros that use this algorithm |
| doc | [algorithm/001_popular_type_generation.md](001_popular_type_generation.md) | Companion algorithm for std type recognition |

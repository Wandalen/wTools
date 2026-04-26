# Feature: Entity Reflection

### Scope

- **Purpose**: Enable runtime type introspection on any value implementing the reflection protocol.
- **Responsibility**: Document the entity reflection subsystem scope, design, and all artifact locations.
- **In Scope**: Instance/Entity trait pair, three descriptor kinds, reflect() entry point, KeyVal, container semantics.
- **Out of Scope**: Fields iteration (→ `feature/002_fields_iteration.md`); OptionalCow internals (→ `data_structure/001_optional_cow.md`).

### Design

The entity reflection subsystem exposes two complementary traits per reflectable type. The first trait provides upcast capability — converting a concrete value into a type-erased reflection handle. The second trait defines the reflection protocol — type name, type identity, container detection, element count, element iteration, and ordering semantics.

Three descriptor kinds classify reflected values: key descriptors for simple identifiers, primitive descriptors for scalar values, and key-value descriptors pairing a key with its associated value. The key-value descriptor is the primary unit returned by element iteration on containers.

The reflect() function is the single entry point, returning a type-erased reference that callers interrogate through the reflection protocol. Container types (collections, arrays, slices) report positive for container detection and expose their elements through an iterator of key-value descriptors. Non-container types (scalars, references) report negative and yield no elements.

Ordering semantics distinguish containers that preserve insertion or index order (arrays, slices, vectors) from those that do not (hash-based collections). This property is exposed as a boolean query on the reflected entity.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/reflect/axiomatic.rs` | Core trait definitions and descriptor types |
| source | `src/reflect/entity_vec.rs` | Entity implementation for Vec |
| source | `src/reflect/entity_array.rs` | Entity implementation for arrays |
| source | `src/reflect/entity_slice.rs` | Entity implementation for slices |
| source | `src/reflect/entity_hashmap.rs` | Entity implementation for HashMap |
| source | `src/reflect/entity_hashset.rs` | Entity implementation for HashSet |
| source | `src/reflect/primitive.rs` | Primitive enum definition |
| test | `tests/inc/group1/common_test.rs` | Comprehensive primitive reflection tests |
| test | `tests/inc/group1/vec_test.rs` | Vec reflection tests |
| test | `tests/inc/group1/array_test.rs` | Array reflection tests |
| test | `tests/inc/group1/slice_test.rs` | Slice reflection tests |
| test | `tests/inc/group1/hashmap_test.rs` | HashMap reflection tests |
| test | `tests/inc/group1/hashset_test.rs` | HashSet reflection tests |
| test | `tests/inc/group1/is_ordered_test.rs` | Container ordering invariant tests |
| doc | [`docs/api/001_reflection_api.md`](../api/001_reflection_api.md) | Public reflection operations contract |
| doc | [`docs/invariant/001_container_ordering.md`](../invariant/001_container_ordering.md) | Container ordering correctness property |
| doc | [`docs/invariant/002_element_iteration_contract.md`](../invariant/002_element_iteration_contract.md) | Element count consistency property |
| doc | [`docs/feature/002_fields_iteration.md`](002_fields_iteration.md) | Fields iteration subsystem — foundation for entity reflection |
| doc | [`docs/data_structure/002_primitive.md`](../data_structure/002_primitive.md) | Primitive enum description |

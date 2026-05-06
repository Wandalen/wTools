# API: Reflection API

### Scope

- **Purpose**: Define the public operations contract for entity reflection.
- **Responsibility**: Document reflection operations, return value semantics, and compatibility guarantees.
- **In Scope**: reflect(), type_name(), type_id(), is_container(), is_ordered(), len(), elements().
- **Out of Scope**: Fields iteration API (→ `api/002_fields_api.md`); implementation logic (→ `src/`).

### Abstract

Provides runtime type introspection for any value implementing the reflection protocol. Callers invoke a single entry point to obtain a type-erased handle, then query type identity, container status, ordering, element count, and element contents through a uniform interface regardless of the underlying concrete type.

### Operations

**reflect()**: Accepts any reflectable value, returns a type-erased reference implementing the reflection protocol.

**type_name()**: Returns a human-readable string identifying the concrete type of the reflected value.

**type_id()**: Returns the compiler-assigned type identity for exact type comparison.

**is_container()**: Returns true if the value is a collection type holding elements, false for scalars and references.

**is_ordered()**: Returns true if the container preserves insertion or index ordering, false for unordered containers. Undefined for non-containers.

**len()**: Returns the number of elements in a container. Returns zero for non-containers.

**elements()**: Returns an iterator of key-value descriptors over the container's elements. Empty for non-containers.

### Error Handling

All reflection operations are infallible on types that implement the reflection protocol. No runtime errors or panics are produced. Types that do not implement the protocol cannot be reflected — this is enforced at compile time through trait bounds.

### Compatibility Guarantees

Experimental status — no stability guarantees. Adding new type implementations is non-breaking. Changes to trait definitions or descriptor kinds are breaking.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/reflect/axiomatic.rs` | Core trait definitions and reflection entry point |
| source | `src/reflect/entity_vec.rs` | Vec reflection implementation |
| source | `src/reflect/entity_hashmap.rs` | HashMap reflection implementation |
| test | `tests/inc/group1/common_test.rs` | Comprehensive primitive reflection tests |
| test | `tests/inc/group1/vec_test.rs` | Vec reflection tests |
| doc | [`docs/feature/001_entity_reflection.md`](../feature/001_entity_reflection.md) | Feature scope and design decisions |
| doc | [`docs/invariant/001_container_ordering.md`](../invariant/001_container_ordering.md) | is_ordered() correctness property |
| doc | [`docs/invariant/002_element_iteration_contract.md`](../invariant/002_element_iteration_contract.md) | elements()/len() consistency property |
| doc | [`docs/data_structure/002_primitive.md`](../data_structure/002_primitive.md) | Primitive descriptor enum |

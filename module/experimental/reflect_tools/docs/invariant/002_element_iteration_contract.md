# Invariant: Element Iteration Contract

### Scope

- **Purpose**: Guarantee consistency between element count and reported length for all containers.
- **Responsibility**: Document the count-length invariant, its enforcement, and consequences of violation.
- **In Scope**: elements().count() and len() consistency for all container types.
- **Out of Scope**: Container ordering (→ `invariant/001_container_ordering.md`); API details (→ `api/001_reflection_api.md`).

### Invariant Statement

For every container type implementing the reflection protocol: at any single point in time, the number of elements yielded by the element iterator must equal the value returned by the length query. Formally: elements().count() == len() for all containers at any snapshot.

### Enforcement Mechanism

All entity implementations derive both the element iterator and the length from the same underlying data structure, making divergence structurally impossible. Test suites for individual container types verify both values and compare them.

### Violation Consequences

Inconsistency between element count and reported length causes incorrect collection processing. Code that pre-allocates based on len() then fills from elements() would either overflow or leave uninitialized slots. Code that uses len() for bounds checking would accept or reject iterations incorrectly.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/reflect/entity_vec.rs` | Vec entity — derives both from backing Vec |
| source | `src/reflect/entity_array.rs` | Array entity — derives both from array length |
| test | `tests/inc/group1/vec_test.rs` | Vec element count verification |
| test | `tests/inc/group1/array_test.rs` | Array element count verification |
| doc | [`docs/api/001_reflection_api.md`](../api/001_reflection_api.md) | len() and elements() operation definitions |
| doc | [`docs/feature/001_entity_reflection.md`](../feature/001_entity_reflection.md) | Entity reflection feature scope |

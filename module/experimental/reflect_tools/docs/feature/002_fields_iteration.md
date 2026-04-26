# Feature: Fields Iteration

### Scope

- **Purpose**: Enable key-value iteration over standard collection types through a unified trait.
- **Responsibility**: Document the fields iteration subsystem scope, design, and all artifact locations.
- **In Scope**: Fields trait with GAT-based key/value types, three value forms, TypeName, OptionalCow.
- **Out of Scope**: Entity reflection (→ `feature/001_entity_reflection.md`); Primitive enum (→ `data_structure/002_primitive.md`).

### Design

The fields iteration subsystem provides a single trait parameterized by key and value types. The trait uses generic associated types for its key and value accessors, allowing implementations to return borrowed references with correct lifetimes without allocation.

Three value forms govern how field data is accessed: reference form returns borrowed views, owned form returns cloned or moved values, and optional form wraps the result in an optional borrowed-or-owned container for fields that may be absent.

A companion type name trait provides a stable string identifier for any type, used by reflection to report human-readable type names without relying on compiler-internal naming.

Implementations exist for seven standard collection types: Vec, HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, and LinkedList. Each implementation maps the collection's natural iteration pattern to the fields trait interface.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/reflect/fields.rs` | Fields trait definition with GAT key/value types |
| source | `src/reflect/fields/vec.rs` | Fields implementation for Vec |
| source | `src/reflect/fields/hmap.rs` | Fields implementation for HashMap |
| source | `src/reflect/fields/hset.rs` | Fields implementation for HashSet |
| source | `src/reflect/fields/bmap.rs` | Fields implementation for BTreeMap |
| source | `src/reflect/fields/bset.rs` | Fields implementation for BTreeSet |
| source | `src/reflect/fields/deque.rs` | Fields implementation for VecDeque |
| source | `src/reflect/fields/llist.rs` | Fields implementation for LinkedList |
| source | `src/reflect/wrapper/optional_cow.rs` | OptionalCow wrapper for optional field values |
| test | `tests/inc/fundamental/fields_test.rs` | Fields trait general behavior tests |
| test | `tests/inc/fundamental/fields_vec.rs` | Fields Vec tests |
| test | `tests/inc/fundamental/fields_hmap.rs` | Fields HashMap tests |
| test | `tests/inc/fundamental/fields_hset.rs` | Fields HashSet tests |
| test | `tests/inc/fundamental/fields_bmap.rs` | Fields BTreeMap tests |
| test | `tests/inc/fundamental/fields_bset.rs` | Fields BTreeSet tests |
| test | `tests/inc/fundamental/fields_deque.rs` | Fields VecDeque tests |
| test | `tests/inc/fundamental/fields_llist.rs` | Fields LinkedList tests |
| doc | [`docs/api/002_fields_api.md`](../api/002_fields_api.md) | Fields trait operations contract |
| doc | [`docs/data_structure/001_optional_cow.md`](../data_structure/001_optional_cow.md) | OptionalCow wrapper description |

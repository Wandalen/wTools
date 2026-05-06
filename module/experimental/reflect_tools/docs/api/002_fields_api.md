# API: Fields API

### Scope

- **Purpose**: Define the public operations contract for key-value field iteration.
- **Responsibility**: Document the Fields trait interface, value form semantics, and collection coverage.
- **In Scope**: Fields trait operations, GAT key/value accessors, three value forms, supported collections.
- **Out of Scope**: Entity reflection API (→ `api/001_reflection_api.md`); OptionalCow internals (→ `data_structure/001_optional_cow.md`).

### Abstract

Provides a unified key-value iteration interface over standard collection types. Each collection maps its natural iteration pattern to a common trait, allowing generic code to iterate fields of any supported collection without knowing the concrete type. Uses generic associated types for zero-allocation borrowed access.

### Operations

**fields() / key-value access**: Iterate over all key-value pairs in the collection using the collection's natural ordering. Keys and values are accessed through lifetime-parameterized associated types.

**Reference form**: Returns borrowed views of keys and values without allocation.

**Owned form**: Returns cloned or moved copies of keys and values.

**Optional form**: Wraps field access in an optional borrowed-or-owned container for fields that may be absent.

**Supported collections**: Vec, HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, LinkedList.

### Error Handling

All fields operations are infallible. Iteration always succeeds and terminates. Empty collections produce empty iterators.

### Compatibility Guarantees

Experimental status — no stability guarantees. Requires a compiler supporting generic associated types. Adding new collection implementations is non-breaking.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/reflect/fields.rs` | Fields trait definition |
| source | `src/reflect/fields/vec.rs` | Vec implementation |
| source | `src/reflect/fields/hmap.rs` | HashMap implementation |
| source | `src/reflect/fields/hset.rs` | HashSet implementation |
| test | `tests/inc/fundamental/fields_test.rs` | General Fields trait tests |
| test | `tests/inc/fundamental/fields_vec.rs` | Vec Fields tests |
| doc | [`docs/feature/002_fields_iteration.md`](../feature/002_fields_iteration.md) | Feature scope and design decisions |
| doc | [`docs/data_structure/001_optional_cow.md`](../data_structure/001_optional_cow.md) | OptionalCow wrapper type |

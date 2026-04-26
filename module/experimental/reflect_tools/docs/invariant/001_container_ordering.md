# Invariant: Container Ordering

### Scope

- **Purpose**: Guarantee that is_ordered() returns correct values for all container types.
- **Responsibility**: Document the ordering invariant, its enforcement, and consequences of violation.
- **In Scope**: is_ordered() return values for all implemented container types.
- **Out of Scope**: Element iteration contract (→ `invariant/002_element_iteration_contract.md`); API details (→ `api/001_reflection_api.md`).

### Invariant Statement

For every container type implementing the reflection protocol: hash-based containers (HashMap, HashSet) must return false from is_ordered(); index-based and tree-based containers (Vec, arrays, slices, BTreeMap, BTreeSet, VecDeque, LinkedList) must return true.

### Enforcement Mechanism

A dedicated test suite exercises is_ordered() on every implemented container type. The test file is marked as a bug reproducer (issue-manual-test-001) because a prior implementation returned incorrect values. Each container type has an explicit assertion verifying the expected boolean result.

### Violation Consequences

Incorrect is_ordered() causes silent data corruption in algorithms that rely on deterministic element ordering. A false positive (reporting ordered when unordered) allows hash-based iteration — whose order is non-deterministic — to be treated as stable, producing inconsistent results across runs.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/reflect/entity_hashset.rs` | HashSet entity — must return false |
| source | `src/reflect/entity_hashmap.rs` | HashMap entity — must return false |
| source | `src/reflect/entity_vec.rs` | Vec entity — must return true |
| test | `tests/inc/group1/is_ordered_test.rs` | Bug reproducer verifying all container types |
| doc | [`docs/api/001_reflection_api.md`](../api/001_reflection_api.md) | is_ordered() operation definition |
| doc | [`docs/feature/001_entity_reflection.md`](../feature/001_entity_reflection.md) | Entity reflection feature scope |

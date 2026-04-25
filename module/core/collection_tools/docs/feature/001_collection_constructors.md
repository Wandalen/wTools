# Feature: Collection Constructors

### Scope

- **Purpose**: Enable ergonomic single-expression initialization of Rust collections with homogeneous elements and pre-allocated capacity.
- **Responsibility**: Document the strict constructor macros and their usage patterns.
- **In Scope**: `vec!`, `hmap!`, `hset!`, `bmap!`, `bset!`, `llist!`, `deque!`, `dlist!` macros; capacity pre-allocation behavior; type inference characteristics.
- **Out of Scope**: Into-based constructors (see `002_into_constructors.md`); no_std collection source selection (see `../invariant/001_no_std_alloc.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/collection/hash_map.rs` | `hmap!` implementation |
| source | `src/collection/hash_set.rs` | `hset!` implementation |
| source | `src/collection/btree_map.rs` | `bmap!` implementation |
| source | `src/collection/btree_set.rs` | `bset!` implementation |
| source | `src/collection/linked_list.rs` | `llist!` implementation |
| source | `src/collection/vec_deque.rs` | `deque!` implementation |
| source | `src/collection/vector.rs` | `vec!` implementation |
| source | `src/collection/mod.rs` | `count!` macro used for capacity pre-allocation |
| test | `tests/smoke_test.rs` | Smoke tests covering all strict constructor macros |
| test | `tests/inc/hmap.rs` | `hmap!` macro tests |
| test | `tests/inc/hset.rs` | `hset!` macro tests |
| test | `tests/inc/vec.rs` | `vec!` macro tests |
| test | `tests/inc/bmap.rs` | `bmap!` macro tests |
| test | `tests/inc/bset.rs` | `bset!` macro tests |
| test | `tests/inc/llist.rs` | `llist!` macro tests |
| test | `tests/inc/deque.rs` | `deque!` macro tests |
| doc | `../api/001_collection_macros.md` | Complete macro signature contract |
| doc | `002_into_constructors.md` | Into-based counterpart for heterogeneous types |
| doc | `../invariant/001_no_std_alloc.md` | Invariant governing HashMap/HashSet source |
| doc | `../invariant/002_capacity_preallocated.md` | Invariant guaranteeing pre-allocation |

### Design

#### Motivation

Rust's standard library provides no ergonomic way to initialize a populated collection in a single expression. The typical pattern requires a `let mut` binding, multiple `.insert()` calls, and then reassignment to an immutable binding. The strict constructor macros collapse this into one expression, improving readability and eliminating the mutable intermediate.

#### Collection Coverage

Each standard collection has a corresponding strict macro:

- `vec!( e1, e2, e3 )` — `Vec<T>` from elements
- `hmap!( k1 => v1, k2 => v2 )` — `HashMap<K, V>` from key-value pairs
- `hset!( e1, e2, e3 )` — `HashSet<T>` from elements
- `bmap!( k1 => v1, k2 => v2 )` — `BTreeMap<K, V>` from key-value pairs
- `bset!( e1, e2, e3 )` — `BTreeSet<T>` from elements
- `llist!( e1, e2, e3 )` — `LinkedList<T>` from elements
- `deque!( e1, e2, e3 )` — `VecDeque<T>` from elements
- `dlist!( e1, e2, e3 )` — alias for `vec!`

#### Type Inference

Strict macros infer the element and key/value types from the argument expressions. When all elements share the same type, no annotation is required. An explicit type annotation on the binding remains valid and takes precedence over inference.

#### Capacity Pre-allocation

Every macro pre-allocates for exactly N elements before the first insert, where N is the number of arguments at the call site. This is equivalent to constructing with capacity N then inserting — zero reallocations occur during construction. The standard approach without pre-allocation starts at capacity zero and triggers one or more reallocations during insertion.

#### Feature Gate

All strict macros are gated on `feature = "collection_constructors"`, which is enabled by default via the `enabled` meta-feature. When `collection_constructors` is absent from the active feature set, none of the strict macros are compiled.

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Migrated; sections: Overview → Variadic Constructor Macros (Strict), Architecture → Macro Expansion Pattern, Usage Patterns → Pattern 1/4/5, Design Rationale → Why Variadic Constructor Macros, Adoption Guidelines; siblings: api/001, feature/002, invariant/001, invariant/002. spec.md has been deleted — Sources entry retained as migration record. |

# Feature: Into Constructors

### Scope

- **Purpose**: Enable collection initialization from heterogeneous elements that share a common `Into<T>` target type.
- **Responsibility**: Document the into-based constructor macros and their type annotation requirements.
- **In Scope**: `into_vec!`, `into_hmap!`, `into_hset!`, `into_bmap!`, `into_bset!`, `into_llist!`, `into_vecd!`, `into_dlist!` macros; `.into()` coercion contract; type annotation requirements.
- **Out of Scope**: Strict constructors (see `001_collection_constructors.md`); no_std collection source (see `../invariant/001_no_std_alloc.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/collection/hash_map.rs` | `into_hmap!` implementation |
| source | `src/collection/hash_set.rs` | `into_hset!` implementation |
| source | `src/collection/btree_map.rs` | `into_bmap!` implementation |
| source | `src/collection/btree_set.rs` | `into_bset!` implementation |
| source | `src/collection/linked_list.rs` | `into_llist!` implementation |
| source | `src/collection/vec_deque.rs` | `into_vecd!` implementation |
| source | `src/collection/vector.rs` | `into_vec!` implementation |
| source | `src/collection/mod.rs` | `count!` macro used for capacity pre-allocation |
| test | `tests/smoke_test.rs` | Smoke tests including into-based constructor macros |
| test | `tests/inc/hmap.rs` | `into_hmap!` macro tests |
| test | `tests/inc/hset.rs` | `into_hset!` macro tests |
| test | `tests/inc/vec.rs` | `into_vec!` macro tests |
| test | `tests/inc/bmap.rs` | `into_bmap!` macro tests |
| test | `tests/inc/bset.rs` | `into_bset!` macro tests |
| test | `tests/inc/llist.rs` | `into_llist!` macro tests |
| test | `tests/inc/deque.rs` | `into_vecd!` macro tests |
| doc | `../api/001_collection_macros.md` | Complete macro signature contract |
| doc | `001_collection_constructors.md` | Strict-type counterpart |
| doc | `../invariant/001_no_std_alloc.md` | Invariant governing HashMap/HashSet source (out of scope here; see invariant for full coverage) |
| doc | `../invariant/002_capacity_preallocated.md` | Invariant guaranteeing pre-allocation |

### Design

#### Motivation

Strict macros require all elements to share the same literal type. Into-based macros lift this restriction by calling `.into()` on each element before insertion, allowing initialization from heterogeneous sources that share a common `Into<T>` target type.

#### Collection Coverage

Each strict macro has an into-based counterpart:

- `into_vec!( e1, e2, e3 )` — `Vec<T>` via `.into()` on each element
- `into_hmap!( k1 => v1, k2 => v2 )` — `HashMap<K, V>` via `.into()` on keys and values
- `into_hset!( e1, e2, e3 )` — `HashSet<T>` via `.into()` on each element
- `into_bmap!( k1 => v1, k2 => v2 )` — `BTreeMap<K, V>` via `.into()` on keys and values
- `into_bset!( e1, e2, e3 )` — `BTreeSet<T>` via `.into()` on each element
- `into_llist!( e1, e2, e3 )` — `LinkedList<T>` via `.into()` on each element
- `into_vecd!( e1, e2, e3 )` — `VecDeque<T>` via `.into()` on each element
- `into_dlist!( e1, e2, e3 )` — alias for `into_vec!`

#### Type Annotations

Because coercion requires a known target type, the compiler cannot always infer `T` from the arguments alone. Explicit type annotation on the binding is required in most cases — the argument types alone do not determine the target. Without annotation, compilation fails with a type-inference error. For map macros, both the key and value types each need annotation.

#### Tradeoff vs Strict Macros

Into macros trade implicit type inference for flexibility. Prefer strict macros when all elements already share the same type — inference works automatically and no annotation is needed. Use into macros when elements come from heterogeneous sources that share a common target type — the annotation cost is worth the initialization flexibility.

#### Feature Gate

All into-based macros are gated on `feature = "collection_into_constructors"`, which is enabled by default. This feature is independent of `collection_constructors`; each can be enabled or disabled separately without affecting the other.

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Migrated; sections: Overview → Variadic Constructor Macros (Into-based), Architecture → Macro Expansion Pattern, Usage Patterns → Pattern 2/6, Design Rationale → Why Two Classes of Macros, Adoption Guidelines; siblings: api/001, feature/001, invariant/001, invariant/002. spec.md has been deleted — Sources entry retained as migration record. |

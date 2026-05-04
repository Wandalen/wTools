# Feature: Into Constructors

### Scope

- **Purpose**: Enable collection initialization from heterogeneous elements that share a common `Into<T>` target type.
- **Responsibility**: Document the into-based constructor macros and their type annotation requirements.
- **In Scope**: 9 into-based constructor macros for heterogeneous element initialization; coercion via the Into trait; explicit type annotation requirements.
- **Out of Scope**: Strict constructors (see `001_collection_constructors.md`); no_std collection source (see `../invariant/001_no_std_alloc.md`).

### Sources

| File | Relationship |
|------|-------------|
| `src/collection/binary_heap.rs` | `into_heap!` implementation |
| `src/collection/hash_map.rs` | `into_hmap!` implementation |
| `src/collection/hash_set.rs` | `into_hset!` implementation |
| `src/collection/btree_map.rs` | `into_bmap!` implementation |
| `src/collection/btree_set.rs` | `into_bset!` implementation |
| `src/collection/linked_list.rs` | `into_llist!` implementation |
| `src/collection/vec_deque.rs` | `into_vecd!` implementation |
| `src/collection/vector.rs` | `into_vec!` and `into_dlist!` implementation |
| `src/collection/mod.rs` | `count!` macro used for capacity pre-allocation |

### Tests

| File | Relationship |
|------|-------------|
| `tests/heap_macro_availability_test.rs` | Bug reproducer — into_heap! public API accessibility |
| `tests/inc/heap.rs` | `into_heap!` macro tests |
| `tests/inc/hmap.rs` | `into_hmap!` macro tests |
| `tests/inc/hset.rs` | `into_hset!` macro tests |
| `tests/inc/vec.rs` | `into_vec!` and `into_dlist!` macro tests |
| `tests/inc/bmap.rs` | `into_bmap!` macro tests |
| `tests/inc/bset.rs` | `into_bset!` macro tests |
| `tests/inc/llist.rs` | `into_llist!` macro tests |
| `tests/inc/deque.rs` | `into_vecd!` macro tests |
| `tests/manual_corner_cases_test.rs` | Heterogeneous types, capacity, and move-semantics corner cases |
| `tests/docs/feature/02_into_constructors.md` | Test spec for this feature |

### APIs

| File | Relationship |
|------|-------------|
| `../api/001_collection_macros.md` | Complete macro signature contract |

### Features

| File | Relationship |
|------|-------------|
| `001_collection_constructors.md` | Strict-type counterpart |

### Invariants

| File | Relationship |
|------|-------------|
| `../invariant/001_no_std_alloc.md` | Invariant governing HashMap/HashSet allocation source |
| `../invariant/002_capacity_preallocated.md` | Invariant guaranteeing pre-allocation |

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
- `into_heap!( e1, e2, e3 )` — `BinaryHeap<T>` via `.into()` on each element (max-heap ordering)
- `into_dlist!( e1, e2, e3 )` — alias for `into_vec!`

#### Type Annotations

Because coercion requires a known target type, the compiler cannot always infer `T` from the arguments alone. Explicit type annotation on the binding is required in most cases — the argument types alone do not determine the target. Without annotation, compilation fails with a type-inference error. For map macros, both the key and value types each need annotation.

#### Tradeoff vs Strict Macros

Into macros trade implicit type inference for flexibility. Prefer strict macros when all elements already share the same type — inference works automatically and no annotation is needed. Use into macros when elements come from heterogeneous sources that share a common target type — the annotation cost is worth the initialization flexibility.

#### Feature Gate

All into-based macros are gated on `feature = "collection_into_constructors"`, which is enabled by default. This feature is independent of `collection_constructors`; each can be enabled or disabled separately without affecting the other.

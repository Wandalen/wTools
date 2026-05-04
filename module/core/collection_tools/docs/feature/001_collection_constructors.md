# Feature: Collection Constructors

### Scope

- **Purpose**: Enable ergonomic single-expression initialization of Rust collections with homogeneous elements and pre-allocated capacity.
- **Responsibility**: Document the strict constructor macros and their usage patterns.
- **In Scope**: 9 strict constructor macros covering all standard collection types; capacity pre-allocation behavior for applicable types; homogeneous type inference.
- **Out of Scope**: Into-based constructors (see `002_into_constructors.md`); no_std collection source selection (see `../invariant/001_no_std_alloc.md`).

### Sources

| File | Relationship |
|------|-------------|
| `src/collection/binary_heap.rs` | `heap!` implementation |
| `src/collection/hash_map.rs` | `hmap!` implementation |
| `src/collection/hash_set.rs` | `hset!` implementation |
| `src/collection/btree_map.rs` | `bmap!` implementation |
| `src/collection/btree_set.rs` | `bset!` implementation |
| `src/collection/linked_list.rs` | `llist!` implementation |
| `src/collection/vec_deque.rs` | `deque!` implementation |
| `src/collection/vector.rs` | `vec!` and `dlist!` implementation |
| `src/collection/mod.rs` | `count!` macro used for capacity pre-allocation |

### Tests

| File | Relationship |
|------|-------------|
| `tests/heap_macro_availability_test.rs` | Bug reproducer — heap macro public API accessibility |
| `tests/inc/heap.rs` | `heap!` macro tests |
| `tests/inc/hmap.rs` | `hmap!` macro tests |
| `tests/inc/hset.rs` | `hset!` macro tests |
| `tests/inc/vec.rs` | `vec!` and `dlist!` macro tests |
| `tests/inc/bmap.rs` | `bmap!` macro tests |
| `tests/inc/bset.rs` | `bset!` macro tests |
| `tests/inc/llist.rs` | `llist!` macro tests |
| `tests/inc/deque.rs` | `deque!` macro tests |
| `tests/manual_corner_cases_test.rs` | Empty, trailing-comma, capacity, and type-inference corner cases |
| `tests/docs/feature/01_collection_constructors.md` | Test spec for this feature |

### APIs

| File | Relationship |
|------|-------------|
| `../api/001_collection_macros.md` | Complete macro signature contract |

### Features

| File | Relationship |
|------|-------------|
| `002_into_constructors.md` | Into-based counterpart for heterogeneous types |

### Invariants

| File | Relationship |
|------|-------------|
| `../invariant/001_no_std_alloc.md` | Invariant governing HashMap/HashSet allocation source |
| `../invariant/002_capacity_preallocated.md` | Invariant guaranteeing pre-allocation |

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
- `heap!( e1, e2, e3 )` — `BinaryHeap<T>` from elements (max-heap ordering)
- `dlist!( e1, e2, e3 )` — alias for `vec!`

#### Type Inference

Strict macros infer the element and key/value types from the argument expressions. When all elements share the same type, no annotation is required. An explicit type annotation on the binding remains valid and takes precedence over inference.

#### Capacity Pre-allocation

Macros for collection types that support `with_capacity` (Vec, HashMap, HashSet, VecDeque, BinaryHeap) pre-allocate for exactly N elements before the first insert, where N is the number of arguments at the call site. Zero reallocations occur during construction for these types. BTreeMap, BTreeSet, and LinkedList macros call `new()` instead — these types have no `with_capacity` API in std Rust.

#### Feature Gate

All strict macros are gated on `feature = "collection_constructors"`, which is enabled by default via the `enabled` meta-feature. When `collection_constructors` is absent from the active feature set, none of the strict macros are compiled.

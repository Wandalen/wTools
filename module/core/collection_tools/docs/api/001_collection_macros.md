# API: Collection Constructor Macros

### Scope

- **Purpose**: Document the complete public API for all variadic collection constructor macros.
- **Responsibility**: Define macro signatures, expansion contracts, and type re-export behavior.
- **In Scope**: Strict and into-based constructor macro groups for all standard Rust collection types; unified collection type re-exports.
- **Out of Scope**: Feature-flag selection rationale (see `../feature/`); invariant proofs (see `../invariant/`).

### Sources

| File | Relationship |
|------|-------------|
| `src/collection/hash_map.rs` | `HashMap` re-export, `hmap!`, `into_hmap!` |
| `src/collection/hash_set.rs` | `HashSet` re-export, `hset!`, `into_hset!` |
| `src/collection/btree_map.rs` | `BTreeMap` re-export, `bmap!`, `into_bmap!` |
| `src/collection/btree_set.rs` | `BTreeSet` re-export, `bset!`, `into_bset!` |
| `src/collection/linked_list.rs` | `LinkedList` re-export, `llist!`, `into_llist!` |
| `src/collection/vec_deque.rs` | `VecDeque` re-export, `deque!`, `into_vecd!` |
| `src/collection/vector.rs` | `Vec` re-export, `vec!`, `dlist!`, `into_vec!`, `into_dlist!` |
| `src/collection/binary_heap.rs` | `BinaryHeap` re-export, `heap!`, `into_heap!` |
| `src/collection/mod.rs` | `count!` macro, namespace re-exports |

### Tests

| File | Relationship |
|------|-------------|
| `tests/inc/namespace_test.rs` | Accessibility of all types and macros from root and exposed modules |
| `tests/docs/api/01_collection_macros.md` | Test spec for this API |

### Features

| File | Relationship |
|------|-------------|
| `../feature/001_collection_constructors.md` | Strict macro usage guide |
| `../feature/002_into_constructors.md` | Into-based macro usage guide |

### Invariants

| File | Relationship |
|------|-------------|
| `../invariant/001_no_std_alloc.md` | Allocation source selection invariant |
| `../invariant/002_capacity_preallocated.md` | Capacity pre-allocation invariant |

### Abstract

The collection constructor macros provide ergonomic, variadic initialization of all standard Rust collections. Each macro accepts zero or more elements (or key-value pairs) and returns a fully initialized collection. Two classes exist: strict macros require homogeneous types; into-based macros coerce each element via `.into()` for heterogeneous initialization. For collection types that support `with_capacity` (Vec, HashMap, HashSet, VecDeque, BinaryHeap), macros pre-allocate exact capacity before any insertion. For types without `with_capacity` (BTreeMap, BTreeSet, LinkedList), macros use `new()` and insert directly. All macros are feature-gated and conditionally compiled.

### Operations

#### Strict Constructor Macros (feature = `collection_constructors`)

Strict macros require all arguments to share the same type. The collection type is inferred from the elements or from context.

| Macro | Collection | Arguments |
|-------|-----------|-----------|
| `vec!` | `Vec<T>` | zero or more elements |
| `hmap!` | `HashMap<K, V>` | zero or more `key => value` pairs |
| `hset!` | `HashSet<T>` | zero or more elements |
| `bmap!` | `BTreeMap<K, V>` | zero or more `key => value` pairs |
| `bset!` | `BTreeSet<T>` | zero or more elements |
| `llist!` | `LinkedList<T>` | zero or more elements |
| `deque!` | `VecDeque<T>` | zero or more elements |
| `heap!` | `BinaryHeap<T>` | zero or more elements (max-heap ordering) |
| `dlist!` | `Vec<T>` | permanent alias for `vec!` |

#### Into-based Constructor Macros (feature = `collection_into_constructors`)

Into-based macros call `.into()` on each element before insertion. The target element type `T` must be specified via type annotation; it cannot always be inferred.

| Macro | Collection | Arguments |
|-------|-----------|-----------|
| `into_vec!` | `Vec<T>` | zero or more elements, each coerced to T |
| `into_hmap!` | `HashMap<K, V>` | zero or more `key => value` pairs, each coerced to target type |
| `into_hset!` | `HashSet<T>` | zero or more elements, each coerced to T |
| `into_bmap!` | `BTreeMap<K, V>` | zero or more `key => value` pairs, each coerced to target type |
| `into_bset!` | `BTreeSet<T>` | zero or more elements, each coerced to T |
| `into_llist!` | `LinkedList<T>` | zero or more elements, each coerced to T |
| `into_vecd!` | `VecDeque<T>` | zero or more elements, each coerced to T |
| `into_heap!` | `BinaryHeap<T>` | zero or more elements, each coerced to T (max-heap ordering) |
| `into_dlist!` | `Vec<T>` | permanent alias for `into_vec!` |

#### Macro Expansion Contract

Every macro expands to a block expression that calls an appropriate initializer and then inserts each argument in order. Two patterns exist, determined by whether the collection type supports `with_capacity`:

**Pre-allocating macros** (Vec, HashMap, HashSet, VecDeque, BinaryHeap — 10 macros):

1. Compute capacity at compile time: `count!( $( $item ),* )` (or `count!( $( $key ),* )` for maps).
2. Call `CollectionType::with_capacity( cap )` to pre-allocate before any insertion.
3. Call `.insert( element )` (or `.push( element )` for `Vec`) for each argument in order.
4. Return the collection as the block value.

**Non-pre-allocating macros** (BTreeMap, BTreeSet, LinkedList — 6 macros):

1. Call `CollectionType::new()` — `with_capacity` is not available in std Rust for these types.
2. Call `.insert( element )` for each argument in order.
3. Return the collection as the block value.

For all into-based macros, each argument is wrapped in `.into()` before the insert call.

#### Collection Type Re-exports

All standard collections are re-exported through `collection_tools` with automatic source selection governed by feature flags. See `../invariant/001_no_std_alloc.md` for the switching rule.

| Collection Type | Source (std) | Source (no_std + use_alloc) |
|----------------|-------------|------------------------------|
| `HashMap` | standard library | `hashbrown` crate |
| `HashSet` | standard library | `hashbrown` crate |
| `Vec` | standard library | `alloc` crate |
| `BTreeMap` | standard library | `alloc` crate |
| `BTreeSet` | standard library | `alloc` crate |
| `LinkedList` | standard library | `alloc` crate |
| `VecDeque` | standard library | `alloc` crate |
| `BinaryHeap` | standard library | `alloc` crate |

### Error Handling

No runtime errors. All macros are purely syntactic expansions resolved at compile time. Type mismatches and missing `Into` implementations are compile-time errors. Capacity computation via `count!` is also compile-time; no allocation failure is possible at the macro call site.

### Compatibility Guarantees

- `dlist!` is a permanent alias for `vec!`; both names are stable.
- `into_dlist!` is a permanent alias for `into_vec!`; both names are stable.
- Trailing commas are accepted in all macros.
- Zero-element invocation (e.g., `hmap!()`) is valid and produces an empty pre-allocated collection with capacity 0.
- Re-exported types match the standard library API surface exactly; no wrapper types are introduced.
- The `collection_constructors` and `collection_into_constructors` features are independent and can be enabled or disabled separately.

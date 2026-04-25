# API: Collection Constructor Macros

### Scope

- **Purpose**: Document the complete public API for all variadic collection constructor macros.
- **Responsibility**: Define macro signatures, expansion contracts, and type re-export behavior.
- **In Scope**: 8 strict macros (`vec!`, `hmap!`, `hset!`, `bmap!`, `bset!`, `llist!`, `deque!`, `dlist!`); 8 into-based macros (`into_vec!`, `into_hmap!`, `into_hset!`, `into_bmap!`, `into_bset!`, `into_llist!`, `into_vecd!`, `into_dlist!`); unified collection type re-exports.
- **Out of Scope**: Feature-flag selection rationale (see `../feature/`); invariant proofs (see `../invariant/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/collection/hash_map.rs` | `HashMap` re-export, `hmap!`, `into_hmap!` |
| source | `src/collection/hash_set.rs` | `HashSet` re-export, `hset!`, `into_hset!` |
| source | `src/collection/btree_map.rs` | `BTreeMap` re-export, `bmap!`, `into_bmap!` |
| source | `src/collection/btree_set.rs` | `BTreeSet` re-export, `bset!`, `into_bset!` |
| source | `src/collection/linked_list.rs` | `LinkedList` re-export, `llist!`, `into_llist!` |
| source | `src/collection/vec_deque.rs` | `VecDeque` re-export, `deque!`, `into_vecd!` |
| source | `src/collection/vector.rs` | `Vec` re-export, `vec!`, `into_vec!` |
| source | `src/collection/binary_heap.rs` | `BinaryHeap` re-export |
| source | `src/collection/mod.rs` | `count!` macro, namespace re-exports |
| doc | `../feature/001_collection_constructors.md` | Strict macro usage guide |
| doc | `../feature/002_into_constructors.md` | Into-based macro usage guide |
| doc | `../invariant/001_no_std_alloc.md` | Allocation source selection invariant |
| doc | `../invariant/002_capacity_preallocated.md` | Capacity pre-allocation invariant |

### Abstract

The collection constructor macros provide ergonomic, variadic initialization of all standard Rust collections. Each macro accepts zero or more elements (or key-value pairs) and returns a fully initialized collection pre-allocated to the exact element count. Two classes exist: strict macros require homogeneous types; into-based macros coerce each element via `.into()` for heterogeneous initialization. All macros are feature-gated and conditionally compiled.

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
| `into_dlist!` | `Vec<T>` | permanent alias for `into_vec!` |

#### Macro Expansion Contract

Every macro expands to a block expression that:

1. Computes capacity at compile time: `count!( $( $item ),* )` (or `count!( $( $key ),* )` for maps).
2. Calls `CollectionType::with_capacity( cap )` to pre-allocate before any insertion.
3. Calls `.insert( element )` (or `.push( element )` for `Vec`) for each argument in order.
4. Returns the collection as the block value.

For into-based macros, each argument is wrapped in `.into()` before the insert call.

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

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Migrated; sections: Public API → Strict Constructor Macros, Public API → Into-based Constructor Macros, Public API → Collection Type Re-exports, Architecture → Macro Expansion Pattern; siblings: feature/001, feature/002, invariant/001, invariant/002 |

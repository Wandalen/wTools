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

Strict macros infer the element and key/value types from arguments. No type annotation is typically required when all elements share the same type:

```rust
use collection_tools::*;

let map = hmap! { "one" => 1, "two" => 2, "three" => 3 };
let set = hset! { 1, 2, 3, 4, 5 };
let bmap = bmap! { 1 => "one", 2 => "two" };

// Explicit annotation remains valid
let map : HashMap< &str, i32 > = hmap! { "one" => 1 };
```

#### Capacity Pre-allocation

Every macro pre-allocates the exact required capacity before inserting any element, using the `count!` macro at compile time. This avoids the common pitfall of starting with capacity 0 and triggering one or more reallocations during construction:

```rust
use collection_tools::*;

// Equivalent to HashMap::with_capacity(3) + 3 inserts — zero reallocations
let map = hmap! { 1 => "a", 2 => "b", 3 => "c" };

// vs. the standard approach which starts at capacity 0:
// let mut map = HashMap::new();
// map.insert(1, "a"); // may reallocate
```

#### Feature Gate

All strict macros are gated on `feature = "collection_constructors"`, which is enabled by default via the `enabled` meta-feature. When `collection_constructors` is absent from the active feature set, none of the strict macros are compiled.

### Sources

Migrated from `../../spec.md`. Sections contributing to this instance: "Overview → In-Scope → Variadic Constructor Macros (Strict)", "Architecture → Macro Expansion Pattern", "Usage Patterns → Pattern 1", "Usage Patterns → Pattern 4", "Usage Patterns → Pattern 5", "Design Rationale → Why Variadic Constructor Macros", "Adoption Guidelines". Sibling extractions: `../api/001_collection_macros.md`, `002_into_constructors.md`, `../invariant/001_no_std_alloc.md`, `../invariant/002_capacity_preallocated.md`.

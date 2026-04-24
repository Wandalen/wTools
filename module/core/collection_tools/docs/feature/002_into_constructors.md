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
| doc | `../api/001_collection_macros.md` | Complete macro signature contract |
| doc | `001_collection_constructors.md` | Strict-type counterpart |
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

Because `.into()` requires knowing the target type `T`, the compiler cannot always infer it from the arguments alone. Explicit type annotations are required in most cases:

```rust
use collection_tools::*;
use std::borrow::Cow;

// Type annotation required — compiler cannot infer String from mixed sources
let vec : Vec< String > = into_vec!( "&str", "String".to_string(), Cow::from( "Cow" ) );

// Key and value types both need annotation
let map : HashMap< String, String > = into_hmap!( "key" => "value", "key2".to_string() => "value2" );

// Without annotation, compilation fails:
// let vec = into_vec!( "a", "b" ); // Error: type annotations needed
```

#### Tradeoff vs Strict Macros

Into macros trade implicit type inference for flexibility. Prefer strict macros when all elements have the same type; use into macros when elements come from heterogeneous sources that share a common `Into<T>` target:

```rust
use collection_tools::*;

// Prefer strict — type is uniform, no annotation needed
let ids = hset! { 1u32, 2, 3 };

// Use into — sources are heterogeneous, annotation required
let labels : Vec< String > = into_vec!( "static", label_var, name.clone() );
```

#### Feature Gate

All into-based macros are gated on `feature = "collection_into_constructors"`, which is enabled by default. This feature is independent of `collection_constructors`; each can be enabled or disabled separately without affecting the other.

### Sources

Migrated from `../../spec.md`. Sections contributing to this instance: "Overview → In-Scope → Variadic Constructor Macros (Into-based)", "Architecture → Macro Expansion Pattern", "Usage Patterns → Pattern 2", "Usage Patterns → Pattern 6", "Design Rationale → Why Two Classes of Macros", "Adoption Guidelines". Sibling extractions: `../api/001_collection_macros.md`, `001_collection_constructors.md`, `../invariant/001_no_std_alloc.md`, `../invariant/002_capacity_preallocated.md`.

# Invariant: Capacity Pre-allocation

### Scope

- **Purpose**: Define the guarantee that every constructor macro pre-allocates exact capacity before any element is inserted.
- **Responsibility**: Document the `count!` macro contract and its role in zero-reallocation initialization.
- **In Scope**: The 10 macros covering capacity-supporting types (Vec, HashMap, HashSet, VecDeque, BinaryHeap — both strict and into variants); `count!` helper macro; compile-time element counting.
- **Out of Scope**: BTreeMap, BTreeSet, and LinkedList macros — these types have no `with_capacity` in std Rust and use `new()` instead; collection type source selection (see `001_no_std_alloc.md`); runtime capacity growth after construction.

### Sources

| File | Relationship |
|------|-------------|
| `src/collection/mod.rs` | `count!` macro definition |
| `src/collection/hash_map.rs` | `hmap!` and `into_hmap!` using `count!` |
| `src/collection/vector.rs` | `vec!` and `into_vec!` using `count!` |

### Tests

| File | Relationship |
|------|-------------|
| `tests/manual_corner_cases_test.rs` | Capacity assertions for Vec, HashMap, HashSet, VecDeque, BinaryHeap |
| `tests/docs/invariant/02_capacity_preallocated.md` | Test spec for this invariant |

### APIs

| File | Relationship |
|------|-------------|
| `../api/001_collection_macros.md` | Macro expansion contract referencing this invariant |

### Features

| File | Relationship |
|------|-------------|
| `../feature/001_collection_constructors.md` | Feature guide documenting capacity behavior |
| `../feature/002_into_constructors.md` | Into-based macros sharing this invariant |

### Invariants

| File | Relationship |
|------|-------------|
| `001_no_std_alloc.md` | Sibling invariant governing allocation source |

### Invariant Statement

The 10 constructor macros for capacity-supporting collection types — Vec, HashMap, HashSet, VecDeque, and BinaryHeap (both strict and into variants) — allocate storage for exactly N elements before the first insert call, where N is the number of arguments supplied at the macro call site. No reallocation occurs during macro expansion for these types.

The `count!` macro computes N at compile time by counting macro argument tokens. The expansion of each of these 10 macros always calls `CollectionType::with_capacity( count!( $( $item ),* ) )` before inserting elements.

BTreeMap, BTreeSet, and LinkedList macros (6 total) are excluded from this invariant — these types have no `with_capacity` in std Rust and their macros call `new()` instead.

### Enforcement Mechanism

Enforced by the macro expansion pattern itself. The `count!` helper produces a compile-time `usize` literal. Because the capacity argument to `with_capacity` is supplied by `count!` — not computed at runtime — the invariant holds for every invocation of the 10 in-scope macros. BTreeMap, BTreeSet, and LinkedList macros intentionally call `CollectionType::new()` since those types offer no `with_capacity` API.

### Violation Consequences

The invariant cannot be violated by caller code. It can only be broken by modifying the 10 macro definitions themselves to omit the `with_capacity` call. If violated, the resulting collection would reallocate one or more times during insertion, degrading performance but producing a correct result.

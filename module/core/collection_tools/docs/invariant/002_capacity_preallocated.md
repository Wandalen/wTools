# Invariant: Capacity Pre-allocation

### Scope

- **Purpose**: Define the guarantee that every constructor macro pre-allocates exact capacity before any element is inserted.
- **Responsibility**: Document the `count!` macro contract and its role in zero-reallocation initialization.
- **In Scope**: All 16 constructor macros (strict and into-based); `count!` helper macro; compile-time element counting.
- **Out of Scope**: Collection type source selection (see `001_no_std_alloc.md`); runtime capacity growth after construction.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/collection/mod.rs` | `count!` macro definition |
| source | `src/collection/hash_map.rs` | `hmap!` and `into_hmap!` using `count!` |
| source | `src/collection/vector.rs` | `vec!` and `into_vec!` using `count!` |
| doc | `../api/001_collection_macros.md` | Macro expansion contract referencing this invariant |
| doc | `../feature/001_collection_constructors.md` | Feature guide documenting capacity behavior |
| doc | `../feature/002_into_constructors.md` | Into-based macros sharing this invariant |
| doc | `001_no_std_alloc.md` | Sibling invariant governing allocation source |

### Invariant Statement

Every constructor macro provided by `collection_tools` — both strict and into-based — allocates storage for exactly N elements before the first `.insert()` call, where N is the number of arguments supplied at the macro call site. No reallocation occurs during macro expansion.

The `count!` macro computes N at compile time by counting macro argument tokens. The expansion of any collection macro always calls `CollectionType::with_capacity( count!( $( $item ),* ) )` before inserting elements.

### Enforcement Mechanism

Enforced by the macro expansion pattern itself. The `count!` helper produces a compile-time `usize` literal. Because the capacity argument to `with_capacity` is supplied by `count!` — not computed at runtime — the invariant holds for every invocation. There is no code path in any constructor macro that calls `CollectionType::new()` (which would give capacity 0).

### Violation Consequences

The invariant cannot be violated by caller code. It can only be broken by modifying the macro definitions themselves to omit the `with_capacity` call. If violated, the resulting collection would reallocate one or more times during insertion, degrading performance but producing a correct result.

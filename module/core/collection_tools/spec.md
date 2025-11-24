# Specification: collection_tools

## Overview

**collection_tools** provides ergonomic variadic constructor macros for standard Rust collections and unified collection type re-exports with automatic no_std support. It simplifies collection initialization and abstracts over `std::collections` vs `hashbrown` for `HashMap`/`HashSet` in no_std environments.

**Version:** 0.33.0
**Status:** Experimental
**Category:** Collection Utilities
**Dependents:** 9 crates (former, component_model, reflect_tools, implements, strs_tools, unilang, error_tools, meta_tools, typing_tools)

### Scope

#### Responsibility

Provide ergonomic variadic constructor macros for Rust collections and unified type re-exports with automatic no_std support, abstracting over `std::collections` and `hashbrown` based on feature flags.

#### In-Scope

1. **Variadic Constructor Macros (Strict)**
   - `vec!` - Create `Vec<T>` from elements
   - `hmap!` - Create `HashMap<K, V>` from key-value pairs
   - `hset!` - Create `HashSet<T>` from elements
   - `bmap!` - Create `BTreeMap<K, V>` from key-value pairs
   - `bset!` - Create `BTreeSet<T>` from elements
   - `llist!` - Create `LinkedList<T>` from elements
   - `deque!` - Create `VecDeque<T>` from elements
   - `dlist!` - Alias for `vec!` (dynamic list)

2. **Variadic Constructor Macros (Into-based)**
   - `into_vec!` - Create `Vec<T>` with `.into()` conversion
   - `into_hmap!` - Create `HashMap<K, V>` with `.into()` conversion
   - `into_hset!` - Create `HashSet<T>` with `.into()` conversion
   - `into_bmap!` - Create `BTreeMap<K, V>` with `.into()` conversion
   - `into_bset!` - Create `BTreeSet<T>` with `.into()` conversion
   - `into_llist!` - Create `LinkedList<T>` with `.into()` conversion
   - `into_vecd!` - Create `VecDeque<T>` with `.into()` conversion
   - `into_dlist!` - Alias for `into_vec!`

3. **Collection Type Re-exports**
   - `Vec` - From `alloc::vec::Vec` (use_alloc) or `std::vec::Vec`
   - `HashMap` - From `hashbrown::HashMap` (no_std + use_alloc) or `std::collections::HashMap`
   - `HashSet` - From `hashbrown::HashSet` (no_std + use_alloc) or `std::collections::HashSet`
   - `BTreeMap` - From `alloc::collections::BTreeMap` or `std::collections::BTreeMap`
   - `BTreeSet` - From `alloc::collections::BTreeSet` or `std::collections::BTreeSet`
   - `LinkedList` - From `alloc::collections::LinkedList` or `std::collections::LinkedList`
   - `VecDeque` - From `alloc::collections::VecDeque` or `std::collections::VecDeque`
   - `BinaryHeap` - From `alloc::collections::BinaryHeap` or `std::collections::BinaryHeap`

4. **Collection Module Re-exports**
   - `hash_map` module - HashMap-related items
   - `hash_set` module - HashSet-related items
   - `btree_map` module - BTreeMap-related items
   - `btree_set` module - BTreeSet-related items
   - `linked_list` module - LinkedList-related items
   - `vec_deque` module - VecDeque-related items
   - `vector` module - Vec-related items
   - `binary_heap` module - BinaryHeap-related items

5. **No-std Support**
   - `no_std` feature flag for embedded environments
   - `use_alloc` feature flag for allocation-dependent functionality
   - Conditional compilation for hashbrown vs std collections
   - Automatic selection of correct collection source

6. **Feature Architecture**
   - `enabled`: Master feature switch
   - `collection_constructors`: Enable strict constructor macros
   - `collection_into_constructors`: Enable into-based constructor macros
   - `full`: Enable all functionality

7. **Helper Macros**
   - `count!` macro for compile-time element counting
   - Used internally to pre-allocate correct capacity

#### Out-of-Scope

1. **NOT Custom Collection Implementations**
   - Does not provide novel collection types
   - Does not implement specialized data structures
   - **Rationale:** Focus on ergonomics for standard collections, not new data structures

2. **NOT Collection Algorithms**
   - Does not provide sorting, searching, or filtering functions
   - Does not provide collection transformation utilities
   - **Rationale:** Algorithms belong in iterator utilities or dedicated crates

3. **NOT Collection Traits**
   - Does not define custom collection traits
   - Does not provide trait implementations beyond re-exports
   - **Rationale:** Trait definitions are the responsibility of std/alloc/core

4. **NOT Type Conversions Beyond Into**
   - Does not provide From/TryFrom implementations for collections
   - Does not provide conversion between collection types
   - **Rationale:** Conversions are provided by std library

5. **NOT Collection Validation**
   - Does not validate collection constraints
   - Does not provide bounded collections
   - **Rationale:** Validation is application-specific

6. **NOT Persistent Collections**
   - Does not provide immutable/persistent data structures
   - Does not implement copy-on-write collections
   - **Rationale:** Persistent collections require specialized implementations

7. **NOT Concurrent Collections**
   - Does not provide thread-safe collections
   - Does not implement lock-free data structures
   - **Rationale:** Concurrency is handled by dedicated crates

8. **NOT Collection Serialization**
   - Does not provide serialization/deserialization
   - Does not integrate with serde
   - **Rationale:** Serialization is handled by serde crate

#### Boundaries

- **collection_tools vs std::collections**: collection_tools provides macros and unified re-exports, std provides the implementations
- **collection_tools vs hashbrown**: collection_tools conditionally re-exports hashbrown for no_std HashMap/HashSet
- **collection_tools vs iter_tools**: iter_tools provides iteration utilities, collection_tools provides collection construction
- **Strict vs Into macros**: Strict macros require homogeneous types, Into macros allow heterogeneous types with Into trait

## Architecture

### Dependency Structure

```
collection_tools (no_std compatible)
└── External Dependencies
    └── hashbrown (optional, for no_std HashMap/HashSet)
```

### Feature-Based Collection Source

```
HashMap/HashSet:
├── no_std + use_alloc → hashbrown::HashMap/HashSet
└── std (default)      → std::collections::HashMap/HashSet

Other Collections:
├── use_alloc → alloc::collections::{BTreeMap, BTreeSet, LinkedList, VecDeque, BinaryHeap}
├── use_alloc → alloc::vec::Vec
└── std       → std::collections::{BTreeMap, BTreeSet, LinkedList, VecDeque, BinaryHeap}
```

### Module Organization

```
collection_tools
├── lib.rs (traditional namespaces: own, orphan, exposed, prelude)
└── collection/
    ├── mod.rs (count! macro, namespace re-exports)
    ├── hash_map.rs (HashMap re-export, hmap!, into_hmap!)
    ├── hash_set.rs (HashSet re-export, hset!, into_hset!)
    ├── btree_map.rs (BTreeMap re-export, bmap!, into_bmap!)
    ├── btree_set.rs (BTreeSet re-export, bset!, into_bset!)
    ├── linked_list.rs (LinkedList re-export, llist!, into_llist!)
    ├── vec_deque.rs (VecDeque re-export, deque!, into_vecd!)
    ├── vector.rs (Vec re-export, vec!, into_vec!)
    └── binary_heap.rs (BinaryHeap re-export, macros)
```

### Feature Architecture

```
enabled (master switch)
├── collection_constructors (strict macros)
└── collection_into_constructors (into-based macros)

no_std (embedded support)
└── use_alloc (requires alloc, enables hashbrown)
```

**Default Features:** `enabled`, `collection_constructors`, `collection_into_constructors`

### Macro Expansion Pattern

Strict macro example (`hmap!`):

```rust
hmap!( "one" => 1, "two" => 2 )

// Expands to:
{
  let _cap = 2; // count! macro
  let mut _map = HashMap::with_capacity(_cap);
  _map.insert("one", 1);
  _map.insert("two", 2);
  _map
}
```

Into-based macro example (`into_hmap!`):

```rust
into_hmap!( "one" => 1, "two" => 2 )

// Expands to:
{
  let _cap = 2;
  let mut _map = HashMap::with_capacity(_cap);
  _map.insert("one".into(), 1.into());
  _map.insert("two".into(), 2.into());
  _map
}
```

## Public API

### Strict Constructor Macros

```rust
#[cfg(feature = "collection_constructors")]
/// Create Vec from elements
pub macro vec!( $( $item:expr ),* $(,)? );

#[cfg(feature = "collection_constructors")]
/// Create HashMap from key-value pairs
pub macro hmap!( $( $key:expr => $value:expr ),* $(,)? );

#[cfg(feature = "collection_constructors")]
/// Create HashSet from elements
pub macro hset!( $( $item:expr ),* $(,)? );

#[cfg(feature = "collection_constructors")]
/// Create BTreeMap from key-value pairs
pub macro bmap!( $( $key:expr => $value:expr ),* $(,)? );

#[cfg(feature = "collection_constructors")]
/// Create BTreeSet from elements
pub macro bset!( $( $item:expr ),* $(,)? );

#[cfg(feature = "collection_constructors")]
/// Create LinkedList from elements
pub macro llist!( $( $item:expr ),* $(,)? );

#[cfg(feature = "collection_constructors")]
/// Create VecDeque from elements
pub macro deque!( $( $item:expr ),* $(,)? );
```

### Into-based Constructor Macros

```rust
#[cfg(feature = "collection_into_constructors")]
/// Create Vec with .into() conversion
pub macro into_vec!( $( $item:expr ),* $(,)? );

#[cfg(feature = "collection_into_constructors")]
/// Create HashMap with .into() conversion
pub macro into_hmap!( $( $key:expr => $value:expr ),* $(,)? );

#[cfg(feature = "collection_into_constructors")]
/// Create HashSet with .into() conversion
pub macro into_hset!( $( $item:expr ),* $(,)? );

#[cfg(feature = "collection_into_constructors")]
/// Create BTreeMap with .into() conversion
pub macro into_bmap!( $( $key:expr => $value:expr ),* $(,)? );

#[cfg(feature = "collection_into_constructors")]
/// Create BTreeSet with .into() conversion
pub macro into_bset!( $( $item:expr ),* $(,)? );

#[cfg(feature = "collection_into_constructors")]
/// Create LinkedList with .into() conversion
pub macro into_llist!( $( $item:expr ),* $(,)? );

#[cfg(feature = "collection_into_constructors")]
/// Create VecDeque with .into() conversion
pub macro into_vecd!( $( $item:expr ),* $(,)? );
```

### Collection Type Re-exports

```rust
// Conditional HashMap/HashSet re-export
#[cfg(all(feature = "no_std", feature = "use_alloc"))]
pub use hashbrown::{HashMap, HashSet};

#[cfg(not(feature = "no_std"))]
pub use std::collections::{HashMap, HashSet};

// Other collections (from alloc or std)
#[cfg(feature = "use_alloc")]
pub use alloc::collections::{BTreeMap, BTreeSet, LinkedList, VecDeque, BinaryHeap};
#[cfg(feature = "use_alloc")]
pub use alloc::vec::Vec;

#[cfg(not(feature = "no_std"))]
pub use std::collections::{BTreeMap, BTreeSet, LinkedList, VecDeque, BinaryHeap};
#[cfg(not(feature = "no_std"))]
pub use std::vec::Vec;
```

## Usage Patterns

### Pattern 1: Strict Constructor Macros (Homogeneous Types)

```rust
use collection_tools::*;

// HashMap with same types
let map = hmap! { "one" => 1, "two" => 2, "three" => 3 };
assert_eq!(map.get("one"), Some(&1));

// HashSet with same types
let set = hset! { 1, 2, 3, 4, 5 };
assert_eq!(set.contains(&3), true);

// BTreeMap
let bmap = bmap! { 1 => "one", 2 => "two" };

// BTreeSet
let bset = bset! { "apple", "banana", "cherry" };

// LinkedList
let llist = llist! { 1, 2, 3 };

// VecDeque
let deque = deque! { "a", "b", "c" };
```

### Pattern 2: Into-based Constructor Macros (Heterogeneous Types)

```rust
use collection_tools::*;
use std::borrow::Cow;

// Vec with heterogeneous types converted via Into
let vec: Vec<String> = into_vec!( "&str", "String".to_string(), Cow::from("Cow") );

// HashMap with Into conversion
let map: HashMap<String, String> = into_hmap!( "key" => "value", "key2".to_string() => "value2" );

// HashSet with Into conversion
let set: HashSet<String> = into_hset!( "one", "two".to_string(), Cow::from("three") );
```

### Pattern 3: No-std Collection Usage

```rust
// In no_std environment with use_alloc feature
#![no_std]
extern crate alloc;

use collection_tools::{HashMap, HashSet}; // Uses hashbrown

let mut map: HashMap<i32, &str> = HashMap::new();
map.insert(1, "one");

let mut set: HashSet<i32> = HashSet::new();
set.insert(1);
```

### Pattern 4: Pre-allocated Collections

```rust
use collection_tools::*;

// Macros automatically pre-allocate correct capacity
let map = hmap! { 1 => "a", 2 => "b", 3 => "c" }; // Capacity: 3
let set = hset! { 1, 2, 3, 4, 5 }; // Capacity: 5

// More efficient than manual construction:
// let mut map = HashMap::new(); // Capacity: 0, will reallocate
```

### Pattern 5: Type Inference with Strict Macros

```rust
use collection_tools::*;

// Type inferred from context
let map = hmap! { 1 => 2, 3 => 4 }; // HashMap<i32, i32>

// Explicit type annotation
let map: HashMap<&str, i32> = hmap! { "one" => 1 };
```

### Pattern 6: Type Specification with Into Macros

```rust
use collection_tools::*;

// Into macros often require type specification
let vec: Vec<String> = into_vec!( "a", "b", "c" );
let set: HashSet<String> = into_hset!( "x", "y", "z" );

// Without type annotation, may fail to compile:
// let vec = into_vec!( "a", "b" ); // Error: type annotations needed
```

## Dependencies and Consumers

### Direct Dependencies

**External:**
- `hashbrown` (workspace, optional) - no_std HashMap/HashSet implementation

### Consumers (9 crates)

1. **former** - Collection initialization in builder patterns
2. **component_model** - Component collections
3. **reflect_tools** - Reflection over collections
4. **implements** - Implementation detection collections
5. **strs_tools** - String processing collections
6. **unilang** - Language construct collections
7. **error_tools** - Error collections
8. **meta_tools** - Meta-programming collections
9. **typing_tools** - Type-related collections

## Design Rationale

### Why Variadic Constructor Macros?

Rust's standard library lacks ergonomic collection initialization:

```rust
// Standard approach (verbose)
let mut map = HashMap::new();
map.insert("one", 1);
map.insert("two", 2);
map.insert("three", 3);

// With hmap! (concise)
let map = hmap! { "one" => 1, "two" => 2, "three" => 3 };
```

**Benefits:**
1. **Conciseness**: Single expression instead of multiple statements
2. **Capacity Optimization**: Pre-allocates correct capacity
3. **Ergonomics**: Familiar syntax similar to other languages
4. **Type Safety**: Compile-time type checking

### Why Two Classes of Macros (Strict vs Into)?

**Strict Macros:**
- Require homogeneous types
- No type annotations needed (usually)
- More intuitive for simple cases
- Example: `hmap! { 1 => 2, 3 => 4 }`

**Into Macros:**
- Allow heterogeneous types via `.into()`
- Require type annotations (usually)
- More flexible for complex cases
- Example: `into_vec!( "str", String::new() )` → `Vec<String>`

**Tradeoff:** Two macros = more API surface, but better ergonomics for different use cases.

### Why Conditional hashbrown Dependency?

`std::collections::HashMap` and `HashSet` are not available in no_std environments because they depend on randomization and system entropy. The `hashbrown` crate provides no_std-compatible implementations.

**Automatic Selection:**
- `no_std + use_alloc` → `hashbrown::HashMap/HashSet`
- `std` → `std::collections::HashMap/HashSet`

**Benefits:**
1. **Portability**: Same API works in std and no_std
2. **Transparency**: Users don't need to know which implementation is used
3. **Consistency**: Same collection types across wTools ecosystem

### Why count! Macro for Capacity?

The `count!` macro counts elements at compile time to pre-allocate correct capacity:

```rust
let map = hmap! { 1 => "a", 2 => "b", 3 => "c" };
// Equivalent to:
let mut map = HashMap::with_capacity(3); // Pre-allocated!
```

**Benefits:**
1. **Performance**: Avoids reallocations
2. **Compile-time**: No runtime overhead
3. **Automatic**: Users don't need to specify capacity

### Why Not Use vec! from std?

The standard `vec!` macro is re-exported for consistency with other collection macros and to provide a unified API surface in `collection_tools`.

**Note:** `collection_tools::vec!` is identical to `std::vec!` but available through the same import path as other collection macros.

## Testing Strategy

### Test Coverage

- **Example Programs**: 1 comprehensive example (`collection_tools_trivial.rs`)
- **Doc Tests**: Extensive doc tests in each collection module
- **Feature Matrix**: Tests for no_std, use_alloc, and std configurations

### Test Focus

1. **Macro Functionality**: Verify each macro creates correct collection
2. **Capacity Pre-allocation**: Verify correct capacity is set
3. **Into Conversion**: Verify Into macros call .into()
4. **No-std Compatibility**: Verify hashbrown is used in no_std
5. **Type Inference**: Verify type inference works correctly

## Future Considerations

### Potential Enhancements

1. **Additional Macros**: Constructor macros for other collection types
2. **Builder Patterns**: Fluent API for collection construction
3. **Const Constructors**: Const-compatible collection initialization
4. **More Into Variants**: Additional conversion-based constructors

### Breaking Changes to Consider

1. **Macro Names**: Shorter names (e.g., `m!` instead of `hmap!`)
2. **Feature Structure**: Consolidate constructor features
3. **Collection Re-exports**: Add more specialized collections

### Known Limitations

1. **Type Annotation**: Into macros often require type annotations
2. **Circular Dependencies**: Cannot use test_tools (creates circular dependency via component_model_types)
3. **Limited Collections**: Only standard library collections supported

## Adoption Guidelines

### When to Use Strict Macros

- All elements have the same type
- Type can be inferred from context
- Simple, straightforward initialization

### When to Use Into Macros

- Elements have different types that implement Into
- Need type conversion during initialization
- Building collections from heterogeneous sources

### Best Practices

1. **Prefer Strict Macros**: Use strict macros when possible for better type inference
2. **Annotate Into Macros**: Always specify type for into_ macros
3. **Trailing Commas**: Use trailing commas for multi-line collections
4. **Consistent Style**: Use same macro style within a module

## Related Crates

- **hashbrown**: External dependency for no_std HashMap/HashSet
- **std::collections**: Standard library collections (re-exported)
- **alloc::collections**: Allocation-based collections for no_std (re-exported)
- **iter_tools**: Iterator utilities (complementary crate)

## References

- [API Documentation](https://docs.rs/collection_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/collection_tools)
- [hashbrown Documentation](https://docs.rs/hashbrown)
- [readme.md](./readme.md)

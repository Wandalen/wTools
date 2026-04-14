# Specification: reflect_tools

## Overview

**reflect_tools** is a runtime type reflection system providing traits, types, and utilities for dynamic type inspection and manipulation of Rust entities. It enables introspection of type names, sizes, elements, and structure at runtime, facilitating serialization frameworks, dynamic ORMs, generic algorithms on heterogeneous collections, and plugin architectures that require type information unavailable at compile time.

**Version:** 0.7.0
**Status:** Experimental
**Category:** Development Tools (Reflection / Type Introspection)
**Dependents:** Unknown (likely workspace crates requiring dynamic type handling)

### Scope

#### Responsibility

Provide a comprehensive runtime reflection system with traits (Instance, Entity, IsContainer, IsScalar), descriptors (EntityDescriptor, CollectionDescriptor, KeyedCollectionDescriptor), and primitive type representation (Primitive enum), enabling dynamic type inspection, element iteration, and type-safe manipulation of heterogeneous entity collections at runtime.

#### In-Scope

1. **Instance Trait (Reflectable Types)**
   - `Instance` trait - Marker for reflectable types
   - `_reflect(&self)` - Instance-level reflection
   - `Reflect()` - Type-level reflection
   - Associated `Entity` type
   - Automatic implementation via `InstanceMarker`

2. **Entity Trait (Reflection Descriptors)**
   - `Entity` trait - Reflection capability interface
   - `type_name()` / `type_id()` - Type identification
   - `is_container()` / `is_ordered()` - Type classification
   - `len()` - Element count
   - `elements()` - Iterator over key-value pairs
   - `element(i)` - Access specific element

3. **Type Descriptors**
   - `EntityDescriptor<I>` - Sized type descriptor
   - `CollectionDescriptor<I>` - Dynamic collection descriptor (Vec, slice, array)
   - `KeyedCollectionDescriptor<I>` - Key-value collection descriptor (HashMap, HashSet)
   - Runtime size tracking
   - PhantomData-based type association

4. **Primitive Type System**
   - `Primitive` enum - All primitive types
   - Variants: i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64
   - String types: String, &'static str
   - Binary: &'static [u8]
   - `From<T>` implementations for all primitives

5. **Key-Value Representation**
   - `KeyVal` struct - (key, value) pair
   - `key: Primitive` - Generic key type
   - `val: Box<dyn Entity>` - Type-erased value
   - Used for element iteration
   - PartialEq implementation with recursive comparison

6. **Marker Traits**
   - `IsContainer` - Container type marker (Vec, HashMap, array, etc.)
   - `IsScalar` - Scalar type marker (i32, String, etc.)
   - `InstanceMarker` - Internal auto-impl enabler

7. **Built-in Entity Implementations**
   - Primitives: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
   - Strings: String, &'static str
   - References: &T
   - Arrays: [T; N]
   - Slices: &'static [T]
   - Collections: Vec<T>, HashMap<K, V>, HashSet<V>

8. **Reflection Entry Point**
   - `reflect(&impl Instance)` - Runtime reflection function
   - Distinguishes T from &T
   - Returns `impl Entity` descriptor
   - Primary API surface

9. **Feature Architecture**
   - `enabled` - Master switch (default)
   - `reflect_types` - Type reflection system (default)
   - `reflect_newtype` - Newtype pattern support (default)
   - Granular feature control

10. **Traditional Namespace Organization**
    - Standard namespaces: own, orphan, exposed, prelude
    - Dependency namespace for proc macros
    - Submodule organization (axiomatic, primitive, entity_vec, entity_array, etc.)

#### Out-of-Scope

1. **NOT Procedural Derive (in reflect_tools)**
   - Proc macro in reflect_tools_meta
   - `#[derive(Reflect)]` for custom types
   - **Rationale:** Separate proc macro crate required

2. **NOT Compile-Time Reflection**
   - No const reflection
   - Runtime only
   - **Rationale:** Use typing_tools for compile-time checks

3. **NOT Value Mutation**
   - Inspection only, not modification
   - No `set_element()` or similar
   - **Rationale:** Type-safe mutation complex, out of scope

4. **NOT Automatic Serialization**
   - Reflection primitives only
   - No built-in JSON/binary formats
   - **Rationale:** Use serde or build custom serializers on top

5. **NOT Type Construction**
   - No dynamic type creation
   - No runtime type building
   - **Rationale:** Rust type system limitations

6. **NOT Method Reflection**
   - No method introspection
   - Only data reflection
   - **Rationale:** Complex, requires AST access

7. **NOT Trait Object Introspection**
   - No vtable inspection
   - No dyn trait element access
   - **Rationale:** Language limitations

8. **NOT Custom Allocator Support**
   - Uses std collections
   - No allocator generics
   - **Rationale:** Complexity, limited use case

#### Boundaries

- **reflect_tools vs typing_tools**: reflect_tools runtime; typing_tools compile-time
- **reflect_tools vs serde**: reflect_tools low-level primitives; serde full serialization framework
- **reflect_tools vs std::any**: reflect_tools structured inspection; std::any type erasure only

## Architecture

### Dependency Structure

```
reflect_tools (reflection runtime)
├── Internal Dependencies (workspace)
│   ├── reflect_tools_meta (proc macro, #[derive(Reflect)])
│   ├── derive_tools (From, InnerFrom derives)
│   └── collection_tools (collection utilities)
└── Dev Dependencies
    ├── test_tools (workspace, testing)
    └── collection_tools (extended features, constructors)

reflect_tools_meta (proc macro support)
├── Internal Dependencies (workspace)
│   └── macro_tools (AST utilities, enabled feature)
└── Dev Dependencies
    └── test_tools (workspace, testing)
```

**Note:** Two-crate pattern: runtime + proc macro

### Module Organization

```
reflect_tools
├── lib.rs (top-level aggregation)
├── reflect.rs - Main reflection module
│   ├── axiomatic.rs - Core traits (Instance, Entity, IsContainer, IsScalar)
│   │   ├── Instance trait + blanket impl
│   │   ├── Entity trait + methods
│   │   ├── EntityDescriptor<I>
│   │   ├── CollectionDescriptor<I>
│   │   ├── KeyedCollectionDescriptor<I>
│   │   ├── KeyVal struct
│   │   ├── reflect() function
│   │   └── InstanceMarker impls for primitives
│   ├── primitive.rs - Primitive enum and From impls
│   ├── entity_array.rs - [T; N] Entity impl
│   ├── entity_slice.rs - &[T] Entity impl
│   ├── entity_vec.rs - Vec<T> Entity impl
│   ├── entity_hashmap.rs - HashMap<K, V> Entity impl
│   ├── entity_hashset.rs - HashSet<V> Entity impl
│   ├── fields/ - Field reflection utilities
│   │   ├── vec.rs, deque.rs, hmap.rs, hset.rs, llist.rs
│   │   └── Collection-specific field access
│   └── wrapper/ - Wrapper types
│       ├── aref.rs - Reference wrappers
│       ├── maybe_as.rs - Optional conversion
│       └── optional_cow.rs - Copy-on-write wrappers
└── Standard namespaces: own, orphan, exposed, prelude

reflect_tools_meta
├── lib.rs (proc macro entry)
└── implementation/reflect.rs - Proc macro impl
    └── #[derive(Reflect)] - Custom type reflection
```

**Pattern:** Runtime + proc macro crate separation

### Feature Architecture

```
enabled (master switch, default)
│
├── reflect_types (type reflection system, default)
│   └── Core reflection functionality
│
└── reflect_newtype (newtype pattern, default)
    └── Newtype wrapper support

full (all features)
```

**Default Features:** `enabled`, `reflect_types`, `reflect_newtype`

### Reflection Flow

#### reflect() Function Flow

```
let vec = vec![1, 2, 3];
reflect(&vec)
  ↓
Calls vec._reflect()
  ↓
Returns CollectionDescriptor<Vec<i32>>::new(vec.len())
  ↓
CollectionDescriptor implements Entity
  ↓
Can call:
  - .type_name() -> "alloc::vec::Vec<i32>"
  - .type_id() -> TypeId::of::<Vec<i32>>()
  - .is_container() -> true
  - .len() -> 3
  - .elements() -> Iterator<Item=KeyVal>
```

#### Entity Descriptor System

```
Type Hierarchy:
  Instance (trait)
    ├─ Associated type Entity: Entity
    ├─ _reflect(&self) -> Entity
    └─ Reflect() -> Entity (type-level)

  Entity (trait)
    ├─ type_name() -> &'static str
    ├─ type_id() -> TypeId
    ├─ is_container() -> bool
    ├─ is_ordered() -> bool
    ├─ len() -> usize
    ├─ elements() -> Box<dyn Iterator<Item=KeyVal>>
    └─ element(i) -> KeyVal

  Descriptors (implement Entity):
    ├─ EntityDescriptor<T> (for scalars)
    ├─ CollectionDescriptor<T> (for Vec, array, slice)
    └─ KeyedCollectionDescriptor<T> (for HashMap, HashSet)
```

## Public API

### Core Reflection Function

```rust
/// Provides reflection of an instance
///
/// Distinguishes between instances and references
pub fn reflect(src: &impl Instance) -> impl Entity
```

### Instance Trait

```rust
/// Trait for types that can be reflected
pub trait Instance {
  /// The entity descriptor type
  type Entity: Entity;

  /// Instance-level reflection (returns descriptor)
  fn _reflect(&self) -> Self::Entity {
    Self::Reflect()
  }

  /// Type-level reflection (static descriptor)
  #[allow(non_snake_case)]
  fn Reflect() -> Self::Entity;
}

// Blanket impl for all types with InstanceMarker
impl<T> Instance for T
where
  EntityDescriptor<T>: Entity,
  T: InstanceMarker,
{
  type Entity = EntityDescriptor<Self>;
  fn Reflect() -> Self::Entity {
    EntityDescriptor::<Self>::new()
  }
}
```

### Entity Trait

```rust
/// Runtime reflection capability interface
pub trait Entity: core::fmt::Debug {
  /// Is this entity a container?
  fn is_container(&self) -> bool { false }

  /// Are elements in predictable order?
  fn is_ordered(&self) -> bool { true }

  /// Number of elements
  fn len(&self) -> usize { 0 }

  /// Type name (e.g., "i32", "Vec<String>")
  fn type_name(&self) -> &'static str;

  /// Type ID for runtime type checking
  fn type_id(&self) -> core::any::TypeId;

  /// Iterator over elements as key-value pairs
  fn elements(&self) -> Box<dyn Iterator<Item = KeyVal>> {
    Box::new([].into_iter())
  }

  /// Access specific element by index
  fn element(&self, i: usize) -> KeyVal {
    debug_assert!(i < self.len());
    self.elements().skip(i).next().unwrap()
  }
}
```

### Type Descriptors

```rust
/// Type descriptor for sized types
#[derive(PartialEq, Default, Clone)]
pub struct EntityDescriptor<I: Instance> {
  _phantom: core::marker::PhantomData<I>,
}

impl<I: Instance> EntityDescriptor<I> {
  pub fn new() -> Self {
    Self { _phantom: core::marker::PhantomData }
  }
}

/// Descriptor for dynamic collections (Vec, slice, array)
#[derive(PartialEq, Default, Clone)]
pub struct CollectionDescriptor<I: Instance> {
  pub len: usize,
  _phantom: core::marker::PhantomData<I>,
}

impl<I: Instance> CollectionDescriptor<I> {
  pub fn new(size: usize) -> Self {
    Self {
      _phantom: core::marker::PhantomData,
      len: size,
    }
  }
}

/// Descriptor for key-value collections (HashMap, HashSet)
#[derive(PartialEq, Default, Clone)]
pub struct KeyedCollectionDescriptor<I: Instance> {
  pub len: usize,
  pub keys: Vec<Primitive>,
  _phantom: core::marker::PhantomData<I>,
}
```

### Primitive Type

```rust
/// General-purpose primitive data container
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Default, Clone)]
pub enum Primitive {
  #[default]
  None,
  i8(i8), i16(i16), i32(i32), i64(i64), isize(isize),
  u8(u8), u16(u16), u32(u32), u64(u64), usize(usize),
  f32(f32), f64(f64),
  String(String),
  str(&'static str),
  binary(&'static [u8]),
}

// From<T> implementations for all variants
impl From<i32> for Primitive {
  fn from(value: i32) -> Self { Self::i32(value) }
}
// ... (similar for all primitives)
```

### Key-Value Pair

```rust
/// Key-value pair for element reflection
pub struct KeyVal {
  /// Element key (index for Vec, hash key for HashMap)
  pub key: Primitive,
  /// Element value descriptor
  pub val: Box<dyn Entity>,
}

impl PartialEq for KeyVal {
  fn eq(&self, other: &Self) -> bool {
    // Compares key, type_id, type_name, len, and recursively elements
  }
}
```

### Marker Traits

```rust
/// Marker for container types
pub trait IsContainer: Instance {}

/// Marker for scalar types
pub trait IsScalar: Instance {}
```

## Usage Patterns

### Pattern 1: Basic Type Reflection

```rust
use reflect_tools::reflect::{reflect, Entity};

let num = 42i32;
let reflected = reflect(&num);

println!("Type: {}", reflected.type_name()); // "i32"
println!("Container: {}", reflected.is_container()); // false
println!("Length: {}", reflected.len()); // 0
```

### Pattern 2: Vec Reflection and Iteration

```rust
use reflect_tools::reflect::{reflect, Entity, KeyVal, Primitive, Instance};

let vec = vec![1, 2, 3];
let reflected = reflect(&vec);

assert_eq!(reflected.type_name(), "alloc::vec::Vec<i32>");
assert_eq!(reflected.is_container(), true);
assert_eq!(reflected.len(), 3);

// Iterate over elements
for (idx, kv) in reflected.elements().enumerate() {
  println!("Element {}: {:?}", idx, kv);
}

// Access specific element
let elem = reflected.element(1);
assert_eq!(elem.key, Primitive::usize(1));
```

### Pattern 3: HashMap Reflection

```rust
use reflect_tools::reflect::{reflect, Entity};
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert(1, "one");
map.insert(2, "two");

let reflected = reflect(&map);

assert_eq!(reflected.is_container(), true);
assert_eq!(reflected.is_ordered(), false); // HashMap is unordered
assert_eq!(reflected.len(), 2);
```

### Pattern 4: Type Comparison

```rust
use reflect_tools::reflect::{reflect, Entity};
use core::any::TypeId;

let vec_i32 = vec![1, 2, 3];
let vec_str = vec!["a", "b"];

let r1 = reflect(&vec_i32);
let r2 = reflect(&vec_str);

assert_ne!(r1.type_id(), r2.type_id());
assert!(r1.type_name().contains("i32"));
assert!(r2.type_name().contains("str"));
```

### Pattern 5: Generic Algorithm on Heterogeneous Collections

```rust
use reflect_tools::reflect::{reflect, Entity};

fn print_collection_info(value: &impl Instance) {
  let r = reflect(value);
  println!("Type: {}", r.type_name());
  println!("Container: {}", r.is_container());
  println!("Length: {}", r.len());
  println!("Ordered: {}", r.is_ordered());
}

print_collection_info(&vec![1, 2, 3]);
print_collection_info(&"hello");
print_collection_info(&42);
```

### Pattern 6: Primitive Type Wrapping

```rust
use reflect_tools::reflect::Primitive;

let values = vec![
  Primitive::i32(42),
  Primitive::String("hello".to_string()),
  Primitive::f64(3.14),
];

for val in values {
  println!("{:?}", val);
}
```

### Pattern 7: Custom Type Reflection (with derive)

```rust
use reflect_tools::reflect::{reflect, Entity};
use reflect_tools::Reflect;

#[derive(Reflect)]
struct Person {
  name: String,
  age: u32,
}

let person = Person {
  name: "Alice".to_string(),
  age: 30,
};

let reflected = reflect(&person);
println!("Type: {}", reflected.type_name());
// Note: Full struct field reflection requires additional implementation
```

### Pattern 8: Dynamic Serialization Foundation

```rust
use reflect_tools::reflect::{reflect, Entity, Instance};

fn serialize_to_json(value: &impl Instance) -> String {
  let r = reflect(value);

  if r.is_container() {
    let mut result = String::from("[");
    for (idx, kv) in r.elements().enumerate() {
      if idx > 0 { result.push_str(", "); }
      // Recursively serialize elements
      result.push_str(&format!("{:?}", kv.val));
    }
    result.push(']');
    result
  } else {
    format!("\"{}\"", r.type_name())
  }
}
```

## Dependencies and Consumers

### Direct Dependencies

**Workspace:**
- `reflect_tools_meta` (v0.7.0) - Proc macro for #[derive(Reflect)]
- `derive_tools` (derive_from, derive_inner_from features) - Derive utilities
- `collection_tools` - Collection utilities

**Dev:**
- `test_tools` (workspace, full features) - Testing utilities
- `collection_tools` (extended features) - Collection constructors

### reflect_tools_meta Dependencies

**Workspace:**
- `macro_tools` (default features) - AST utilities (syn, quote, proc-macro2)

### Consumers (Unknown)

**Likely used by:**
- Serialization frameworks
- Dynamic ORMs
- Plugin systems
- Generic data processing pipelines
- Type-generic algorithms
- Runtime type validation

**Usage Pattern:** Workspace crates use reflect_tools for runtime type inspection when compile-time type information insufficient for dynamic data handling.

## Design Rationale

### Why Runtime Reflection?

Rust lacks built-in runtime reflection:

**Rationale:**
1. **Dynamic Systems**: Serialization, ORM require runtime type info
2. **Plugin Architectures**: Unknown types at compile time
3. **Generic Algorithms**: Heterogeneous collection processing
4. **Interoperability**: Dynamic language bridges, FFI

**Tradeoff:** Runtime cost vs compile-time type safety

### Why Instance and Entity Separation?

Two-trait design (Instance, Entity):

**Rationale:**
1. **Type vs Value**: Instance is type, Entity is descriptor
2. **Static vs Dynamic**: Instance at compile-time, Entity at runtime
3. **Distinguish T from &T**: reflect() prevents &T/T confusion
4. **Flexibility**: Different descriptors for different types

**Pattern:** Instance marks types, Entity provides capabilities

### Why Three Descriptor Types?

EntityDescriptor, CollectionDescriptor, KeyedCollectionDescriptor:

**Rationale:**
1. **Size Tracking**: Collections need len field
2. **Key Management**: Keyed collections track keys
3. **Type Safety**: Different descriptors for different semantics
4. **Efficiency**: Only store needed information

**Benefit:** Precise descriptor types for each use case

### Why Primitive Enum?

Unified enum for primitive types:

**Rationale:**
1. **Type Erasure**: Box primitive values uniformly
2. **KeyVal Compatibility**: Consistent key representation
3. **Serialization**: Easy primitive handling
4. **Completeness**: All primitive types covered

**Alternative:** Could use Box<dyn Any>, but less ergonomic

### Why Iterator<Item=KeyVal>?

Elements as key-value pairs:

**Rationale:**
1. **Unified Interface**: Arrays (index) and maps (key) both work
2. **Type Information**: Each element has type descriptor
3. **Recursive Reflection**: Elements themselves are Entity
4. **Flexibility**: Works with any container type

**Pattern:** Consistent element access across containers

### Why is_ordered() Method?

Distinguish ordered vs unordered containers:

**Rationale:**
1. **Semantic Information**: Vec vs HashMap iteration semantics
2. **Algorithm Choice**: Some algorithms require ordering
3. **Documentation**: Clear contract about element sequence
4. **Serialization**: Ordering affects output format

**Example:** Vec is ordered, HashMap is not

### Why Separate Proc Macro Crate?

reflect_tools + reflect_tools_meta:

**Rationale:**
1. **Proc Macro Requirements**: Separate crate needed
2. **Dependency Isolation**: macro_tools only in meta crate
3. **Compilation**: Faster builds
4. **Standard Pattern**: Matches serde/serde_derive

**Benefit:** Clean separation of concerns

### Why No Value Mutation?

Inspection only, no modification:

**Rationale:**
1. **Complexity**: Type-safe mutation extremely complex
2. **Ownership**: Rust ownership prevents easy mutation
3. **Scope**: Focused on inspection use cases
4. **Safety**: Avoid runtime type errors

**Future:** Could add in controlled manner

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for comprehensive testing
- Runtime reflection testing
- Collection reflection tests

### Test Focus

1. **Primitive Reflection**: All primitive types
2. **Collection Reflection**: Vec, array, slice, HashMap, HashSet
3. **Type Identification**: type_name(), type_id() accuracy
4. **Container Detection**: is_container(), is_ordered() correctness
5. **Element Iteration**: elements() yields correct KeyVals
6. **Element Access**: element(i) retrieves correct element
7. **Type Comparison**: Descriptors for different types differ
8. **Recursive Reflection**: Nested containers
9. **Edge Cases**: Empty collections, single-element collections
10. **Field Reflection**: Custom types with fields (via derive)

### Test Organization

Tests in `tests/inc/`:
- `group1/vec_test.rs` - Vec reflection
- `group1/array_test.rs` - Array reflection
- `fundamental/fields_*.rs` - Field reflection tests
- Collection-specific test files

### Known Test Limitations

1. **Derive Macro**: Testing derive requires integration tests
2. **Type Names**: Platform-dependent (alloc::vec::Vec vs std::vec::Vec)
3. **HashMap Ordering**: Non-deterministic iteration order
4. **Recursive Comparison**: Deep nesting performance

## Future Considerations

### Potential Enhancements

1. **Value Mutation**: Controlled set_element() API
2. **Method Reflection**: Introspect methods, not just data
3. **Trait Reflection**: Inspect implemented traits
4. **Field Access**: Get/set struct fields by name
5. **Builder Pattern**: Dynamic value construction
6. **Better Derive**: More automatic implementations
7. **Attribute Support**: Reflection metadata via attributes
8. **Const Reflection**: Compile-time reflection where possible
9. **Performance**: Optimize descriptor creation
10. **More Collections**: BTreeMap, BTreeSet, LinkedList

### Breaking Changes to Consider

1. **KeyVal Keys**: Use generic key type instead of Primitive
2. **Entity Trait**: Add more methods (field_names, etc.)
3. **Descriptor Unification**: Single descriptor type
4. **Lifetime Support**: Reflection with non-'static lifetimes
5. **Error Handling**: Return Result instead of panicking

### Known Limitations

1. **No Lifetimes**: Only 'static supported in many places
2. **No Generics**: Can't reflect over generic parameters
3. **No Methods**: Only data reflection
4. **No Traits**: Can't inspect trait implementations
5. **Performance**: Runtime overhead vs compile-time
6. **Type Names**: Platform/version dependent strings
7. **No Custom Metadata**: Can't attach arbitrary reflection data

## Adoption Guidelines

### When to Use reflect_tools

**Good Candidates:**
- Serialization/deserialization libraries
- Dynamic ORMs and database mappers
- Plugin systems with unknown types
- Generic data processing algorithms
- Runtime type validation
- Type-agnostic debugging tools
- Heterogeneous collection algorithms

**Poor Candidates:**
- Static, compile-time known types (use generic traits)
- Performance-critical code (compile-time alternatives)
- Simple type checking (use std::any::TypeId)
- Type-safe algorithms (use trait bounds)

### Choosing Reflection Approach

```rust
// Use reflect() for dynamic inspection
use reflect_tools::reflect::{reflect, Entity};

let data: Box<dyn Any> = Box::new(vec![1, 2, 3]);
// Can't inspect without reflection

// With reflection:
let vec = vec![1, 2, 3];
let r = reflect(&vec);
// Now have access to type name, length, elements, etc.
```

### Best Practices

1. **Check is_container()**: Before calling elements()
2. **Use Type IDs**: For runtime type checking
3. **Respect is_ordered()**: Don't assume element order
4. **Cache Descriptors**: Reflect() is type-level, reuse it
5. **Handle Empty**: Check len() before element()
6. **Recursive Reflection**: Elements are also Entity
7. **Document Reflection**: Explain why runtime reflection needed

### Integration with Serialization

```rust
use reflect_tools::reflect::{reflect, Entity, Instance};

trait Serialize {
  fn serialize(&self) -> String;
}

impl<T: Instance> Serialize for T {
  fn serialize(&self) -> String {
    let r = reflect(self);
    if r.is_container() {
      // Serialize as array/map
      let elements: Vec<_> = r.elements().collect();
      format!("[{} elements]", elements.len())
    } else {
      // Serialize as scalar
      format!("{}", r.type_name())
    }
  }
}
```

## Related Crates

**Dependencies:**
- **reflect_tools_meta**: Proc macro support (workspace)
- **derive_tools**: Derive utilities (workspace)
- **collection_tools**: Collection utilities (workspace)
- **macro_tools**: AST utilities (workspace, via reflect_tools_meta)

**Related:**
- **typing_tools**: Compile-time type checking (workspace)
- **inspect_type**: Type inspection at compile-time (workspace)

**Alternatives:**
- **std::any**: Runtime type identification (standard library, less capable)
- **serde**: Full serialization framework (external)
- **inventory**: Runtime type registration (external)
- **typetag**: Serializable trait objects (external)

## References

- [API Documentation](https://docs.rs/reflect_tools)
- [Proc Macro Documentation](https://docs.rs/reflect_tools_meta)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/reflect_tools)
- [readme.md](./readme.md)
- [typing_tools](../typing_tools/readme.md) - Compile-time type utilities
- [derive_tools](../derive_tools/readme.md) - Derive macro utilities
- [std::any](https://doc.rust-lang.org/std/any/) - Standard library type identification

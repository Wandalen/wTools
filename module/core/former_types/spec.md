# former_types

Core trait definitions and type system integration for the Former builder pattern ecosystem.

## Overview

`former_types` provides the foundational trait definitions and compile-time structures that power the Former builder pattern implementation. Unlike the main `former` crate which provides the derive macro, this crate contains the reusable type system abstractions that are referenced by generated code but not themselves generated.

The crate establishes the conceptual framework for:
- **Formation definitions**: How entities are linked to their builders
- **Storage management**: How intermediate state is maintained during construction
- **Completion handlers**: How the building process terminates and produces results
- **Collection integration**: How standard collections participate in the builder pattern

This separation allows the macro-generated code to remain minimal while the complex type machinery lives in this stable, reusable crate.

### Scope

#### Responsibility

former_types is responsible for defining the trait hierarchy and type relationships that govern the Former builder pattern. It provides the compile-time contract that all Former implementations must satisfy, enabling type-safe builder patterns with nested structures and collection support.

#### In-Scope

- **Formation definition traits**: `FormerDefinition`, `FormerDefinitionTypes`, entity mapping traits
- **Formation process traits**: `FormingEnd`, `FormerMutator`, `FormerBegin`
- **Storage traits**: `Storage`, `StoragePreform`
- **Collection abstractions**: `Collection`, `CollectionAdd`, `CollectionAssign`, `CollectionFormer`
- **Standard collection implementations**: Vec, HashMap, HashSet, BTreeMap, BTreeSet, LinkedList, VecDeque, BinaryHeap
- **Re-export of component_model_types**: `Assign` trait for component setting

#### Out-of-Scope

- **Derive macros**: Provided by `former_meta`, not this crate
- **Code generation**: This crate provides traits referenced by generated code
- **Runtime reflection**: All operations are compile-time
- **Validation logic**: Beyond what `FormerMutator` provides

#### Boundaries

- **Upstream**: Depends on `collection_tools` and `component_model_types`
- **Downstream**: Used by `former` (main crate) and all generated Former implementations
- **Macro boundary**: This crate provides types; `former_meta` provides generation

## Architecture

### Module Structure

```
former_types/
├── src/
│   ├── lib.rs               # Crate root with namespace organization
│   ├── definition.rs        # Entity-to-definition mapping traits
│   ├── forming.rs           # Formation process management traits
│   ├── storage.rs           # Storage interface traits
│   └── collection/          # Collection-specific implementations
│       ├── mod.rs           # Collection traits and CollectionFormer
│       ├── vector.rs        # Vec implementation
│       ├── hash_map.rs      # HashMap implementation
│       ├── hash_set.rs      # HashSet implementation
│       ├── btree_map.rs     # BTreeMap implementation
│       ├── btree_set.rs     # BTreeSet implementation
│       ├── linked_list.rs   # LinkedList implementation
│       ├── vector_deque.rs  # VecDeque implementation
│       └── binary_heap.rs   # BinaryHeap implementation
├── examples/
│   └── former_types_trivial.rs  # Assign trait usage example
├── Cargo.toml
├── readme.md
└── spec.md
```

### Core Type Relationships

```
FormerDefinition
├── Types: FormerDefinitionTypes + FormerMutator
│   ├── Storage: Default
│   ├── Formed: (result type)
│   └── Context: (contextual info)
├── End: FormingEnd<Types>
├── Storage: Default
├── Formed
└── Context

Entity
├── EntityToDefinition<Context, Formed, End> → Definition
├── EntityToDefinitionTypes<Context, Formed> → Types
├── EntityToFormer<Definition> → Former
└── EntityToStorage → Storage
```

### Formation Lifecycle

```
┌─────────────────────────────────────────────────────────────────┐
│                    Formation Lifecycle                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. FormerBegin::former_begin(storage, context, on_end)         │
│     ↓                                                           │
│  2. Builder methods accumulate state in Storage                 │
│     ↓                                                           │
│  3. FormerMutator::form_mutation(&mut storage, &mut context)    │
│     ↓                                                           │
│  4. FormingEnd::call(storage, context) → Formed                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Public API

### Definition Traits

#### `FormerDefinitionTypes`

Specifies the fundamental types involved in formation.

```rust
pub trait FormerDefinitionTypes: Sized
{
  /// Storage type for intermediate state (must be Default)
  type Storage: Default;

  /// The fully formed entity type
  type Formed;

  /// Contextual information during formation
  type Context;
}
```

#### `FormerDefinition`

Complete formation definition linking types with end behavior.

```rust
pub trait FormerDefinition: Sized
{
  /// Types with mutator support
  type Types: FormerDefinitionTypes< Storage = Self::Storage, Formed = Self::Formed, Context = Self::Context >
            + FormerMutator;

  /// End handler for formation completion
  type End: FormingEnd< Self::Types >;

  /// Storage, Formed, Context (duplicated for convenience)
  type Storage: Default;
  type Formed;
  type Context;
}
```

### Entity Mapping Traits

#### `EntityToDefinition`

Maps an entity to its complete formation definition.

```rust
pub trait EntityToDefinition< Context, Formed, End >
{
  type Definition: FormerDefinition;
  type Types: FormerDefinitionTypes;
}
```

#### `EntityToDefinitionTypes`

Simplified mapping without end condition.

```rust
pub trait EntityToDefinitionTypes< Context, Formed >
{
  type Types: FormerDefinitionTypes;
}
```

#### `EntityToFormer`

Maps an entity to its builder implementation.

```rust
pub trait EntityToFormer< Definition >
where
  Definition: FormerDefinition,
{
  type Former;
  fn __f( _: &Definition ) {}  // Phantom type anchor
}
```

#### `EntityToStorage`

Maps an entity to its storage type.

```rust
pub trait EntityToStorage
{
  type Storage;
}
```

### Formation Process Traits

#### `FormingEnd`

Handler invoked at formation completion.

```rust
pub trait FormingEnd< Definition: FormerDefinitionTypes >
{
  fn call(
    &self,
    storage: Definition::Storage,
    context: Option< Definition::Context >
  ) -> Definition::Formed;
}

// Blanket implementation for closures
impl< Definition, F > FormingEnd< Definition > for F
where
  F: Fn( Definition::Storage, Option< Definition::Context > ) -> Definition::Formed,
  Definition: FormerDefinitionTypes,
{ /* ... */ }
```

#### Pre-built FormingEnd Implementations

```rust
/// Returns the preformed entity (most common)
#[ derive( Debug, Default ) ]
pub struct ReturnPreformed;

/// Returns the storage directly as formed
#[ derive( Debug, Default ) ]
pub struct ReturnStorage;

/// Placeholder that panics if called
#[ derive( Debug, Default ) ]
pub struct NoEnd;

/// Boxed closure for dynamic dispatch
pub struct FormingEndClosure< Definition: FormerDefinitionTypes >
{
  closure: Box< dyn Fn( ... ) -> ... >,
}
```

#### `FormerMutator`

Pre-completion mutation hook.

```rust
pub trait FormerMutator
where
  Self: FormerDefinitionTypes,
{
  /// Called immediately before FormingEnd
  fn form_mutation(
    _storage: &mut Self::Storage,
    _context: &mut Option< Self::Context >
  ) {}
}
```

#### `FormerBegin`

Initiates subform creation with context.

```rust
pub trait FormerBegin< 'storage, Definition >
where
  Definition: FormerDefinition,
  Definition::Storage: 'storage,
  Definition::Context: 'storage,
  Definition::End: 'storage,
{
  fn former_begin(
    storage: Option< Definition::Storage >,
    context: Option< Definition::Context >,
    on_end: Definition::End,
  ) -> Self;
}
```

### Storage Traits

#### `Storage`

Interface for intermediate formation state.

```rust
pub trait Storage: Default
{
  /// The preformed type (before any FormingEnd transformation)
  type Preformed;
}
```

#### `StoragePreform`

Transforms storage to final state.

```rust
pub trait StoragePreform: Storage
{
  /// Convert storage to preformed entity
  fn preform( self ) -> Self::Preformed;
}
```

### Collection Traits

#### `Collection`

Base trait for collection types.

```rust
pub trait Collection
{
  /// Entry type for addition operations
  type Entry;

  /// Value type stored in collection
  type Val;

  /// Convert entry to value
  fn entry_to_val( e: Self::Entry ) -> Self::Val;
}
```

#### `CollectionAdd`

Single-entry addition.

```rust
pub trait CollectionAdd: Collection
{
  /// Add entry, return success indicator
  fn add( &mut self, e: Self::Entry ) -> bool;
}
```

#### `CollectionAssign`

Bulk replacement.

```rust
pub trait CollectionAssign: Collection + IntoIterator< Item = Self::Entry >
{
  /// Replace all entries, return count added
  fn assign< Entries >( &mut self, entries: Entries ) -> usize
  where
    Entries: IntoIterator< Item = Self::Entry >;
}
```

#### Entry/Value Conversion Traits

```rust
/// Convert entry to value (entry perspective)
pub trait EntryToVal< Collection >
{
  type Val;
  fn entry_to_val( self ) -> Self::Val;
}

/// Convert value to entry (collection perspective)
pub trait CollectionValToEntry< Val >
{
  type Entry;
  fn val_to_entry( val: Val ) -> Self::Entry;
}

/// Convert value to entry (value perspective)
pub trait ValToEntry< Collection >
{
  type Entry;
  fn val_to_entry( self ) -> Self::Entry;
}
```

### CollectionFormer

Generic builder for any collection.

```rust
#[ derive( Default ) ]
pub struct CollectionFormer< E, Definition >
where
  Definition: FormerDefinition,
  Definition::Storage: CollectionAdd< Entry = E >,
{
  storage: Definition::Storage,
  context: Option< Definition::Context >,
  on_end: Option< Definition::End >,
}

impl< E, Definition > CollectionFormer< E, Definition >
where
  Definition: FormerDefinition,
  Definition::Storage: CollectionAdd< Entry = E >,
{
  pub fn begin( storage: Option<...>, context: Option<...>, on_end: ... ) -> Self;
  pub fn end( self ) -> Definition::Formed;
  pub fn form( self ) -> Definition::Formed;  // Alias for end
  pub fn add< IntoElement >( self, entry: IntoElement ) -> Self;
  pub fn replace( self, storage: Definition::Storage ) -> Self;
}
```

### Collection-Specific Types

Each collection has:

```rust
// Example for Vec
pub struct VectorDefinition< E, Context, Formed, End > { /* ... */ }
pub struct VectorDefinitionTypes< E, Context = (), Formed = Vec< E > > { /* ... */ }
pub type VectorFormer< E, Context, Formed, End > =
  CollectionFormer< E, VectorDefinition< E, Context, Formed, End > >;

// Extension trait for fluent API
pub trait VecExt< E >: sealed::Sealed
{
  fn former() -> VectorFormer< E, (), Vec< E >, ReturnStorage >;
}
```

Similar patterns exist for:
- `HashMap` → `HashMapDefinition`, `HashMapFormer`, `HashMapExt`
- `HashSet` → `HashSetDefinition`, `HashSetFormer`, `HashSetExt`
- `BTreeMap` → `BTreeMapDefinition`, `BTreeMapFormer`, `BTreeMapExt`
- `BTreeSet` → `BTreeSetDefinition`, `BTreeSetFormer`, `BTreeSetExt`
- `LinkedList` → `LinkedListDefinition`, `LinkedListFormer`, `LinkedListExt`
- `VecDeque` → `VectorDequeDefinition`, `VectorDequeFormer`, `VecDequeExt`
- `BinaryHeap` → `BinaryHeapDefinition`, `BinaryHeapFormer`, `BinaryHeapExt`

## Usage Patterns

### Implementing FormingEnd

```rust
use former_types::{ FormingEnd, FormerDefinitionTypes };

// Using a closure (blanket impl)
let end = | storage: MyStorage, _ctx: Option< () > | -> MyFormed
{
  MyFormed::from_storage( storage )
};

// Using ReturnPreformed (most common)
use former_types::ReturnPreformed;
let end = ReturnPreformed;  // Calls storage.preform()

// Using FormingEndClosure for dynamic dispatch
use former_types::FormingEndClosure;
let end = FormingEndClosure::new( | storage, ctx | { /* ... */ } );
```

### Using Collection Formers

```rust
use former_types::{ VecExt, HashMapExt, ReturnStorage };

// Direct Vec building
let vec: Vec< i32 > = Vec::former()
  .add( 1 )
  .add( 2 )
  .add( 3 )
  .form();

// HashMap with tuples
use std::collections::HashMap;
let map: HashMap< String, i32 > = HashMap::former()
  .add( ( "one".to_string(), 1 ) )
  .add( ( "two".to_string(), 2 ) )
  .form();
```

### Implementing Custom Former

```rust
use former_types::*;

// Define storage
#[ derive( Default ) ]
struct MyStorage
{
  name: Option< String >,
  value: Option< i32 >,
}

impl Storage for MyStorage
{
  type Preformed = MyStruct;
}

impl StoragePreform for MyStorage
{
  fn preform( self ) -> Self::Preformed
  {
    MyStruct
    {
      name: self.name.unwrap_or_default(),
      value: self.value.unwrap_or( 0 ),
    }
  }
}

// Define types
struct MyDefinitionTypes;

impl FormerDefinitionTypes for MyDefinitionTypes
{
  type Storage = MyStorage;
  type Formed = MyStruct;
  type Context = ();
}

impl FormerMutator for MyDefinitionTypes {}
```

### Using FormerMutator

```rust
use former_types::{ FormerDefinitionTypes, FormerMutator };

struct MyDefinitionTypes;

impl FormerDefinitionTypes for MyDefinitionTypes
{
  type Storage = MyStorage;
  type Formed = MyStruct;
  type Context = ();
}

impl FormerMutator for MyDefinitionTypes
{
  fn form_mutation(
    storage: &mut Self::Storage,
    _context: &mut Option< Self::Context >
  )
  {
    // Validate or transform storage before completion
    if storage.name.as_ref().map( | n | n.is_empty() ).unwrap_or( true )
    {
      storage.name = Some( "default".to_string() );
    }
  }
}
```

### Nested Subformers with FormerBegin

```rust
use former_types::FormerBegin;

// In generated code, subformers use FormerBegin to integrate with parent
impl< 'a, Definition > FormerBegin< 'a, Definition > for MyFormer< Definition >
where
  Definition: FormerDefinition,
{
  fn former_begin(
    storage: Option< Definition::Storage >,
    context: Option< Definition::Context >,
    on_end: Definition::End,
  ) -> Self
  {
    Self
    {
      storage: storage.unwrap_or_default(),
      context,
      on_end: Some( on_end ),
    }
  }
}
```

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `collection_tools` | Collection utilities and constructors |
| `component_model_types` | `Assign` trait for component setting |

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `types_former` | ✓ | Enable core Former trait definitions |
| `no_std` | - | no_std compatibility |
| `use_alloc` | - | Enable alloc in no_std |
| `full` | - | Enable all features |

### Consumers

| Consumer | Relationship |
|----------|--------------|
| `former` | Main user-facing crate, depends on these types |
| `former_meta` | Proc macro generates code using these traits |
| User code | Generated Former implementations reference these |

## Design Rationale

### Why Separate from former?

1. **Compile time**: Types don't change; macro output changes
2. **Code size**: Shared types reduce generated code duplication
3. **Stability**: Type definitions are more stable than macro output
4. **Testing**: Types can be tested independently of macro

### Why Associated Types Over Generics?

Associated types in `FormerDefinitionTypes` provide:
- **Single instantiation**: One definition per entity
- **Type inference**: Compiler can deduce related types
- **Cleaner API**: No need to specify redundant type parameters

### Why FormingEnd as Trait?

FormingEnd as a trait (vs closure type) enables:
- **Named implementations**: `ReturnPreformed`, `ReturnStorage`
- **Type-level dispatch**: Different behaviors via types
- **Zero-cost abstraction**: Monomorphization eliminates indirection
- **Closure support**: Blanket impl for `Fn(...)`

### Why FormerMutator?

FormerMutator exists separately from FormingEnd because:
- **Different concerns**: Mutation is entity-specific; end is context-specific
- **Composition**: Can have default mutator with custom end
- **Subforms**: Mutator applies regardless of how entity is formed

## Testing Strategy

### Test Categories

1. **Trait implementation tests**: Verify implementations satisfy contracts
2. **Collection former tests**: Verify all collection types work correctly
3. **Integration tests**: Verify interaction between traits

### Example Tests

```rust
#[ test ]
fn vec_former_basic()
{
  let vec: Vec< i32 > = Vec::former()
    .add( 1 )
    .add( 2 )
    .form();
  assert_eq!( vec, vec![ 1, 2 ] );
}

#[ test ]
fn return_preformed_works()
{
  // Test that ReturnPreformed correctly calls preform()
}

#[ test ]
fn former_mutator_is_called()
{
  // Test that form_mutation is invoked before FormingEnd
}
```

## Future Considerations

### Potential Enhancements

1. **Async formation**: `AsyncFormingEnd` for async builders
2. **Validation traits**: Built-in validation during formation
3. **Serialization support**: Serde integration for storage
4. **Error handling**: `TryFormingEnd` with Result return

### Known Limitations

1. **No runtime reflection**: All type relationships are compile-time
2. **Trait complexity**: Deep trait hierarchy can confuse error messages
3. **Collection coverage**: Some exotic collections not yet supported

## Adoption Guidelines

### When to Use Directly

- Implementing custom Former without derive macro
- Extending Former for new collection types
- Building higher-level builder abstractions
- Understanding generated code behavior

### When to Use former Instead

- Normal struct/enum builder pattern usage
- Standard collection building
- Any case where derive macro suffices

### Integration Pattern

```rust
// In Cargo.toml, usually you depend on `former`
[dependencies]
former = "2.32"

// `former` re-exports `former_types`
use former::exposed::*;

// Only use `former_types` directly for advanced scenarios
[dependencies]
former_types = "2.32"
```

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `former` | Main crate; re-exports this + provides derive macro |
| `former_meta` | Proc macro; generates code using these types |
| `collection_tools` | Collection utilities used by this crate |
| `component_model_types` | Provides `Assign` trait |

## References

- [Builder Pattern in Rust](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html)
- [former crate documentation](https://docs.rs/former)

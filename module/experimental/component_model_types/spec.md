# Specification: component_model_types

## Overview

**component_model_types** is a types-only crate providing shared type definitions and traits for the component-based programming model used by the wTools ecosystem. It defines the `Assign` trait system that enables flexible, type-safe component assignment patterns, particularly for builder pattern implementations.

**Version:** 0.19.0
**Status:** Experimental
**Category:** Type Definitions (Types Crate Pattern)
**Dependents:** 9 crates (component_model, macro_tools, former, reflect_tools, implements, strs_tools, unilang, error_tools, meta_tools)

### Scope

#### Responsibility

Provide shared type definitions and traits for component-based programming, enabling type-safe, flexible component assignment without circular dependencies between runtime and procedural macro crates.

#### In-Scope

1. **Assign Trait System**
   - `Assign<T, IntoT>` trait - Generic interface for setting components
   - `assign(&mut self, component: IntoT)` - Mutating assignment
   - `impute(self, component: IntoT) -> Self` - Consuming assignment (builder pattern)
   - Two type parameters: `T` (component type) and `IntoT` (convertible type)

2. **Option Extension**
   - `OptionExt<T>` trait - Extension for `Option<T>`
   - `option_assign(&mut self, src: T)` - Assign to Option (create if None, update if Some)
   - Sealed trait pattern (prevents external implementation)

3. **Explicit Type Assignment**
   - `AssignWithType` trait - Assign with explicit type specification
   - `assign_with_type<T, IntoT>(&mut self, component: IntoT)` - Type-guided assignment
   - Blanket implementation for all types implementing `Assign`

4. **Popular Type Support**
   - `PopularType` marker trait - Identifies standard library types
   - Documentation for derive macro generation
   - No actual implementations (orphan rule prevention)

5. **No-std Compatibility**
   - Feature flag `no_std` for embedded environments
   - Feature flag `use_alloc` for allocation-dependent functionality
   - Depends on `collection_tools` for collections

6. **Feature Architecture**
   - `enabled`: Master feature switch
   - `types_component_assign`: Enable Assign trait system
   - `full`: Enable all functionality

7. **Traditional Module Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Not using mod_interface! pattern (types crate)

#### Out-of-Scope

1. **NOT Runtime Implementations**
   - Does not provide actual component model implementations
   - Does not implement builder pattern logic
   - Does not provide derive macros
   - **Rationale:** Types crate pattern - only type definitions, no runtime behavior

2. **NOT Procedural Macros**
   - Does not generate code
   - Does not provide derive macro implementations
   - **Rationale:** Macros are in component_model_meta crate

3. **NOT Concrete Assign Implementations**
   - Does not implement Assign for specific types
   - Does not provide standard library implementations
   - **Rationale:** Implementations are in component_model or user code

4. **NOT Validation Logic**
   - Does not validate component assignments
   - Does not enforce constraints
   - **Rationale:** Validation is application-specific

5. **NOT Default Implementations**
   - Does not provide default component values
   - Does not implement Default for user types
   - **Rationale:** Default implementations are user-defined

6. **NOT Type Conversions**
   - Does not provide From/Into implementations
   - Does not implement type conversion logic
   - **Rationale:** Conversions use standard Into trait

7. **NOT Component Storage**
   - Does not provide component containers
   - Does not implement component registries
   - **Rationale:** Storage is responsibility of struct fields

8. **NOT Foreign Type Implementations**
   - Cannot implement Assign for foreign types (orphan rule)
   - Cannot provide PopularType implementations for std types
   - **Rationale:** Rust's orphan rule prevents implementation

#### Boundaries

- **component_model_types vs component_model**: types provides traits, component_model provides implementations and derive macro re-export
- **component_model_types vs component_model_meta**: types provides traits, meta provides procedural macro implementation
- **component_model_types vs former_types**: component_model_types is for component assignment, former_types is for builder pattern types
- **Types crate pattern**: Shared between runtime and macro crates to prevent circular dependencies

## Architecture

### Dependency Structure

```
component_model_types (types crate, no_std compatible)
└── Internal Dependencies
    └── collection_tools (workspace, for collections support)
```

### Types Crate Pattern

component_model_types follows the types crate pattern:

```
Component Model Ecosystem
├── component_model_types (types only, no dependencies on runtime/meta)
├── component_model_meta (proc macro, depends on types)
└── component_model (runtime, re-exports meta, depends on types)
```

**Purpose:** Prevent circular dependencies while sharing type definitions.

### Module Organization

```
component_model_types
├── lib.rs (traditional namespaces)
├── component.rs (Assign, OptionExt, AssignWithType traits)
└── popular_types/
    ├── mod.rs (module organization)
    └── std_types.rs (PopularType marker trait)
```

**Note:** Uses traditional module pattern, not mod_interface! (types crate)

### Feature Architecture

```
enabled (master switch)
└── types_component_assign (Assign trait system)

no_std (embedded support)
└── use_alloc (requires alloc)
```

**Default Features:** `enabled`, `types_component_assign`

### Trait Design

```
Assign<T, IntoT>
├── assign(&mut self, component: IntoT)  // Mutating
└── impute(self, component: IntoT) -> Self  // Consuming (builder pattern)

OptionExt<T> : sealed::Sealed
└── option_assign(&mut self, src: T)  // Option-aware assignment

AssignWithType (blanket impl)
└── assign_with_type<T, IntoT>(&mut self, component: IntoT)  // Explicit type
```

## Public API

### Core Traits

```rust
/// Generic interface for setting a component on an object
#[cfg(feature = "types_component_assign")]
pub trait Assign<T, IntoT>
{
  /// Sets or replaces the component (mutating)
  fn assign(&mut self, component: IntoT);

  /// Sets or replaces the component (consuming, for builder pattern)
  #[must_use]
  fn impute(self, component: IntoT) -> Self
  where
    Self: Sized;
}

/// Extension trait for Option<T>
#[cfg(feature = "types_component_assign")]
pub trait OptionExt<T> : sealed::Sealed
where
  T: Sized + Assign<T, T>,
{
  /// Assigns to Option (creates if None, updates if Some)
  fn option_assign(&mut self, src: T);
}

/// Assign with explicit type specification
#[cfg(feature = "types_component_assign")]
pub trait AssignWithType
{
  /// Sets component with explicit type parameters
  fn assign_with_type<T, IntoT>(&mut self, component: IntoT)
  where
    IntoT: Into<T>,
    Self: Assign<T, IntoT>;
}
```

### Marker Traits

```rust
/// Marker trait for standard library types that need special treatment
pub trait PopularType {}
```

### Trait Implementations

```rust
// OptionExt for Option<T>
impl<T> OptionExt<T> for Option<T>
where
  T: Sized + Assign<T, T>;

// AssignWithType blanket implementation
impl<S> AssignWithType for S;
```

## Usage Patterns

### Pattern 1: Basic Component Assignment

```rust
use component_model_types::Assign;

#[derive(Default, PartialEq, Debug)]
struct Person
{
  age: i32,
  name: String,
}

impl<IntoT> Assign<i32, IntoT> for Person
where
  IntoT: Into<i32>,
{
  fn assign(&mut self, component: IntoT)
  {
    self.age = component.into();
  }
}

impl<IntoT> Assign<String, IntoT> for Person
where
  IntoT: Into<String>,
{
  fn assign(&mut self, component: IntoT)
  {
    self.name = component.into();
  }
}

let mut person = Person::default();
person.assign(13);           // Sets age
person.assign("John");       // Sets name
assert_eq!(person, Person { age: 13, name: "John".to_string() });
```

### Pattern 2: Builder Pattern with impute

```rust
use component_model_types::Assign;

struct Builder
{
  field: i32,
}

impl<IntoT> Assign<i32, IntoT> for Builder
where
  IntoT: Into<i32>,
{
  fn assign(&mut self, component: IntoT)
  {
    self.field = component.into();
  }
}

// Chaining with impute (consuming)
let builder = Builder { field: 0 }
  .impute(10)
  .impute(20);  // Last value wins

assert_eq!(builder.field, 20);
```

### Pattern 3: Option Assignment

```rust
use component_model_types::{Assign, OptionExt};

struct MyStruct
{
  name: String,
}

impl<IntoT> Assign<MyStruct, IntoT> for MyStruct
where
  IntoT: Into<MyStruct>,
{
  fn assign(&mut self, component: IntoT)
  {
    self.name = component.into().name;
  }
}

// Assign to Option (creates if None, updates if Some)
let mut opt: Option<MyStruct> = None;
opt.option_assign(MyStruct { name: "First".to_string() });
assert_eq!(opt.as_ref().unwrap().name, "First");

opt.option_assign(MyStruct { name: "Second".to_string() });
assert_eq!(opt.unwrap().name, "Second");
```

### Pattern 4: Explicit Type Assignment

```rust
use component_model_types::{Assign, AssignWithType};

struct UserProfile
{
  username: String,
}

impl<IntoT> Assign<String, IntoT> for UserProfile
where
  IntoT: Into<String>,
{
  fn assign(&mut self, component: IntoT)
  {
    self.username = component.into();
  }
}

let mut profile = UserProfile { username: String::new() };
profile.assign_with_type::<String, _>("john_doe");
assert_eq!(profile.username, "john_doe");
```

### Pattern 5: Generic Component System

```rust
use component_model_types::Assign;

// Generic container with component assignment
struct Container<T>
{
  value: T,
}

impl<T, IntoT> Assign<T, IntoT> for Container<T>
where
  IntoT: Into<T>,
{
  fn assign(&mut self, component: IntoT)
  {
    self.value = component.into();
  }
}

let mut container = Container { value: 0 };
container.assign(42);
assert_eq!(container.value, 42);
```

## Dependencies and Consumers

### Direct Dependencies

**Internal:**
- `collection_tools` (workspace) - Collection types support

### Consumers (9 crates)

1. **component_model** - Runtime component model implementation
2. **macro_tools** - Uses Assign for attribute parsing
3. **former** - Builder pattern implementation
4. **reflect_tools** - Reflection utilities
5. **implements** - Implementation detection
6. **strs_tools** - String tools
7. **unilang** - Language utilities
8. **error_tools** - Error handling
9. **meta_tools** - Meta-programming utilities

## Design Rationale

### Why Types Crate Pattern?

**Problem:** Circular dependency risk between runtime and procedural macro crates:

```
component_model (runtime)
↓ depends on
component_model_meta (proc macro)
↓ wants to depend on (for types)
component_model (runtime)  ← CIRCULAR!
```

**Solution:** Extract shared types into separate crate:

```
component_model_types (types only)
↑                    ↑
component_model      component_model_meta
(runtime)            (proc macro)
```

**Benefits:**
1. **No Circular Dependencies**: Both runtime and meta can depend on types
2. **Shared API**: Same trait definitions used by generated and handwritten code
3. **Minimal Crate**: Types crate is small, fast to compile
4. **Clear Separation**: Type definitions vs implementations

### Why Assign<T, IntoT> Trait?

Traditional setters require exact type match:

```rust
struct Person { age: i32 }
impl Person {
  fn set_age(&mut self, age: i32) { self.age = age; }
}

person.set_age(13);  // OK
person.set_age(13u8); // Error: expected i32, found u8
```

Assign trait uses Into for flexibility:

```rust
impl<IntoT> Assign<i32, IntoT> for Person
where IntoT: Into<i32>
{
  fn assign(&mut self, component: IntoT) { self.age = component.into(); }
}

person.assign(13);   // OK (i32)
person.assign(13u8); // OK (u8 can Into<i32>)
```

**Benefits:**
1. **Flexibility**: Accept any type convertible to target type
2. **Type Safety**: Into trait ensures safe conversion
3. **Generic**: Single implementation for all convertible types
4. **Builder Pattern**: Works well with chaining via impute

### Why Two Type Parameters (T and IntoT)?

**T (Component Type):**
- Distinguishes which component is being set
- Enables multiple Assign implementations for different components
- Example: `Assign<i32, _>` for age, `Assign<String, _>` for name

**IntoT (Input Type):**
- Accepts various types convertible to T
- Provides flexibility in what can be assigned
- Enables Into trait usage

**Alternative (Single Parameter):**
```rust
trait Assign<T> where T: Into<TargetType> {
  fn assign(&mut self, component: T);
}
```

**Problem:** Can't distinguish between different components of same type.

### Why impute() Method?

Builder pattern requires consuming methods:

```rust
let builder = Builder::new()
  .field1(value1)
  .field2(value2)
  .build();
```

Traditional assign is mutating:

```rust
builder.assign(value1);  // Returns (), can't chain
```

impute is consuming:

```rust
builder
  .impute(value1)  // Returns Self, can chain
  .impute(value2)
```

**Benefits:**
1. **Chaining**: Enable fluent API
2. **Builder Pattern**: Natural fit for builder implementations
3. **Immutability**: Supports immutable style

### Why OptionExt Trait?

Common pattern: initialize or update Option field:

```rust
// Without OptionExt (manual)
match self.field {
  Some(ref mut val) => val.assign(component),
  None => self.field = Some(default_with_component()),
}

// With OptionExt (automatic)
self.field.option_assign(component);
```

**Benefits:**
1. **Convenience**: Single method call
2. **Correctness**: Handles both None and Some cases
3. **Consistency**: Same pattern as assign

### Why Sealed Trait Pattern for OptionExt?

OptionExt is implemented only for `Option<T>`. Sealed trait prevents external implementations:

```rust
trait OptionExt<T> : sealed::Sealed { ... }
impl<T> OptionExt<T> for Option<T> { ... }

mod sealed {
  pub trait Sealed {}
  impl<T> Sealed for Option<T> { ... }
}
```

**Benefits:**
1. **API Control**: Prevent breaking changes from external implementations
2. **Type Safety**: Ensure only Option<T> implements OptionExt
3. **Future-Proof**: Can add methods without breaking changes

### Why No Concrete Implementations?

component_model_types is a types-only crate. Concrete implementations would:
1. **Create Dependencies**: Require depending on specific types
2. **Violate Orphan Rule**: Can't implement for foreign types
3. **Bloat Crate**: Types crate should be minimal

Implementations belong in:
- **User Code**: For user-defined types
- **component_model**: For wTools types
- **Generated Code**: From component_model_meta derive macro

## Testing Strategy

### Test Coverage

- **Example Programs**: 1 basic example (`component_model_types_trivial.rs`)
- **Doc Tests**: Embedded in trait documentation
- **Integration Tests**: Used through component_model and former crates

### Test Focus

1. **Trait Implementations**: Verify Assign can be implemented
2. **Option Assignment**: Verify OptionExt handles None and Some
3. **Type Inference**: Verify compiler can infer types correctly
4. **Chaining**: Verify impute enables method chaining

### Note on Circular Dependency

```rust
// component_model_types cannot use test_tools
// Reason: Creates circular dependency
// test_tools → impls_index_meta → macro_tools → component_model_types
```

Testing is done through consuming crates.

## Future Considerations

### Potential Enhancements

1. **Additional Traits**: More component manipulation traits
2. **Error Handling**: Fallible assignment (Result-returning)
3. **Validation**: Optional validation in assign
4. **Async Support**: Async assignment methods

### Breaking Changes to Consider

1. **Additional Methods**: Add to Assign trait (breaking for implementers)
2. **Generic Constraints**: Change trait bounds
3. **Seal Traits**: Make Assign sealed to control implementations

### Known Limitations

1. **Orphan Rule**: Cannot provide Assign implementations for foreign types
2. **No PopularType Implementations**: Marker trait only, implementations in macro
3. **No Validation**: Assign doesn't validate components
4. **Circular Dependency Risk**: Cannot use many wTools crates

## Adoption Guidelines

### When to Use Assign Trait

**Good Candidates:**
- Builder pattern implementations
- Component-based architectures
- Flexible initialization patterns
- Derive macro-generated code

**Poor Candidates:**
- Simple setters (use traditional methods)
- Performance-critical code (trait overhead)
- Foreign types (orphan rule)

### Implementation Pattern

```rust
// 1. Define struct
struct MyStruct {
  field1: Type1,
  field2: Type2,
}

// 2. Implement Assign for each component type
impl<IntoT> Assign<Type1, IntoT> for MyStruct
where IntoT: Into<Type1>
{
  fn assign(&mut self, component: IntoT) {
    self.field1 = component.into();
  }
}

impl<IntoT> Assign<Type2, IntoT> for MyStruct
where IntoT: Into<Type2>
{
  fn assign(&mut self, component: IntoT) {
    self.field2 = component.into();
  }
}
```

### Best Practices

1. **Use Into Bounds**: `IntoT: Into<T>` for flexibility
2. **Implement for All Components**: One Assign impl per field
3. **Don't Mix Patterns**: Use Assign consistently within a type
4. **Document Type Parameters**: Clarify T vs IntoT in docs

## Related Crates

- **component_model**: Runtime component model implementation (consumer)
- **component_model_meta**: Procedural macro implementation (consumer)
- **former_types**: Builder pattern types (similar types crate pattern)
- **clone_dyn_types**: Clone trait objects (similar types crate pattern)
- **collection_tools**: Collection types (dependency)

## References

- [API Documentation](https://docs.rs/component_model_types)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/component_model_types)
- [component_model](https://docs.rs/component_model) - Runtime implementation
- [readme.md](./readme.md)

# Specification: clone_dyn_types

## Overview

`clone_dyn_types` is a zero-dependency core crate providing trait implementations for cloning dynamically sized types (DST) and trait objects. It serves as the implementation foundation for the `clone_dyn` facade crate, which provides the procedural macro interface.

## Design Goals

1. **Zero Dependencies**: Must have zero production dependencies to avoid circular dependency chains
2. **No_std Compatibility**: Must work in `no_std` environments with `extern crate alloc`
3. **Type Safety**: Provide safe public API while encapsulating unsafe operations
4. **Sealed Trait Pattern**: Prevent external implementations of core traits
5. **DST Support**: Enable cloning of dynamically sized types (slices, str, trait objects)

## Architecture

### Core Components

#### 1. CloneDyn Trait

The primary trait enabling cloning of trait objects.

**Requirements**:
- Must be object-safe (cannot have Clone as supertrait)
- Must be sealed to prevent external implementations
- Must support unsized types (`?Sized`)
- Must provide internal mechanism for type-erased cloning

**Interface**:
```rust
pub trait CloneDyn: Sealed
{
  #[doc(hidden)]
  fn __clone_dyn(&self, _: DontCallMe) -> *mut ();
}
```

**Implementation Strategy**:
- Internal `__clone_dyn` method returns type-erased pointer (`*mut ()`)
- Uses `DontCallMe` marker to prevent direct invocation
- Actual cloning delegated to concrete type's Clone implementation

#### 2. clone_into_box Function

Public API for cloning trait objects into boxed values.

**Requirements**:
- Must accept any type implementing CloneDyn
- Must support unsized types
- Must return properly typed Box
- Must encapsulate all unsafe operations

**Signature**:
```rust
pub fn clone_into_box<T>(ref_dyn: &T) -> Box<T>
where
  T: ?Sized + CloneDyn
```

**Implementation**:
- Extracts type-erased data pointer via `__clone_dyn`
- Reconstructs fat pointer for DST types
- Creates Box from raw pointer
- All pointer operations confined to this function

#### 3. clone Function

Convenience function for cloning sized types.

**Requirements**:
- Must work with any Clone type
- Must be ergonomic for common use cases
- Must not require CloneDyn trait

**Signature**:
```rust
pub fn clone<T>(src: &T) -> T
where
  T: Clone
```

#### 4. Sealed Trait Pattern

**Requirements**:
- Prevent external trait implementations
- Allow internal implementations only
- Provide clear compile error for violation attempts

**Components**:
- Private `Sealed` trait with private `DontCallMe` type
- CloneDyn extends Sealed
- All implementations in this crate only

### Type Support Matrix

| Type Category | Support | Example | Special Handling |
|--------------|---------|---------|------------------|
| Sized Clone types | ✅ Full | `i32`, `String`, `Vec<T>` | Direct Clone delegation |
| Slices | ✅ Full | `[T]` | Requires double reference (`&&[T]`) for coercion |
| String slices | ✅ Full | `str` | Requires double reference (`&&str`) for coercion |
| Trait objects | ✅ Full | `dyn Trait` | User implements Clone for Box<dyn Trait> |
| Tuples | ✅ Up to 16 | `(T1, T2, ...)` | Macro-generated implementations |
| Arrays | ✅ Up to 32 | `[T; N]` | Macro-generated implementations |

## Public API

### Exports

```rust
pub use clone_dyn_types::
{
  CloneDyn,           // Core trait
  clone_into_box,     // DST/trait object cloning
  clone,              // Convenience function
};
```

### Usage Patterns

#### Pattern 1: Trait Object Cloning

```rust
trait MyTrait: CloneDyn
{
  fn method(&self);
}

impl Clone for Box<dyn MyTrait>
{
  fn clone(&self) -> Self
  {
    clone_into_box(&**self)
  }
}
```

#### Pattern 2: DST Cloning

```rust
let data = vec![1, 2, 3];
let slice: &[i32] = &data;

// Requires double reference for unsized type coercion
let boxed = clone_into_box(&slice as &dyn CloneDyn);
```

#### Pattern 3: Generic Cloning

```rust
#[derive(Clone)]
struct MyStruct { value: i32 }

let original = MyStruct { value: 42 };
let cloned = clone(&original);
```

## Implementation Details

### Memory Safety

**Invariants**:
1. Type-erased pointer from `__clone_dyn` must match original type
2. Fat pointer reconstruction must preserve metadata (length, vtable)
3. Box must take ownership of allocated memory exactly once
4. No double-free or memory leak

**Safety Mechanisms**:
- Sealed trait ensures all implementations are verified
- Type erasure confined to single function
- Pointer operations use documented unsafe patterns
- Fat pointer handling follows Rust Reference guidelines

### DST (Dynamically Sized Types) Handling

**Challenge**: DST types like `[T]` and `str` have runtime-sized metadata

**Solution**:
1. Use fat pointers (pointer + metadata)
2. Type erase data pointer only, preserve metadata pointer
3. Reconstruct fat pointer before Box::from_raw

**Implementation**:
```rust
unsafe {
  let mut ptr = ref_dyn as *const T;              // Fat pointer
  let data_ptr = &mut ptr as *mut *const T as *mut *mut ();
  *data_ptr = <T as CloneDyn>::__clone_dyn(ref_dyn, DontCallMe);
  Box::from_raw(ptr as *mut T)                    // Reconstruct fat pointer
}
```

## Testing Strategy

### Test Organization

Tests organized by domain (what is tested):
- `clone_generic_types`: Sized types (structs, primitives, collections)
- `clone_slices`: Slice types `[T]`
- `clone_str_slices`: String slice type `str`
- `clone_trait_objects`: Boxed trait objects `Box<dyn Trait>`

### Coverage Requirements

1. **Smoke Tests** (tests/smoke_test.rs):
   - Basic clone functionality
   - clone_into_box with trait objects

2. **Comprehensive Tests** (tests/tests.rs):
   - All primitive types (i32, bool, char)
   - Structs with multiple fields
   - Collections (String, Vec)
   - Empty slices/strings
   - Unicode string handling
   - Trait object cloning with methods

### Test Quality Standards

- **No Silent Failures**: All tests use explicit assertions (assert_eq!)
- **No Mocking**: Use real implementations only
- **Loud Failures**: Tests fail clearly with descriptive messages
- **Self-Contained**: No external test framework dependencies (test_tools removed)

## Dependencies

### Production Dependencies

**NONE** - Zero production dependencies required.

### Dev Dependencies

**NONE** - test_tools removed due to circular dependency:
```
macro_tools → clone_dyn_types → test_tools → impls_index_meta → macro_tools
```

## Non-Goals

1. **Not a macro crate**: Procedural macros belong in `clone_dyn` facade
2. **Not polymorphic over Clone**: Cannot make CloneDyn a subtrait of Clone (object safety)
3. **Not for non-Clone types**: All types must implement Clone
4. **Not thread-safe by default**: Cloning does not imply Send/Sync

## Known Limitations

1. **Double Reference for DST**: Slices and str require `&&T` for trait object coercion
2. **Manual Box Clone impl**: Users must implement `Clone for Box<dyn Trait>` manually
3. **Array Size Limit**: Array implementations limited to size 32
4. **Tuple Size Limit**: Tuple implementations limited to arity 16

## Future Considerations

1. **const fn support**: When const traits stabilize
2. **Extended array sizes**: If const generics improve
3. **Allocation API**: When Allocator trait stabilizes
4. **Dyn-safe Clone**: If Rust gains object-safe Clone mechanism

## Compliance

- ✅ No_std compatible
- ✅ Zero production dependencies
- ✅ Sealed trait pattern
- ✅ Safe public API
- ✅ Comprehensive test coverage
- ✅ Zero clippy warnings
- ⚠️ No specification (addressed by this document)

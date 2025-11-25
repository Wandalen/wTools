# Specification: is_slice

## Overview

**is_slice** is a zero-dependency no_std crate providing a compile-time macro for detecting slice types. It uses trait specialization-like techniques to distinguish `&[T]` slice references from arrays `&[T; N]` and other types at runtime with compile-time guarantees, serving as a type discrimination utility for generic code handling both slices and arrays.

**Version:** 0.14.0
**Status:** Production
**Category:** Type Utilities (Type Detection)
**Dependents:** 2 workspace crates (likely collection/iterator utilities)

### Scope

#### Responsibility

Provide a simple macro interface for detecting whether a value is a slice reference (`&[T]`) at runtime, enabling generic code to discriminate between slice types and array types without requiring procedural macros or type introspection.

#### In-Scope

1. **is_slice! Macro**
   - Syntax: `is_slice!( value )`
   - Returns `bool` at runtime
   - Detects `&[T]` (slice references)
   - Distinguishes slices from arrays

2. **Slice vs Array Detection**
   - `&[T]` (slice) → returns `true`
   - `&[T; N]` (array) → returns `false`
   - Other types → returns `false`

3. **Type Specialization Technique**
   - Internal `NotSlice` trait for `&PhantomData<T>` (unconditional, returns false)
   - Internal `Slice` trait for `PhantomData<&'a &[T]>` (slice-specific, returns true)
   - Rust's trait resolution picks more specific implementation
   - Returns true/false via trait methods

4. **Zero Dependencies**
   - No production dependencies
   - No test_tools dependency (prevents circular dependencies)
   - Uses only core library (PhantomData)

5. **no_std Compatibility**
   - `#![no_std]` compatible
   - Works in embedded environments
   - No allocation required

6. **Traditional Module Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Not using mod_interface! (utility crate)
   - Simple feature gating

#### Out-of-Scope

1. **NOT Array Reference Detection**
   - Does not return true for `&[T; N]`
   - Only detects dynamically-sized slices `&[T]`
   - **Rationale:** Specific focus on slice types

2. **NOT Element Type Reporting**
   - Does not return element type
   - Only returns boolean
   - **Rationale:** Use inspect_type for type names

3. **NOT Slice Length Checking**
   - Does not return length
   - Only checks if type is slice
   - **Rationale:** Use `.len()` method for length

4. **NOT Compile-Time Boolean**
   - Returns runtime bool, not compile-time constant
   - Cannot use in const contexts
   - **Rationale:** Requires const fn and const trait support

5. **NOT Owned Slice Detection**
   - Does not detect `Box<[T]>` or `Vec<T>`
   - Only detects borrowed slices `&[T]`
   - **Rationale:** Focused on reference types

6. **NOT Mutable Slice Detection**
   - Does not distinguish `&mut [T]` from `&[T]`
   - Works with both
   - **Rationale:** Both are slice references

7. **NOT str Slice Detection**
   - Specifically for `&[T]`, not `&str`
   - Does not detect string slices
   - **Rationale:** `&str` is different type (though internally a slice)

8. **NOT Procedural Macro**
   - Declarative macro only
   - No custom derive
   - **Rationale:** Simpler implementation

#### Boundaries

- **is_slice vs array checking**: is_slice detects slices (`&[T]`); arrays are `&[T; N]`
- **is_slice vs implements**: is_slice is specialized for slice detection; implements checks arbitrary traits
- **is_slice vs type inspection**: is_slice checks specific type pattern; inspect_type reports type names

## Architecture

### Dependency Structure

```
is_slice (type utilities, zero dependencies)
├── Internal Dependencies
│   └── (none - foundational utility)
└── Dev Dependencies
    └── (intentionally empty to prevent circular dependencies)
```

**Note:** Intentionally avoids test_tools to prevent circular dependency chains.

### Module Organization

```
is_slice
├── lib.rs (traditional namespaces, macro definition)
│   ├── NotSlice trait (unconditional impl)
│   ├── Slice trait (slice-specific impl)
│   └── does() helper function
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Traditional namespace organization, not mod_interface! (utility crate convention)

### Feature Architecture

```
enabled (master switch)
└── full (all features, same as enabled)
```

**Default Features:** `enabled`

**Note:** Minimal feature structure - main functionality always available

### Trait Specialization Flow

```
is_slice!( value )
  ↓
Create PhantomData from value reference
  ↓
Is value a slice reference (&[T])?
  ├─ YES → Slice trait applies (PhantomData<&'a &[T]>)
  │         + NotSlice trait also applies (&PhantomData<...>)
  │         → Rust picks Slice (more specific)
  │         → .is_slice() returns true
  │
  └─ NO  → Only NotSlice trait applies (&PhantomData<T>)
            → .is_slice() returns false
```

### Macro Expansion

```rust
is_slice!( &[1, 2, 3][..] )
  ↓ expands to
{
  trait NotSlice { fn is_slice(&self) -> bool { false } }
  impl<T: ?Sized> NotSlice for &PhantomData<T> {}

  trait Slice { fn is_slice(&self) -> bool { true } }
  impl<'a, T> Slice for PhantomData<&'a &[T]> {}

  fn does<T: Sized>(_: &T) -> PhantomData<&T> { PhantomData }

  (&does(&&[1, 2, 3][..])).is_slice()
  // &[i32] is a slice → PhantomData<&'a &[i32]> : Slice
  // → returns true
}

is_slice!( &[1, 2, 3] )
  ↓
  // &[i32; 3] is an array reference → only NotSlice applies
  // → returns false
```

## Public API

### Macro

```rust
/// Check if value is a slice reference
#[macro_export]
macro_rules! is_slice {
  ( $value:expr ) => {
    // Returns: bool
  };
}
```

### Re-export

```rust
pub use is_slice;
```

## Usage Patterns

### Pattern 1: Basic Slice Detection

```rust
use is_slice::*;

dbg!( is_slice!( Box::new( true ) ) );
// Output: is_slice!(Box::new(true)) = false

dbg!( is_slice!( &[ 1, 2, 3 ] ) );
// Output: is_slice!(&[1, 2, 3]) = false (array reference!)

dbg!( is_slice!( &[ 1, 2, 3 ][ .. ] ) );
// Output: is_slice!(&[1, 2, 3][..]) = true (slice!)
```

**Key Difference:** Arrays (`&[T; N]`) vs Slices (`&[T]`)

### Pattern 2: Slice vs Array Discrimination

```rust
use is_slice::*;

let array = [1, 2, 3, 4, 5];
let array_ref = &array;              // &[i32; 5]
let slice_ref = &array[..];          // &[i32]
let sub_slice = &array[1..3];        // &[i32]

assert!( !is_slice!( array_ref ) );  // Array reference → false
assert!( is_slice!( slice_ref ) );   // Slice → true
assert!( is_slice!( sub_slice ) );   // Sub-slice → true
```

### Pattern 3: Generic Function Handling Both

```rust
use is_slice::*;

fn process<T>(data: &T)
where
  T: ?Sized,
{
  if is_slice!( data ) {
    println!("Processing a slice");
    // Slice-specific logic
  } else {
    println!("Processing non-slice");
    // Array or other type logic
  }
}

process(&[1, 2, 3]);        // "Processing non-slice" (array)
process(&[1, 2, 3][..]);    // "Processing a slice"
```

### Pattern 4: Vector Slicing

```rust
use is_slice::*;

let vec = vec![1, 2, 3, 4, 5];

assert!( !is_slice!( &vec ) );       // Vec reference → false
assert!( is_slice!( &vec[..] ) );    // Vec slice → true
assert!( is_slice!( &vec[1..3] ) );  // Vec sub-slice → true
```

### Pattern 5: String vs String Slice

```rust
use is_slice::*;

let string = String::from("hello");
let str_slice: &str = "world";

// Note: is_slice! checks for &[T], not &str
assert!( !is_slice!( &string ) );    // String reference → false
assert!( !is_slice!( str_slice ) );  // &str is different type → false

// But byte slices work:
assert!( is_slice!( string.as_bytes() ) );  // &[u8] → true
```

### Pattern 6: Conditional Slicing

```rust
use is_slice::*;

fn ensure_slice<T>(data: &T) -> &[T::Item]
where
  T: AsRef<[T::Item]> + ?Sized,
  T::Item: Sized,
{
  if is_slice!( data ) {
    // Already a slice, return as-is
    unsafe { &*(data as *const T as *const [T::Item]) }
  } else {
    // Convert to slice
    data.as_ref()
  }
}
```

**Note:** Simplified example - real code needs proper bounds.

### Pattern 7: Debug Logging

```rust
use is_slice::*;

fn debug_type<T>(value: &T) {
  println!("Is slice: {}", is_slice!( value ));

  if is_slice!( value ) {
    println!("Working with dynamic slice");
  } else {
    println!("Working with fixed-size data");
  }
}
```

### Pattern 8: Empty Slice Detection

```rust
use is_slice::*;

let empty: &[i32] = &[];
let empty_array = &[] as &[i32; 0];

assert!( is_slice!( empty ) );         // Empty slice → true
assert!( !is_slice!( empty_array ) );  // Empty array → false
```

## Dependencies and Consumers

### Direct Dependencies

**Production:** (none - zero dependencies)

**Dev:** (intentionally empty)
- Avoids test_tools to prevent circular dependencies

### Consumers (2 workspace crates)

**Identified:** Likely used by:
- Collection utilities
- Iterator tools
- Generic slice/array handling code

**Usage Pattern:** Workspace utilities use is_slice to discriminate between slice and array types in generic code, enabling optimized paths for dynamically-sized vs fixed-size data.

## Design Rationale

### Why Trait Specialization Technique?

**Problem:** Need to distinguish `&[T]` from `&[T; N]` at runtime.

**Solution:** Trait resolution picks more specific impl:

```rust
trait NotSlice { fn is_slice(&self) -> bool { false } }
impl<T: ?Sized> NotSlice for &PhantomData<T> {}  // Unconditional

trait Slice { fn is_slice(&self) -> bool { true } }
impl<'a, T> Slice for PhantomData<&'a &[T]> {}  // Slice-specific!

// Rust picks Slice if type is &[T] (more specific)
```

**Benefits:**
1. **No Dependencies**: Pure language features
2. **Compile-Time Safe**: Type errors caught at compile time
3. **Zero-Cost**: Optimizes to constant true/false

**Tradeoff:** Subtle technique for simplicity and zero dependencies

### Why Only &[T], Not &[T; N]?

The macro specifically excludes array references:

```rust
is_slice!( &[1, 2, 3] )      // false (array)
is_slice!( &[1, 2, 3][..] )  // true (slice)
```

**Rationale:**
- **Type Distinction**: Arrays have compile-time known size, slices don't
- **Use Case**: Generic code often needs different handling for sized vs unsized
- **Performance**: Arrays can be stack-allocated, slices require size info at runtime

**Tradeoff:** More specific check for clearer use case

### Why Runtime Bool, Not Compile-Time Const?

The macro returns runtime `bool`, not `const bool`:

**Limitations:**
- Cannot use in const contexts
- Cannot use for conditional compilation
- Cannot use in type system

**Rationale:**
- Requires const trait bounds (unstable)
- Runtime bool is sufficient for most use cases
- Can be optimized away by compiler

**Workaround:** For compile-time checks, use trait bounds or type parameters

### Why Not Detect Box<[T]> or Vec<T>?

Only detects borrowed slices `&[T]`, not owned:

**Rationale:**
- **Simplicity**: Owned types have different memory semantics
- **Ownership**: Box/Vec own their data, slices just borrow
- **Use Case**: Most generic code works with references

**Workaround:** Convert to slice first: `is_slice!( &boxed_slice[..] )`

### Why Not Detect &str?

Does not detect string slices:

```rust
is_slice!( "hello" )  // false (&str is different type)
```

**Rationale:**
- **Type Difference**: `&str` is conceptually a slice but has different type
- **Semantic Difference**: `&str` guarantees UTF-8, `&[u8]` doesn't
- **Specificity**: Focus on `&[T]` for generic T

**Workaround:** Check bytes: `is_slice!( "hello".as_bytes() )` returns true

### Why No Test Dependencies?

Intentionally avoids test_tools to prevent circular dependencies:

```
collection_tools → ... → test_tools → ... → is_slice
```

**Tradeoff:** Less comprehensive testing for ecosystem stability

### Why Simpler Than implements?

Comparison with implements crate:

**implements:**
- Checks any trait
- More complex implementation
- General-purpose

**is_slice:**
- Checks specific type pattern (`&[T]`)
- Simpler implementation
- Specialized purpose

**Benefit:** Simpler code for common specific case

## Testing Strategy

### Test Coverage

**Limited by Circular Dependency:**
- Cannot use test_tools (would create circular dependency)
- Relies on doc tests and consumer integration tests

### Test Files

```
tests/
└── (minimal or none - doc tests preferred)
```

### Test Focus

1. **Doc Tests**: Embedded in readme.md and lib.rs
2. **Example**: examples/is_slice_trivial.rs demonstrates usage
3. **Consumer Tests**: Integration tests in dependent crates

### Known Test Limitations

1. **No Unit Tests**: Cannot use test framework due to circular dependency prevention
2. **Manual Testing**: Relies on examples and doc tests
3. **Consumer Validation**: Dependent crates serve as integration tests

## Future Considerations

### Potential Enhancements

1. **Const Support**: Make is_slice! usable in const contexts when const traits stabilize
2. **Mutable Detection**: Separate is_mut_slice! for distinguishing &mut [T]
3. **String Slice**: Separate is_str! for &str detection
4. **Owned Slices**: Detect Box<[T]>, Rc<[T]>, Arc<[T]>
5. **Generic DST**: Extend to other dynamically-sized types

### Breaking Changes to Consider

1. **Const Bool Return**: Change to const bool when possible
2. **Rename**: More specific name like is_borrowed_slice!
3. **Expand Scope**: Detect more slice-like types

### Known Limitations

1. **Runtime Bool**: Cannot use in const contexts
2. **Reference Only**: Doesn't detect owned slice types
3. **No &str**: Doesn't detect string slices
4. **No Mutability**: Doesn't distinguish &[T] from &mut [T]

## Adoption Guidelines

### When to Use is_slice

**Good Candidates:**
- Generic code handling both arrays and slices
- Performance optimization based on type
- Conditional logic for sized vs unsized data
- Debugging and logging type information

**Poor Candidates:**
- Compile-time type discrimination (use trait bounds)
- const contexts (use trait bounds)
- Owned slice types (use type parameters)
- String slice detection (check manually or use different approach)

### Migration from Manual Type Checking

```rust
// Before: Cannot distinguish slice from array at runtime
fn process<T: ?Sized>(data: &T) {
  // No way to know if &[T] or &[T; N]
}

// After: Runtime slice detection
use is_slice::*;

fn process<T: ?Sized>(data: &T) {
  if is_slice!( data ) {
    println!("Dynamic slice - unknown size at compile time");
  } else {
    println!("Fixed-size type - size known at compile time");
  }
}
```

### Best Practices

1. **Understand Difference**: Know the distinction between `&[T]` and `&[T; N]`
2. **Prefer Type Bounds**: Use trait bounds when possible, is_slice for edge cases
3. **Document Usage**: Explain why slice detection is needed
4. **Cache Results**: If checking same value repeatedly, cache the bool
5. **Consider Alternatives**: Sometimes generic impl is better than runtime check

## Related Crates

- **implements**: General trait checking (related technique)
- **inspect_type**: Type name inspection (complementary)
- **std::any::Any**: Type ID and downcasting (different approach)
- **static_assertions**: Compile-time assertions (compile-time alternative)

## References

- [API Documentation](https://docs.rs/is_slice)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/is_slice)
- [Example](./examples/is_slice_trivial.rs)
- [readme.md](./readme.md)
- [Rust Reference - Slice Types](https://doc.rust-lang.org/reference/types/slice.html)
- [Rust Reference - Array Types](https://doc.rust-lang.org/reference/types/array.html)

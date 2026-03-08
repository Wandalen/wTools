# Specification: implements

## Overview

**implements** is a zero-dependency no_std crate providing a compile-time macro for checking trait implementation. It uses trait specialization-like techniques to answer "does this value implement this trait?" at runtime with compile-time guarantees, serving as a diagnostic and introspection utility for generic code and type checking.

**Version:** 0.13.0
**Status:** Production
**Category:** Type Utilities (Trait Checking)
**Dependents:** 2 workspace crates (likely diagnostic/macro utilities)

### Scope

#### Responsibility

Provide a simple macro interface for compile-time trait implementation checking, enabling runtime boolean queries about whether a value implements a specific trait without requiring procedural macros or compiler plugins.

#### In-Scope

1. **implements! Macro**
   - Syntax: `implements!( value => Trait )`
   - Returns `bool` at runtime
   - Compile-time trait checking
   - Supports any trait (with Fn trait caveats)

2. **instance_of! Macro**
   - Alias for `implements!`
   - Identical functionality
   - Alternative naming convention

3. **Type-Erased Trait Checking**
   - Works with concrete types
   - Works with generic types
   - Works with trait objects (checks concrete type)
   - Does not move or consume value

4. **Trait Specialization Technique**
   - Internal `False` trait for `&PhantomData<T>` (unconditional)
   - Internal `True` trait for `PhantomData<T>` (conditional on T: Trait)
   - Rust's trait resolution picks more specific implementation
   - Returns true/false via trait methods

5. **Zero Dependencies**
   - No production dependencies
   - No test_tools dependency (prevents circular dependencies)
   - Uses only core library (PhantomData, marker traits)

6. **no_std Compatibility**
   - `#![no_std]` compatible
   - Optional `use_alloc` feature
   - Works in embedded environments

7. **Traditional Module Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Not using mod_interface! (utility crate)
   - Simple feature gating

8. **Nightly Feature Support**
   - Optional `nightly` feature flag
   - Currently no nightly-specific functionality
   - Future-proofing for potential enhancements

#### Out-of-Scope

1. **NOT Fn/FnMut/FnOnce Checking for Functions**
   - Known limitation: checking Fn traits on function items fails at compile-time
   - Reason: Rust's type system peculiarities with function items
   - **Workaround:** Check on closure types instead
   - **Rationale:** Cannot solve without compiler support

2. **NOT Compile-Time Boolean**
   - Returns runtime bool, not compile-time constant
   - Cannot use in const contexts
   - **Rationale:** Requires const fn and const trait support

3. **NOT Trait Listing**
   - Does not enumerate implemented traits
   - Only checks specific trait
   - **Rationale:** Requires reflection/compiler plugin

4. **NOT Negative Trait Checking**
   - Cannot check "does NOT implement"
   - Only positive checks
   - **Rationale:** Can negate result manually (`!implements!(...)`)

5. **NOT Type Name Reporting**
   - Does not return type name
   - Only returns boolean
   - **Rationale:** Use inspect_type crate for type names

6. **NOT Value Inspection**
   - Does not inspect value contents
   - Only checks type traits
   - **Rationale:** Orthogonal concern (use Debug/Display)

7. **NOT Generic Trait Bounds**
   - Cannot check parametric traits like `Into<T>` for arbitrary T
   - Must specify concrete target type
   - **Rationale:** Macro limitation

8. **NOT Procedural Macro**
   - Declarative macro only
   - No custom derive
   - **Rationale:** Simpler implementation, no proc-macro dependency

#### Boundaries

- **implements vs trait bounds**: implements provides runtime check; trait bounds enforce compile-time requirements
- **implements vs specialization**: implements simulates specialization for trait checking without nightly features
- **implements vs Any**: implements checks specific trait; Any provides type erasure and downcasting

## Architecture

### Dependency Structure

```
implements (type utilities, zero dependencies)
├── Internal Dependencies
│   └── (none - foundational utility)
└── Dev Dependencies
    └── (intentionally empty to prevent circular dependencies)
```

**Note:** Intentionally avoids test_tools to prevent circular dependency chains.

### Module Organization

```
implements
├── lib.rs (traditional namespaces, macro definitions)
├── implements_impl.rs (_implements! internal macro)
│   ├── False trait (unconditional impl)
│   ├── True trait (conditional impl)
│   └── does() helper function
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Traditional namespace organization, not mod_interface! (utility crate convention)

### Feature Architecture

```
enabled (master switch)
├── full (all features)
│
no_std (embedded support)
└── use_alloc (allocation support, requires no_std)
│
nightly (future enhancements)
```

**Default Features:** `enabled`

**Note:** Minimal feature structure - main functionality always available

### Trait Specialization Flow

```
implements!( value => Trait )
  ↓
_implements!( value => Trait )
  ↓
Create type T from value
  ↓
PhantomData<T> construction
  ↓
Does T implement Trait?
  ├─ YES → True trait applies (PhantomData<T> where T: Trait)
  │         + False trait also applies (&PhantomData<T>)
  │         → Rust picks True (more specific)
  │         → .get() returns true
  │
  └─ NO  → Only False trait applies (&PhantomData<T>)
            → .get() returns false
```

### Macro Expansion

```rust
implements!( 13_i32 => Copy )
  ↓ expands to
{
  trait False { fn get(&self) -> bool { false } }
  impl<T: ?Sized> False for &PhantomData<T> {}

  trait True { fn get(&self) -> bool { true } }
  impl<T: Copy + ?Sized> True for PhantomData<T> {}

  fn does<T: Sized>(_: &T) -> PhantomData<T> { PhantomData }

  (&does(&13_i32)).get()
  // i32 implements Copy → PhantomData<i32> : True
  // → returns true
}
```

## Public API

### Macros

```rust
/// Check if value implements trait
#[macro_export]
macro_rules! implements {
  ( $value:expr => $( $Trait:tt )+ ) => {
    // Returns: bool
  };
}

/// Alias for implements!
#[macro_export]
macro_rules! instance_of {
  ( $value:expr => $( $Trait:tt )+ ) => {
    // Returns: bool
  };
}
```

### Re-exports

```rust
pub use implements;
pub use instance_of;
```

## Usage Patterns

### Pattern 1: Basic Trait Checking

```rust
use implements::*;

dbg!( implements!( 13_i32 => Copy ) );
// Output: implements!( 13_i32 => Copy ) : true

dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
// Output: implements!( Box::new( 13_i32 ) => Copy ) : false
```

### Pattern 2: Generic Type Checking

```rust
use implements::*;

fn analyze<T>(value: T) {
  if implements!( value => Clone ) {
    println!("Value is cloneable");
  } else {
    println!("Value is NOT cloneable");
  }
}

analyze(42);           // "Value is cloneable" (i32: Clone)
analyze(Box::new(42)); // "Value is cloneable" (Box<i32>: Clone)
```

### Pattern 3: Conditional Logic

```rust
use implements::*;

fn process<T>(data: T) {
  if implements!( data => Send ) {
    // Safe to send across threads
    std::thread::spawn(move || {
      // Use data
    });
  } else {
    // Must process in current thread
    // Process data here
  }
}
```

### Pattern 4: Debug vs Display

```rust
use implements::*;
use std::fmt::{Debug, Display};

fn print_value<T>(value: &T) {
  if implements!( *value => Display ) {
    // Would use Display if we could call it dynamically
    println!("Has Display");
  } else if implements!( *value => Debug ) {
    println!("Has Debug only");
  } else {
    println!("No formatting traits");
  }
}
```

**Note:** The macro only checks, doesn't enable dynamic dispatch.

### Pattern 5: instance_of! Alias

```rust
use implements::instance_of;

assert!( instance_of!( vec![1, 2, 3] => Clone ) );
assert!( instance_of!( "hello" => Copy ) );
assert!( !instance_of!( String::from("hello") => Copy ) );
```

### Pattern 6: Multiple Trait Bounds

```rust
use implements::*;

fn check<T>(value: T) {
  // Single trait
  let is_send = implements!( value => Send );

  // Multiple traits (AND)
  let is_send_sync = implements!( value => Send + Sync );

  println!("Send: {}, Send+Sync: {}", is_send, is_send_sync);
}

check(42);              // Send: true, Send+Sync: true
check(std::rc::Rc::new(42)); // Send: false, Send+Sync: false
```

### Pattern 7: Working with References

```rust
use implements::*;

let value = String::from("hello");

// Check value itself
assert!( implements!( value => Clone ) );

// Check reference
let reference = &value;
assert!( implements!( *reference => Clone ) );

// Reference types
assert!( implements!( reference => Copy ) );  // &String is Copy!
```

### Pattern 8: Avoiding Fn Limitation

```rust
use implements::*;

// DON'T: Check Fn trait on function items (compile error)
// fn my_func() {}
// let check = implements!( my_func => Fn() ); // ERROR!

// DO: Check Fn trait on closure types
let closure = || {};
let check = implements!( closure => Fn() );   // Works!
assert!(check);
```

## Dependencies and Consumers

### Direct Dependencies

**Production:** (none - zero dependencies)

**Dev:** (intentionally empty)
- Avoids test_tools to prevent circular dependencies

### Consumers (2 workspace crates)

**Identified:** Likely used by:
- Macro development utilities
- Type diagnostic tools
- Generic algorithm libraries

**Usage Pattern:** Workspace tools use implements for runtime trait checking in generic code, debugging macro expansions, and implementing conditional logic based on trait implementation.

## Design Rationale

### Why Trait Specialization Technique?

**Problem:** Need to check trait implementation at runtime without reflection.

**Solution:** Clever use of trait resolution:

```rust
trait False { fn get(&self) -> bool { false } }
impl<T: ?Sized> False for &PhantomData<T> {}  // Unconditional

trait True { fn get(&self) -> bool { true } }
impl<T: Trait + ?Sized> True for PhantomData<T> {}  // Conditional!

// Rust picks True if T: Trait (more specific)
// Otherwise falls back to False
```

**Benefits:**
1. **No Dependencies**: Pure language features
2. **Compile-Time Safe**: Type errors caught at compile time
3. **Zero-Cost**: Optimizes to constant true/false

**Tradeoff:** Somewhat obscure technique for simplicity

### Why Runtime Bool, Not Compile-Time Const?

The macro returns runtime `bool`, not `const bool`:

**Limitations:**
- Cannot use in const contexts
- Cannot use for conditional compilation
- Cannot use in array sizes

**Rationale:**
- Requires const trait bounds (unstable)
- Requires const fn (has limitations)
- Runtime bool is sufficient for most use cases

**Workaround:** For compile-time checks, use trait bounds directly

### Why Limitation with Fn Traits?

Checking Fn/FnMut/FnOnce on function items fails:

```rust
fn my_func() {}
implements!( my_func => Fn() );  // Compile error!
```

**Reason:** Function items have unique types that don't directly implement Fn traits without coercion.

**Workaround:** Use closures or function pointers:

```rust
let closure = || {};
implements!( closure => Fn() );  // Works!

let fn_ptr: fn() = my_func;
implements!( fn_ptr => Fn() );   // Works!
```

**Rationale:** Rust's type system limitation, cannot solve at macro level

### Why Both implements! and instance_of!?

Two macros with identical functionality:

1. **implements!**: Trait-centric naming ("does it implement X?")
2. **instance_of!**: OOP-style naming (familiar from Java/JavaScript)

**Benefits:**
- **Flexibility**: Use whichever reads better
- **Familiarity**: instance_of for OOP background
- **Clarity**: implements for Rust-centric code

**Tradeoff:** Slight API duplication for better UX

### Why Not Use std::any::Any?

Comparison with Any trait:

**Any provides:**
- Type ID comparison
- Downcasting for trait objects
- Runtime type information

**implements provides:**
- Trait implementation checking
- Works with any trait
- No type erasure required

**Different Use Cases:**
- **Any**: "What concrete type is this?"
- **implements**: "Does this support operation X?"

### Why No Test Dependencies?

Intentionally avoids test_tools to prevent circular dependencies:

```
macro_tools → ... → test_tools → ... → implements
```

**Tradeoff:** Less comprehensive testing for ecosystem stability

### Why Zero Dependencies?

implements has no dependencies because:

1. **Foundation**: Many crates depend on it
2. **Simplicity**: Implementation is trivial (< 40 lines)
3. **Portability**: Works everywhere Rust works

**Tradeoff:** No external utilities, but maximum compatibility

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
2. **Example**: examples/implements_trivial.rs demonstrates usage
3. **Consumer Tests**: Integration tests in dependent crates

### Known Test Limitations

1. **No Unit Tests**: Cannot use test framework due to circular dependency prevention
2. **Manual Testing**: Relies on examples and doc tests
3. **Consumer Validation**: Dependent crates serve as integration tests

## Future Considerations

### Potential Enhancements

1. **Const Support**: Make implements! usable in const contexts when const traits stabilize
2. **Negative Checking**: Add not_implements! macro for negative checks
3. **Trait Listing**: Enumerate all implemented traits (requires compiler support)
4. **Better Fn Support**: Workaround for function item limitation
5. **Generic Trait Checking**: Support parametric traits like Into<T> for arbitrary T

### Breaking Changes to Consider

1. **Const Bool Return**: Change to const bool when possible (breaking for non-const usage)
2. **Rename**: More descriptive names than implements/instance_of
3. **Error Messages**: Improve compile errors for Fn trait limitation

### Known Limitations

1. **Fn Trait Checking**: Doesn't work for function items (only closures/fn pointers)
2. **Runtime Bool**: Cannot use in const contexts
3. **No Type Info**: Only returns boolean, no type details
4. **No Generic Traits**: Cannot check Into<T> for arbitrary T

## Adoption Guidelines

### When to Use implements

**Good Candidates:**
- Conditional logic in generic functions
- Debugging generic code
- Runtime trait inspection
- Educational/diagnostic code
- Logging and error messages

**Poor Candidates:**
- Compile-time trait bounds (use trait bounds directly)
- const contexts (use trait bounds)
- Performance-critical paths (prefer static dispatch)
- Fn trait checking on functions (use closures)

### Migration from Manual Trait Checking

```rust
// Before: Cannot check trait implementation at runtime
fn process<T>(value: T) {
  // No way to know if T: Clone at runtime!
}

// After: Runtime trait checking
use implements::*;

fn process<T>(value: T) {
  if implements!( value => Clone ) {
    println!("Cloneable");
  } else {
    println!("Not cloneable");
  }
}
```

### Best Practices

1. **Prefer Trait Bounds**: Use trait bounds when possible, implements for edge cases
2. **Don't Replace Dispatch**: Don't use implements to simulate dynamic dispatch (use trait objects)
3. **Diagnostic Use**: Best for debugging, logging, conditional non-critical behavior
4. **Document Limitations**: Note Fn trait limitation in user-facing APIs
5. **Cache Results**: If checking same type repeatedly, cache the bool result

## Related Crates

- **std::any::Any**: Type ID and downcasting (different purpose)
- **inspect_type**: Type name inspection (complementary)
- **static_assertions**: Compile-time assertions (compile-time alternative)
- **inventory**: Type registration and enumeration (different approach)

## References

- [API Documentation](https://docs.rs/implements)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/implements)
- [Example](./examples/implements_trivial.rs)
- [readme.md](./readme.md)
- [Rust Trait Specialization RFC](https://rust-lang.github.io/rfcs/1210-impl-specialization.html)

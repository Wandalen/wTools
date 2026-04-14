# Specification: typing_tools

## Overview

**typing_tools** is a facade crate aggregating type system utilities from workspace crates (inspect_type, is_slice, implements), providing a unified interface for compile-time type checking, trait implementation verification, and slice type detection. It serves as the workspace's central hub for type-level programming and compile-time type introspection.

**Version:** 0.11.0
**Status:** Experimental
**Category:** Development Tools (Type System)
**Dependents:** Unknown (likely workspace crates using type-level programming)

### Scope

#### Responsibility

Aggregate and re-export type system utilities from workspace crates (inspect_type, is_slice, implements), providing a unified feature-gated interface for compile-time type checking and type introspection across the workspace.

#### In-Scope

1. **Trait Implementation Checking (implements)**
   - `implements!` macro - Check if type implements trait
   - Compile-time trait verification
   - Boolean return (true/false)
   - Works with standard and custom traits
   - Zero runtime cost

2. **Type Inspection (inspect_type)**
   - Type name extraction
   - Type size and alignment queries
   - Compile-time type properties
   - Generic type information
   - Debug formatting for types

3. **Slice Type Detection (is_slice)**
   - `is_slice!` macro - Check if type is slice
   - `[T]` vs `&[T]` vs other types
   - Compile-time slice detection
   - Array vs slice distinction

4. **Feature Architecture**
   - `enabled` - Master switch (default)
   - `typing_implements` - implements! macro (default)
   - `typing_inspect_type` - Type inspection (default)
   - `typing_is_slice` - Slice detection (default)
   - `no_std` / `use_alloc` support

5. **Traditional Namespace Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Dependency namespace for explicit access
   - Feature-gated re-exports
   - Typing-specific namespace

6. **Unified Import Point**
   - Single `use typing_tools::*;` for all utilities
   - Consistent macro naming
   - Cross-crate compatibility

#### Out-of-Scope

1. **NOT Runtime Type Information (RTTI)**
   - No runtime type queries
   - Compile-time only
   - **Rationale:** Rust doesn't have RTTI by design

2. **NOT Dynamic Dispatch Utilities**
   - No trait object helpers beyond type checking
   - No vtable manipulation
   - **Rationale:** Use reflect_tools for reflection

3. **NOT Type Conversions**
   - No From/Into implementations
   - No casting utilities
   - **Rationale:** Use derive_tools for conversions

4. **NOT Const Generic Utilities**
   - No const generic manipulation
   - Basic type checking only
   - **Rationale:** Limited const generic support in stable Rust

5. **NOT Procedural Macro Authoring Tools**
   - No syn/quote wrappers
   - No AST utilities
   - **Rationale:** Use macro_tools for proc macros

6. **NOT Generic Type Construction**
   - No type builder patterns
   - No type-level programming framework
   - **Rationale:** Limited scope to checking

7. **NOT Phantom Type Utilities**
   - No PhantomData helpers beyond basic usage
   - **Rationale:** Use derive_tools::Phantom

8. **NOT Trait Object Utilities**
   - No dyn trait helpers
   - No object safety checking
   - **Rationale:** Basic checks only

#### Boundaries

- **typing_tools vs inspect_type**: typing_tools aggregates inspect_type; inspect_type is standalone inspection
- **typing_tools vs macro_tools**: typing_tools checks types; macro_tools manipulates AST
- **typing_tools vs reflect_tools**: typing_tools compile-time; reflect_tools runtime reflection

## Architecture

### Dependency Structure

```
typing_tools (facade, aggregation)
├── Internal Dependencies (workspace)
│   ├── inspect_type (type inspection)
│   ├── is_slice (slice detection)
│   └── implements (trait checking)
└── Dev Dependencies
    └── test_tools (workspace, testing)
```

**Note:** All production dependencies are workspace-internal

### Module Organization

```
typing_tools
├── lib.rs (facade aggregation)
├── typing.rs - Typing utilities namespace
│   ├── Re-exports from inspect_type
│   ├── Re-exports from is_slice
│   └── Re-exports from implements
├── dependency/ - Explicit dependency access
│   ├── inspect_type
│   ├── is_slice
│   └── implements
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Pure facade with traditional namespace organization

### Feature Architecture

```
enabled (master switch, default)
│
├── typing_implements (default)
│   └── implements! - Trait implementation checking
│
├── typing_inspect_type (default)
│   ├── Type name extraction
│   ├── Size and alignment queries
│   └── Type property inspection
│
└── typing_is_slice (default)
    └── is_slice! - Slice type detection

full (all features)
no_std (embedded support)
use_alloc (no_std + allocation)
```

**Default Features:** `enabled`, `typing_implements`, `typing_inspect_type`, `typing_is_slice`

### Macro Flow

#### implements! Macro Flow

```
let src = Box::new(true);
implements!(src => Clone)
  ↓
Compile-time check: Does Box<bool> implement Clone?
  ↓
Expands to: true or false
  ↓
Boolean value available at compile-time
```

#### is_slice! Macro Flow

```
is_slice!(Vec<i32>)
  ↓
Compile-time check: Is Vec<i32> a slice type?
  ↓
Expands to: false (Vec is not a slice)
  ↓
Boolean value available at compile-time

is_slice!(&[i32])
  ↓
Expands to: true (slice reference)
```

## Public API

### Trait Implementation Checking (implements)

```rust
#[cfg(feature = "typing_implements")]
pub use ::implements::*;

// Primary macro:
// implements!(value => Trait)
// implements!(Type => Trait)
```

### Type Inspection (inspect_type)

```rust
#[cfg(feature = "typing_inspect_type")]
pub use ::inspect_type::*;

// Type inspection utilities
// (specific API depends on inspect_type crate)
```

### Slice Detection (is_slice)

```rust
#[cfg(feature = "typing_is_slice")]
pub use ::is_slice::*;

// Primary macro:
// is_slice!(Type)
// is_slice!(value)
```

### Dependency Namespace

```rust
pub mod dependency {
  #[cfg(feature = "typing_inspect_type")]
  pub use ::inspect_type;

  #[cfg(feature = "typing_is_slice")]
  pub use ::is_slice;

  #[cfg(feature = "typing_implements")]
  pub use ::implements;
}
```

## Usage Patterns

### Pattern 1: Trait Implementation Checking

```rust
use typing_tools::*;

let boxed = Box::new(42);

// Check if type implements trait
assert_eq!(implements!(boxed => Copy), false);
assert_eq!(implements!(boxed => Clone), true);
assert_eq!(implements!(boxed => Send), true);

// Works with values or types
assert_eq!(implements!(i32 => Copy), true);
assert_eq!(implements!(String => Copy), false);
```

### Pattern 2: Conditional Compilation Based on Traits

```rust
use typing_tools::*;

fn process<T>(value: T) {
  if implements!(T => Copy) {
    // Fast path for Copy types
    let copy = value;
    // Use copy
  } else {
    // Slow path for non-Copy types
    // Move value
  }
}
```

### Pattern 3: Slice Type Detection

```rust
use typing_tools::*;

assert_eq!(is_slice!(Vec<i32>), false);
assert_eq!(is_slice!(&[i32]), true);
assert_eq!(is_slice!([i32; 5]), false);
assert_eq!(is_slice!(&str), false);
```

### Pattern 4: Generic Constraints Checking

```rust
use typing_tools::*;

fn requires_clone<T>(value: T) {
  // Compile-time assertion
  assert!(implements!(T => Clone));
  // Now safe to clone
}
```

### Pattern 5: Type Property Queries

```rust
use typing_tools::*;

// Type inspection (exact API depends on inspect_type)
// Get type name, size, alignment, etc.
```

### Pattern 6: Debug Type Information

```rust
use typing_tools::*;

fn debug_type_info<T>() {
  println!("Implements Copy: {}", implements!(T => Copy));
  println!("Implements Clone: {}", implements!(T => Clone));
  println!("Implements Send: {}", implements!(T => Send));
  println!("Implements Sync: {}", implements!(T => Sync));
}

debug_type_info::<String>();
```

### Pattern 7: Slice vs Array Distinction

```rust
use typing_tools::*;

fn process_sequence<T>(data: T) {
  if is_slice!(T) {
    // Handle slice types
  } else {
    // Handle other types
  }
}
```

### Pattern 8: Combined Type Checking

```rust
use typing_tools::*;

fn optimize<T>(value: T) {
  let is_copyable = implements!(T => Copy);
  let is_cloneable = implements!(T => Clone);

  if is_copyable {
    // Use bitwise copy
  } else if is_cloneable {
    // Use clone method
  } else {
    // Must move
  }
}
```

## Dependencies and Consumers

### Direct Dependencies

**Workspace:**
- `inspect_type` (feature: `typing_inspect_type`) - Type inspection utilities
- `is_slice` (feature: `typing_is_slice`) - Slice type detection
- `implements` (feature: `typing_implements`) - Trait implementation checking

**Dev:**
- `test_tools` (workspace) - Testing utilities

### Consumers (Unknown)

**Likely used by:**
- Workspace crates using type-level programming
- Generic code requiring trait bounds verification
- Compile-time optimization paths
- Meta-programming utilities

**Usage Pattern:** Workspace crates use typing_tools for compile-time type checking and conditional compilation based on type properties.

## Design Rationale

### Why Facade Pattern?

Aggregates type system utilities into single crate:

**Benefits:**
1. **Single Import**: One `use typing_tools::*;` for all type utilities
2. **Unified Documentation**: Centralized type system reference
3. **Feature Control**: Granular dependency management
4. **Consistency**: Common interface across utilities

**Tradeoff:** Indirection layer, but provides simplicity

### Why Three Separate Utilities?

Separates implements, inspect_type, and is_slice:

**Rationale:**
1. **Single Responsibility**: Each utility has focused purpose
2. **Reusability**: Can use individually if needed
3. **Testing**: Isolated test suites
4. **Performance**: Only compile what you need

**Aggregation:** typing_tools provides convenient unified access

### Why Compile-Time Only?

No runtime type checking:

**Rationale:**
1. **Zero Cost**: No runtime overhead
2. **Rust Philosophy**: Type safety at compile-time
3. **Performance**: Optimized away entirely
4. **Simplicity**: No runtime complexity

**Alternative:** Use reflect_tools for runtime reflection

### Why Feature-Gate Everything?

Each utility has own feature flag:

**Rationale:**
1. **Compile Time**: Only compile needed utilities
2. **Dependencies**: Minimize dependency tree
3. **Binary Size**: Exclude unused checking code
4. **Flexibility**: Fine-grained control

**Default:** Enable all utilities for convenience

### Why implements! Instead of Trait Bounds?

Macro for checking vs generic bounds:

**Rationale:**
1. **Dynamic Checking**: Check without generic parameters
2. **Conditional Logic**: Use in if expressions
3. **Debugging**: Verify trait implementations
4. **Flexibility**: Works with values and types

**Complement:** Use with trait bounds, not replacement

### Why Traditional Namespaces?

Uses own/orphan/exposed/prelude pattern:

**Rationale:**
1. **Consistency**: Matches other workspace crates
2. **Control**: Fine-grained re-export control
3. **Documentation**: Clear import paths
4. **Compatibility**: Standard Rust patterns

**Benefit:** Familiar to workspace developers

### Why No AST Utilities?

Doesn't provide proc macro tools:

**Rationale:**
1. **Scope**: Focus on type checking
2. **Separation**: macro_tools handles AST
3. **Simplicity**: Declarative macros only
4. **Performance**: Fast compilation

**Alternative:** Use macro_tools for AST manipulation

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for comprehensive testing
- Macro expansion testing

### Test Focus

1. **Trait Checking**: Verify implements! accuracy
2. **Slice Detection**: Test is_slice! correctness
3. **Edge Cases**: Empty types, ZSTs, trait objects
4. **Feature Combinations**: Different feature sets
5. **Compilation**: Ensure macros expand correctly

### Known Test Limitations

1. **Macro Testing**: Cannot easily test intermediate expansion
2. **Compile-Time**: Hard to test negative cases
3. **Trait Checking**: Limited by Rust type system
4. **Coverage**: Some type properties hard to test

## Future Considerations

### Potential Enhancements

1. **More Type Queries**: Additional type property checks
2. **Better Error Messages**: Clearer macro diagnostics
3. **Const Generic Support**: When stable Rust supports it
4. **Type-Level Programming**: More type manipulation utilities
5. **Performance Optimization**: Faster macro expansion
6. **Documentation**: More usage examples
7. **Integration**: Better integration with other tools

### Breaking Changes to Consider

1. **Rename Features**: Shorter feature names
2. **Change Defaults**: Adjust default feature set
3. **Unify API**: Single checking macro
4. **Namespace Simplification**: Flatten module structure
5. **Extend Checking**: More comprehensive checks

### Known Limitations

1. **Compile-Time Only**: No runtime type information
2. **Macro Limitations**: Subject to Rust macro constraints
3. **Trait Checking**: Only works with traits in scope
4. **Type System**: Limited by Rust's type system
5. **No Reflection**: Not a full reflection framework

## Adoption Guidelines

### When to Use typing_tools

**Good Candidates:**
- Generic code with trait-dependent logic
- Compile-time type verification
- Conditional compilation based on traits
- Debug type information
- Meta-programming
- Type-level programming

**Poor Candidates:**
- Runtime type checking (use reflect_tools)
- Type conversions (use derive_tools)
- AST manipulation (use macro_tools)
- Simple trait bounds (use where clauses)

### Choosing Which Utilities

```rust
// Trait implementation checking
use typing_tools::*;
if implements!(T => Clone) {
  // Clone-specific logic
}

// Slice detection
if is_slice!(T) {
  // Slice-specific handling
}

// Type inspection
// Use inspect_type utilities for type properties
```

### Best Practices

1. **Prefer Trait Bounds**: Use where clauses when possible
2. **Compile-Time Checks**: Use for conditional compilation
3. **Documentation**: Document why type checks needed
4. **Feature Selection**: Only enable needed utilities
5. **Test Edge Cases**: Verify with various types
6. **Combine Tools**: Use with other type system tools

## Related Crates

**Dependencies:**
- **inspect_type**: Type inspection utilities (workspace)
- **is_slice**: Slice type detection (workspace)
- **implements**: Trait checking (workspace)

**Related:**
- **macro_tools**: AST manipulation utilities (workspace)
- **reflect_tools**: Runtime reflection (workspace)
- **derive_tools**: Derive macro utilities (workspace)

**Alternatives:**
- **std::any**: Runtime type identification (standard library)
- **typetag**: Serializable trait objects
- **inventory**: Type collection at runtime

## References

- [API Documentation](https://docs.rs/typing_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/typing_tools)
- [readme.md](./readme.md)
- [inspect_type](../inspect_type/readme.md) - Type inspection
- [is_slice](../is_slice/readme.md) - Slice detection
- [implements](../implements/readme.md) - Trait checking

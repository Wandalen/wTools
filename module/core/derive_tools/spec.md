# Specification: derive_tools

## Overview

**derive_tools** is a comprehensive facade crate aggregating derive macros from multiple sources - both workspace-internal and external - providing a unified, feature-gated interface for deriving common Rust traits. It serves as the workspace's one-stop solution for ergonomic trait implementations, eliminating boilerplate through 30+ derive macros spanning arithmetic operations, conversions, display formatting, enum utilities, and trait object cloning.

**Version:** 0.56.0
**Status:** Experimental
**Category:** Development Tools (Derive Macros)
**Dependents:** Unknown (likely most workspace crates)

### Scope

#### Responsibility

Aggregate and re-export derive macros from workspace (derive_tools_meta, variadic_from, clone_dyn) and external sources (derive_more, strum, parse-display), providing a unified feature-gated interface for ergonomic trait derivation across the workspace.

#### In-Scope

1. **Workspace Derive Macros (derive_tools_meta)**
   - `From` - core::convert::From trait
   - `InnerFrom` - Conversion from inner type
   - `New` - Constructor methods
   - `Not` - Bitwise/logical negation
   - `VariadicFrom` - Variadic From implementations
   - `AsMut` / `AsRef` - Reference conversions
   - `Deref` / `DerefMut` - Smart pointer dereferencing
   - `Index` / `IndexMut` - Indexing operations
   - `Phantom` - PhantomData field management

2. **Arithmetic Derives (derive_more)**
   - `Add` / `Sub` - Addition and subtraction
   - `Mul` / `Div` - Multiplication and division
   - `AddAssign` / `SubAssign` - Compound assignment
   - `MulAssign` / `DivAssign` - Compound assignment
   - `Sum` - Iterator summation

3. **Conversion Derives (derive_more)**
   - `Into` - Conversion into target type
   - `TryInto` - Fallible conversion
   - `IntoIterator` - Iterator conversion
   - `Constructor` - Struct constructors

4. **Enum Utilities (derive_more + strum)**
   - `IsVariant` - Variant checking methods
   - `Unwrap` - Variant extraction
   - Strum enum utilities (AsRefStr, Display, EnumIter, etc.)
   - Perfect hash functions (strum_phf feature)

5. **Display/Parsing (parse-display)**
   - `Display` - Custom Display formatting
   - `FromStr` - String parsing

6. **Trait Object Cloning (clone_dyn)**
   - `CloneDyn` - Clone for trait objects
   - no_std compatible trait object cloning

7. **Variadic From (variadic_from)**
   - Type-level variadic From implementations
   - Multi-argument From constructors

8. **Feature Architecture**
   - `enabled` - Master switch (default)
   - ~30 individual derive features (e.g., `derive_from`, `derive_add`)
   - Granular dependency control
   - `no_std` / `use_alloc` support

9. **Traditional Namespace Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Dependency namespace for explicit access
   - Feature-gated re-exports

10. **Unified Documentation**
    - Single import point: `use derive_tools::*;`
    - Consistent attribute syntax across derives
    - Comprehensive examples for each derive

#### Out-of-Scope

1. **NOT Custom Proc Macro Implementation**
   - Aggregates existing macros, doesn't implement new ones directly
   - Custom workspace macros in derive_tools_meta
   - **Rationale:** Facade pattern for unification

2. **NOT Runtime Trait Implementation**
   - No dynamic trait dispatch beyond clone_dyn
   - Compile-time code generation only
   - **Rationale:** Derive macros are compile-time

3. **NOT Custom Syntax Beyond Attributes**
   - Standard `#[derive(...)]` syntax
   - No procedural attribute macros
   - **Rationale:** Simplicity and familiarity

4. **NOT Auto-Trait Implementation**
   - Doesn't implement Send/Sync/Unpin automatically
   - Only explicit user-requested traits
   - **Rationale:** Auto-traits are compiler magic

5. **NOT Generic Derive Dispatch**
   - No single "derive everything" macro
   - Each derive must be explicit
   - **Rationale:** Clarity over convenience

6. **NOT Foreign Trait Derivation**
   - Cannot derive traits from external crates (beyond aggregated ones)
   - Only known trait implementations
   - **Rationale:** Orphan rules and proc macro limitations

7. **NOT Dynamic Derive Selection**
   - Feature flags determined at compile-time
   - No conditional derive based on runtime config
   - **Rationale:** Derive macros are compile-time

8. **NOT Custom Error Types**
   - Macros use default error handling from sources
   - No unified error type across derives
   - **Rationale:** Each source has own error handling

#### Boundaries

- **derive_tools vs derive_more**: derive_tools aggregates derive_more; derive_more is standalone
- **derive_tools vs derive_tools_meta**: derive_tools is facade; derive_tools_meta implements workspace derives
- **derive_tools vs std derives**: derive_tools provides additional derives; std provides core derives (Clone, Debug, etc.)

## Architecture

### Dependency Structure

```
derive_tools (facade, aggregation)
├── Internal Dependencies (workspace)
│   ├── derive_tools_meta (proc macros: From, AsMut, Deref, Index, etc.)
│   ├── variadic_from (variadic From implementations)
│   └── clone_dyn (trait object cloning)
├── External Dependencies (crates.io)
│   ├── derive_more (optional, arithmetic/conversion derives)
│   ├── strum (optional, enum utilities)
│   └── parse-display (optional, Display/FromStr)
└── Dev Dependencies
    ├── test_tools (workspace, testing)
    └── macro_tools (workspace, macro utilities for tests)
```

**Note:** All production dependencies are optional and feature-gated

### Module Organization

```
derive_tools
├── lib.rs (facade aggregation)
│   ├── Re-exports from derive_tools_meta
│   ├── Re-exports from derive_more module
│   ├── Re-exports from strum
│   ├── Re-exports from parse-display
│   ├── Re-exports from clone_dyn
│   └── Re-exports from variadic_from
├── dependency/ - Explicit dependency access
│   ├── derive_tools_meta
│   ├── derive_more
│   ├── strum
│   ├── parse_display
│   ├── clone_dyn
│   └── variadic_from
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Pure facade with traditional namespace organization

### Feature Architecture

```
enabled (master switch, default)
│
├── Workspace Derives
│   ├── derive_from (From trait)
│   ├── derive_inner_from (InnerFrom)
│   ├── derive_new (New constructor)
│   ├── derive_not (Not trait)
│   ├── derive_variadic_from (VariadicFrom)
│   ├── derive_as_mut / derive_as_ref
│   ├── derive_deref / derive_deref_mut
│   ├── derive_index / derive_index_mut
│   └── derive_phantom (PhantomData)
│
├── derive_more Derivatives (require derive_more dependency)
│   ├── derive_add / derive_add_assign
│   ├── derive_mul / derive_mul_assign
│   ├── derive_constructor
│   ├── derive_error
│   ├── derive_into / derive_try_into
│   ├── derive_into_iterator
│   ├── derive_sum
│   ├── derive_is_variant
│   └── derive_unwrap
│
├── Enum Utilities
│   ├── derive_strum (enable strum derives)
│   └── strum_phf (perfect hash functions)
│
├── Display/Parsing
│   ├── derive_display (parse-display)
│   └── derive_from_str (parse-display)
│
├── Trait Objects
│   └── derive_clone_dyn (clone_dyn)
│
└── Variadic
    ├── type_variadic_from (types only)
    └── derive_variadic_from (derive + types)

full (all features)
no_std (embedded support)
use_alloc (no_std + allocation)
```

**Default Features:** `enabled` + most individual derives (~25 features)

### Derive Flow

#### Basic Derive Flow

```
#[derive(From)]
struct Wrapper(i32);
  ↓
Compiler invokes From proc macro
  ↓
derive_tools_meta::From expands
  ↓
impl From<i32> for Wrapper {
  fn from(value: i32) -> Self {
    Self(value)
  }
}
  ↓
Generated code compiled into user's crate
```

#### Facade Re-export Flow

```
use derive_tools::*;
  ↓
Import exposed namespace
  ↓
Access to all enabled derive macros:
  ├─ derive_tools_meta::* (workspace)
  ├─ derive_more::* (external)
  ├─ strum::* (external)
  ├─ parse_display::* (external)
  ├─ clone_dyn::* (workspace)
  └─ variadic_from::* (workspace)
```

## Public API

### Workspace Derives (derive_tools_meta)

```rust
#[cfg(feature = "derive_from")]
pub use derive_tools_meta::From;

#[cfg(feature = "derive_inner_from")]
pub use derive_tools_meta::InnerFrom;

#[cfg(feature = "derive_new")]
pub use derive_tools_meta::New;

#[cfg(feature = "derive_not")]
pub use derive_tools_meta::Not;

#[cfg(feature = "derive_variadic_from")]
pub use derive_tools_meta::VariadicFrom;

#[cfg(feature = "derive_as_mut")]
pub use derive_tools_meta::AsMut;

#[cfg(feature = "derive_as_ref")]
pub use derive_tools_meta::AsRef;

#[cfg(feature = "derive_deref")]
pub use derive_tools_meta::Deref;

#[cfg(feature = "derive_deref_mut")]
pub use derive_tools_meta::DerefMut;

#[cfg(feature = "derive_index")]
pub use derive_tools_meta::Index;

#[cfg(feature = "derive_index_mut")]
pub use derive_tools_meta::IndexMut;

#[cfg(feature = "derive_phantom")]
pub use derive_tools_meta::Phantom;
```

### External Derives (derive_more)

```rust
#[cfg(feature = "derive_add")]
pub use ::derive_more::{Add, Sub};

#[cfg(feature = "derive_add_assign")]
pub use ::derive_more::{AddAssign, SubAssign};

#[cfg(feature = "derive_mul")]
pub use ::derive_more::{Mul, Div};

#[cfg(feature = "derive_mul_assign")]
pub use ::derive_more::{MulAssign, DivAssign};

#[cfg(feature = "derive_constructor")]
pub use ::derive_more::Constructor;

#[cfg(feature = "derive_error")]
pub use ::derive_more::Error;

#[cfg(feature = "derive_into")]
pub use ::derive_more::Into;

#[cfg(feature = "derive_try_into")]
pub use ::derive_more::TryInto;

#[cfg(feature = "derive_into_iterator")]
pub use ::derive_more::IntoIterator;

#[cfg(feature = "derive_sum")]
pub use ::derive_more::Sum;

#[cfg(feature = "derive_is_variant")]
pub use ::derive_more::IsVariant;

#[cfg(feature = "derive_unwrap")]
pub use ::derive_more::Unwrap;
```

### Display/Parsing Derives

```rust
#[cfg(feature = "derive_display")]
pub use ::parse_display::Display;

#[cfg(feature = "derive_from_str")]
pub use ::parse_display::FromStr;
```

### Enum Utilities

```rust
#[cfg(feature = "derive_strum")]
pub use ::strum::*; // All strum derives
```

### Trait Object Cloning

```rust
#[cfg(feature = "derive_clone_dyn")]
pub use ::clone_dyn::exposed::*;
```

### Variadic From

```rust
#[cfg(any(feature = "derive_variadic_from", feature = "type_variadic_from"))]
pub use variadic_from as variadic;
```

### Dependency Namespace

```rust
pub mod dependency {
  pub use ::derive_tools_meta;

  #[cfg(feature = "derive_clone_dyn")]
  pub use ::clone_dyn::{self, dependency::*};

  #[cfg(any(feature = "derive_variadic_from", feature = "type_variadic_from"))]
  pub use ::variadic_from::{self, dependency::*};

  #[cfg(feature = "derive_more")]
  pub use ::derive_more;

  #[cfg(feature = "derive_strum")]
  pub use ::strum;

  #[cfg(feature = "parse_display")]
  pub use ::parse_display;
}
```

## Usage Patterns

### Pattern 1: Basic From Derive

```rust
use derive_tools::*;

#[derive(From, PartialEq, Debug)]
struct UserId(u64);

let id: UserId = 42u64.into();
assert_eq!(id, UserId(42));
```

### Pattern 2: Display and FromStr

```rust
use derive_tools::*;
use std::str::FromStr;

#[derive(From, Display, FromStr, PartialEq, Debug)]
#[display("{0}")]
struct Percentage(i32);

// Derived Display
let p = Percentage(75);
assert_eq!(format!("{}", p), "75");

// Derived FromStr
let p = Percentage::from_str("42").unwrap();
assert_eq!(p, Percentage(42));
```

### Pattern 3: Arithmetic Operations

```rust
use derive_tools::*;

#[derive(From, Add, Mul, PartialEq, Debug)]
struct Distance(f64);

let d1 = Distance(10.0);
let d2 = Distance(20.0);
let sum = d1 + d2;
assert_eq!(sum, Distance(30.0));
```

### Pattern 4: Deref for Newtype

```rust
use derive_tools::*;

#[derive(From, Deref, DerefMut)]
struct Username(String);

let mut name = Username("alice".to_string());
name.push_str("_42"); // Deref to String
assert_eq!(&*name, "alice_42");
```

### Pattern 5: Enum IsVariant

```rust
use derive_tools::*;

#[derive(IsVariant)]
enum Status {
  Active,
  Inactive,
  Pending(String),
}

let status = Status::Active;
assert!(status.is_active());
assert!(!status.is_inactive());
```

### Pattern 6: Constructor Derive

```rust
use derive_tools::*;

#[derive(Constructor)]
struct Point {
  x: i32,
  y: i32,
}

let p = Point::new(10, 20);
assert_eq!(p.x, 10);
assert_eq!(p.y, 20);
```

### Pattern 7: AsRef and AsMut

```rust
use derive_tools::*;

#[derive(AsRef, AsMut)]
struct Wrapper(Vec<i32>);

let mut w = Wrapper(vec![1, 2, 3]);
let v: &Vec<i32> = w.as_ref();
assert_eq!(v.len(), 3);

let v_mut: &mut Vec<i32> = w.as_mut();
v_mut.push(4);
assert_eq!(w.0.len(), 4);
```

### Pattern 8: Error Derive

```rust
use derive_tools::*;

#[derive(Error, Debug)]
#[error("Invalid value: {value}")]
struct ValidationError {
  value: String,
}

let err = ValidationError {
  value: "bad".to_string(),
};
println!("{}", err); // "Invalid value: bad"
```

### Pattern 9: Variadic From

```rust
use derive_tools::*;

#[derive(VariadicFrom)]
struct Config {
  host: String,
  port: u16,
}

// Can construct from multiple argument counts
let cfg = Config::from(("localhost".to_string(), 8080u16));
```

### Pattern 10: Clone for Trait Objects

```rust
use derive_tools::*;

#[derive(CloneDyn)]
trait MyTrait: CloneDyn {}

// Now can clone Box<dyn MyTrait>
```

## Dependencies and Consumers

### Direct Dependencies

**Workspace:**
- `derive_tools_meta` (optional, most derives) - Custom workspace derive macros
- `variadic_from` (optional) - Variadic From implementations
- `clone_dyn` (optional) - Trait object cloning

**External:**
- `derive_more` (optional) - Comprehensive derive macro collection
- `strum` (optional) - Enum utilities and derives
- `parse-display` (optional) - Display/FromStr parsing

**Build:**
- `cfg_aliases` (workspace) - Feature flag aliases

**Dev:**
- `test_tools` (workspace) - Testing utilities
- `macro_tools` (workspace) - Macro testing utilities

### Consumers (Unknown)

**Likely used by:**
- Most workspace crates for trait derives
- Application code for reducing boilerplate
- Library code for ergonomic APIs

**Usage Pattern:** Workspace crates use derive_tools as primary derive macro source, enabling specific features as needed for trait implementations.

## Design Rationale

### Why Facade Pattern?

Aggregates multiple derive macro sources into single crate:

**Benefits:**
1. **Single Import**: One `use derive_tools::*;` for all derives
2. **Unified Documentation**: Centralized derive reference
3. **Feature Control**: Granular dependency management
4. **Version Control**: Single version for all workspace derives

**Tradeoff:** Indirection layer, but provides consistency

### Why Feature-Gate Everything?

Each derive has its own feature flag:

**Rationale:**
1. **Compile Time**: Only compile needed derives
2. **Dependencies**: Minimize external dependencies
3. **Binary Size**: Exclude unused macro code
4. **Flexibility**: Fine-grained control

**Default:** Enable most common derives for convenience

### Why Mix Internal and External?

Combines workspace and crates.io derives:

**Rationale:**
1. **Completeness**: Fill gaps in external crates
2. **Control**: Own critical derive implementations
3. **Standards**: Use proven external solutions when available
4. **Flexibility**: Choose best-of-breed for each derive

**Pattern:** Internal for workspace-specific, external for standard

### Why Traditional Namespaces?

Uses own/orphan/exposed/prelude pattern:

**Rationale:**
1. **Consistency**: Matches other workspace crates
2. **Control**: Fine-grained re-export control
3. **Documentation**: Clear import paths
4. **Compatibility**: Standard Rust patterns

**Benefit:** Familiar to workspace developers

### Why Dependency Namespace?

Explicit `dependency` module:

**Rationale:**
1. **Explicit Access**: Direct access to source crates
2. **Debugging**: Check which crate provides derive
3. **Advanced Usage**: Access non-derive items
4. **Transparency**: Clear dependency relationships

**Use Case:** When you need source-specific features

### Why No Custom Implementations?

Aggregates existing macros instead of reimplementing:

**Rationale:**
1. **Maintenance**: Don't maintain duplicate implementations
2. **Quality**: Use battle-tested external crates
3. **Focus**: Focus on workspace-specific derives only
4. **Community**: Leverage ecosystem expertise

**Exception:** Workspace derives in derive_tools_meta

### Why Not One Giant Derive?

Requires explicit derive for each trait:

**Rationale:**
1. **Explicitness**: Clear what traits are implemented
2. **Compile Errors**: Better error messages
3. **Opt-In**: Users choose what to derive
4. **No Magic**: Predictable behavior

**Tradeoff:** More verbose but clearer

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for comprehensive testing
- Integration tests with derived traits

### Test Files

```
tests/
├── derive_from_tests.rs - From derive tests
├── derive_display_tests.rs - Display/FromStr tests
├── derive_arithmetic_tests.rs - Add/Mul etc. tests
└── integration/ - Cross-derive integration tests
```

### Test Focus

1. **Individual Derives**: Each derive tested in isolation
2. **Combinations**: Multiple derives on same type
3. **Edge Cases**: Empty structs, unit variants, etc.
4. **Feature Gates**: Test with different feature combinations
5. **Error Messages**: Verify helpful compilation errors

### Known Test Limitations

1. **Proc Macro Testing**: Cannot test expansion directly
2. **Compilation Tests**: Rely on successful compilation
3. **Error Testing**: trybuild for compile-fail tests
4. **Feature Matrix**: Combinatorial explosion of features

## Future Considerations

### Potential Enhancements

1. **More Workspace Derives**: Expand derive_tools_meta coverage
2. **Better Error Messages**: Improve proc macro diagnostics
3. **Documentation Generation**: Auto-generate derive docs
4. **Derive Combinations**: Optimize common derive sets
5. **Attribute Validation**: Better attribute error checking
6. **no_std Expansion**: More no_std compatible derives
7. **Async Derives**: Async trait implementations

### Breaking Changes to Consider

1. **Rename Features**: Shorter feature names
2. **Change Defaults**: Adjust default feature set
3. **Remove External Deps**: Replace with workspace impls
4. **Unified Attributes**: Common attribute syntax
5. **Namespace Simplification**: Flatten module structure

### Known Limitations

1. **Proc Macro Limitations**: Cannot access type information across crates
2. **Feature Overhead**: Many features slow down compilation
3. **Documentation**: Each derive documented separately
4. **Error Messages**: Vary by source crate
5. **No Dynamic Selection**: All features compile-time only

## Adoption Guidelines

### When to Use derive_tools

**Good Candidates:**
- Newtype pattern wrappers
- Data transfer objects
- Configuration structs
- Value objects
- Enums with common operations
- Trait object wrappers

**Poor Candidates:**
- Complex trait implementations requiring custom logic
- Performance-critical code paths (proc macros have overhead)
- Foreign traits (orphan rules)
- Dynamic trait selection

### Choosing Which Derives

```rust
// Newtype wrapper: From + Deref + AsRef
#[derive(From, Deref, AsRef)]
struct UserId(u64);

// Data object: Display + FromStr + Constructor
#[derive(Display, FromStr, Constructor)]
#[display("{name}:{age}")]
struct Person { name: String, age: u32 }

// Numeric wrapper: arithmetic operations
#[derive(From, Add, Mul, AddAssign)]
struct Meters(f64);

// Enum: variant utilities
#[derive(IsVariant, Unwrap)]
enum Result { Ok(i32), Err(String) }
```

### Best Practices

1. **Minimal Features**: Only enable derives you need
2. **Default First**: Start with default features
3. **Combine Derives**: Use multiple compatible derives
4. **Test Compilation**: Verify derives work as expected
5. **Read Source Docs**: Check source crate documentation
6. **Use Attributes**: Configure derives with attributes

## Related Crates

**Dependencies:**
- **derive_more**: Comprehensive derive macro collection
- **strum**: Enum utilities and string conversions
- **parse-display**: Display/FromStr with custom syntax
- **derive_tools_meta**: wTools workspace custom derives
- **variadic_from**: Variadic From implementations
- **clone_dyn**: Clone for trait objects

**Alternatives:**
- **bon**: Builder pattern derives
- **getset**: Getter/setter derives
- **educe**: Extended derive macros
- **smart-default**: Default with customization

## References

- [API Documentation](https://docs.rs/derive_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/derive_tools)
- [readme.md](./readme.md)
- [derive_more](https://docs.rs/derive_more) - Arithmetic and conversion derives
- [strum](https://docs.rs/strum) - Enum utilities
- [parse-display](https://docs.rs/parse-display) - Display/FromStr parsing
- [derive_tools_meta](../derive_tools_meta/readme.md) - Workspace custom derives
- [variadic_from](../variadic_from/readme.md) - Variadic From implementations
- [clone_dyn](../clone_dyn/readme.md) - Trait object cloning

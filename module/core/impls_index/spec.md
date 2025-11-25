# Specification: impls_index

## Overview

**impls_index** is a code organization utility providing macros that separate function definitions from their implementations by wrapping each function in a named macro, enabling developers to create high-level code indexes that show the big picture without implementation details. It encourages writing better code by making the structure explicit through mandatory function listing in index macros.

**Version:** 0.11.0
**Status:** Experimental
**Category:** Development Tools (Code Organization)
**Dependents:** Unknown (likely workspace crates using structured code organization)

### Scope

#### Responsibility

Provide macro-based function indexing utilities that wrap function definitions in named macros, enabling separation of code structure (what functions exist) from implementation details (how functions work), improving code comprehension and maintainability through explicit function indexes.

#### In-Scope

1. **Function Indexing (impls1)**
   - `impls1!` macro - Basic declarative function indexing
   - Wraps each function in a named macro
   - `#[deny(unused_macros)]` enforcement
   - Simple syntax for quick adoption
   - Direct function definition

2. **Advanced Indexing (impls3/impls)**
   - `impls3!` proc macro - Advanced function indexing
   - Aliased as `impls!` for convenience
   - Renaming support via `as` syntax
   - Better error messages
   - Full procedural macro capabilities

3. **Function Invocation (index)**
   - `index!` macro - Invoke indexed functions
   - Comma-separated function list
   - Rename support: `f1 as f1_alias`
   - Generates actual function definitions
   - Ensures all indexed functions used

4. **Optional Functions (impls_optional)**
   - `impls_optional!` - Allow unused indexed functions
   - `#[allow(unused_macros)]` instead of deny
   - For conditional compilation
   - Flexible function usage
   - No mandatory indexing

5. **Test Function Indexing (tests_impls)**
   - `tests_impls!` - Index test functions
   - Automatically adds `#[test]` attribute
   - Works with `tests_index!` (alias of `index!`)
   - `tests_impls_optional!` for optional tests
   - Structured test organization

6. **Function Manipulation Utilities**
   - `fn_name!` - Extract function name from definition
   - `fn_rename!` - Rename function with prefix/postfix
   - `fns!` / `fns2!` - Split and process multiple functions
   - Token tree manipulation
   - Callback-based processing

7. **Alternative Implementations (impls2)**
   - `impls2!` - Uses `fns!` callback internally
   - Different implementation approach
   - More complex parsing
   - Renaming support

8. **Traditional Namespace Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Dependency namespace for impls_index_meta
   - implsindex submodule organization
   - Clean re-export structure

#### Out-of-Scope

1. **NOT Runtime Function Registration**
   - No dynamic function registration
   - Compile-time only
   - **Rationale:** Macro-based system, not runtime reflection

2. **NOT Method Indexing**
   - Only free functions supported
   - No impl block methods
   - **Rationale:** Focus on function-level organization

3. **NOT Struct/Enum Indexing**
   - No type definition indexing
   - Functions only
   - **Rationale:** Different use case, different tools

4. **NOT Code Generation Beyond Functions**
   - No struct generation
   - No trait impl generation
   - **Rationale:** Focused scope on function organization

5. **NOT Automatic Index Generation**
   - Manual `index!` invocation required
   - No automatic discovery
   - **Rationale:** Explicit is better than implicit

6. **NOT Cross-Module Indexing**
   - Index works within single scope
   - No cross-module function collection
   - **Rationale:** Scope limitations

7. **NOT Function Overloading**
   - Each function name must be unique
   - No overload support
   - **Rationale:** Rust doesn't support overloading

8. **NOT Documentation Generation**
   - No automatic doc generation from index
   - Manual documentation required
   - **Rationale:** Use rustdoc for documentation

#### Boundaries

- **impls_index vs mod_interface**: impls_index indexes functions; mod_interface manages module visibility
- **impls_index vs derive macros**: impls_index organizes code; derive macros generate code from types
- **impls_index vs test frameworks**: impls_index structures tests; test frameworks run tests

## Architecture

### Dependency Structure

```
impls_index (function indexing runtime)
├── Internal Dependencies (workspace)
│   └── impls_index_meta (proc macro, impls3!)
└── Dev Dependencies
    └── test_tools (workspace, testing)

impls_index_meta (proc macro support)
├── Internal Dependencies (workspace)
│   └── macro_tools (AST utilities, enabled feature)
└── Dev Dependencies
    └── (none)
```

**Note:** Two-crate pattern: runtime + proc macro support

### Module Organization

```
impls_index
├── lib.rs (top-level aggregation)
├── implsindex/ - Function indexing module
│   ├── mod.rs - Module aggregation
│   ├── func.rs - Function manipulation utilities
│   │   ├── fn_name! - Extract function name
│   │   ├── fn_rename! - Rename function
│   │   ├── fns! - Process functions with callback
│   │   └── fns2! - Alternative function processing
│   └── impls.rs - Implementation indexing macros
│       ├── index! - Invoke indexed functions
│       ├── impls1! - Basic indexing
│       ├── impls2! - Alternative indexing
│       ├── impls_optional! - Optional functions
│       ├── tests_impls! - Test indexing
│       ├── tests_impls_optional! - Optional tests
│       └── _impls_callback - Internal callback
└── Standard namespaces: own, orphan, exposed, prelude

impls_index_meta
├── lib.rs (proc macro entry)
└── impls.rs - Proc macro implementation
    └── impls3! - Advanced indexing proc macro
```

**Pattern:** Runtime + proc macro crate separation

### Feature Architecture

```
enabled (master switch, default)
├── Controls macro availability
└── Enables impls_index_meta/enabled

full (all features, same as default)
```

**Default Features:** `enabled`

### Macro Expansion Flow

#### impls1! Flow (Basic Indexing)

```
impls1! {
  fn f1() { println!("f1"); }
  pub fn f2() { println!("f2"); }
}
  ↓
Generates for each function:
  #[deny(unused_macros)]
  macro_rules! f1 {
    () => { fn f1() { println!("f1"); } };
  }
  #[deny(unused_macros)]
  macro_rules! f2 {
    () => { pub fn f2() { println!("f2"); } };
  }
```

#### index! Flow (Function Invocation)

```
index! {
  f1,
  f2 as f2_renamed,
}
  ↓
Expands to:
  f1!();              // Generates: fn f1() { println!("f1"); }
  f2!(as f2_renamed); // Generates: pub fn f2_renamed() { println!("f2"); }
```

#### impls3! Flow (Proc Macro Indexing)

```
impls3! {
  fn f1() { println!("f1"); }
}
  ↓
Proc macro parses functions
  ↓
Generates:
  #[deny(unused_macros)]
  macro_rules! f1 {
    (as $Name2:ident) => {
      fn $Name2() { println!("f1"); }
    };
    () => {
      fn f1() { println!("f1"); }
    };
  }
```

#### tests_impls! Flow

```
tests_impls! {
  fn test1() { assert!(true); }
}
  ↓
Generates:
  #[deny(unused_macros)]
  macro_rules! test1 {
    () => {
      #[test]
      fn test1() { assert!(true); }
    };
  }
```

## Public API

### Main Indexing Macros

```rust
/// Basic function indexing (declarative macro)
#[macro_export]
macro_rules! impls1 {
  // Wraps each function in a named macro with #[deny(unused_macros)]
}

/// Advanced function indexing (proc macro, aliased as impls!)
#[proc_macro]
pub fn impls3(input: TokenStream) -> TokenStream;

/// Optional function indexing (allows unused)
#[macro_export]
macro_rules! impls_optional {
  // Same as impls1 but with #[allow(unused_macros)]
}

/// Alternative indexing using fns! callback
#[macro_export]
macro_rules! impls2 {
  // Uses fns! for parsing
}
```

### Index Invocation Macro

```rust
/// Invoke indexed functions to generate definitions
#[macro_export]
macro_rules! index {
  // Empty
  () => {};

  // With renaming
  ($Name:ident as $Alias:ident, $($Rest:tt)*) => {
    $Name!(as $Alias);
    index!($($Rest)*);
  };

  // Without renaming
  ($Name:ident $(,$($Rest:tt)*)?) => {
    $Name!();
    index!($($Rest)*);
  };
}

/// Alias for test function indexing
pub use index as tests_index;
```

### Test Indexing Macros

```rust
/// Index test functions with automatic #[test] attribute
#[macro_export]
macro_rules! tests_impls {
  // Adds #[test] to each function
}

/// Optional test indexing
#[macro_export]
macro_rules! tests_impls_optional {
  // Same with #[allow(unused_macros)]
}
```

### Function Manipulation Utilities

```rust
/// Extract function name from definition
#[macro_export]
macro_rules! fn_name {
  (fn $Name:ident $($Rest:tt)*) => { $Name };
  ($First:tt $($Rest:tt)*) => { fn_name!($($Rest)*) };
}

/// Rename function
#[macro_export]
macro_rules! fn_rename {
  (@Prefix { $($Prefix:tt)* }
   @Name { $Name:ident }
   @Postfix { fn $OldName:ident $($Postfix:tt)* }
  ) => {
    $($Prefix)* fn $Name $($Postfix)*
  };
}

/// Process functions with callback
#[macro_export]
macro_rules! fns {
  (@Callback { $Callback:path }
   @Rest { $($Item:item)* }
  ) => {
    $($Callback!{ $Item })*
  };
}

/// Alternative function processing
#[macro_export]
macro_rules! fns2 {
  // Similar to fns! but different parsing approach
}
```

## Usage Patterns

### Pattern 1: Basic Function Indexing

```rust
use impls_index::*;

impls1! {
  fn f1() -> i32 {
    println!("f1() : 13");
    13
  }
}

index! {
  f1,
}

assert_eq!(f1(), 13);
// prints: f1() : 13
```

### Pattern 2: Multiple Functions with Visibility

```rust
use impls_index::*;

impls1! {
  fn internal_fn() {
    println!("internal");
  }

  pub fn public_fn() {
    println!("public");
  }
}

index! {
  internal_fn,
  public_fn,
}

internal_fn();
public_fn();
```

### Pattern 3: Function Renaming

```rust
use impls_index::*;

impls3! {
  fn original_name() {
    println!("function");
  }
}

// Rename during index
index! {
  original_name as new_name,
}

new_name(); // calls the function
```

### Pattern 4: Optional Functions

```rust
use impls_index::*;

impls_optional! {
  fn sometimes_used() {
    println!("conditional");
  }

  fn rarely_used() {
    println!("rarely");
  }
}

// Only index what's needed
index! {
  sometimes_used,
}
// rarely_used macro exists but not invoked (no warning)
```

### Pattern 5: Test Organization

```rust
use impls_index::*;

tests_impls! {
  fn test_feature_a() {
    assert_eq!(1 + 1, 2);
  }

  fn test_feature_b() {
    assert!(true);
  }
}

tests_index! {
  test_feature_a,
  test_feature_b,
}

// Generates:
// #[test]
// fn test_feature_a() { ... }
// #[test]
// fn test_feature_b() { ... }
```

### Pattern 6: Advanced Indexing (impls3)

```rust
use impls_index::*;

impls3! {
  fn func1() { println!("1"); }
  fn func2() { println!("2"); }
  fn func3() { println!("3"); }
}

index! {
  func1,
  func2 as renamed_func2,
  func3,
}

func1();
renamed_func2();
func3();
```

### Pattern 7: Function Name Extraction

```rust
use impls_index::*;

macro_rules! show_name {
  ($($fn_def:tt)*) => {
    println!("Function: {}", stringify!(fn_name!($($fn_def)*)));
  };
}

show_name!(fn example() {});
// prints: Function: example
```

### Pattern 8: Structured Module Organization

```rust
use impls_index::*;

// Define all functions in impls block
impls3! {
  fn init() { /* ... */ }
  fn process() { /* ... */ }
  fn cleanup() { /* ... */ }
}

// Create explicit index showing module structure
index! {
  init,
  process,
  cleanup,
}

// Usage shows clear function hierarchy
pub fn run() {
  init();
  process();
  cleanup();
}
```

## Dependencies and Consumers

### Direct Dependencies

**Workspace:**
- `impls_index_meta` (v0.13.0) - Proc macro support for impls3!

**Dev:**
- `test_tools` (workspace) - Testing utilities

### impls_index_meta Dependencies

**Workspace:**
- `macro_tools` (enabled feature) - AST utilities (syn, quote, proc-macro2)

### Consumers (Unknown)

**Likely used by:**
- Workspace crates with structured function organization
- Test suites with explicit test indexes
- Modules with many related functions
- Code requiring clear structure documentation

**Usage Pattern:** Workspace crates use impls_index to separate "what functions exist" from "how functions work", improving code comprehension and maintainability.

## Design Rationale

### Why Wrap Functions in Macros?

Each function gets a corresponding macro:

**Rationale:**
1. **Lazy Expansion**: Functions only generated when indexed
2. **Compile-Time Checking**: Unused functions detected via deny(unused_macros)
3. **Renaming**: Macro can rename function during expansion
4. **Organization**: Separates structure from implementation

**Benefit:** Clear separation between what exists and what's used

### Why Require Explicit index! Invocation?

Functions must be listed in `index!`:

**Rationale:**
1. **Intentionality**: Every function explicitly chosen
2. **Documentation**: Index shows module structure
3. **Dead Code Detection**: Unused functions caught by compiler
4. **Big Picture**: See all functions at a glance

**Philosophy:** Explicit is better than implicit

### Why Three Indexing Variants (impls1/impls2/impls3)?

Multiple implementations available:

**Rationale:**
1. **Evolution**: impls1 is simple, impls3 is advanced
2. **Compatibility**: Keep old versions for existing code
3. **Choice**: Different complexity/capability tradeoffs
4. **Learning**: Start with impls1, upgrade to impls3

**Recommendation:** Use `impls3!` (aliased as `impls!`) for new code

### Why Optional vs Mandatory Variants?

Both `impls1!` and `impls_optional!` exist:

**Rationale:**
1. **Default Strict**: Enforce usage by default
2. **Conditional Compilation**: Optional for cfg scenarios
3. **Flexibility**: Choose enforcement level
4. **Clear Intent**: Explicit about optionality

**Pattern:** Strict by default, flexible when needed

### Why Separate tests_impls! Macro?

Special variant for test functions:

**Rationale:**
1. **Convenience**: Automatic `#[test]` attribute
2. **Consistency**: Same pattern for tests and functions
3. **Organization**: Structured test suites
4. **Clarity**: Clear test index

**Benefit:** Test organization follows function organization pattern

### Why Function Manipulation Utilities?

Includes fn_name!, fn_rename!, fns!:

**Rationale:**
1. **Reusability**: Useful for other macro authors
2. **Composability**: Build advanced macros
3. **Completeness**: Full function manipulation toolkit
4. **Dogfooding**: Used internally by impls2!

**Benefit:** Enable advanced use cases

### Why Two-Crate Pattern?

Runtime (impls_index) + proc macro (impls_index_meta):

**Rationale:**
1. **Compilation**: Proc macros in separate crate
2. **Dependencies**: Isolate macro_tools dependency
3. **Organization**: Clear separation
4. **Standard Pattern**: Follows Rust conventions

**Example:** Similar to serde/serde_derive

### Why deny(unused_macros)?

Generated macros have `#[deny(unused_macros)]`:

**Rationale:**
1. **Dead Code Detection**: Catch unused functions
2. **Enforcement**: Ensure index completeness
3. **Cleanup**: Encourage removing unused code
4. **Intent**: Make usage explicit

**Exception:** `impls_optional!` uses `allow` instead

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for comprehensive testing
- Macro expansion testing
- Integration tests

### Test Focus

1. **Basic Indexing**: impls1! with single/multiple functions
2. **Advanced Indexing**: impls3! with renaming
3. **Index Invocation**: Various index! syntaxes (with/without commas)
4. **Renaming**: Function aliasing via `as` syntax
5. **Optional Functions**: impls_optional! behavior
6. **Test Indexing**: tests_impls! and tests_index!
7. **Edge Cases**: Empty index, empty impls
8. **Visibility**: pub vs private functions
9. **Attributes**: Preserving function attributes
10. **Function Manipulation**: fn_name!, fn_rename!, fns!

### Test Organization

Tests located in `tests/inc/`:
- `impls1_test.rs` - Basic indexing tests
- `impls2_test.rs` - Alternative indexing
- `impls3_test.rs` - Proc macro tests
- `index_test.rs` - Index invocation tests
- `func_test.rs` - Function utilities
- `tests_index_test.rs` - Test indexing

### Known Test Limitations

1. **Macro Expansion**: Hard to test intermediate expansion
2. **Error Messages**: Macro errors can be cryptic
3. **Hygiene**: Macro hygiene edge cases
4. **Compile Failures**: Testing deny(unused_macros) requires negative tests

## Future Considerations

### Potential Enhancements

1. **Method Indexing**: Support impl block methods
2. **Struct Indexing**: Index type definitions
3. **Module Indexing**: Cross-module function collection
4. **Auto-Index**: Automatic index generation (optional)
5. **Documentation**: Generate docs from index
6. **Better Errors**: Improved error messages
7. **Performance**: Optimize macro expansion
8. **Unification**: Single indexing macro (remove impls1/impls2)

### Breaking Changes to Consider

1. **Remove impls1/impls2**: Keep only impls3
2. **Change Syntax**: Different index! syntax
3. **Default Optional**: Make optional by default
4. **Rename Macros**: Shorter/clearer names
5. **Automatic Testing**: Auto-add #[test] based on context

### Known Limitations

1. **Single Scope**: Index works within one scope only
2. **No Overloading**: Each function name unique
3. **Manual Index**: Must list all functions
4. **Proc Macro Dependency**: impls3 requires proc macro
5. **No Cross-Module**: Can't index across modules
6. **Renaming Limitations**: Rename only during index, not in impls

## Adoption Guidelines

### When to Use impls_index

**Good Candidates:**
- Modules with many related functions
- Test suites needing structure
- Code where "big picture" important
- Functions with complex implementations
- Projects valuing explicit structure
- Teams wanting clear organization

**Poor Candidates:**
- Single-function modules
- Simple utility functions
- Code with stable structure
- Performance-critical macro expansion
- Projects preferring minimal macros

### Choosing Which Variant

```rust
// New code: use impls3 (aliased as impls)
use impls_index::*;
impls! { /* functions */ }

// Legacy code: impls1 simpler but less capable
impls1! { /* functions */ }

// Optional functions: use impls_optional
impls_optional! { /* functions */ }

// Tests: use tests_impls
tests_impls! { /* test functions */ }
```

### Best Practices

1. **Use impls3**: Prefer advanced variant for new code
2. **Complete Index**: List all functions in index!
3. **Clear Names**: Use descriptive function names
4. **Group Related**: Keep related functions together
5. **Document Intent**: Comment why using impls_index
6. **Consistent Style**: Use same variant throughout module
7. **Remove Unused**: Delete functions not in index

### Migration Path

```rust
// Before: Regular functions
fn f1() { /* ... */ }
fn f2() { /* ... */ }

// After: Indexed functions
impls! {
  fn f1() { /* ... */ }
  fn f2() { /* ... */ }
}

index! {
  f1,
  f2,
}
```

## Related Crates

**Dependencies:**
- **impls_index_meta**: Proc macro support (workspace)
- **macro_tools**: AST utilities (workspace)

**Related:**
- **mod_interface**: Module visibility management (workspace)
- **meta_tools**: Aggregates impls_index (workspace)
- **for_each**: Macro iteration (workspace)

**Alternatives:**
- **inventory**: Runtime type collection
- **linkme**: Distributed slice pattern
- None match exact use case

## References

- [API Documentation](https://docs.rs/impls_index)
- [Proc Macro Documentation](https://docs.rs/impls_index_meta)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/impls_index)
- [readme.md](./readme.md)
- [meta_tools](../meta_tools/readme.md) - Meta-programming utilities
- [mod_interface](../mod_interface/readme.md) - Module interface pattern

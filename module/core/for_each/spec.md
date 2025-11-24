# Specification: for_each

## Overview

**for_each** is a powerful macro iteration utility providing a declarative macro that applies a callback macro to each element of a list, supporting both function-style and map-style invocation with optional prefix/postfix tokens and callbackless mode. It serves as a fundamental meta-programming building block for repetitive macro invocations across the workspace.

**Version:** 0.10.0
**Status:** Experimental
**Category:** Development Tools (Macro Utilities)
**Dependents:** meta_tools and other workspace crates using macro iteration

### Scope

#### Responsibility

Provide a flexible macro iteration utility that applies a callback macro to each element of a list, supporting prefix/postfix token injection, complex token trees, and callbackless invocation for repetitive code generation.

#### In-Scope

1. **Function-Style Invocation**
   - `for_each!(macro_name, elem1, elem2, ...)`
   - Simple comma-delimited syntax
   - Direct macro application
   - Minimal boilerplate

2. **Map-Style Invocation**
   - `for_each! { macro_name where @Prefix ... @Postfix ... @Each ... }`
   - Named parameters with @-prefix
   - Optional @Prefix and @Postfix
   - Mandatory @Each elements
   - Order: @Prefix, @Postfix, @Each

3. **Prefix/Postfix Injection**
   - @Prefix tokens prepended to each element
   - @Postfix tokens appended to each element
   - Supports single tokens or braced token trees
   - Automatic brace unwrapping

4. **Callbackless Mode**
   - Omit callback macro
   - Use @Prefix and @Postfix only
   - Identity macro used internally
   - Generates prefix + element + postfix

5. **Token Tree Handling**
   - `braces_unwrap!` helper macro
   - Unwraps outer braces
   - Preserves inner structure
   - Handles complex expressions

6. **Identity Macro**
   - `identity!` - Returns input as-is
   - Used internally for callbackless mode
   - Available for user code
   - Pass-through functionality

7. **Feature Architecture**
   - `enabled` - Master switch (default)
   - `no_std` - Embedded support (available)
   - `use_alloc` - Allocation in no_std (available)
   - Zero external dependencies

8. **Traditional Namespace Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Public macros in prelude
   - Clean re-export structure

#### Out-of-Scope

1. **NOT Runtime Iteration**
   - Compile-time only
   - No dynamic iteration
   - **Rationale:** Macro-based, not runtime loops

2. **NOT Conditional Iteration**
   - No filtering or conditional logic
   - All elements processed
   - **Rationale:** Keep macro simple

3. **NOT Nested Iteration**
   - No automatic nesting support
   - Manual nesting required
   - **Rationale:** Avoid complexity

4. **NOT Index Access**
   - No element indexing
   - No position awareness
   - **Rationale:** Positional information hard in macros

5. **NOT Type-Level Iteration**
   - No type list iteration
   - Works with token trees only
   - **Rationale:** Token-based macro

6. **NOT Procedural Macro**
   - Declarative macro only
   - No custom syntax
   - **Rationale:** Simplicity and portability

7. **NOT Error Handling**
   - No validation of callback expansion
   - No error recovery
   - **Rationale:** Rely on macro system errors

8. **NOT Transformation**
   - No element transformation
   - Pure iteration only
   - **Rationale:** Transformation is callback's job

#### Boundaries

- **for_each vs std::iter**: for_each compile-time; std::iter runtime
- **for_each vs procedural macros**: for_each declarative; proc macros have full AST access
- **for_each vs macro_rules patterns**: for_each higher-level abstraction; macro_rules lower-level

## Architecture

### Dependency Structure

```
for_each (macro iteration)
├── Internal Dependencies
│   └── (none - zero dependencies)
├── External Dependencies
│   └── (none - pure declarative macros)
└── Dev Dependencies
    └── test_tools (workspace, testing)
```

**Note:** Completely self-contained, zero dependencies

### Module Organization

```
for_each
├── lib.rs (single-file implementation)
│   ├── for_each! - Main iteration macro
│   ├── braces_unwrap! - Helper for unwrapping braces
│   └── identity! - Identity macro for callbackless mode
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Single-file declarative macro implementation

### Feature Architecture

```
enabled (master switch, default)
│
no_std (embedded support)
│
use_alloc (no_std + allocation)
│
full (all features, same as default)
```

**Default Features:** `enabled`

### Macro Expansion Flow

#### Function-Style Flow

```
for_each!(dbg, "a", "b", "c")
  ↓
Expand to:
  braces_unwrap!(dbg, "a");
  braces_unwrap!(dbg, "b");
  braces_unwrap!(dbg, "c");
  ↓
Expand to:
  dbg!("a");
  dbg!("b");
  dbg!("c");
```

#### Map-Style with Prefix/Postfix Flow

```
for_each! {
  dbg where
  @Prefix { "prefix" + }
  @Postfix { + "postfix" }
  @Each "a" "b" "c"
}
  ↓
Expand to:
  braces_unwrap! {
    dbg where
    @Prefix { "prefix" + }
    @Postfix { + "postfix" }
    @SRC { "a" }
  };
  ... (repeated for "b" and "c")
  ↓
Expand to:
  dbg!("prefix" + "a" + "postfix");
  dbg!("prefix" + "b" + "postfix");
  dbg!("prefix" + "c" + "postfix");
```

#### Callbackless Flow

```
for_each! {
  @Prefix { println! }
  @Each ("a") ("b") ("c")
}
  ↓
Transforms to:
for_each! {
  identity where
  @Prefix { println! }
  @Each ("a") ("b") ("c")
}
  ↓
Expand to:
  println!("a");
  println!("b");
  println!("c");
```

## Public API

### Main Iteration Macro

```rust
/// Apply a macro for each element of a list.
///
/// Function-style:
/// for_each!(macro_name, elem1, elem2, ...)
///
/// Map-style:
/// for_each! {
///   macro_name where
///   @Prefix { prefix_tokens }
///   @Postfix { postfix_tokens }
///   @Each elem1 elem2 ...
/// }
///
/// Callbackless:
/// for_each! {
///   @Prefix { prefix_tokens }
///   @Postfix { postfix_tokens }
///   @Each elem1 elem2 ...
/// }
#[macro_export]
macro_rules! for_each { /* ... */ }
```

### Helper Macros

```rust
/// Unwrap braces of token tree and pass to callback.
/// If not braced, passes as-is.
///
/// braces_unwrap!(dbg, { a, b, c }) -> dbg!(a, b, c)
/// braces_unwrap!(dbg, a, b, c) -> dbg!(a, b, c)
#[macro_export]
macro_rules! braces_unwrap { /* ... */ }

/// Macro which returns its input as is.
/// Used internally for callbackless mode.
///
/// identity!(foo) -> foo
#[macro_export]
macro_rules! identity {
  ($($Src: tt)*) => { $($Src)* };
}
```

## Usage Patterns

### Pattern 1: Function-Style Basic Iteration

```rust
use for_each::for_each;

// Apply dbg! to each element
for_each!(dbg, "a", "b", "c");

// Generates:
// dbg!("a");
// dbg!("b");
// dbg!("c");
```

### Pattern 2: Map-Style with Prefix and Postfix

```rust
use for_each::for_each;

for_each! {
  dbg where
  @Prefix { "prefix".to_string() + }
  @Postfix { + "postfix" }
  @Each "a" "b" "c"
};

// Generates:
// dbg!("prefix".to_string() + "a" + "postfix");
// dbg!("prefix".to_string() + "b" + "postfix");
// dbg!("prefix".to_string() + "c" + "postfix");
```

### Pattern 3: Complex Token Trees

```rust
use for_each::for_each;

for_each! {
  dbg where
  @Prefix { "prefix".to_string() + }
  @Postfix { + "postfix" }
  @Each { "a" + "1" } { "b" + "2" } { "c" + "3" }
};

// Generates:
// dbg!("prefix".to_string() + "a" + "1" + "postfix");
// dbg!("prefix".to_string() + "b" + "2" + "postfix");
// dbg!("prefix".to_string() + "c" + "3" + "postfix");
```

### Pattern 4: Callbackless Mode

```rust
use for_each::for_each;

for_each! {
  @Prefix { dbg! }
  @Each ( "a" ) ( "b" ) ( "c" )
};

// Generates:
// dbg!("a");
// dbg!("b");
// dbg!("c");
```

### Pattern 5: Only Prefix (No Postfix)

```rust
use for_each::for_each;

for_each! {
  println where
  @Prefix { "Value: " }
  @Each "a" "b" "c"
};

// Generates:
// println!("Value: " "a");
// println!("Value: " "b");
// println!("Value: " "c");
```

### Pattern 6: Struct Field Generation

```rust
use for_each::for_each;

macro_rules! define_fields {
  ($name:ident) => {
    pub $name: String,
  };
}

struct MyStruct {
  for_each!(define_fields, field_a, field_b, field_c)
}

// Generates:
// struct MyStruct {
//   pub field_a: String,
//   pub field_b: String,
//   pub field_c: String,
// }
```

### Pattern 7: Function Generation

```rust
use for_each::for_each;

macro_rules! define_getter {
  ($name:ident) => {
    pub fn $name(&self) -> &str {
      &self.$name
    }
  };
}

impl MyStruct {
  for_each!(define_getter, field_a, field_b, field_c)
}
```

### Pattern 8: Test Case Generation

```rust
use for_each::for_each;

macro_rules! test_value {
  ($val:expr) => {
    #[test]
    fn test_value() {
      assert!(validate($val));
    }
  };
}

for_each!(test_value, 1, 2, 3, 4, 5);
```

## Dependencies and Consumers

### Direct Dependencies

**None** - Completely self-contained

**Dev:**
- `test_tools` (workspace) - Testing utilities

### Consumers

**Known:**
- `meta_tools` - Aggregates for_each

**Likely:**
- Workspace crates using meta-programming
- Code generation utilities
- Macro-heavy libraries

**Usage Pattern:** Workspace crates use for_each for repetitive macro invocations during compile-time code generation.

## Design Rationale

### Why Two Invocation Styles?

Function-style and map-style:

**Rationale:**
1. **Simplicity**: Function-style for simple cases
2. **Flexibility**: Map-style for complex scenarios
3. **Familiarity**: Function-style like regular macros
4. **Power**: Map-style with prefix/postfix

**Benefit:** Choose appropriate style for use case

### Why Optional Callback (Callbackless Mode)?

Allows omitting callback macro:

**Rationale:**
1. **Convenience**: When only prefix/postfix needed
2. **Clarity**: Clear intent
3. **Simplicity**: Less boilerplate
4. **Common Pattern**: Often just concatenating tokens

**Implementation:** Uses identity! macro internally

### Why braces_unwrap! Helper?

Separate macro for unwrapping braces:

**Rationale:**
1. **Reusability**: Can be used independently
2. **Clarity**: Clear responsibility
3. **Complexity**: Handles 16 brace combinations
4. **Modularity**: Separate concern

**Pattern:** Composition of simple macros

### Why @-Prefixed Keywords?

Uses @Prefix, @Postfix, @Each:

**Rationale:**
1. **Clarity**: Clear parameter names
2. **No Conflicts**: @ prefix avoids collisions
3. **Convention**: Similar to other macro systems
4. **Readability**: Self-documenting

**Alternative:** Could use plain identifiers, but less clear

### Why Support Braced Token Trees?

Allows `{ ... }` around elements:

**Rationale:**
1. **Complex Expressions**: Multi-token elements
2. **Grouping**: Clear grouping semantics
3. **Flexibility**: Single or multiple tokens
4. **Automatic Unwrapping**: Transparent handling

**Benefit:** Works with any token complexity

### Why Declarative Macro?

Uses macro_rules! not proc macro:

**Rationale:**
1. **Simplicity**: Easier to understand
2. **Portability**: Works everywhere
3. **Compile Time**: Fast compilation
4. **No Dependencies**: Zero deps
5. **Debugging**: Easier to debug

**Tradeoff:** Less powerful than proc macros

### Why Zero Dependencies?

No external dependencies:

**Rationale:**
1. **Fundamental**: Core building block
2. **Reliability**: No dependency churn
3. **Compile Time**: Fast builds
4. **Simplicity**: Minimal crate

**Benefit:** Suitable for any environment

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for macro testing
- Expansion verification

### Test Focus

1. **Function-Style**: Basic iteration
2. **Map-Style**: Prefix/postfix variations
3. **Callbackless**: Without callback
4. **Edge Cases**: Empty lists, single element
5. **Nesting**: Nested invocations
6. **Token Trees**: Complex expressions
7. **All Combinations**: 16 brace variants

### Macro Testing Approach

1. **Expansion Tests**: Verify correct expansion
2. **Compilation Tests**: Ensure compiles
3. **Behavior Tests**: Check runtime behavior
4. **Error Tests**: Invalid syntax

### Known Test Limitations

1. **Expansion Inspection**: Hard to test intermediate expansion
2. **Error Messages**: Macro errors cryptic
3. **Hygiene**: Macro hygiene hard to test
4. **Combinatorial**: Many edge cases

## Future Considerations

### Potential Enhancements

1. **Indexed Iteration**: Access element index
2. **Filtering**: Conditional iteration
3. **Transformation**: Map-like functionality
4. **Nested Support**: Better nesting utilities
5. **Better Errors**: Improved error messages
6. **Documentation**: More examples
7. **Performance**: Optimize expansion

### Breaking Changes to Consider

1. **Syntax Changes**: Different keyword syntax
2. **Order Requirements**: Relax ordering
3. **Default Behavior**: Change defaults
4. **Naming**: Rename macros

### Known Limitations

1. **No Filtering**: All elements processed
2. **No Indexing**: Can't access position
3. **No Transformation**: Pure iteration
4. **Macro Errors**: Cryptic error messages
5. **No Validation**: Doesn't validate callback

## Adoption Guidelines

### When to Use for_each

**Good Candidates:**
- Repetitive macro invocations
- Field/method generation
- Test case generation
- Code pattern repetition
- Token concatenation with pattern

**Poor Candidates:**
- Runtime iteration (use std::iter)
- Complex transformations (use proc macros)
- One-off macro calls
- Dynamic lists

### Choosing Invocation Style

```rust
// Simple cases: function-style
for_each!(dbg, a, b, c);

// Need prefix/postfix: map-style
for_each! {
  dbg where
  @Prefix { x + }
  @Each a b c
};

// No callback: callbackless
for_each! {
  @Prefix { println! }
  @Each (a) (b) (c)
};
```

### Best Practices

1. **Keep Simple**: Use simplest style for task
2. **Document Intent**: Comment why using for_each
3. **Test Expansion**: Verify generated code
4. **Brace Consistently**: Use braces for complex tokens
5. **Callback First**: Prefer function-style when possible
6. **Avoid Deep Nesting**: Keep nesting shallow

## Related Crates

**Workspace:**
- **meta_tools**: Aggregates for_each
- **impls_index**: Uses for_each internally
- **macro_tools**: Procedural macro utilities

**External:**
- **paste**: Token pasting
- **seq-macro**: Sequential macro iteration
- **iter-each**: Runtime iteration macros

## References

- [API Documentation](https://docs.rs/for_each)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/for_each)
- [readme.md](./readme.md)
- [meta_tools](../meta_tools/readme.md) - Meta-programming utilities
- [Rust macro_rules](https://doc.rust-lang.org/reference/macros-by-example.html) - Declarative macros

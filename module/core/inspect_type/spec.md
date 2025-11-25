# Specification: inspect_type

## Overview

**inspect_type** is a zero-dependency diagnostic utility providing macros for runtime type inspection and size reporting. It wraps Rust's type introspection capabilities (`type_name_of_val`, `size_of_val`) in convenient macros for debugging and development diagnostics, serving as a lightweight type introspection foundation for workspace tools.

**Version:** 0.16.0
**Status:** Production
**Category:** Type Utilities (Diagnostic/Introspection)
**Dependents:** 3 workspace crates (likely diagnostic/testing utilities)

### Scope

#### Responsibility

Provide lightweight, zero-dependency macros for inspecting types and sizes of variables at runtime, primarily for diagnostic and debugging purposes during development.

#### In-Scope

1. **Type Inspection Macros**
   - `inspect_type_of!()` - Print type name and size to stdout
   - `inspect_to_str_type_of!()` - Return type name and size as String
   - Both macros support single or multiple expressions

2. **Type Name Reporting**
   - Uses `std::any::type_name_of_val()` for type names
   - Includes full type path (e.g., `&[i32]`, `&[i32; 3]`)
   - Distinguishes between similar types (slices vs arrays)

3. **Size Reporting**
   - Uses `std::mem::size_of_val()` for runtime size
   - Reports size in bytes
   - Handles both sized and unsized types

4. **Formatted Output**
   - Format: `sizeof( expression : type ) = size_bytes`
   - Example: `sizeof( &[1, 2, 3][..] : &[i32] ) = 16`
   - Preserves expression text via `stringify!()`

5. **Reference Handling**
   - Macros take references intentionally (`let value = $src`)
   - Allows inspection without moving values
   - Handles double-reference patterns correctly

6. **Zero Dependencies**
   - No production dependencies
   - No test_tools dependency (prevents circular dependencies)
   - Uses only std library introspection APIs

7. **Traditional Module Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Not using mod_interface! (utility crate)
   - Simple feature gating

8. **Build-Time Rust Version Detection**
   - Uses `rustc_version` (build-dependency) for version detection
   - Historically supported nightly-only features
   - Now works on stable (comment: "no need in nightly anymore")

#### Out-of-Scope

1. **NOT Compile-Time Type Checking**
   - Does not provide static type assertions
   - Does not validate types at compile time
   - **Rationale:** Diagnostic tool for runtime inspection only

2. **NOT Type Introspection Framework**
   - Does not provide trait reflection
   - Does not enumerate struct fields
   - Does not inspect type hierarchies
   - **Rationale:** Simple diagnostic utility, not reflection system

3. **NOT Performance Profiling**
   - Does not measure actual memory usage
   - Does not track allocations
   - **Rationale:** Static size only, not runtime profiling

4. **NOT Type Serialization**
   - Does not serialize type information
   - Does not generate type descriptions
   - **Rationale:** Display-only utility

5. **NOT Custom Formatting**
   - Output format is fixed
   - No customization options
   - **Rationale:** Simplicity over flexibility

6. **NOT Type Comparison**
   - Does not compare types
   - Does not check type equality
   - **Rationale:** Inspection only, not assertion

7. **NOT Trait Inspection**
   - Does not list implemented traits
   - Does not check trait bounds
   - **Rationale:** Beyond scope of simple diagnostic

8. **NOT Debug Formatting**
   - Does not use Debug trait
   - Does not print values
   - **Rationale:** Type inspection only, not value inspection

#### Boundaries

- **inspect_type vs std::any::type_name**: inspect_type provides macro convenience and size reporting; type_name is lower-level
- **inspect_type vs std::mem::size_of**: inspect_type combines type name with size in formatted output
- **inspect_type vs dbg!**: dbg! prints values, inspect_type prints types and sizes

## Architecture

### Dependency Structure

```
inspect_type (type utilities, zero dependencies)
├── Internal Dependencies
│   └── (none - foundational utility)
├── Build Dependencies
│   └── rustc_version (0.4, version detection)
└── Dev Dependencies
    └── (intentionally empty to prevent circular dependencies)
```

**Note:** Intentionally avoids test_tools to prevent circular dependency chains.

### Module Organization

```
inspect_type
├── lib.rs (traditional namespaces)
├── nightly module (inline, macro definitions)
│   ├── inspect_type_of! - Print to stdout
│   └── inspect_to_str_type_of! - Return String
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Traditional namespace organization, not mod_interface! (utility crate convention)

### Feature Architecture

```
enabled (master switch)
└── full (all features, same as enabled)
```

**Default Features:** `enabled`

**Note:** Minimal feature structure - no optional functionality

### Macro Expansion Flow

#### inspect_type_of! Macro

```
inspect_type_of!(&[1, 2, 3][..])
  ↓
inspect_to_str_type_of!(&[1, 2, 3][..])
  ↓
{
  let value = &[1, 2, 3][..];
  let stringified = "&[1, 2, 3][..]";
  let size = std::mem::size_of_val(&value); // 16
  let type_name = std::any::type_name_of_val(&value); // "&[i32]"
  format!("sizeof( {} : {} ) = {}", stringified, type_name, size)
}
  ↓
println!("sizeof( &[1, 2, 3][..] : &[i32] ) = 16")
  ↓
Returns: "sizeof( &[1, 2, 3][..] : &[i32] ) = 16"
```

## Public API

### Macros

```rust
/// Inspect type and size, return as String
#[macro_export]
macro_rules! inspect_to_str_type_of {
  ( $src: expr ) => {{
    let value = $src;
    let stringified = stringify!($src);
    let size = std::mem::size_of_val(&value);
    let type_name = std::any::type_name_of_val(&value);
    format!("sizeof( {} : {} ) = {}", stringified, type_name, size)
  }};
}

/// Inspect type and size, print to stdout and return as String
#[macro_export]
macro_rules! inspect_type_of {
  ( $src: expr ) => {{
    let result = inspect_to_str_type_of!($src);
    println!("{}", result);
    result
  }};
}
```

### Re-exports

```rust
pub use inspect_type_of;
pub use inspect_to_str_type_of;
```

## Usage Patterns

### Pattern 1: Basic Type Inspection

```rust
use inspect_type::*;

inspect_type_of!(&[1, 2, 3][..]);
// Prints: sizeof( &[1, 2, 3][..] : &[i32] ) = 16

inspect_type_of!(&[1, 2, 3]);
// Prints: sizeof( &[1, 2, 3] : &[i32; 3] ) = 8
```

**Explanation:** Slice (`&[i32]`) is 16 bytes (pointer + length), array reference (`&[i32; 3]`) is 8 bytes (pointer only).

### Pattern 2: String Return (Silent Inspection)

```rust
use inspect_type::*;

let type_info = inspect_to_str_type_of!(&vec![1, 2, 3]);
// No output to stdout
// type_info = "sizeof( &vec![1, 2, 3] : &Vec<i32> ) = 8"

assert!(type_info.contains("Vec<i32>"));
```

### Pattern 3: Comparing Types

```rust
use inspect_type::*;

let data = vec![1, 2, 3];

inspect_type_of!(&data);
// Prints: sizeof( &data : &Vec<i32> ) = 8

inspect_type_of!(&data[..]);
// Prints: sizeof( &data[..] : &[i32] ) = 16

inspect_type_of!(&data[0]);
// Prints: sizeof( &data[0] : &i32 ) = 8
```

### Pattern 4: Struct Size Inspection

```rust
use inspect_type::*;

struct Small { a: u8 }
struct Large { a: u64, b: u64, c: u64 }

inspect_type_of!(&Small { a: 1 });
// Prints: sizeof( &Small { a: 1 } : &Small ) = 8 (reference size)

inspect_type_of!(Small { a: 1 });
// Prints: sizeof( Small { a: 1 } : Small ) = 1

inspect_type_of!(Large { a: 1, b: 2, c: 3 });
// Prints: sizeof( Large { a: 1, b: 2, c: 3 } : Large ) = 24
```

### Pattern 5: Generic Type Inspection

```rust
use inspect_type::*;

fn analyze<T>(value: &T) {
  inspect_type_of!(value);
}

analyze(&42i32);
// Prints: sizeof( value : &i32 ) = 8

analyze(&"hello");
// Prints: sizeof( value : &&str ) = 8
```

### Pattern 6: Debugging Type Issues

```rust
use inspect_type::*;

fn process_slice(data: &[i32]) {
  inspect_type_of!(data);
  // Helps debug if accidentally passing wrong type
}

let array = [1, 2, 3];
process_slice(&array);
// Prints: sizeof( data : &[i32] ) = 16
```

## Dependencies and Consumers

### Direct Dependencies

**Production:** (none - zero dependencies)

**Build:**
- `rustc_version` (0.4) - Rust compiler version detection for feature compatibility

**Dev:** (intentionally empty)
- Avoids test_tools to prevent circular dependencies

### Consumers (3 workspace crates)

**Identified:** Likely used by:
- Testing/diagnostic utilities
- Macro development tools
- Type introspection utilities

**Usage Pattern:** Workspace tools use inspect_type for debugging macro expansions, understanding type behavior, and diagnosing size/layout issues during development.

## Design Rationale

### Why Macros Instead of Functions?

**Problem:** Need to capture both value and expression text for meaningful output.

**Solution:** Macros can use `stringify!()` to preserve expression.

```rust
inspect_type_of!(&data[..]);
// Prints: sizeof( &data[..] : &[i32] ) = 16
//         ^^^^^^^^^^^^^^ preserved via stringify!
```

**Benefits:**
1. **Readable Output**: Shows what expression was inspected
2. **Context**: Helps locate inspection in code
3. **Debugging**: Clear connection between code and output

**Tradeoff:** Macro complexity for better UX

### Why Take Reference in Macro?

The macro does `let value = $src` which takes the expression by value/reference as passed:

```rust
let value = $src; // Takes expression as-is
std::mem::size_of_val(&value); // Then takes reference
```

**Rationale:**
1. **Non-Moving**: Allows inspection without consuming values
2. **Flexibility**: Works with both owned values and references
3. **Correctness**: size_of_val requires reference anyway

**Note:** Clippy warning `clippy::size_of_ref` is intentionally allowed because we're specifically inspecting reference sizes.

### Why Two Macros?

Separate `inspect_type_of!` and `inspect_to_str_type_of!` because:

1. **Common Case**: Usually want to print (inspect_type_of!)
2. **Testing**: Sometimes need string for assertions (inspect_to_str_type_of!)
3. **Composability**: String version can be used in other contexts

**Implementation:** `inspect_type_of!` calls `inspect_to_str_type_of!` internally (no duplication).

### Why Zero Dependencies?

inspect_type avoids all dependencies because:

1. **Foundation**: Diagnostic utilities shouldn't add dependency weight
2. **Circular Deps**: test_tools and other utilities depend on this
3. **Simplicity**: Implementation is trivial, no need for dependencies

**Tradeoff:** No additional features for maximum compatibility

### Why Not Use dbg! Macro?

Standard library's `dbg!` macro prints values, not types:

```rust
dbg!(&data);
// Prints: &data = [1, 2, 3]  (values)

inspect_type_of!(&data);
// Prints: sizeof( &data : &Vec<i32> ) = 8  (types and sizes)
```

**Use Cases:**
- `dbg!`: Inspect runtime values
- `inspect_type_of!`: Inspect compile-time types and layout

### Why Fixed Format?

Output format is hardcoded because:

1. **Consistency**: Always know what to expect
2. **Simplicity**: No configuration complexity
3. **Readability**: Format is optimized for human consumption

**Tradeoff:** No customization for simplicity

## Testing Strategy

### Test Coverage

**Limited by Circular Dependency:**
- Cannot use test_tools (would create circular dependency)
- Relies on doc tests and integration tests in consumer crates

### Test Files

```
tests/
└── (minimal, if any - doc tests preferred)
```

### Test Focus

1. **Doc Tests**: Embedded in readme.md and lib.rs
2. **Example**: examples/inspect_type_trivial.rs demonstrates usage
3. **Consumer Tests**: Integration tests in dependent crates

### Known Test Limitations

1. **No Unit Tests**: Cannot use test framework due to circular dependency prevention
2. **Manual Testing**: Relies on examples and doc tests
3. **Visual Inspection**: Output must be manually verified

## Future Considerations

### Potential Enhancements

1. **Custom Formatting**: Add macro variants with format control
2. **Value Inspection**: Optionally include Debug output
3. **Trait Listing**: Enumerate implemented traits (requires reflection)
4. **Const Evaluation**: Support const context when stabilized
5. **JSON Output**: Machine-readable type information

### Breaking Changes to Consider

1. **Format Change**: Modify output format (breaks string matching)
2. **Additional Info**: Add more type metadata to output
3. **Rename Macros**: More descriptive names

### Known Limitations

1. **Fixed Format**: Cannot customize output format
2. **No Value Display**: Only shows types, not values
3. **No Trait Info**: Cannot list implemented traits
4. **Manual Comparison**: No built-in type comparison

## Adoption Guidelines

### When to Use inspect_type

**Good Candidates:**
- Debugging generic code
- Understanding type inference
- Diagnosing size/layout issues
- Macro development and testing
- Learning Rust type system
- Temporary diagnostic code

**Poor Candidates:**
- Production code (diagnostic only)
- Performance-critical paths (println overhead)
- Type assertions (use static_assertions crate)
- Value inspection (use dbg! macro)

### Migration from dbg!

```rust
// Before: dbg! (shows values)
let data = vec![1, 2, 3];
dbg!(&data);
// Output: &data = [1, 2, 3]

// After: inspect_type_of! (shows types)
inspect_type_of!(&data);
// Output: sizeof( &data : &Vec<i32> ) = 8
```

**Use Both:**
- Use `dbg!` to see values
- Use `inspect_type_of!` to see types and sizes

### Best Practices

1. **Temporary Use**: Add during debugging, remove before commit
2. **Type Learning**: Use to understand Rust's type system
3. **Macro Testing**: Inspect macro expansion results
4. **Size Verification**: Check struct padding and alignment
5. **Reference Understanding**: Clarify slice vs array vs reference sizes

## Related Crates

- **std::any::type_name**: Lower-level type name function (no size info)
- **std::mem::size_of**: Lower-level size function (no type name)
- **static_assertions**: Compile-time type assertions
- **dbg!**: Standard library value inspection macro

## References

- [API Documentation](https://docs.rs/inspect_type)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/inspect_type)
- [Example](./examples/inspect_type_trivial.rs)
- [readme.md](./readme.md)
- [std::any::type_name_of_val](https://doc.rust-lang.org/std/any/fn.type_name_of_val.html)
- [std::mem::size_of_val](https://doc.rust-lang.org/std/mem/fn.size_of_val.html)

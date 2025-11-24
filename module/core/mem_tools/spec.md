# Specification: mem_tools

## Overview

**mem_tools** is a lightweight memory manipulation utility crate providing performant pointer comparison, size checking, region validation, and data comparison functions. It serves as a type-agnostic alternative to standard library pointer operations, enabling comparisons between references of different types through unsafe but carefully validated operations.

**Version:** 0.9.0
**Status:** Experimental
**Category:** Development Tools (Memory Utilities)
**Dependents:** Unknown (likely workspace crates needing low-level memory operations)

### Scope

#### Responsibility

Provide type-agnostic memory comparison utilities for pointer equality, size checking, region validation, and data content comparison, enabling safe high-performance memory operations across different types.

#### In-Scope

1. **Pointer Comparison**
   - `same_ptr(src1, src2)` - Check pointer equality
   - Type-agnostic (works with different types)
   - Faster than type-safe `std::ptr::eq()`
   - Based on address comparison

2. **Size Comparison**
   - `same_size(src1, src2)` - Check memory size equality
   - Uses `core::mem::size_of_val()`
   - Works with dynamically-sized types (DSTs)
   - Compile-time for sized types

3. **Region Validation**
   - `same_region(src1, src2)` - Check pointer AND size
   - Combines `same_ptr` and `same_size`
   - Validates complete memory region match
   - Useful for buffer validation

4. **Data Comparison**
   - `same_data(src1, src2)` - Compare actual data content
   - Uses C `memcmp` for performance
   - Byte-by-byte comparison
   - Works with any reference types
   - Validates sizes before comparing

5. **Feature Architecture**
   - `enabled` - Master switch (default)
   - `no_std` - Embedded support (available)
   - `use_alloc` - Allocation in no_std (available)
   - Zero external dependencies

6. **Traditional Namespace Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Dependency namespace (empty)
   - mem submodule with functions

7. **Safety Guarantees**
   - Size validation before data comparison
   - Pointer validity checks
   - No out-of-bounds access
   - Careful unsafe usage

8. **Performance**
   - Direct memory operations
   - No allocations
   - Minimal overhead
   - Optimized comparisons

#### Out-of-Scope

1. **NOT Memory Allocation**
   - No allocator utilities
   - No heap management
   - **Rationale:** Use std::alloc or allocator crates

2. **NOT Memory Copying**
   - No memcpy/memmove wrappers
   - No buffer copying utilities
   - **Rationale:** Use std::ptr::copy or copy_nonoverlapping

3. **NOT Memory Initialization**
   - No zeroing utilities
   - No initialization patterns
   - **Rationale:** Use std::mem::MaybeUninit

4. **NOT Smart Pointers**
   - No Box/Rc/Arc alternatives
   - No custom smart pointers
   - **Rationale:** Use standard library

5. **NOT Memory Alignment**
   - No alignment utilities
   - No padding calculation
   - **Rationale:** Use std::mem::align_of

6. **NOT Unsafe Abstractions**
   - No additional unsafe utilities
   - Minimal unsafe surface
   - **Rationale:** Keep scope focused

7. **NOT Type Conversions**
   - No transmute utilities
   - No casting helpers
   - **Rationale:** Use std::mem::transmute

8. **NOT Memory Layout**
   - No struct layout utilities
   - No padding inspection
   - **Rationale:** Use std::mem or diagnostics_tools

#### Boundaries

- **mem_tools vs std::ptr**: mem_tools type-agnostic; std::ptr type-safe
- **mem_tools vs diagnostics_tools**: mem_tools runtime comparison; diagnostics_tools compile-time assertions
- **mem_tools vs libc**: mem_tools safe wrappers; libc raw C functions

## Architecture

### Dependency Structure

```
mem_tools (memory utilities)
├── Internal Dependencies
│   └── (none - zero dependencies)
├── External Dependencies
│   └── (none - uses only core/std)
└── Dev Dependencies
    └── test_tools (workspace, testing)
```

**Note:** Completely self-contained, zero production dependencies

### Module Organization

```
mem_tools
├── lib.rs (top-level aggregation)
├── mem.rs - Memory utilities module
│   ├── same_data() - Data comparison
│   ├── same_ptr() - Pointer comparison
│   ├── same_size() - Size comparison
│   └── same_region() - Region validation
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Simple single-module organization

### Feature Architecture

```
enabled (master switch, default)
│
no_std (embedded support)
│
use_alloc (no_std + allocation)
│
full (all features)
```

**Default Features:** `enabled`

### Function Flow

#### same_data Flow

```
same_data(&src1, &src2)
  ↓
Check same_size(src1, src2)
  ├─ false → return false
  └─ true → continue
      ↓
Get pointers as *const u8
      ↓
Call C memcmp(ptr1, ptr2, size)
      ↓
Return comparison result (== 0)
```

#### same_ptr Flow

```
same_ptr(&src1, &src2)
  ↓
Convert references to *const ()
  ↓
Compare pointer addresses
  ↓
Return equality result
```

#### same_size Flow

```
same_size(&src1, &src2)
  ↓
size_of_val(src1) == size_of_val(src2)
  ↓
Return boolean result
```

#### same_region Flow

```
same_region(&src1, &src2)
  ↓
same_ptr(src1, src2) && same_size(src1, src2)
  ↓
Return combined result
```

## Public API

### Memory Comparison Functions

```rust
/// Are two pointers points on the same data.
/// Does not require arguments to have the same type.
pub fn same_data<T1: ?Sized, T2: ?Sized>(src1: &T1, src2: &T2) -> bool

/// Are two pointers are the same, not taking into account type.
/// Unlike `std::ptr::eq()` does not require arguments to have the same type.
pub fn same_ptr<T1: ?Sized, T2: ?Sized>(src1: &T1, src2: &T2) -> bool

/// Are two pointers points on data of the same size.
pub fn same_size<T1: ?Sized, T2: ?Sized>(src1: &T1, src2: &T2) -> bool

/// Are two pointers points on the same region, ie same size and same pointer.
/// Does not require arguments to have the same type.
pub fn same_region<T1: ?Sized, T2: ?Sized>(src1: &T1, src2: &T2) -> bool
```

### Type Parameters

All functions:
- Generic over `T1: ?Sized` and `T2: ?Sized`
- Accept any reference types
- Work with dynamically-sized types
- No trait bounds required

## Usage Patterns

### Pattern 1: Pointer Equality Checking

```rust
use mem_tools as mem;

let src1 = (1,);
let src2 = (1,);

// Different allocations
assert!(!mem::same_ptr(&src1, &src2));

// Same allocation
assert!(mem::same_ptr(&src1, &src1));

// Works across types
let num: i32 = 42;
let bytes: [u8; 4] = num.to_ne_bytes();
assert!(!mem::same_ptr(&num, &bytes));
```

### Pattern 2: Size Comparison

```rust
use mem_tools as mem;

let src1 = "abc";
let src2 = "xyz";
assert!(mem::same_size(src1, src2)); // Both are 3 bytes

let src3 = "abcd";
assert!(!mem::same_size(src1, src3)); // Different sizes

// Works with different types of same size
let num: u32 = 42;
let arr: [u8; 4] = [1, 2, 3, 4];
assert!(mem::same_size(&num, &arr));
```

### Pattern 3: Region Validation

```rust
use mem_tools as mem;

let src1 = "abc";
let src2 = "abc"; // String literal deduplication

// Same region (pointer AND size)
assert!(mem::same_region(src1, src2));

let src3 = String::from("abc");
// Different pointer, same size
assert!(!mem::same_region(src1, src3.as_str()));
```

### Pattern 4: Data Content Comparison

```rust
use mem_tools as mem;

let src1 = "abc";
let src2 = "abc";
assert!(mem::same_data(src1, src2)); // Same content

let src3 = "xyz";
assert!(!mem::same_data(src1, src3)); // Different content

// Works with structs
#[derive(Clone)]
struct Point { x: i32, y: i32 }
let p1 = Point { x: 10, y: 20 };
let p2 = Point { x: 10, y: 20 };
assert!(mem::same_data(&p1, &p2));
```

### Pattern 5: Buffer Comparison

```rust
use mem_tools as mem;

fn validate_buffer(expected: &[u8], actual: &[u8]) -> bool {
  mem::same_data(expected, actual)
}

let buf1 = [1, 2, 3, 4];
let buf2 = [1, 2, 3, 4];
assert!(validate_buffer(&buf1, &buf2));
```

### Pattern 6: Type-Agnostic Comparison

```rust
use mem_tools as mem;

// Compare across different types
let tuple: (u8, u8, u8, u8) = (1, 2, 3, 4);
let array: [u8; 4] = [1, 2, 3, 4];

// Same size
assert!(mem::same_size(&tuple, &array));

// Same data content
assert!(mem::same_data(&tuple, &array));
```

### Pattern 7: String Literal Deduplication Check

```rust
use mem_tools as mem;

let s1 = "hello";
let s2 = "hello";

// Compiler may deduplicate identical string literals
if mem::same_ptr(s1, s2) {
  println!("String literals deduplicated");
} else {
  println!("String literals not deduplicated");
}
```

### Pattern 8: DST (Dynamically Sized Type) Handling

```rust
use mem_tools as mem;

let slice1: &[i32] = &[1, 2, 3];
let slice2: &[i32] = &[1, 2, 3];

// Works with slices (DSTs)
assert!(mem::same_size(slice1, slice2));
assert!(mem::same_data(slice1, slice2));
assert!(!mem::same_ptr(slice1, slice2)); // Different allocations
```

## Dependencies and Consumers

### Direct Dependencies

**None** - Completely self-contained

**Dev:**
- `test_tools` (workspace) - Testing utilities

### Consumers (Unknown)

**Likely used by:**
- Low-level workspace utilities
- Buffer validation code
- Memory-intensive algorithms
- Performance-critical code
- FFI boundary code

**Usage Pattern:** Workspace crates use mem_tools for type-agnostic memory comparisons where standard library functions are too restrictive or require type matching.

## Design Rationale

### Why Type-Agnostic Functions?

Allows comparing references of different types:

**Benefits:**
1. **Flexibility**: Compare any two references
2. **Convenience**: No type casting required
3. **Generic Code**: Works in generic contexts
4. **Utility**: Useful for FFI and low-level code

**Tradeoff:** Less type safety, but validated for correctness

### Why Use C memcmp?

Uses foreign function call to C memcmp:

**Rationale:**
1. **Performance**: Highly optimized implementation
2. **Standards**: Well-tested standard library function
3. **Efficiency**: Often SIMD-optimized
4. **Portability**: Available on all platforms

**Safety:** Size validation ensures safe usage

### Why No Dependencies?

Zero production dependencies:

**Rationale:**
1. **Simplicity**: Minimal crate
2. **Compile Time**: Fast compilation
3. **Binary Size**: Tiny footprint
4. **Reliability**: No dependency churn

**Benefit:** Suitable for no_std and minimal environments

### Why Separate Functions?

Four distinct functions instead of one:

**Rationale:**
1. **Clarity**: Each function has clear purpose
2. **Performance**: Only pay for what you use
3. **Composability**: Can combine as needed
4. **Debugging**: Clear semantics

**Pattern:** Unix philosophy - do one thing well

### Why same_region Combines Two?

`same_region` is `same_ptr && same_size`:

**Rationale:**
1. **Common Pattern**: Often need both checks
2. **Convenience**: Single function call
3. **Clarity**: Clear intent
4. **Efficiency**: Short-circuit evaluation

**Use Case:** Buffer validation and region checks

### Why Support ?Sized Types?

Generic over `T: ?Sized`:

**Rationale:**
1. **DST Support**: Works with slices, trait objects
2. **Flexibility**: Maximum utility
3. **Correctness**: size_of_val handles DSTs
4. **Completeness**: Comprehensive solution

**Benefit:** Works with all reference types

### Why Traditional Namespaces?

Uses own/orphan/exposed/prelude pattern:

**Rationale:**
1. **Consistency**: Matches workspace conventions
2. **Organization**: Clear module structure
3. **Re-exports**: Standard pattern
4. **Documentation**: Familiar structure

**Benefit:** Workspace developers recognize pattern

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for comprehensive testing
- Memory comparison testing

### Test Focus

1. **Pointer Equality**: Same and different allocations
2. **Size Comparison**: Various type combinations
3. **Data Content**: Equal and unequal data
4. **Region Validation**: Combined checks
5. **Edge Cases**: Empty types, ZSTs, DSTs
6. **Cross-Type**: Different types, same layout

### Safety Testing

1. **Size Validation**: Ensure no out-of-bounds
2. **Pointer Validity**: Valid references only
3. **DST Handling**: Slice and trait object tests
4. **Alignment**: Various alignment scenarios

### Known Test Limitations

1. **Platform-Specific**: Some behaviors platform-dependent
2. **Optimization**: Compiler optimizations affect results
3. **String Deduplication**: Depends on compiler
4. **Unsafe Code**: Limited testing of unsafe blocks

## Future Considerations

### Potential Enhancements

1. **More Utilities**: Additional memory comparison functions
2. **SIMD Optimization**: Explicit SIMD data comparison
3. **Const Support**: Const-compatible versions
4. **Alignment Checking**: Alignment comparison utilities
5. **Better Documentation**: More usage examples
6. **Benchmarks**: Performance benchmarks
7. **Error Reporting**: More detailed comparison information

### Breaking Changes to Consider

1. **Return Values**: Return difference instead of bool
2. **Error Types**: Result<(), Diff> instead of bool
3. **API Expansion**: Additional comparison modes
4. **Naming**: More descriptive function names
5. **Generic Constraints**: Add trait bounds

### Known Limitations

1. **No Padding Awareness**: Compares padding bytes
2. **No Indirection**: Doesn't follow pointers
3. **No Deep Comparison**: Shallow comparison only
4. **Platform-Dependent**: memcmp behavior varies
5. **No Custom Comparison**: Fixed comparison semantics

## Adoption Guidelines

### When to Use mem_tools

**Good Candidates:**
- Type-agnostic pointer comparisons
- Buffer validation
- Memory region checks
- Low-level FFI code
- Performance-critical comparisons
- no_std environments

**Poor Candidates:**
- Type-safe pointer equality (use std::ptr::eq)
- Deep structural comparison (use PartialEq)
- Sorted data (padding issues)
- Pointer arithmetic (use std::ptr)

### Choosing Which Function

```rust
use mem_tools as mem;

// Pointer equality (addresses)
if mem::same_ptr(&a, &b) { /* same location */ }

// Size comparison
if mem::same_size(&a, &b) { /* same size */ }

// Combined (location AND size)
if mem::same_region(&a, &b) { /* same region */ }

// Content comparison (data)
if mem::same_data(&a, &b) { /* same content */ }
```

### Best Practices

1. **Understand Unsafe**: Know limitations of unsafe code
2. **Size First**: Check sizes before data comparison
3. **Avoid Padding**: Be aware of struct padding
4. **Document Usage**: Explain why mem_tools chosen
5. **Test Thoroughly**: Validate all use cases
6. **Consider Alternatives**: Use std when possible

## Related Crates

**Standard Library:**
- **std::ptr**: Type-safe pointer utilities
- **std::mem**: Memory introspection
- **core::slice**: Slice utilities

**Workspace:**
- **diagnostics_tools**: Compile-time memory assertions
- **typing_tools**: Type-level utilities

**External:**
- **libc**: Raw C function bindings
- **memchr**: Optimized byte searching
- **bytemuck**: Safe type casting

## References

- [API Documentation](https://docs.rs/mem_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/mem_tools)
- [readme.md](./readme.md)
- [std::ptr](https://doc.rust-lang.org/std/ptr/) - Standard pointer utilities
- [std::mem](https://doc.rust-lang.org/std/mem/) - Memory utilities
- [memcmp](https://man7.org/linux/man-pages/man3/memcmp.3.html) - C memcmp documentation

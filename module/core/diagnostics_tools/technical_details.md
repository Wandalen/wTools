# Technical Details

This document contains implementation details and technical information for the `diagnostics_tools` crate.

## Architecture Overview

The crate is organized into three main modules:

- **`rta`** - Runtime assertions (Runtime-Time Assertions)
- **`cta`** - Compile-time assertions (Compile-Time Assertions)  
- **`layout`** - Memory layout validation

## Module Structure

### Runtime Assertions (`rta`)

All runtime assertion macros follow the pattern `a_*` (assertion):

- `a_true!(condition)` - Assert condition is true
- `a_false!(condition)` - Assert condition is false
- `a_id!(left, right)` - Assert values are identical (equal)
- `a_not_id!(left, right)` - Assert values are not identical

Debug variants (`a_dbg_*`) print values even when assertions pass:

- `a_dbg_true!(condition)`
- `a_dbg_false!(condition)`
- `a_dbg_id!(left, right)`
- `a_dbg_not_id!(left, right)`

### Compile-Time Assertions (`cta`)

- `cta_true!(condition)` - Compile-time boolean check using `cfg` conditions

### Memory Layout Validation (`layout`)

- `cta_type_same_size!(Type1, Type2)` - Verify types have same size
- `cta_type_same_align!(Type1, Type2)` - Verify types have same alignment
- `cta_ptr_same_size!(ptr1, ptr2)` - Verify pointers have same size
- `cta_mem_same_size!(val1, val2)` - Verify values have same memory size

## Implementation Details

### Error Message Enhancement

The crate uses `pretty_assertions` internally to provide:
- Colored diff output
- Structured comparison formatting
- Better visual distinction between expected and actual values

### Compile-Time Validation

Compile-time assertions use Rust's `compile_error!` macro combined with `cfg` attributes to validate conditions during compilation.

### Memory Layout Checks

Memory layout assertions use:
- `core::mem::size_of::<T>()` for size validation
- `core::mem::align_of::<T>()` for alignment validation
- Array length tricks to force compile-time evaluation

## Feature Flags

The crate supports several feature flags for conditional compilation:

- `enabled` - Master switch for all functionality (default)
- `diagnostics_runtime_assertions` - Runtime assertion macros (default)
- `diagnostics_compiletime_assertions` - Compile-time assertion macros (default) 
- `diagnostics_memory_layout` - Memory layout validation macros (default)
- `no_std` - Support for no_std environments
- `full` - Enable all features

## Performance Considerations

### Runtime Overhead

- Runtime assertions have the same overhead as standard `assert!` macros
- Debug variants have additional overhead for value formatting
- All assertions are removed in release builds unless explicitly enabled

### Compile-Time Impact

- Compile-time assertions have zero runtime overhead
- They may slightly increase compilation time due to additional checking
- Memory layout assertions are resolved entirely at compile time

## Namespace Organization

The crate uses a hierarchical namespace structure:

```
diagnostics_tools/
├── own/       - Direct exports
├── orphan/    - Re-exports from submodules  
├── exposed/   - Extended API surface
└── prelude/   - Common imports
```

## Integration with Testing Frameworks

The runtime assertions integrate seamlessly with:
- Built-in Rust test framework (`#[test]`)
- Custom test harnesses
- Benchmark frameworks

## Error Handling Philosophy

The crate follows Rust's philosophy of "fail fast":
- Runtime assertions panic on failure (like standard assertions)
- Compile-time assertions prevent compilation on failure
- Clear, actionable error messages help identify root causes quickly

## Cross-Platform Compatibility

- Full support for all Rust-supported platforms
- `no_std` compatibility for embedded systems
- Consistent behavior across different architectures
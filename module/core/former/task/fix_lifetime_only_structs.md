# Task: Fix Lifetime-Only Structs Support

## Problem

The Former derive macro fails when applied to structs that have only lifetime parameters without any type parameters.

### Example of failing code:
```rust
#[derive(Former)]
struct MyStruct<'a> {
    data: &'a str,
}
```

### Error:
```
error: expected `while`, `for`, `loop` or `{` after a label
```

## Root Cause Analysis

The issue occurs because:

1. The macro generates code like `Former<'a, Definition>` where `'a` is in a position that expects a type parameter
2. Many code generation patterns assume at least one non-lifetime generic parameter
3. The `build_generics_with_params` function doesn't distinguish between lifetime and type parameters

## Solution Overview

### Phase 1: Create Generic Handling Utilities in macro_tools

1. Add utilities to `macro_tools` for better generic parameter handling
2. Create functions to separate and recombine lifetimes and type parameters
3. Add helpers to build generic lists with proper parameter ordering

### Phase 2: Update former_meta to Use New Utilities

1. Update `former_struct.rs` to properly handle lifetime-only cases
2. Generate different code patterns based on generic parameter types
3. Ensure all impl blocks handle lifetime parameters correctly

## Detailed Implementation Plan

### Step 1: Analyze Current Generic Decomposition

The current `generic_params::decompose` returns:
- `struct_generics_impl` - includes both lifetimes and type params
- `struct_generics_ty` - includes both lifetimes and type params

We need to separate these into:
- Lifetime parameters only
- Type/const parameters only
- Combined parameters with proper ordering

### Step 2: Create New macro_tools Utilities

Add to `macro_tools/src/generic_params.rs`:

```rust
/// Split generics into lifetime and non-lifetime parameters
pub fn split_generics(generics: &syn::Generics) -> (
    Punctuated<syn::LifetimeDef, syn::token::Comma>,  // lifetimes
    Punctuated<syn::GenericParam, syn::token::Comma>, // types/consts
) {
    // Implementation
}

/// Build a properly ordered generic parameter list
pub fn build_ordered_generics(
    lifetimes: &Punctuated<syn::LifetimeDef, syn::token::Comma>,
    type_params: &Punctuated<syn::GenericParam, syn::token::Comma>,
) -> Punctuated<syn::GenericParam, syn::token::Comma> {
    // Lifetimes must come first, then types/consts
}
```

### Step 3: Update former_meta

Key areas to update in `former_struct.rs`:

1. **Former type generation**:
   - When only lifetimes: `Former<Definition>`
   - When types exist: `Former<T1, T2, Definition>`
   - When both: `Former<'a, 'b, T1, T2, Definition>`

2. **Impl block headers**:
   - Handle empty type params: `impl<'a, Definition>`
   - Handle mixed: `impl<'a, T, Definition>`

3. **Associated type projections**:
   - Ensure lifetime parameters are properly passed through

### Step 4: Test Cases

Create comprehensive tests:
1. Struct with only lifetimes
2. Struct with only types
3. Struct with both
4. Multiple lifetimes
5. Complex lifetime bounds

## Success Criteria

1. All lifetime-only struct tests pass
2. No regression in existing tests
3. Clear separation of concerns between macro_tools and former_meta
4. Reusable utilities in macro_tools for other macros

## Files to Modify

1. `/home/user1/pro/lib/wTools/module/core/macro_tools/src/generic_params.rs`
2. `/home/user1/pro/lib/wTools/module/core/former_meta/src/derive_former/former_struct.rs`
3. `/home/user1/pro/lib/wTools/module/core/former/tests/inc/struct_tests/a_basic.rs` (re-enable test)
4. Create new test files for comprehensive coverage

## Dependencies

- This task depends on understanding the current generic parameter handling
- Requires careful testing to avoid regressions
- Should maintain backward compatibility
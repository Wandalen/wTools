# Task: Add Generic Parameter Utilities to macro_tools

## Purpose

Add comprehensive utilities for handling generic parameters in procedural macros, with special focus on separating and managing lifetime vs type/const parameters.

## Problem

Currently, macro_tools provides `generic_params::decompose` which returns all parameters together. Many procedural macros need to:
1. Separate lifetime parameters from type/const parameters
2. Build generic lists with proper ordering (lifetimes first)
3. Handle edge cases like empty parameter lists
4. Combine parameters from different sources maintaining correct syntax

## Proposed API

### New Functions in `generic_params` module

```rust
/// Separate lifetime parameters from type/const parameters
pub fn split_generic_params(
    params: &Punctuated<syn::GenericParam, syn::token::Comma>
) -> GenericParamSplit {
    // Returns struct with lifetime_params and type_const_params
}

pub struct GenericParamSplit {
    pub lifetime_params: Punctuated<syn::GenericParam, syn::token::Comma>,
    pub type_const_params: Punctuated<syn::GenericParam, syn::token::Comma>,
}

/// Check if parameters contain only lifetimes
pub fn has_only_lifetimes(params: &Punctuated<syn::GenericParam, syn::token::Comma>) -> bool

/// Check if parameters contain any lifetimes
pub fn has_lifetimes(params: &Punctuated<syn::GenericParam, syn::token::Comma>) -> bool

/// Check if parameters contain any type/const params
pub fn has_type_or_const_params(params: &Punctuated<syn::GenericParam, syn::token::Comma>) -> bool

/// Build a combined parameter list with proper ordering
/// Ensures lifetimes come first, then types, then consts
pub fn merge_generic_params(
    lifetimes: &Punctuated<syn::GenericParam, syn::token::Comma>,
    types: &Punctuated<syn::GenericParam, syn::token::Comma>,
    consts: &Punctuated<syn::GenericParam, syn::token::Comma>,
) -> Punctuated<syn::GenericParam, syn::token::Comma>

/// Add parameters to existing list with proper comma handling
pub fn append_generic_params(
    base: &mut Punctuated<syn::GenericParam, syn::token::Comma>,
    additional: &[syn::GenericParam],
) -> &mut Punctuated<syn::GenericParam, syn::token::Comma>
```

### Enhanced decompose function

```rust
/// Enhanced version that provides split parameters
pub fn decompose_with_split(generics: &syn::Generics) -> DecomposedWithSplit {
    // Returns regular decomposed + split version
}

pub struct DecomposedWithSplit {
    // Regular decomposed fields
    pub generics_with_defaults: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    pub generics_impl: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    pub generics_ty: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    pub generics_where: syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
    
    // Split versions
    pub lifetimes_impl: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    pub types_impl: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    pub lifetimes_ty: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    pub types_ty: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
}
```

## Implementation Details

### Key Considerations

1. **Maintain Order**: Generic parameters must maintain their relative order within each category
2. **Comma Handling**: Properly handle commas between parameters (no trailing commas)
3. **Empty Lists**: Handle empty parameter lists gracefully
4. **Performance**: These utilities will be called frequently in macro expansion

### Implementation Steps

1. Create new types for split results
2. Implement splitting logic using syn's GenericParam enum
3. Add comprehensive tests for edge cases
4. Update documentation with examples
5. Consider deprecating some existing functions if new ones are more general

## Testing Requirements

Test cases should cover:
1. Empty generics
2. Only lifetimes: `<'a>`, `<'a, 'b>`
3. Only types: `<T>`, `<T, U>`
4. Only consts: `<const N: usize>`
5. Mixed: `<'a, T, const N: usize>`
6. Complex bounds: `<'a: 'b, T: Clone>`
7. Default values: `<T = i32>`

## Benefits

1. **Reusability**: Other macros in wTools can use these utilities
2. **Correctness**: Centralized logic for proper generic handling
3. **Maintainability**: Single source of truth for generic parameter manipulation
4. **Type Safety**: Strongly typed APIs prevent common mistakes

## Migration Guide

For existing users of macro_tools:
1. The existing `decompose` function remains unchanged
2. New functions are additive, not breaking
3. Consider using `decompose_with_split` for new code that needs separated parameters

## Example Usage

```rust
use macro_tools::generic_params;

let generics: &syn::Generics = /* ... */;
let split = generic_params::decompose_with_split(generics);

if split.lifetimes_impl.is_empty() {
    // Handle no lifetimes case
    quote! { impl<#types_impl, Definition> }
} else if split.types_impl.is_empty() {
    // Handle only lifetimes case  
    quote! { impl<#lifetimes_impl, Definition> }
} else {
    // Handle mixed case
    quote! { impl<#lifetimes_impl, #types_impl, Definition> }
}
```
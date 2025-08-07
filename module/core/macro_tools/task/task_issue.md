# Task Issue: Fix Trailing Comma Generation in `generic_params::decompose`

## Issue Summary

The `generic_params::decompose` function in the `macro_tools` crate generates invalid Rust syntax by adding trailing commas to all generic parameters, causing "proc-macro derive produced unparsable tokens" errors when used in procedural macros.

## Root Cause

The `decompose` function in `/module/core/macro_tools/src/generic_params.rs` automatically adds trailing commas to all punctuated generic parameter lists on lines 501, 513, 527, 539, 544, and 553:

```rust
generics_for_impl.push_punct(syn::token::Comma::default());
generics_for_ty.push_punct(syn::token::Comma::default());
```

This creates invalid syntax when the generated parameters are used in contexts like:
- `impl < 'a, > Trait for Struct` (invalid - trailing comma after lifetime)
- `Struct < T, >` (invalid - trailing comma in type parameters)

## Problem Details

### Current Behavior
The function returns punctuated lists that always end with commas, even when used in contexts where trailing commas are not allowed or create invalid syntax.

### Impact
- Causes compilation failures in derive macros that use `decompose`
- Creates "expected `while`, `for`, `loop` or `{` after a label" errors
- Generates "comparison operators cannot be chained" errors
- Results in "proc-macro derive produced unparsable tokens" errors

### Affected Code Locations
In `generic_params.rs`, lines:
- 501: `generics_for_impl.push_punct(syn::token::Comma::default());`
- 513: `generics_for_ty.push_punct(syn::token::Comma::default());` 
- 527: `generics_for_impl.push_punct(syn::token::Comma::default());`
- 539: `generics_for_ty.push_punct(syn::token::Comma::default());`
- 544: `generics_for_impl.push_punct(syn::token::Comma::default());`
- 553: `generics_for_ty.push_punct(syn::token::Comma::default());`

## Suggested Fix

### Option 1: Remove Automatic Trailing Commas (Recommended)
Remove the automatic `push_punct` calls and let the caller decide when commas are needed:

```rust
// Remove these lines:
// generics_for_impl.push_punct(syn::token::Comma::default());
// generics_for_ty.push_punct(syn::token::Comma::default());

// Instead, only add commas between parameters, not at the end
```

### Option 2: Add Flag Parameter
Add a boolean parameter to control trailing comma behavior:

```rust
pub fn decompose(
  generics: &syn::Generics,
  trailing_commas: bool,
) -> (
  syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
) {
  // ... existing logic ...
  
  if trailing_commas {
    generics_for_impl.push_punct(syn::token::Comma::default());
    generics_for_ty.push_punct(syn::token::Comma::default());
  }
  
  // ... rest of function
}
```

### Option 3: Provide Utility Functions
Add helper functions for different use cases:

```rust
/// Get generics without trailing commas (for type usage)
pub fn decompose_clean(generics: &syn::Generics) -> (...) {
  let (mut with_defaults, mut impl_gen, mut ty_gen, where_gen) = decompose(generics);
  
  // Remove trailing commas
  if impl_gen.trailing_punct() {
    impl_gen.pop_punct();
  }
  if ty_gen.trailing_punct() {
    ty_gen.pop_punct();
  }
  
  (with_defaults, impl_gen, ty_gen, where_gen)
}

/// Get generics with trailing commas (for contexts that need them)
pub fn decompose_with_commas(generics: &syn::Generics) -> (...) {
  decompose(generics) // Current behavior
}
```

## Testing Requirements

The fix should be tested with:

1. **Empty generics**: `<>` → should not generate trailing commas
2. **Single lifetime**: `<'a>` → should not have trailing comma
3. **Multiple lifetimes**: `<'a, 'b>` → comma between, no trailing comma
4. **Mixed generics**: `<'a, T, const N: usize>` → commas between, no trailing comma
5. **Complex bounds**: `<T: Clone + Send>` → no trailing comma after bounds

## Backward Compatibility

### Breaking Change Assessment
- **Option 1**: Breaking change - existing code expecting trailing commas will need updates
- **Option 2**: Non-breaking - adds optional parameter with default to current behavior  
- **Option 3**: Non-breaking - adds new functions while keeping existing function unchanged

### Migration Strategy
If implementing Option 1 (recommended):
1. Update all internal usage sites to handle the new format
2. Provide temporary wrapper functions for backward compatibility
3. Update documentation with examples of correct usage

## Related Issues

This issue was discovered while fixing lifetime parameter handling in the `former` crate, where structs like:

```rust
#[derive(Former)]
pub struct Simple<'a> {
    name: &'a str,
}
```

Would generate invalid syntax due to trailing commas in the macro expansion.

## Priority

**High** - This affects the fundamental functionality of procedural macros using `generic_params::decompose` and causes compilation failures.

## Implementation Notes

- The function should maintain separator commas between parameters
- Only trailing commas (at the end of the list) should be controlled/removed
- Consider the `syn::punctuated::Punctuated` API methods like `trailing_punct()` and `pop_punct()` for clean removal
- Ensure `ensure_trailing_comma` helper function (line 482) behavior is also reviewed for consistency

## Minimal Reproducible Example (MRE)

### Failing Code
```rust
use macro_tools::generic_params;
use quote::quote;
use syn::parse_quote;

fn main() {
    // Parse a simple struct with lifetime parameter
    let generics: syn::Generics = parse_quote! { <'a> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // This generates invalid syntax due to trailing comma
    let invalid_impl = quote! { impl< #impl_gen > MyTrait for MyStruct };
    let invalid_type = quote! { MyStruct< #ty_gen > };
    
    println!("Invalid impl: {}", invalid_impl);
    // Outputs: impl< 'a, > MyTrait for MyStruct  (invalid syntax)
    
    println!("Invalid type: {}", invalid_type);  
    // Outputs: MyStruct< 'a, >  (invalid syntax)
}
```

### Expected Output
```rust
// Should generate:
impl< 'a > MyTrait for MyStruct    // No trailing comma
MyStruct< 'a >                     // No trailing comma
```

### Actual Output
```rust
// Currently generates:
impl< 'a, > MyTrait for MyStruct   // Invalid: trailing comma
MyStruct< 'a, >                    // Invalid: trailing comma
```

### Compilation Error
When used in procedural macros, this produces:
```
error: expected `while`, `for`, `loop` or `{` after a label
error: comparison operators cannot be chained
error: proc-macro derive produced unparsable tokens
```

### Real-World Usage Example
```rust
// In a derive macro using decompose:
#[derive(Former)]
pub struct Simple<'a> {
    name: &'a str,
}

// Expands to invalid code like:
impl< 'a, Definition > former::FormerBegin< 'a, Definition > 
for SimpleFormer< 'a, Definition >  // Invalid: 'a, should be just Definition
```

## Example Test Cases

```rust
#[test]
fn test_decompose_no_trailing_commas() {
    let generics: syn::Generics = syn::parse_quote! { <'a, T: Clone> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Should generate: 'a, T: Clone (no trailing comma)
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Should still have separating commas
    assert_eq!(impl_gen.len(), 2);
}

#[test]
fn test_decompose_empty_generics() {
    let generics: syn::Generics = syn::parse_quote! { };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Empty generics should not have any punctuation
    assert!(impl_gen.is_empty());
    assert!(ty_gen.is_empty());
}

#[test]
fn test_decompose_single_lifetime() {
    let generics: syn::Generics = syn::parse_quote! { <'a> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Single parameter should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    assert_eq!(impl_gen.len(), 1);
    assert_eq!(ty_gen.len(), 1);
}
```
# Task: Add Generic Parameter Utilities to macro_tools - Improved

## Purpose

Enhance the `generic_params` module with utilities for better lifetime and type/const parameter separation, building on the existing architecture and patterns of macro_tools.

## Problem Analysis

The current `generic_params::decompose` function provides excellent functionality for splitting generics into impl/ty/where components, but procedural macros often need:

1. **Parameter Type Detection**: Distinguish between lifetime, type, and const parameters
2. **Selective Filtering**: Extract only specific parameter types (e.g., only types, no lifetimes)
3. **Smart Combination**: Merge parameters from different sources with proper ordering
4. **Comma-Safe Building**: Build generic lists without trailing comma issues

## Proposed API (Revised)

### Core Detection Functions

```rust
/// Classify parameters by type
pub fn classify_generics(generics: &syn::Generics) -> GenericsClassification 
{
    // Separates into lifetimes, types, and consts
}

pub struct GenericsClassification 
{
    pub lifetimes: Vec<&syn::LifetimeParam>,
    pub types: Vec<&syn::TypeParam>,
    pub consts: Vec<&syn::ConstParam>,
    pub has_only_lifetimes: bool,
    pub has_only_types: bool,
    pub has_mixed: bool,
}

/// Filter generic parameters by type
pub fn filter_params<F>(
    params: &Punctuated<syn::GenericParam, syn::token::Comma>,
    predicate: F
) -> Punctuated<syn::GenericParam, syn::token::Comma>
where
    F: Fn(&syn::GenericParam) -> bool
{
    // Returns filtered params maintaining punctuation
}

/// Common filters as constants
pub const FILTER_LIFETIMES: fn(&syn::GenericParam) -> bool = |p| matches!(p, syn::GenericParam::Lifetime(_));
pub const FILTER_TYPES: fn(&syn::GenericParam) -> bool = |p| matches!(p, syn::GenericParam::Type(_));
pub const FILTER_CONSTS: fn(&syn::GenericParam) -> bool = |p| matches!(p, syn::GenericParam::Const(_));
pub const FILTER_NON_LIFETIMES: fn(&syn::GenericParam) -> bool = |p| !matches!(p, syn::GenericParam::Lifetime(_));
```

### Enhanced Decomposition

```rust
/// Extended decompose that provides classified parameters
pub fn decompose_classified(generics: &syn::Generics) -> DecomposedClassified 
{
    let (with_defaults, impl_params, ty_params, where_clause) = decompose(generics);
    let classification = classify_generics(generics);
    
    DecomposedClassified {
        // Original decomposed fields
        generics_with_defaults: with_defaults,
        generics_impl: impl_params,
        generics_ty: ty_params,
        generics_where: where_clause,
        
        // Classification
        classification,
        
        // Filtered versions (for convenience)
        generics_impl_only_types: filter_params(&impl_params, FILTER_TYPES),
        generics_impl_no_lifetimes: filter_params(&impl_params, FILTER_NON_LIFETIMES),
        generics_ty_only_types: filter_params(&ty_params, FILTER_TYPES),
        generics_ty_no_lifetimes: filter_params(&ty_params, FILTER_NON_LIFETIMES),
    }
}

pub struct DecomposedClassified 
{
    // Original fields from decompose
    pub generics_with_defaults: Punctuated<syn::GenericParam, syn::token::Comma>,
    pub generics_impl: Punctuated<syn::GenericParam, syn::token::Comma>,
    pub generics_ty: Punctuated<syn::GenericParam, syn::token::Comma>,
    pub generics_where: Punctuated<syn::WherePredicate, syn::token::Comma>,
    
    // Classification info
    pub classification: GenericsClassification,
    
    // Pre-filtered common cases
    pub generics_impl_only_types: Punctuated<syn::GenericParam, syn::token::Comma>,
    pub generics_impl_no_lifetimes: Punctuated<syn::GenericParam, syn::token::Comma>,
    pub generics_ty_only_types: Punctuated<syn::GenericParam, syn::token::Comma>,
    pub generics_ty_no_lifetimes: Punctuated<syn::GenericParam, syn::token::Comma>,
}
```

### Smart Combination Utilities

```rust
/// Merge multiple parameter lists maintaining proper order (lifetimes, types, consts)
pub fn merge_params_ordered(
    param_lists: &[&Punctuated<syn::GenericParam, syn::token::Comma>]
) -> Punctuated<syn::GenericParam, syn::token::Comma> {
    // Merges while maintaining lifetime->type->const order
}

/// Add parameters to existing list with smart comma handling
pub fn params_with_additional(
    base: &Punctuated<syn::GenericParam, syn::token::Comma>,
    additional: &[syn::GenericParam],
) -> Punctuated<syn::GenericParam, syn::token::Comma> {
    // Similar to build_generics_with_params from former_meta
}

/// Create a new parameter list from individual components
pub fn params_from_components(
    lifetimes: &[syn::LifetimeParam],
    types: &[syn::TypeParam], 
    consts: &[syn::ConstParam],
) -> Punctuated<syn::GenericParam, syn::token::Comma> {
    // Builds proper generic parameter list
}
```

### Integration with Existing GenericsRef

Extend `GenericsRef` with new methods:

```rust
impl<'a> GenericsRef<'a> {
    /// Get classification of the generics
    pub fn classification(&self) -> GenericsClassification 
{
        classify_generics(self.syn_generics)
    }
    
    /// Get impl generics without lifetimes
    pub fn impl_generics_no_lifetimes(&self) -> proc_macro2::TokenStream 
{
        let filtered = filter_params(&self.syn_generics.params, FILTER_NON_LIFETIMES);
        // Generate tokens...
    }
    
    /// Check if only contains lifetimes
    pub fn has_only_lifetimes(&self) -> bool 
{
        self.classification().has_only_lifetimes
    }
}
```

## Implementation Strategy

### Phase 1: Core Functions
1. Implement `classify_generics` with thorough testing
2. Implement `filter_params` with predicate support
3. Create common filter constants

### Phase 2: Enhanced Decomposition
1. Build `decompose_classified` on top of existing `decompose`
2. Add pre-filtered common cases for performance
3. Ensure backward compatibility

### Phase 3: Combination Utilities
1. Implement `merge_params_ordered` 
2. Add `params_with_additional` (similar to former's solution)
3. Create `params_from_components`

### Phase 4: Integration
1. Extend `GenericsRef` with new methods
2. Update documentation with examples
3. Add integration tests

## Key Design Principles

1. **Build on Existing**: Leverage existing `decompose` rather than replacing it
2. **Composable**: Small, focused functions that can be combined
3. **Type-Safe**: Use strong types (GenericsClassification) over tuples
4. **Performance**: Pre-compute common filtered cases
5. **Backward Compatible**: All changes are additive

## Testing Strategy

### Unit Tests
- Empty generics
- Single parameter type (only lifetimes, only types, only consts)
- Mixed parameters with complex bounds
- Edge cases (no params, many params)

### Integration Tests
- Use with former_meta patterns
- Verify comma handling
- Test with real macro scenarios

### Property Tests
- Order preservation
- No trailing commas
- Proper classification

## Migration Examples

### Before (in former_meta):
```rust
let has_only_lifetimes = struct_generics_impl.iter()
    .all(|param| matches!(param, syn::GenericParam::Lifetime(_)));
```

### After:
```rust
let decomposed = generic_params::decompose_classified(&ast.generics);
if decomposed.classification.has_only_lifetimes {
    // Handle lifetime-only case
}
```

### Building generics with additional params:
```rust
// Instead of manual building
let entity_generics = generic_params::params_with_additional(
    &struct_generics_impl,
    &[parse_quote! { Definition }],
);
```

## Benefits Over Original Proposal

1. **Simpler API**: Fewer functions, more composable
2. **Better Integration**: Extends existing types rather than creating parallel APIs
3. **Performance**: Pre-computed common cases in DecomposedClassified
4. **Cleaner Code**: Filter predicates are more flexible than fixed functions
5. **Type Safety**: GenericsClassification provides clear, typed information

## Documentation Requirements

1. Update module docs with new functionality
2. Add examples showing lifetime-only handling
3. Document the classification system
4. Show migration from manual filtering
5. Include performance considerations
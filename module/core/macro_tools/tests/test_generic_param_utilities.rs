//!
//! Tests for new generic parameter utilities in `macro_tools`
//!

use macro_tools::generic_params::*;
use quote::quote;
use syn::parse_quote;

// Test Matrix for classify_generics
// | ID    | Input                                      | Expected Classification                           |
// |-------|--------------------------------------------|-------------------------------------------------|
// | C1.1  | Empty generics                             | is_empty: true, all others false                |
// | C1.2  | Only lifetimes: <'a>                       | has_only_lifetimes: true                        |
// | C1.3  | Only lifetimes: <'a, 'b, 'c>               | has_only_lifetimes: true                        |
// | C1.4  | Only types: <T>                            | has_only_types: true                            |
// | C1.5  | Only types: <T, U, V>                      | has_only_types: true                            |
// | C1.6  | Only consts: <const N: usize>              | has_only_consts: true                           |
// | C1.7  | Only consts: <const N: usize, const M: i32>| has_only_consts: true                           |
// | C1.8  | Mixed: <'a, T>                             | has_mixed: true                                 |
// | C1.9  | Mixed: <T, const N: usize>                 | has_mixed: true                                 |
// | C1.10 | Mixed: <'a, T, const N: usize>             | has_mixed: true                                 |

#[ test ]
fn test_classify_generics_empty() {
    let generics: syn::Generics = parse_quote! {};
    let classification = classify_generics(&generics);
    
    assert!(classification.is_empty);
    assert!(!classification.has_only_lifetimes);
    assert!(!classification.has_only_types);
    assert!(!classification.has_only_consts);
    assert!(!classification.has_mixed);
    assert_eq!(classification.lifetimes.len(), 0);
    assert_eq!(classification.types.len(), 0);
    assert_eq!(classification.consts.len(), 0);
}

#[ test ]
fn test_classify_generics_only_lifetimes() {
    // Single lifetime
    let generics: syn::Generics = parse_quote! { <'a> };
    let classification = classify_generics(&generics);
    
    assert!(!classification.is_empty);
    assert!(classification.has_only_lifetimes);
    assert!(!classification.has_only_types);
    assert!(!classification.has_only_consts);
    assert!(!classification.has_mixed);
    assert_eq!(classification.lifetimes.len(), 1);
    
    // Multiple lifetimes
    let generics: syn::Generics = parse_quote! { <'a, 'b, 'c> };
    let classification = classify_generics(&generics);
    
    assert!(classification.has_only_lifetimes);
    assert_eq!(classification.lifetimes.len(), 3);
}

#[ test ]
fn test_classify_generics_only_types() {
    // Single type
    let generics: syn::Generics = parse_quote! { <T> };
    let classification = classify_generics(&generics);
    
    assert!(!classification.is_empty);
    assert!(!classification.has_only_lifetimes);
    assert!(classification.has_only_types);
    assert!(!classification.has_only_consts);
    assert!(!classification.has_mixed);
    assert_eq!(classification.types.len(), 1);
    
    // Multiple types with bounds
    let generics: syn::Generics = parse_quote! { <T: Clone, U: Default, V> };
    let classification = classify_generics(&generics);
    
    assert!(classification.has_only_types);
    assert_eq!(classification.types.len(), 3);
}

#[ test ]
fn test_classify_generics_only_consts() {
    // Single const
    let generics: syn::Generics = parse_quote! { <const N: usize> };
    let classification = classify_generics(&generics);
    
    assert!(!classification.is_empty);
    assert!(!classification.has_only_lifetimes);
    assert!(!classification.has_only_types);
    assert!(classification.has_only_consts);
    assert!(!classification.has_mixed);
    assert_eq!(classification.consts.len(), 1);
    
    // Multiple consts
    let generics: syn::Generics = parse_quote! { <const N: usize, const M: i32> };
    let classification = classify_generics(&generics);
    
    assert!(classification.has_only_consts);
    assert_eq!(classification.consts.len(), 2);
}

#[ test ]
fn test_classify_generics_mixed() {
    // Lifetime + Type
    let generics: syn::Generics = parse_quote! { <'a, T> };
    let classification = classify_generics(&generics);
    
    assert!(classification.has_mixed);
    assert!(!classification.has_only_lifetimes);
    assert!(!classification.has_only_types);
    assert!(!classification.has_only_consts);
    
    // Type + Const
    let generics: syn::Generics = parse_quote! { <T, const N: usize> };
    let classification = classify_generics(&generics);
    
    assert!(classification.has_mixed);
    
    // All three types
    let generics: syn::Generics = parse_quote! { <'a, T: Clone, const N: usize> };
    let classification = classify_generics(&generics);
    
    assert!(classification.has_mixed);
    assert_eq!(classification.lifetimes.len(), 1);
    assert_eq!(classification.types.len(), 1);
    assert_eq!(classification.consts.len(), 1);
}

// Test filter_params
#[ test ]
fn test_filter_params_lifetimes() {
    let generics: syn::Generics = parse_quote! { <'a, 'b, T, U, const N: usize> };
    let filtered = filter_params(&generics.params, filter_lifetimes);
    
    assert_eq!(filtered.len(), 2);
    assert!(!filtered.trailing_punct());
    
    // Verify all items are lifetimes
    for param in &filtered {
        assert!(matches!(param, syn::GenericParam::Lifetime(_)));
    }
}

#[ test ]
fn test_filter_params_types() {
    let generics: syn::Generics = parse_quote! { <'a, T: Clone, U, const N: usize> };
    let filtered = filter_params(&generics.params, filter_types);
    
    assert_eq!(filtered.len(), 2);
    assert!(!filtered.trailing_punct());
    
    // Verify all items are types
    for param in &filtered {
        assert!(matches!(param, syn::GenericParam::Type(_)));
    }
}

#[ test ]
fn test_filter_params_consts() {
    let generics: syn::Generics = parse_quote! { <'a, T, const N: usize, const M: i32> };
    let filtered = filter_params(&generics.params, filter_consts);
    
    assert_eq!(filtered.len(), 2);
    assert!(!filtered.trailing_punct());
    
    // Verify all items are consts
    for param in &filtered {
        assert!(matches!(param, syn::GenericParam::Const(_)));
    }
}

#[ test ]
fn test_filter_params_non_lifetimes() {
    let generics: syn::Generics = parse_quote! { <'a, 'b, T, const N: usize> };
    let filtered = filter_params(&generics.params, filter_non_lifetimes);
    
    assert_eq!(filtered.len(), 2); // T and const N
    assert!(!filtered.trailing_punct());
    
    // Verify no lifetimes
    for param in &filtered {
        assert!(!matches!(param, syn::GenericParam::Lifetime(_)));
    }
}

#[ test ]
fn test_filter_params_custom_predicate() {
    let generics: syn::Generics = parse_quote! { <T: Clone, U: Default, V> };
    
    // Filter types with bounds
    let with_bounds = filter_params(&generics.params, |p| {
        if let syn::GenericParam::Type(ty) = p {
            !ty.bounds.is_empty()
        } else {
            false
        }
    });
    
    assert_eq!(with_bounds.len(), 2); // T and U have bounds
}

// Test decompose_classified
#[ test ]
fn test_decompose_classified_basic() {
    let generics: syn::Generics = parse_quote! { <'a, T: Clone, const N: usize> };
    let decomposed = decompose_classified(&generics);
    
    // Check classification
    assert!(decomposed.classification.has_mixed);
    assert_eq!(decomposed.classification.lifetimes.len(), 1);
    assert_eq!(decomposed.classification.types.len(), 1);
    assert_eq!(decomposed.classification.consts.len(), 1);
    
    // Check pre-filtered lists
    assert_eq!(decomposed.generics_impl_only_types.len(), 1);
    assert_eq!(decomposed.generics_impl_no_lifetimes.len(), 2); // T and const N
    assert_eq!(decomposed.generics_ty_only_types.len(), 1);
    assert_eq!(decomposed.generics_ty_no_lifetimes.len(), 2);
    
    // Check that original decomposition still works
    assert!(decomposed.generics_with_defaults.trailing_punct());
    assert!(!decomposed.generics_impl.trailing_punct());
    assert!(!decomposed.generics_ty.trailing_punct());
}

#[ test ]
fn test_decompose_classified_lifetime_only() {
    let generics: syn::Generics = parse_quote! { <'a, 'b> };
    let decomposed = decompose_classified(&generics);
    
    assert!(decomposed.classification.has_only_lifetimes);
    assert!(decomposed.generics_impl_only_types.is_empty());
    assert!(decomposed.generics_impl_no_lifetimes.is_empty());
}

// Test merge_params_ordered
#[ test ]
fn test_merge_params_ordered_basic() {
    let list1: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        parse_quote! { T, const N: usize };
    let list2: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        parse_quote! { 'a, U };
    
    let merged = merge_params_ordered(&[&list1, &list2]);
    
    // Should be ordered: lifetimes, types, consts
    assert_eq!(merged.len(), 4);
    assert!(!merged.trailing_punct());
    
    // Check order
    let params: Vec<_> = merged.iter().collect();
    assert!(matches!(params[0], syn::GenericParam::Lifetime(_))); // 'a
    assert!(matches!(params[1], syn::GenericParam::Type(_)));     // T
    assert!(matches!(params[2], syn::GenericParam::Type(_)));     // U
    assert!(matches!(params[3], syn::GenericParam::Const(_)));    // const N
}

#[ test ]
fn test_merge_params_ordered_empty() {
    let list1: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        syn::punctuated::Punctuated::new();
    let list2: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        parse_quote! { T };
    
    let merged = merge_params_ordered(&[&list1, &list2]);
    assert_eq!(merged.len(), 1);
    
    let merged_empty = merge_params_ordered(&[&list1, &list1]);
    assert!(merged_empty.is_empty());
}

#[ test ]
fn test_merge_params_ordered_complex() {
    let list1: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        parse_quote! { 'b, T: Clone, const N: usize };
    let list2: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        parse_quote! { 'a, U: Default };
    let list3: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        parse_quote! { const M: i32, V };
    
    let merged = merge_params_ordered(&[&list1, &list2, &list3]);
    
    // Should have: 'b, 'a (lifetimes), T, U, V (types), const N, const M (consts)
    assert_eq!(merged.len(), 7);
    
    let params: Vec<_> = merged.iter().collect();
    // First two should be lifetimes
    assert!(matches!(params[0], syn::GenericParam::Lifetime(_)));
    assert!(matches!(params[1], syn::GenericParam::Lifetime(_)));
    // Next three should be types
    assert!(matches!(params[2], syn::GenericParam::Type(_)));
    assert!(matches!(params[3], syn::GenericParam::Type(_)));
    assert!(matches!(params[4], syn::GenericParam::Type(_)));
    // Last two should be consts
    assert!(matches!(params[5], syn::GenericParam::Const(_)));
    assert!(matches!(params[6], syn::GenericParam::Const(_)));
}

// Test params_with_additional
#[ test ]
fn test_params_with_additional_basic() {
    let base: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        parse_quote! { T, U };
    let additional = vec![parse_quote! { V }, parse_quote! { const N: usize }];
    
    let extended = params_with_additional(&base, &additional);
    
    assert_eq!(extended.len(), 4);
    assert!(!extended.trailing_punct());
    
    // Verify order is preserved
    let params: Vec<_> = extended.iter().collect();
    if let syn::GenericParam::Type(ty) = params[0] {
        assert_eq!(ty.ident.to_string(), "T");
    }
    if let syn::GenericParam::Type(ty) = params[2] {
        assert_eq!(ty.ident.to_string(), "V");
    }
}

#[ test ]
fn test_params_with_additional_empty_base() {
    let base: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        syn::punctuated::Punctuated::new();
    let additional = vec![parse_quote! { T }];
    
    let extended = params_with_additional(&base, &additional);
    
    assert_eq!(extended.len(), 1);
    assert!(!extended.trailing_punct());
}

#[ test ]
fn test_params_with_additional_with_trailing_comma() {
    let mut base: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
        parse_quote! { T };
    base.push_punct(syn::token::Comma::default()); // Test edge case where base params already have trailing punctuation
    
    let additional = vec![parse_quote! { U }];
    let extended = params_with_additional(&base, &additional);
    
    assert_eq!(extended.len(), 2);
    assert!(!extended.trailing_punct()); // Should not have trailing comma
}

// Test params_from_components
#[ test ]
fn test_params_from_components_basic() {
    let lifetimes = vec![parse_quote! { 'a }, parse_quote! { 'b }];
    let types = vec![parse_quote! { T: Clone }];
    let consts = vec![parse_quote! { const N: usize }];
    
    let params = params_from_components(&lifetimes, &types, &consts);
    
    assert_eq!(params.len(), 4);
    assert!(!params.trailing_punct());
    
    // Check order
    let param_vec: Vec<_> = params.iter().collect();
    assert!(matches!(param_vec[0], syn::GenericParam::Lifetime(_)));
    assert!(matches!(param_vec[1], syn::GenericParam::Lifetime(_)));
    assert!(matches!(param_vec[2], syn::GenericParam::Type(_)));
    assert!(matches!(param_vec[3], syn::GenericParam::Const(_)));
}

#[ test ]
fn test_params_from_components_empty() {
    let params = params_from_components(&[], &[], &[]);
    assert!(params.is_empty());
    assert!(!params.trailing_punct());
}

#[ test ]
fn test_params_from_components_partial() {
    // Only types
    let types = vec![parse_quote! { T }, parse_quote! { U }];
    let params = params_from_components(&[], &types, &[]);
    
    assert_eq!(params.len(), 2);
    for param in &params {
        assert!(matches!(param, syn::GenericParam::Type(_)));
    }
}

// Test GenericsRef extensions
#[ test ]
fn test_generics_ref_classification() {
    let generics: syn::Generics = parse_quote! { <'a, T, const N: usize> };
    let generics_ref = GenericsRef::new(&generics);
    let classification = generics_ref.classification();
    
    assert!(classification.has_mixed);
    assert_eq!(classification.lifetimes.len(), 1);
    assert_eq!(classification.types.len(), 1);
    assert_eq!(classification.consts.len(), 1);
}

#[ test ]
fn test_generics_ref_has_only_methods() {
    // Only lifetimes
    let generics: syn::Generics = parse_quote! { <'a, 'b> };
    let generics_ref = GenericsRef::new(&generics);
    assert!(generics_ref.has_only_lifetimes());
    assert!(!generics_ref.has_only_types());
    assert!(!generics_ref.has_only_consts());
    
    // Only types
    let generics: syn::Generics = parse_quote! { <T, U> };
    let generics_ref = GenericsRef::new(&generics);
    assert!(!generics_ref.has_only_lifetimes());
    assert!(generics_ref.has_only_types());
    assert!(!generics_ref.has_only_consts());
    
    // Only consts
    let generics: syn::Generics = parse_quote! { <const N: usize, const M: i32> };
    let generics_ref = GenericsRef::new(&generics);
    assert!(!generics_ref.has_only_lifetimes());
    assert!(!generics_ref.has_only_types());
    assert!(generics_ref.has_only_consts());
}

#[ test ]
fn test_generics_ref_impl_no_lifetimes() {
    let generics: syn::Generics = parse_quote! { <'a, T: Clone, const N: usize> };
    let generics_ref = GenericsRef::new(&generics);
    let impl_no_lifetimes = generics_ref.impl_generics_no_lifetimes();
    
    let expected = quote! { < T : Clone , const N : usize > };
    assert_eq!(impl_no_lifetimes.to_string(), expected.to_string());
}

#[ test ]
fn test_generics_ref_ty_no_lifetimes() {
    let generics: syn::Generics = parse_quote! { <'a, T, const N: usize> };
    let generics_ref = GenericsRef::new(&generics);
    let ty_no_lifetimes = generics_ref.ty_generics_no_lifetimes();
    
    let expected = quote! { < T , const N : usize > };
    assert_eq!(ty_no_lifetimes.to_string(), expected.to_string());
}

#[ test ]
fn test_generics_ref_type_path_no_lifetimes() {
    use quote::format_ident;
    
    let generics: syn::Generics = parse_quote! { <'a, T, const N: usize> };
    let generics_ref = GenericsRef::new(&generics);
    let base = format_ident!("MyType");
    let path = generics_ref.type_path_no_lifetimes(&base);
    
    let expected = quote! { MyType < T , const N : usize > };
    assert_eq!(path.to_string(), expected.to_string());
    
    // Test with only lifetimes
    let generics2: syn::Generics = parse_quote! { <'a, 'b> };
    let generics_ref2 = GenericsRef::new(&generics2);
    let path2 = generics_ref2.type_path_no_lifetimes(&base);
    
    let expected2 = quote! { MyType };
    assert_eq!(path2.to_string(), expected2.to_string());
}

// Integration tests
#[ test ]
fn test_integration_former_meta_pattern() {
    // Simulate the former_meta use case
    let struct_generics: syn::Generics = parse_quote! { <'a, T: Clone, const N: usize> };
    
    // Old way (manual check)
    let has_only_lifetimes_old = struct_generics.params.iter()
        .all(|param| matches!(param, syn::GenericParam::Lifetime(_)));
    
    // New way
    let decomposed = decompose_classified(&struct_generics);
    let has_only_lifetimes_new = decomposed.classification.has_only_lifetimes;
    
    assert_eq!(has_only_lifetimes_old, has_only_lifetimes_new);
    assert!(!has_only_lifetimes_new); // Should be false for mixed generics
    
    // Building generics with additional param
    let additional_param: syn::GenericParam = parse_quote! { Definition };
    let entity_generics = params_with_additional(&decomposed.generics_impl, &[additional_param]);
    
    // Should have original 3 params + 1 new one
    assert_eq!(entity_generics.len(), 4);
}

#[ test ]
fn test_edge_cases() {
    // Empty filter result
    let generics: syn::Generics = parse_quote! { <'a, 'b> };
    let filtered = filter_params(&generics.params, filter_types);
    assert!(filtered.is_empty());
    assert!(!filtered.trailing_punct());
    
    // Single param filter
    let generics: syn::Generics = parse_quote! { <T> };
    let filtered = filter_params(&generics.params, filter_types);
    assert_eq!(filtered.len(), 1);
    assert!(!filtered.trailing_punct());
    
    // Merge with all empty
    let empty = syn::punctuated::Punctuated::new();
    let merged = merge_params_ordered(&[&empty, &empty, &empty]);
    assert!(merged.is_empty());
}
//!
//! Full coverage tests for `generic_params::decompose` function
//!

#![allow(unused_variables)]

use macro_tools::generic_params;
use quote::quote;
use syn::parse_quote;

// Test Matrix for generic_params::decompose
// 
// The decompose function processes generic parameters and returns four punctuated lists:
// 1. generics_with_defaults (preserves all, adds trailing comma via ensure_trailing_comma)
// 2. generics_for_impl (removes defaults, preserves bounds)
// 3. generics_for_ty (removes defaults and bounds, keeps only identifiers)
// 4. generics_where (where clause predicates with trailing comma)
//
// Code paths to cover:
// - Empty generics (no parameters, no where clause)
// - Type parameters (with/without bounds, with/without defaults)
// - Lifetime parameters (with/without bounds)
// - Const parameters (with/without defaults)
// - Where clause (present/absent)
// - Single vs multiple parameters (affects comma insertion logic)
// - Mixed parameter types in various orders
//
// Test Matrix:
// | ID    | Description                                      | Input                                                | Expected Behavior                                                           |
// |-------|--------------------------------------------------|------------------------------------------------------|-----------------------------------------------------------------------------|
// | D1.1  | Empty generics                                   | ``                                                   | All outputs empty                                                           |
// | D1.2  | Single lifetime                                  | `<'a>`                                               | No trailing commas, lifetime preserved                                      |
// | D1.3  | Single lifetime with bounds                      | `<'a: 'static>`                                      | impl keeps bounds, ty removes bounds                                        |
// | D1.4  | Multiple lifetimes                               | `<'a, 'b, 'c>`                                       | Commas between params, no trailing                                          |
// | D1.5  | Multiple lifetimes with bounds                   | `<'a: 'b, 'b: 'c, 'c>`                               | impl keeps bounds, ty removes all bounds                                    |
// | D1.6  | Single type parameter                            | `<T>`                                                | No trailing commas, type preserved                                          |
// | D1.7  | Single type with bounds                          | `<T: Clone>`                                         | impl keeps bounds, ty removes bounds                                        |
// | D1.8  | Single type with multiple bounds                 | `<T: Clone + Send + 'static>`                        | impl keeps all bounds, ty removes all                                       |
// | D1.9  | Single type with default                         | `<T = String>`                                       | with_defaults keeps default, impl/ty remove it                              |
// | D1.10 | Single type with bounds and default              | `<T: Clone = String>`                                | with_defaults keeps all, impl keeps bounds only, ty removes all            |
// | D1.11 | Multiple type parameters                         | `<T, U, V>`                                          | Commas between params, no trailing                                          |
// | D1.12 | Multiple types with mixed bounds/defaults        | `<T: Clone, U = i32, V: Send + Sync>`                | Appropriate handling of each parameter                                       |
// | D1.13 | Single const parameter                           | `<const N: usize>`                                   | No trailing commas, const preserved                                         |
// | D1.14 | Single const with default                        | `<const N: usize = 10>`                              | with_defaults keeps default, impl/ty remove it                              |
// | D1.15 | Multiple const parameters                        | `<const N: usize, const M: i32>`                     | Commas between params, no trailing                                          |
// | D1.16 | Mixed single params (lifetime, type, const)      | `<'a, T, const N: usize>`                            | Each handled appropriately, commas between                                   |
// | D1.17 | All param types with multiple of each            | `<'a, 'b, T: Clone, U, const N: usize, const M: u8>` | Correct ordering and comma placement                                         |
// | D1.18 | Empty where clause                               | `<T> where`                                          | Where clause empty in output                                                 |
// | D1.19 | Where clause with single predicate               | `<T> where T: Clone`                                 | Where predicate with trailing comma                                          |
// | D1.20 | Where clause with multiple predicates            | `<T, U> where T: Clone, U: Default`                  | All predicates preserved with trailing comma                                 |
// | D1.21 | Where clause with lifetime bounds                | `<'a, T> where 'a: 'static, T: 'a`                   | Lifetime bounds in where clause                                              |
// | D1.22 | Complex nested generics in bounds                | `<T: Iterator<Item = U>, U>`                         | Nested generics preserved in impl, removed in ty                            |
// | D1.23 | Associated type constraints                      | `<T: Iterator<Item = String>>`                       | Associated types preserved in impl, removed in ty                           |
// | D1.24 | Higher-ranked trait bounds in where              | `<T> where for<'a> T: Fn(&'a str)`                   | HRTB preserved in where clause                                               |
// | D1.25 | Const generics with complex types                | `<const N: [u8; 32]>`                                | Complex const type preserved                                                 |
// | D1.26 | Attributes on generic parameters                 | `<#[ cfg( feature = "foo" ) ] T>`                        | Attributes stripped in impl/ty                                               |
// | D1.27 | All features combined                            | Complex generics with all features                    | Everything handled correctly                                                 |

#[ test ]
fn test_d1_1_empty_generics() {
    let generics: syn::Generics = parse_quote! {};
    let (with_defaults, impl_gen, ty_gen, where_gen) = generic_params::decompose(&generics);
    
    assert!(with_defaults.is_empty());
    assert!(impl_gen.is_empty());
    assert!(ty_gen.is_empty());
    assert!(where_gen.is_empty());
}

#[ test ]
fn test_d1_2_single_lifetime() {
    let generics: syn::Generics = parse_quote! { <'a> };
    let (with_defaults, impl_gen, ty_gen, where_gen) = generic_params::decompose(&generics);
    
    assert!(with_defaults.trailing_punct()); // ensure_trailing_comma adds it
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    assert!(where_gen.is_empty());
    
    assert_eq!(impl_gen.len(), 1);
    assert_eq!(ty_gen.len(), 1);
    
    let impl_code = quote! { impl< #impl_gen > };
    let ty_code = quote! { Type< #ty_gen > };
    assert_eq!(impl_code.to_string(), "impl < 'a >");
    assert_eq!(ty_code.to_string(), "Type < 'a >");
}

#[ test ]
fn test_d1_3_single_lifetime_with_bounds() {
    let generics: syn::Generics = parse_quote! { <'a: 'static> };
    let (with_defaults, impl_gen, ty_gen, _where_gen) = generic_params::decompose(&generics);
    
    assert!(with_defaults.trailing_punct());
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Check that impl preserves bounds
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("'a : 'static"));
    
    // Check that ty removes bounds
    let ty_code = quote! { #ty_gen };
    assert_eq!(ty_code.to_string(), "'a");
}

#[ test ]
fn test_d1_4_multiple_lifetimes() {
    let generics: syn::Generics = parse_quote! { <'a, 'b, 'c> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    assert_eq!(impl_gen.len(), 3);
    assert_eq!(ty_gen.len(), 3);
    
    let impl_code = quote! { impl< #impl_gen > };
    assert_eq!(impl_code.to_string(), "impl < 'a , 'b , 'c >");
}

#[ test ]
fn test_d1_5_multiple_lifetimes_with_bounds() {
    let generics: syn::Generics = parse_quote! { <'a: 'b, 'b: 'c, 'c> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("'a : 'b"));
    assert!(impl_code.to_string().contains("'b : 'c"));
    
    let ty_code = quote! { #ty_gen };
    assert_eq!(ty_code.to_string(), "'a , 'b , 'c");
}

#[ test ]
fn test_d1_6_single_type_parameter() {
    let generics: syn::Generics = parse_quote! { <T> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    assert_eq!(impl_gen.len(), 1);
    assert_eq!(ty_gen.len(), 1);
}

#[ test ]
fn test_d1_7_single_type_with_bounds() {
    let generics: syn::Generics = parse_quote! { <T: Clone> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("T : Clone"));
    
    let ty_code = quote! { #ty_gen };
    assert_eq!(ty_code.to_string(), "T");
}

#[ test ]
fn test_d1_8_single_type_with_multiple_bounds() {
    let generics: syn::Generics = parse_quote! { <T: Clone + Send + 'static> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("Clone + Send + 'static"));
    
    let ty_code = quote! { #ty_gen };
    assert_eq!(ty_code.to_string(), "T");
}

#[ test ]
fn test_d1_9_single_type_with_default() {
    let generics: syn::Generics = parse_quote! { <T = String> };
    let (with_defaults, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let with_defaults_code = quote! { #with_defaults };
    assert!(with_defaults_code.to_string().contains("= String"));
    
    let impl_code = quote! { #impl_gen };
    assert!(!impl_code.to_string().contains("= String"));
    
    let ty_code = quote! { #ty_gen };
    assert!(!ty_code.to_string().contains("= String"));
}

#[ test ]
fn test_d1_10_single_type_with_bounds_and_default() {
    let generics: syn::Generics = parse_quote! { <T: Clone = String> };
    let (with_defaults, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let with_defaults_code = quote! { #with_defaults };
    assert!(with_defaults_code.to_string().contains("Clone"));
    assert!(with_defaults_code.to_string().contains("= String"));
    
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("Clone"));
    assert!(!impl_code.to_string().contains("= String"));
    
    let ty_code = quote! { #ty_gen };
    assert_eq!(ty_code.to_string(), "T");
}

#[ test ]
fn test_d1_11_multiple_type_parameters() {
    let generics: syn::Generics = parse_quote! { <T, U, V> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    assert_eq!(impl_gen.len(), 3);
    
    let impl_code = quote! { impl< #impl_gen > };
    assert_eq!(impl_code.to_string(), "impl < T , U , V >");
}

#[ test ]
fn test_d1_12_multiple_types_with_mixed_bounds_defaults() {
    let generics: syn::Generics = parse_quote! { <T: Clone, U = i32, V: Send + Sync> };
    let (with_defaults, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let with_defaults_code = quote! { #with_defaults };
    assert!(with_defaults_code.to_string().contains("= i32"));
    
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("T : Clone"));
    assert!(!impl_code.to_string().contains("= i32"));
    assert!(impl_code.to_string().contains("V : Send + Sync"));
    
    let ty_code = quote! { #ty_gen };
    assert_eq!(ty_code.to_string(), "T , U , V");
}

#[ test ]
fn test_d1_13_single_const_parameter() {
    let generics: syn::Generics = parse_quote! { <const N: usize> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    let impl_code = quote! { impl< #impl_gen > };
    assert_eq!(impl_code.to_string(), "impl < const N : usize >");
    
    let ty_code = quote! { Type< #ty_gen > };
    assert_eq!(ty_code.to_string(), "Type < const N : usize >");
}

#[ test ]
fn test_d1_14_single_const_with_default() {
    let generics: syn::Generics = parse_quote! { <const N: usize = 10> };
    let (with_defaults, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let with_defaults_code = quote! { #with_defaults };
    assert!(with_defaults_code.to_string().contains("= 10"));
    
    let impl_code = quote! { #impl_gen };
    assert!(!impl_code.to_string().contains("= 10"));
}

#[ test ]
fn test_d1_15_multiple_const_parameters() {
    let generics: syn::Generics = parse_quote! { <const N: usize, const M: i32> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    assert!(!impl_gen.trailing_punct());
    assert_eq!(impl_gen.len(), 2);
    
    let impl_code = quote! { impl< #impl_gen > };
    assert_eq!(impl_code.to_string(), "impl < const N : usize , const M : i32 >");
}

#[ test ]
fn test_d1_16_mixed_single_params() {
    let generics: syn::Generics = parse_quote! { <'a, T, const N: usize> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    assert!(!impl_gen.trailing_punct());
    assert_eq!(impl_gen.len(), 3);
    
    let impl_code = quote! { impl< #impl_gen > };
    assert_eq!(impl_code.to_string(), "impl < 'a , T , const N : usize >");
}

#[ test ]
fn test_d1_17_all_param_types_multiple() {
    let generics: syn::Generics = parse_quote! { <'a, 'b, T: Clone, U, const N: usize, const M: u8> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    assert!(!impl_gen.trailing_punct());
    assert_eq!(impl_gen.len(), 6);
    
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("'a , 'b"));
    assert!(impl_code.to_string().contains("T : Clone"));
    assert!(impl_code.to_string().contains("const N : usize"));
}

#[ test ]
fn test_d1_18_empty_where_clause() {
    // Note: syn doesn't parse empty where clause, so this test ensures empty where is handled
    let generics: syn::Generics = parse_quote! { <T> };
    let (_, _, _, where_gen) = generic_params::decompose(&generics);
    
    assert!(where_gen.is_empty());
}

#[ test ]
fn test_d1_19_where_clause_single_predicate() {
    // Parse from a struct to get proper where clause
    let item: syn::ItemStruct = parse_quote! {
        struct Test<T> where T: Clone {
            field: T,
        }
    };
    let (_, _, _, where_gen) = generic_params::decompose(&item.generics);
    
    assert!(where_gen.trailing_punct()); // ensure_trailing_comma adds it
    assert_eq!(where_gen.len(), 1);
    
    let where_code = quote! { where #where_gen };
    assert!(where_code.to_string().contains("T : Clone"));
}

#[ test ]
fn test_d1_20_where_clause_multiple_predicates() {
    let item: syn::ItemStruct = parse_quote! {
        struct Test<T, U> where T: Clone, U: Default {
            field1: T,
            field2: U,
        }
    };
    let (_, _, _, where_gen) = generic_params::decompose(&item.generics);
    
    assert!(where_gen.trailing_punct());
    assert_eq!(where_gen.len(), 2);
    
    let where_code = quote! { where #where_gen };
    assert!(where_code.to_string().contains("T : Clone"));
    assert!(where_code.to_string().contains("U : Default"));
}

#[ test ]
fn test_d1_21_where_clause_lifetime_bounds() {
    let item: syn::ItemStruct = parse_quote! {
        struct Test<'a, T> where 'a: 'static, T: 'a {
            field: &'a T,
        }
    };
    let (_, _, _, where_gen) = generic_params::decompose(&item.generics);
    
    let where_code = quote! { where #where_gen };
    assert!(where_code.to_string().contains("'a : 'static"));
    assert!(where_code.to_string().contains("T : 'a"));
}

#[ test ]
fn test_d1_22_complex_nested_generics() {
    let generics: syn::Generics = parse_quote! { <T: Iterator<Item = U>, U> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("Iterator < Item = U >"));
    
    let ty_code = quote! { #ty_gen };
    assert_eq!(ty_code.to_string(), "T , U");
}

#[ test ]
fn test_d1_23_associated_type_constraints() {
    let generics: syn::Generics = parse_quote! { <T: Iterator<Item = String>> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("Iterator < Item = String >"));
    
    let ty_code = quote! { #ty_gen };
    assert_eq!(ty_code.to_string(), "T");
}

#[ test ]
fn test_d1_24_higher_ranked_trait_bounds() {
    let item: syn::ItemStruct = parse_quote! {
        struct Test<T> where for<'a> T: Fn(&'a str) {
            field: T,
        }
    };
    let (_, _, _, where_gen) = generic_params::decompose(&item.generics);
    
    let where_code = quote! { where #where_gen };
    assert!(where_code.to_string().contains("for < 'a > T : Fn"));
}

#[ test ]
fn test_d1_25_const_generics_complex_types() {
    let generics: syn::Generics = parse_quote! { <const N: [u8; 32]> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("const N : [u8 ; 32]"));
    
    let ty_code = quote! { #ty_gen };
    assert!(ty_code.to_string().contains("const N : [u8 ; 32]"));
}

#[ test ]
fn test_d1_26_attributes_on_generic_params() {
    // Note: Attributes are stripped by decompose
    let generics: syn::Generics = parse_quote! { <#[ cfg( feature = "foo" ) ] T> };
    let (with_defaults, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Verify attributes are preserved in with_defaults but stripped in impl/ty
    // This requires checking the actual parameter attributes
    if let Some(syn::GenericParam::Type(tp)) = with_defaults.first() {
        assert!(!tp.attrs.is_empty(), "with_defaults should preserve attributes");
    }
    
    if let Some(syn::GenericParam::Type(tp)) = impl_gen.first() {
        assert!(tp.attrs.is_empty(), "impl_gen should strip attributes");
    }
}

#[ test ]
fn test_d1_27_all_features_combined() {
    let item: syn::ItemStruct = parse_quote! {
        struct Complex<'a: 'static, 'b, T: Clone + Send = String, U, const N: usize = 10> 
        where 
            T: Iterator<Item = U> + 'a,
            U: Default,
            for<'c> U: Fn(&'c str) -> &'c str
        {
            field1: &'a T,
            field2: U,
            array: [u8; N],
        }
    };
    
    let (with_defaults, impl_gen, ty_gen, where_gen) = generic_params::decompose(&item.generics);
    
    // Verify with_defaults preserves everything
    assert!(with_defaults.trailing_punct());
    let with_defaults_code = quote! { #with_defaults };
    assert!(with_defaults_code.to_string().contains("= String"));
    assert!(with_defaults_code.to_string().contains("= 10"));
    
    // Verify impl_gen removes defaults but keeps bounds
    assert!(!impl_gen.trailing_punct());
    let impl_code = quote! { #impl_gen };
    assert!(impl_code.to_string().contains("'a : 'static"));
    assert!(impl_code.to_string().contains("T : Clone + Send"));
    assert!(!impl_code.to_string().contains("= String"));
    assert!(!impl_code.to_string().contains("= 10"));
    
    // Verify ty_gen removes bounds and defaults
    assert!(!ty_gen.trailing_punct());
    let ty_code = quote! { #ty_gen };
    assert_eq!(ty_code.to_string(), "'a , 'b , T , U , const N : usize");
    
    // Verify where clause
    assert!(where_gen.trailing_punct());
    assert_eq!(where_gen.len(), 3);
    let where_code = quote! { where #where_gen };
    assert!(where_code.to_string().contains("T : Iterator < Item = U > + 'a"));
    assert!(where_code.to_string().contains("U : Default"));
    assert!(where_code.to_string().contains("for < 'c > U : Fn"));
}

// Edge case tests

#[ test ]
fn test_edge_case_single_param_is_last() {
    // Verify is_last logic works correctly with single parameter
    let generics: syn::Generics = parse_quote! { <T> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Single parameter should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
}

#[ test ]
fn test_edge_case_comma_placement_between_different_types() {
    // Verify commas are correctly placed between different parameter types
    let generics: syn::Generics = parse_quote! { <'a, T, const N: usize> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Verify that decompose preserves original comma formatting between parameters
    let impl_str = quote! { #impl_gen }.to_string();
    assert_eq!(impl_str, "'a , T , const N : usize");
}

#[ test ]
fn test_edge_case_preserve_original_params() {
    // Verify original generics are not modified
    let original_generics: syn::Generics = parse_quote! { <T: Clone = String> };
    let original_str = quote! { #original_generics }.to_string();
    
    let _ = generic_params::decompose(&original_generics);
    
    let after_str = quote! { #original_generics }.to_string();
    assert_eq!(original_str, after_str, "Original generics should not be modified");
}

#[ test ]
fn test_edge_case_where_clause_none() {
    // Verify None where clause is handled correctly
    let generics: syn::Generics = parse_quote! { <T> };
    assert!(generics.where_clause.is_none());
    
    let (_, _, _, where_gen) = generic_params::decompose(&generics);
    assert!(where_gen.is_empty());
}

#[ test ]
fn test_edge_case_empty_punctuated_lists() {
    // Verify empty punctuated lists are handled correctly
    let generics: syn::Generics = syn::Generics {
        lt_token: Some(syn::token::Lt::default()),
        params: syn::punctuated::Punctuated::new(),
        gt_token: Some(syn::token::Gt::default()),
        where_clause: None,
    };
    
    let (with_defaults, impl_gen, ty_gen, where_gen) = generic_params::decompose(&generics);
    
    assert!(with_defaults.is_empty());
    assert!(impl_gen.is_empty());
    assert!(ty_gen.is_empty());
    assert!(where_gen.is_empty());
}
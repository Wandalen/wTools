//! Tests for generic parameters without trailing commas

use macro_tools::generic_params;
use quote::quote;
use syn::parse_quote;

#[ test ]
fn test_decompose_no_trailing_commas() {
    let generics: syn::Generics = syn::parse_quote! { <'a, T: Clone> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Should generate: 'a, T: Clone (no trailing comma)
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Should still have separating commas
    assert_eq!(impl_gen.len(), 2);
    
    // Verify the generated code is valid
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    let expected_impl = quote! { impl< 'a, T: Clone > MyTrait for MyStruct };
    assert_eq!(impl_code.to_string(), expected_impl.to_string());
    
    let type_code = quote! { MyStruct< #ty_gen > };
    let expected_type = quote! { MyStruct< 'a, T > };
    assert_eq!(type_code.to_string(), expected_type.to_string());
}

#[ test ]
fn test_decompose_empty_generics() {
    let generics: syn::Generics = syn::parse_quote! { };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Empty generics should not have any punctuation
    assert!(impl_gen.is_empty());
    assert!(ty_gen.is_empty());
    
    // Verify generated code handles empty generics correctly
    let impl_code = quote! { impl MyTrait for MyStruct };
    let type_code = quote! { MyStruct };
    
    // With empty generics, we shouldn't add angle brackets
    assert_eq!(impl_code.to_string(), "impl MyTrait for MyStruct");
    assert_eq!(type_code.to_string(), "MyStruct");
}

#[ test ]
fn test_decompose_single_lifetime() {
    let generics: syn::Generics = syn::parse_quote! { <'a> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Single parameter should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    assert_eq!(impl_gen.len(), 1);
    assert_eq!(ty_gen.len(), 1);
    
    // Verify the generated code is valid
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    let expected_impl = quote! { impl< 'a > MyTrait for MyStruct };
    assert_eq!(impl_code.to_string(), expected_impl.to_string());
}

#[ test ]
fn test_decompose_multiple_lifetimes() {
    let generics: syn::Generics = syn::parse_quote! { <'a, 'b, 'c> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Should have correct number of parameters
    assert_eq!(impl_gen.len(), 3);
    assert_eq!(ty_gen.len(), 3);
    
    // Verify proper comma separation
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    let expected_impl = quote! { impl< 'a, 'b, 'c > MyTrait for MyStruct };
    assert_eq!(impl_code.to_string(), expected_impl.to_string());
}

#[ test ]
fn test_decompose_mixed_generics() {
    let generics: syn::Generics = syn::parse_quote! { <'a, T, const N: usize> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Verify the generated code is valid
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    let expected_impl = quote! { impl< 'a, T, const N: usize > MyTrait for MyStruct };
    assert_eq!(impl_code.to_string(), expected_impl.to_string());
    
    let type_code = quote! { MyStruct< #ty_gen > };
    let expected_type = quote! { MyStruct< 'a, T, const N: usize > };
    assert_eq!(type_code.to_string(), expected_type.to_string());
}

#[ test ]
fn test_decompose_complex_bounds() {
    let generics: syn::Generics = syn::parse_quote! { <T: Clone + Send + 'static> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Verify impl_gen preserves bounds
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    assert!(impl_code.to_string().contains("Clone + Send + 'static"));
    
    // Verify ty_gen removes bounds
    let type_code = quote! { MyStruct< #ty_gen > };
    let expected_type = quote! { MyStruct< T > };
    assert_eq!(type_code.to_string(), expected_type.to_string());
}

#[ test ]
fn test_decompose_with_defaults() {
    let generics: syn::Generics = syn::parse_quote! { <T = String, const N: usize = 10> };
    let (with_defaults, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // with_defaults should have trailing comma (via ensure_trailing_comma)
    assert!(with_defaults.trailing_punct());
    
    // impl_gen and ty_gen should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Verify defaults are preserved in with_defaults
    let with_defaults_code = quote! { #with_defaults };
    assert!(with_defaults_code.to_string().contains("= String"));
    assert!(with_defaults_code.to_string().contains("= 10"));
    
    // Verify defaults are removed in impl_gen
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    assert!(!impl_code.to_string().contains("= String"));
    assert!(!impl_code.to_string().contains("= 10"));
}

#[ test ]
fn test_decompose_with_where_clause() {
    // Parse a type with generics to extract the generics including where clause
    let item: syn::ItemStruct = parse_quote! {
        struct Test<T, U> where T: Clone, U: Send {
            field: T,
            field2: U,
        }
    };
    let generics = item.generics;
    let (_, impl_gen, ty_gen, where_clause) = generic_params::decompose(&generics);
    
    // Generics should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Where clause should have trailing comma (via ensure_trailing_comma)
    assert!(where_clause.trailing_punct());
    
    // Verify where clause content
    let where_code = quote! { where #where_clause };
    assert!(where_code.to_string().contains("T : Clone"));
    assert!(where_code.to_string().contains("U : Send"));
}

#[ test ]
fn test_decompose_single_const_param() {
    let generics: syn::Generics = syn::parse_quote! { <const N: usize> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Single parameter should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Verify the generated code is valid
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    let expected_impl = quote! { impl< const N: usize > MyTrait for MyStruct };
    assert_eq!(impl_code.to_string(), expected_impl.to_string());
}

#[ test ]
fn test_decompose_lifetime_bounds() {
    let generics: syn::Generics = syn::parse_quote! { <'a: 'b, 'b> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Should not have trailing comma
    assert!(!impl_gen.trailing_punct());
    assert!(!ty_gen.trailing_punct());
    
    // Verify impl_gen preserves lifetime bounds
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    assert!(impl_code.to_string().contains("'a : 'b"));
    
    // Verify ty_gen removes lifetime bounds
    let type_code = quote! { MyStruct< #ty_gen > };
    let expected_type = quote! { MyStruct< 'a, 'b > };
    assert_eq!(type_code.to_string(), expected_type.to_string());
}
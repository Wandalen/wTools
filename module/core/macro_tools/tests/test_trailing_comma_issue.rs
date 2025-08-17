//! Test for trailing comma issue fix in `generic_params::decompose`

use macro_tools::generic_params;
use quote::quote;
use syn::parse_quote;

#[ test ]
fn test_trailing_comma_issue_mre() {
    // Test case 1: Simple lifetime parameter
    let generics: syn::Generics = parse_quote! { <'a> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    // Generate code using the decomposed generics
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    let type_code = quote! { MyStruct< #ty_gen > };
    
    println!("Test 1 - Single lifetime:");
    println!("  impl_gen: {}", quote! { #impl_gen });
    println!("  ty_gen: {}", quote! { #ty_gen });
    println!("  Generated impl: {impl_code}");
    println!("  Generated type: {type_code}");
    
    // Check if trailing commas exist (they shouldn't)
    assert!(!impl_gen.trailing_punct(), "impl_gen should not have trailing comma");
    assert!(!ty_gen.trailing_punct(), "ty_gen should not have trailing comma");
    
    // Test case 2: Multiple generic parameters
    let generics: syn::Generics = parse_quote! { <'a, T: Clone, const N: usize> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    let type_code = quote! { MyStruct< #ty_gen > };
    
    println!("\nTest 2 - Multiple parameters:");
    println!("  impl_gen: {}", quote! { #impl_gen });
    println!("  ty_gen: {}", quote! { #ty_gen });
    println!("  Generated impl: {impl_code}");
    println!("  Generated type: {type_code}");
    
    // Check if trailing commas exist (they shouldn't)
    assert!(!impl_gen.trailing_punct(), "impl_gen should not have trailing comma");
    assert!(!ty_gen.trailing_punct(), "ty_gen should not have trailing comma");
    
    // Test case 3: Empty generics
    let generics: syn::Generics = parse_quote! { };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    println!("\nTest 3 - Empty generics:");
    println!("  impl_gen is empty: {}", impl_gen.is_empty());
    println!("  ty_gen is empty: {}", ty_gen.is_empty());
    
    // Test case 4: Type parameter only
    let generics: syn::Generics = parse_quote! { <T> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    let impl_code = quote! { impl< #impl_gen > MyTrait for MyStruct };
    let type_code = quote! { MyStruct< #ty_gen > };
    
    println!("\nTest 4 - Single type parameter:");
    println!("  impl_gen: {}", quote! { #impl_gen });
    println!("  ty_gen: {}", quote! { #ty_gen });
    println!("  Generated impl: {impl_code}");
    println!("  Generated type: {type_code}");
    
    assert!(!impl_gen.trailing_punct(), "impl_gen should not have trailing comma");
    assert!(!ty_gen.trailing_punct(), "ty_gen should not have trailing comma");
}
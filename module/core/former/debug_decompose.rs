use macro_tools::generic_params;
use syn::parse_quote;

fn main() {
    // Test case from the issue
    let generics: syn::Generics = parse_quote! { <'a> };
    let (_, impl_gen, ty_gen, _) = generic_params::decompose(&generics);
    
    println!("Input generics: {}", quote::quote!(#generics));
    println!("impl_gen: {}", quote::quote!(#impl_gen));
    println!("ty_gen: {}", quote::quote!(#ty_gen));
    
    // Test with multiple parameters
    let generics2: syn::Generics = parse_quote! { <'a, T> };
    let (_, impl_gen2, ty_gen2, _) = generic_params::decompose(&generics2);
    
    println!("Input generics2: {}", quote::quote!(#generics2));
    println!("impl_gen2: {}", quote::quote!(#impl_gen2));
    println!("ty_gen2: {}", quote::quote!(#ty_gen2));
}
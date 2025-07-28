use syn::{parse_quote, Generics};
use macro_tools::generic_params;

fn main() {
    println!("Testing decompose function with lifetime parameters...");
    
    // Test case 1: Simple lifetime parameter
    let generics1: Generics = parse_quote! { <'a> };
    let (with_defaults, impl_gen, ty_gen, where_gen) = generic_params::decompose(&generics1);
    
    println!("Test 1 - Single lifetime:");
    println!("  with_defaults: {}", quote::quote! { #with_defaults });
    println!("  impl_gen: {}", quote::quote! { #impl_gen });
    println!("  ty_gen: {}", quote::quote! { #ty_gen });
    println!("  where_gen: {}", quote::quote! { #where_gen });
    
    // Test case 2: Multiple lifetime parameters
    let generics2: Generics = parse_quote! { <'a, 'b> };
    let (with_defaults2, impl_gen2, ty_gen2, where_gen2) = generic_params::decompose(&generics2);
    
    println!("\nTest 2 - Multiple lifetimes:");
    println!("  with_defaults: {}", quote::quote! { #with_defaults2 });
    println!("  impl_gen: {}", quote::quote! { #impl_gen2 });
    println!("  ty_gen: {}", quote::quote! { #ty_gen2 });
    println!("  where_gen: {}", quote::quote! { #where_gen2 });
    
    // Test case 3: Mixed generics with lifetimes first
    let generics3: Generics = parse_quote! { <'a, T> };
    let (with_defaults3, impl_gen3, ty_gen3, where_gen3) = generic_params::decompose(&generics3);
    
    println!("\nTest 3 - Mixed (lifetime first):");
    println!("  with_defaults: {}", quote::quote! { #with_defaults3 });
    println!("  impl_gen: {}", quote::quote! { #impl_gen3 });
    println!("  ty_gen: {}", quote::quote! { #ty_gen3 });
    println!("  where_gen: {}", quote::quote! { #where_gen3 });
    
    // Test case 4: Mixed generics with types first
    let generics4: Generics = parse_quote! { <T, 'a> };
    let (with_defaults4, impl_gen4, ty_gen4, where_gen4) = generic_params::decompose(&generics4);
    
    println!("\nTest 4 - Mixed (type first):");
    println!("  with_defaults: {}", quote::quote! { #with_defaults4 });
    println!("  impl_gen: {}", quote::quote! { #impl_gen4 });
    println!("  ty_gen: {}", quote::quote! { #ty_gen4 });
    println!("  where_gen: {}", quote::quote! { #where_gen4 });
    
    // Test if generated code can be parsed back
    println!("\nTesting if generated code is valid Rust syntax:");
    
    // Test parsing the impl_gen output
    let impl_gen_str = format!("<{}>", quote::quote! { #impl_gen });
    match syn::parse_str::<Generics>(&impl_gen_str) {
        Ok(_) => println!("  impl_gen is valid: {}", impl_gen_str),
        Err(e) => println!("  impl_gen is INVALID: {} - Error: {}", impl_gen_str, e),
    }
    
    let ty_gen_str = format!("<{}>", quote::quote! { #ty_gen });
    match syn::parse_str::<Generics>(&ty_gen_str) {
        Ok(_) => println!("  ty_gen is valid: {}", ty_gen_str),
        Err(e) => println!("  ty_gen is INVALID: {} - Error: {}", ty_gen_str, e),
    }
}
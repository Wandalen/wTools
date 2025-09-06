//! Example: Extract Type Parameters
//! 
//! This example demonstrates how to use the `typ::type_parameters` function
//! to extract type parameters from a Rust type. This is useful in procedural
//! macros when you need to analyze generic types and work with their parameters.

#[ cfg( not( all( feature = "enabled", feature = "typ" ) ) ) ]
fn main() 
{
  println!( "This example requires the 'enabled' and 'typ' features to be enabled." );
  println!( "Try running with: cargo run --example macro_tools_extract_type_parameters --all-features" );
}

#[ cfg( all( feature = "enabled", feature = "typ" ) ) ]
fn main() 
{
  use macro_tools::{ typ, qt };

  println!( "=== Extract Type Parameters Example ===" );
  println!();

  // Example 1: Extract parameters from Option<i32>
  {
    println!( "Example 1: Extracting from Option<i32>" );
    
    // Generate a token stream representing the type Option<i32>
    let code = qt!( Option< i32 > );
    
    // Parse the token stream into a syn::Type
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    
    // Extract the first type parameter (index 0)
    let params = typ::type_parameters( &tree_type, 0..=0 );
    
    print!( "Type parameters: " );
    for param in &params
    {
      print!( "{} ", qt!( #param ) );
    }
    println!();
    println!();
  }

  // Example 2: Extract multiple parameters from a complex type
  {
    println!( "Example 2: Extracting from HashMap<String, Vec<u8>>" );
    
    let code = qt!( std::collections::HashMap< String, Vec< u8 > > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    
    // Extract both type parameters (indices 0 and 1)
    let params = typ::type_parameters( &tree_type, 0..=1 );
    
    println!( "Type parameters:" );
    for (i, param) in params.iter().enumerate()
    {
      println!( "  [{}]: {}", i, qt!( #param ) );
    }
    println!();
  }

  // Example 3: Extract a subset of parameters
  {
    println!( "Example 3: Extracting subset from custom type with many parameters" );
    
    // A type with multiple generic parameters
    let code = qt!( MyType< 'a, String, i32, Vec< u8 >, bool > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    
    // Extract only parameters at indices 1, 2, and 3 (String, i32, Vec<u8>)
    let params = typ::type_parameters( &tree_type, 1..=3 );
    
    println!( "Selected type parameters (indices 1-3):" );
    params.iter().enumerate().for_each( |(i, param)| {
      println!( "  [{}]: {}", i + 1, qt!( #param ) );
    });
    println!();
  }

  // Example 4: Handle nested types
  {
    println!( "Example 4: Extracting from nested generic types" );
    
    let code = qt!( Result< Option< String >, std::io::Error > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    
    // Extract the outer type parameters
    let params = typ::type_parameters( &tree_type, 0..=1 );
    
    println!( "Outer type parameters of Result:" );
    params.iter().enumerate().for_each( |(i, param)| {
      println!( "  [{}]: {}", i, qt!( #param ) );
      
      // If the parameter is itself a generic type, we can extract its parameters too
      if let Ok( inner_type ) = syn::parse2::< syn::Type >( qt!( #param ) ) {
        if let Ok( inner_params ) = std::panic::catch_unwind( || {
          typ::type_parameters( &inner_type, 0..=0 )
        }) {
          if !inner_params.is_empty() {
            println!( "    Inner parameters:" );
            for inner in &inner_params {
              println!( "      - {}", qt!( #inner ) );
            }
          }
        }
      }
    });
  }

  println!();
  println!( "=== End of Examples ===" );
}
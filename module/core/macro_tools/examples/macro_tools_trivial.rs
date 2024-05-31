//! This example demonstrates the use of `typ::type_parameters` from the `macro_tools` crate.
//!
//! ### Example: Trivial One
//!
//! The purpose of `typ::type_parameters` is to extract type parameters from a given Rust type.
//! In this example, we generate a type `core::option::Option<i8, i16, i32, i64>` and extract its type parameters.
//!

#[ cfg( not( feature = "enabled" ) ) ]
fn main(){}
#[ cfg( feature = "enabled" ) ]
fn main()
{
  // Import necessary macros and modules from the `macro_tools` crate.
  use macro_tools::{ typ, qt };

  // Generate a token stream representing the type `core::option::Option<i8, i16, i32, i64>`.
  let code = qt!( core::option::Option< i8, i16, i32, i64 > );

  // Parse the generated token stream into a `syn::Type` object.
  // `syn::Type` is a syntax tree node representing a Rust type.
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();

  // Extract type parameters from the parsed type.
  // `typ::type_parameters` takes a reference to a `syn::Type` and a range.
  // It returns a vector of type parameters within the specified range.
  // Here, `0..=2` specifies that we are interested in the first three type parameters.
  let got = typ::type_parameters( &tree_type, 0..=2 );

  // Iterate over the extracted type parameters and print each one.
  // The `qt!` macro is used to convert the type parameter back to a token stream for printing.
  got.iter().for_each( | e | println!( "{}", qt!( #e ) ) );

  /* Expected output:
     i8
     i16
     i32
  */
}

//! Demonstrates basic usage of the `inspect_type` utilities.
//!
//! This example shows how to inspect types and sizes of various Rust expressions
//! using the `inspect_type_of!` and `inspect_to_str_type_of!` macros.

pub use inspect_type ::*;

fn main()
{
  println!( "Type Inspection Examples\n" );

  // Inspect slice type (fat pointer: data pointer + length)
  println!( "Slice inspection:" );
  inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  // Output: sizeof( &[1, 2, 3][..] : &[i32] ) = 16

  // Inspect array reference (thin pointer only)
  println!( "\nArray reference inspection:" );
  inspect_type_of!( &[ 1, 2, 3 ] );
  // Output: sizeof( &[1, 2, 3] : &[i32; 3] ) = 8

  // Silent inspection using inspect_to_str_type_of!
  println!( "\nSilent inspection (no stdout):" );
  let type_info = inspect_to_str_type_of!( vec![ 1, 2, 3 ] );
  println!( "Captured: {type_info}" );

  // Compare different representations
  println!( "\nComparing representations:" );
  let data = vec![ 1, 2, 3 ];
  inspect_type_of!( &data );       // Vec reference
  inspect_type_of!( &data[ .. ] ); // Slice from Vec
  inspect_type_of!( data[ 0 ] );   // Element value
}

//! Demonstrates basic usage of the `implements!` macro from wtools typing module.
//!
//! This example shows how to check at compile-time whether a type implements a specific trait.
//! The `implements!` macro from the typing module evaluates to a boolean indicating trait implementation.

#[ cfg( any( feature = "typing_implements", feature = "typing") ) ]
use wtools::implements;

fn main()
{
  #[ cfg( feature = "typing" ) ]
  {
  println!( "implements!( 13_i32 => Copy ) : {}", implements!( 13_i32 => Copy ) );
  println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", implements!( Box::new( 13_i32 ) => Copy ) );
  }
}

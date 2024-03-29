//! This example demonstrates the usage of some standard and non-standard functions
//! from the `iter_tools` crate. The `iter_tools` crate provides additional iterator
//! methods beyond those provided by the standard library.
#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  // Importing functions from the `iter_tools` crate
  use iter_tools::*;

  /* standard functions */
  // Creating a vector
  let vec = vec![ 5, 1, -2 ];
  // Finding the minimum value in the vector
  let min = min( &vec );
  assert_eq!( *min.unwrap(), -2 );

  /* non standard functions */
  // Creating another vector
  let vec = vec![ 5, 1, -2 ];
  // Initializing an empty vector to store the result
  let mut result = vec![];
  // Reversing the vector using the `rev` function from `iter_tools`
  let reversed = rev( &vec );
  // Iterating over the reversed vector
  for v in reversed
  {
    // Pushing the dereferenced value into the result vector
    result.push( *v );
  }
  assert_eq!( result, vec![ -2, 1, 5, ] );

}

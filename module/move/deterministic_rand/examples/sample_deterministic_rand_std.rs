//! Dealing with non-determinism in STD.
//!
//! In the standard library, randomness can also be a factor; for instance, iterating over the keys of a hashmap or hashset is non-deterministic. To achieve deterministic enumeration, you can use the `deterministic_rand::IfDeterminismIteratorExt` extension for iterators. By applying `if_determinism_then_sort` or `if_determinism_then_sort_by` before processing the keys, you can ensure a consistent order. The `if_determinism_then_sort_by` method acts as a no-op (no operation) when determinism is off, but it performs sorting when the determinism feature is on.

// Import the necessary modules from the standard library and the `deterministic_rand` crate.
use std::collections::HashMap;
use deterministic_rand::IfDeterminismIteratorExt;

fn main()
{
  // Create a HashMap with three key-value pairs.
  let map: HashMap<_, _> = HashMap::from_iter( [ ( 1, "first" ), ( 2, "second" ), ( 3, "third" ) ] );

  // Convert the HashMap into an iterator, apply deterministic sorting to the keys,
  // and then map each (key, value) pair to just the value.
  let _keys: Vec< _ > = map
  .into_iter()
  .if_determinism_then_sort_by( | ( a, _ ), ( b, _ ) | a.cmp( &b ) )
  .map( | e | e.1 )
  .collect();

  // If the 'determinism' feature is enabled, assert that the sorted keys match the expected order.
  // This is a conditional compilation check that ensures the code block is compiled and run only
  // if the 'determinism' feature is enabled.
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( _keys, vec![ "first", "second", "third" ] );

}

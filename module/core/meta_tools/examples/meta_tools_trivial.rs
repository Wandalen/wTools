//! This example showcases the usage of the `hmap!` macro from the `meta_tools` crate to create a hashmap and compares it with a hashmap created using `std::collections::HashMap`.
use meta_tools::*;

fn main()
{
  for_each!( dbg, "a", "b", "c" );

  // generates
  dbg!( "a" );
  dbg!( "b" );
  dbg!( "c" );
}

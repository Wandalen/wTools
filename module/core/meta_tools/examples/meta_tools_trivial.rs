//! This example showcases the usage of the `for_each!` macro from the `meta_tools` crate to apply a macro to multiple elements at compile-time.
use meta_tools :: *;

fn main()
{
  for_each!( dbg, "a", "b", "c" );

  // generates
  dbg!( "a" );
  dbg!( "b" );
  dbg!( "c" );
}

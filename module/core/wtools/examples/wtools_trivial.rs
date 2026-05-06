//! Demonstrates basic collection utilities via wtools.
//!
//! Shows variadic constructors for `HashMap` and `HashSet` from the re-exported `collection_tools`.

use wtools::*;

fn main()
{
  let map = hmap!{ "one" => 1, "two" => 2 };
  println!( "hmap : {map:?}" );

  let set = hset!{ 1_i32, 2, 3 };
  println!( "hset : {set:?}" );
}

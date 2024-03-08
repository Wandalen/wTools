//! # Example Usage
//!
//! Demonstrates how to use `HashMapSubformer` with the `HashMapLike` trait to build a `std::collections::HashMap`:
//!

fn main()
{
  use test_tools::exposed::*;

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithMap
  {
    #[ subformer( former::HashMapSubformer ) ]
    map : std::collections::HashMap< &'static str, &'static str >,
  }

  let struct1 = StructWithMap::former()
  .map()
    .insert( "a", "b" )
    .insert( "c", "d" )
    .end()
  .form()
  ;
  assert_eq!( struct1, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
}

//! # Example Usage
//!
//! Demonstrates how to use `HashMapSubformer` with the `HashMapLike` trait to build a `std::collections::HashMap`:
//!

fn main()
{
  use test_tools::exposed::*;

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithSet
  {
    #[ subformer( former::runtime::HashSetSubformer ) ]
    set : std::collections::HashSet< &'static str >,
  }

  let instance = StructWithSet::former()
  .set()
    .insert("apple")
    .insert("banana")
    .end()
  .form();

  assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });

}

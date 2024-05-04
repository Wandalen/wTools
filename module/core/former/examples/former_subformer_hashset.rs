//! # Example Usage
//!
//! Demonstrates how to use `HashMapSubformer` with the `HashMapLike` trait to build a `std::collections::HashMap`:
//!

#[ cfg( not( all( feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}

#[ cfg( all( feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use test_tools::exposed::*;

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithSet
  {
    #[ container ]
    set : std::collections::HashSet< &'static str >,
  }

  let instance = StructWithSet::former()
  .set()
    .add( "apple" )
    .add( "banana" )
    .end()
  .form();

  assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });

}

//! # Example Usage
//!
//! Demonstrates how to use `HashMapSubformer` with the `HashMapLike` trait to build a `std::collections::HashMap`:
//!

#[ cfg( not( all( feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}

#[ cfg( all( feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithVec
  {
    #[ subformer( former::VectorSubformer ) ]
    vec : Vec< &'static str >,
  }

  let instance = StructWithVec::former()
  .vec()
    .push( "apple" )
    .push( "banana" )
    .end()
  .form();

  assert_eq!( instance, StructWithVec { vec: vec![ "apple", "banana" ] } );

}

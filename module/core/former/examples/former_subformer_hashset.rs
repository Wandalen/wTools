//!
//! This example demonstrates the use of the `Former` trait to build a `collection_tools::HashSet` through subforming.
//!

#[ cfg( not( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::{ HashSet, hset };

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithSet
  {
    #[ container ]
    set : HashSet< &'static str >,
  }

  let instance = StructWithSet::former()
  .set()
    .add( "apple" )
    .add( "banana" )
    .end()
  .form();

  assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });
  dbg!( instance );

}

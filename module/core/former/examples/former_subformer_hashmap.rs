//!
//! This example demonstrates how to effectively employ the `Former` trait to configure a `HashMap` using a container setter.
//!

#[ cfg( not( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use test_tools::exposed::*;

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithMap
  {
    #[ container ]
    map : collection_tools::HashMap< &'static str, &'static str >,
  }

  let struct1 = StructWithMap::former()
  .map()
    .add( ( "a", "b" ) )
    .add( ( "c", "d" ) )
    .end()
  .form()
  ;
  assert_eq!( struct1, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
}

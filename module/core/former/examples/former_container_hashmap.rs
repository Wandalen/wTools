//!
//! This example demonstrates how to effectively employ the `Former` trait to configure a `HashMap` using a container setter.
//!

#[ cfg( not( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::{ HashMap, hmap };

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithMap
  {
    #[ container ]
    map : HashMap< &'static str, &'static str >,
  }

  let instance = StructWithMap::former()
  .map()
    .add( ( "a", "b" ) )
    .add( ( "c", "d" ) )
    .end()
  .form()
  ;
  assert_eq!( instance, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
  dbg!( instance );

}

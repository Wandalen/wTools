//!
//! This example demonstrates how to effectively employ the `Former` to configure a `HashMap` using a collection setter.
//!

#[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
fn main()
{
  use collection_tools::{ HashMap, hmap };

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithMap
  {
    #[ subform_collection ]
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

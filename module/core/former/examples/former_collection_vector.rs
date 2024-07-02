//!
//! This example demonstrates how to employ the `Former` to configure a `Vec` using a collection setter in a structured manner.
//!

#[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
fn main()
{

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithVec
  {
    #[ subform_collection ]
    vec : Vec< &'static str >,
  }

  let instance = StructWithVec::former()
  .vec()
    .add( "apple" )
    .add( "banana" )
    .end()
  .form();

  assert_eq!( instance, StructWithVec { vec: vec![ "apple", "banana" ] } );
  dbg!( instance );

}

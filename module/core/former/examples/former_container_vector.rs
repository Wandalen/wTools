//!
//! This example demonstrates how to employ the `Former` trait to configure a `Vec` using a container setter in a structured manner.
//!

#[ cfg( not( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithVec
  {
    #[ container ]
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

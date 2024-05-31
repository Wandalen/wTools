use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code ) ]
#[ derive( Deref, DerefMut ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

include!( "./only_tests/struct_named.rs" );

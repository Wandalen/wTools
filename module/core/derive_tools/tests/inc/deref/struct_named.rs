use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref) ]
struct StructNamed
{
  a : String,
  b : i32,
}

include!( "./only_tests/struct_named.rs" );

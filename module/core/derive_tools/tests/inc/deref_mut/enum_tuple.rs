use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code) ]
#[ derive( Deref, DerefMut ) ]
enum EnumTuple
{
  A( String, i32 ),
  B( String, i32 ),
}

include!( "./only_tests/enum_tuple.rs" );

use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code) ]
#[ derive( Deref ) ]
enum EnumTuple
{
  A( String, i32 ),
  B( String, i32 ),
}

include!( "./only_tests/enum_tuple.rs" );

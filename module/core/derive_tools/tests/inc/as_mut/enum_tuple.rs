use derive_tools::AsMut;

#[ allow( dead_code) ]
#[ derive( AsMut ) ]
enum EnumTuple
{
  A( String, i32 ),
  B( String, i32 ),
}

include!( "./only_test/enum_tuple.rs" );

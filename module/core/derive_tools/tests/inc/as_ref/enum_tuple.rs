use derive_tools::AsRef;

#[ allow( dead_code) ]
#[ derive( AsRef ) ]
enum EnumTuple
{
  A( String, i32 ),
  B( String, i32 ),
}

include!( "./only_test/enum_tuple.rs" );

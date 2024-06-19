use derive_tools::InnerFrom;

#[ allow( dead_code) ]
#[ derive( InnerFrom ) ]
enum EnumTuple
{
  A( String, i32 ),
  B( String, i32 ),
}

include!( "./only_test/enum_tuple.rs" );

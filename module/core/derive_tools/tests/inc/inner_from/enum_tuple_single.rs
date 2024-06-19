use derive_tools::InnerFrom;

#[ allow( dead_code) ]
#[ derive( InnerFrom ) ]
enum EnumTupleSingle
{
  A( i32 ),
  B( i32 ),
}

include!( "./only_test/enum_tuple_single.rs" );

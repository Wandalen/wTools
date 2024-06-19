use derive_tools::InnerFrom;

#[ allow( dead_code) ]
#[ derive( InnerFrom ) ]
enum EnumTupleEmpty
{
  A(),
  B(),
}

include!( "./only_test/enum_tuple_empty.rs" );

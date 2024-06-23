use derive_tools::InnerFrom;

#[ allow( dead_code) ]
#[ derive( InnerFrom ) ]
enum EnumUnit
{
  A,
  B,
}

include!( "./only_test/enum_unit.rs" );

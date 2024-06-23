use derive_tools::AsRef;

#[ allow( dead_code) ]
#[ derive( AsRef ) ]
enum EnumUnit
{
  A,
  B,
}

include!( "./only_test/enum_unit.rs" );

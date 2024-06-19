use derive_tools::InnerFrom;

#[ allow( dead_code) ]
#[ derive( InnerFrom ) ]
enum EnumNamedEmpty
{
  A {},
  B {},
}

include!( "./only_test/enum_named_empty.rs" );

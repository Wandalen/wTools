use derive_tools::AsRef;

#[ allow( dead_code) ]
#[ derive( AsRef ) ]
enum EnumNamedEmpty
{
  A {},
  B {},
}

include!( "./only_test/enum_named_empty.rs" );

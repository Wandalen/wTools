
use derive_tools::AsRef;

#[ allow( dead_code) ]
#[ derive( AsRef ) ]
enum EnumTupleEmpty
{
  A(),
  B(),
}

include!( "./only_test/enum_tuple_empty.rs" );

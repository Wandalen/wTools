use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code) ]
#[ derive( Deref ) ]
enum EnumTupleEmpty
{
  A(),
  B(),
}

include!( "./only_test/enum_tuple_empty.rs" );

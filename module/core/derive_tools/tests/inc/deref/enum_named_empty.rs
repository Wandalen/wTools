use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code) ]
#[ derive( Deref ) ]
enum EnumNamedEmpty
{
  A {},
  B {},
}

include!( "./only_test/enum_named_empty.rs" );

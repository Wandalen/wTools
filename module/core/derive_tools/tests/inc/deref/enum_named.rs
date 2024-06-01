use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code) ]
#[ derive( Deref ) ]
enum EnumNamed
{
  A { a : String, b : i32 },
  B { a : String, b : i32 },
}

include!( "./only_tests/enum_named.rs" );

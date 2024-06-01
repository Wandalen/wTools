use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code) ]
#[ derive( Deref, DerefMut ) ]
enum EnumNamed
{
  A { a : String, b : i32 },
  B { a : String, b : i32 },
}

include!( "./only_test/enum_named.rs" );

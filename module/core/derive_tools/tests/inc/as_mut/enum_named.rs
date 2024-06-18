use derive_tools::AsMut;

#[ allow( dead_code) ]
#[ derive( AsMut ) ]
enum EnumNamed
{
  A { a : String, b : i32 },
  B { a : String, b : i32 },
}

include!( "./only_test/enum_named.rs" );

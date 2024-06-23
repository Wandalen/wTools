use derive_tools::AsRef;

#[ allow( dead_code) ]
#[ derive( AsRef ) ]
enum EnumNamed
{
  A { a : String, b : i32 },
  B { a : String, b : i32 },
}

include!( "./only_test/enum_named.rs" );

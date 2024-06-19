use derive_tools::InnerFrom;

#[ allow( dead_code) ]
#[ derive( InnerFrom ) ]
enum EnumNamed
{
  A { a : String, b : i32 },
  B { a : String, b : i32 },
}

include!( "./only_test/enum_named.rs" );

use derive_tools::InnerFrom;

#[ allow( dead_code) ]
#[ derive( InnerFrom ) ]
enum EnumNamedSingle
{
  A { a : i32 },
  B { a : i32 },
}

include!( "./only_test/enum_named_single.rs" );

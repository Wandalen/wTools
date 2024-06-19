use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct StructNamedSingle
{
  a : i32,
}

include!( "./only_test/struct_named_single.rs" );

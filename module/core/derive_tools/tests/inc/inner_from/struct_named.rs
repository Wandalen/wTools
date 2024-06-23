use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

include!( "./only_test/struct_named.rs" );

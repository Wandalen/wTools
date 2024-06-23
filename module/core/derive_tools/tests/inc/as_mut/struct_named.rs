use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

include!( "./only_test/struct_named.rs" );

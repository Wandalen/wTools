use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

include!( "./only_test/struct_named.rs" );

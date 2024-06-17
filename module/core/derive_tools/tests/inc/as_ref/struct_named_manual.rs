#[ allow( dead_code ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

impl AsRef< String > for StructNamed
{
  fn as_ref( &self ) -> &String
  {
    &self.a
  }
}

include!( "./only_test/struct_named.rs" );

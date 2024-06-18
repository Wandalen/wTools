#[ allow( dead_code ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

impl AsMut< String > for StructNamed
{
  fn as_mut( &mut self ) -> &mut String
  {
    &mut self.a
  }
}

include!( "./only_test/struct_named.rs" );

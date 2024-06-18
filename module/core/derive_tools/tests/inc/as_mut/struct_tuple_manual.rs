#[ allow( dead_code ) ]
struct StructTuple( String, i32 );

impl AsMut< String > for StructTuple
{
  fn as_mut( &mut self ) -> &mut String
  {
    &mut self.0
  }
}

include!( "./only_test/struct_tuple.rs" );

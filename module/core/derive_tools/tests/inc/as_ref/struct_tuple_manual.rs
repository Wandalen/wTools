#[ allow( dead_code ) ]
struct StructTuple( String, i32 );

impl AsRef< String > for StructTuple
{
  fn as_ref( &self ) -> &String
  {
    &self.0
  }
}

include!( "./only_test/struct_tuple.rs" );

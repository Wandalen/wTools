#[ allow( dead_code ) ]
struct StructTuple( String, i32 );

impl From< StructTuple > for ( String, i32 )
{
  fn from( other : StructTuple ) -> Self
  {
    ( other.0, other.1 )
  }
}

include!( "./only_test/struct_tuple.rs" );

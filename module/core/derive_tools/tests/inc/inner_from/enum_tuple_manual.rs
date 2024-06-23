#[ allow( dead_code) ]
enum EnumTuple
{
  A( String, i32 ),
  B( String, i32 ),
}

impl From< EnumTuple > for ( String, i32 )
{
  fn from( other : EnumTuple ) -> Self
  {
    match other
    {
      EnumTuple::A( a, b ) | EnumTuple::B( a, b ) => ( a, b ),
    }
  }
}

include!( "./only_test/enum_tuple.rs" );

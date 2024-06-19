#[ allow( dead_code) ]
enum EnumTupleSingle
{
  A( i32 ),
  B( i32 ),
}

impl From< EnumTupleSingle > for i32
{
  fn from( other : EnumTupleSingle ) -> Self
  {
    match other
    {
      EnumTupleSingle::A( a ) | EnumTupleSingle::B( a ) => a,
    }
  }
}

include!( "./only_test/enum_tuple_single.rs" );

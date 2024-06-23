#[ allow( dead_code) ]
enum EnumTuple
{
  A( String, i32 ),
  B( String, i32 ),
}

impl AsMut< String > for EnumTuple
{
  fn as_mut( &mut self ) -> &mut String
  {
    match self
    {
      Self::A( v, .. ) | Self::B( v, .. ) => v
    }
  }
}

include!( "./only_test/enum_tuple.rs" );

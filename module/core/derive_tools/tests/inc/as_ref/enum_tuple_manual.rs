#[ allow( dead_code) ]
enum EnumTuple
{
  A( String, i32 ),
  B( String, i32 ),
}

impl AsRef< String > for EnumTuple
{
  fn as_ref( &self ) -> &String
  {
    match self
    {
      Self::A( v, .. ) | Self::B( v, .. ) => v
    }
  }
}

include!( "./only_test/enum_tuple.rs" );

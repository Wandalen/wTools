use core::ops::Deref;

#[ allow( dead_code) ]
enum EnumTuple
{
  A( String, i32 ),
  B( String, i32 ),
}

impl Deref for EnumTuple
{
  type Target = String;
  fn deref( &self ) -> &Self::Target
  {
    match self
    {
      Self::A( v, .. ) | Self::B( v, .. ) => v
    }
  }
}

include!( "./only_tests/enum_tuple.rs" );

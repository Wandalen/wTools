use core::ops::Deref;
use derive_tools::DerefMut;

#[ allow( dead_code) ]
#[ derive( DerefMut ) ]
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

include!( "./only_test/enum_tuple.rs" );

use core::ops::{ Deref, DerefMut };

#[ allow( dead_code) ]
enum EnumNamed
{
  A { a : String, b : i32 },
  B { a : String, b : i32 },
}

impl Deref for EnumNamed
{
  type Target = String;
  fn deref( &self ) -> &Self::Target
  {
    match self
    {
      Self::A { a : v, ..} | Self::B { a : v, .. } => v
    }
  }
}
impl DerefMut for EnumNamed
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    match self
    {
      Self::A { a : v, ..} | Self::B { a : v, .. } => v
    }
  }
}

include!( "./only_tests/enum_named.rs" );

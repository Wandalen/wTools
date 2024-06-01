use core::ops::{ Deref, DerefMut };

#[ allow( dead_code ) ]
struct StructNamed
{
  a : String,
  b : i32,
}

impl Deref for StructNamed
{
  type Target = String;
  fn deref( &self ) -> &Self::Target
  {
    &self.a
  }
}
impl DerefMut for StructNamed
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.a
  }
}

include!( "./only_tests/struct_named.rs" );

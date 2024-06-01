use core::ops::{ Deref, DerefMut };

#[ allow( dead_code ) ]
struct StructTuple( String, i32 );

impl Deref for StructTuple
{
  type Target = String;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl DerefMut for StructTuple
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

include!( "./only_tests/struct_tuple.rs" );

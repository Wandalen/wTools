use core::ops::{ Deref, DerefMut };

#[ allow( dead_code ) ]
struct GenericsConstants< const N : usize >( i32 );

impl< const N : usize > Deref for GenericsConstants< N >
{
  type Target = i32;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl< const N : usize > DerefMut for GenericsConstants< N >
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

include!( "./only_tests/generics_constants.rs" );

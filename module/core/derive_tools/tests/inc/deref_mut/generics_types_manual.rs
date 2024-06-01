use core::ops::{ Deref, DerefMut };

#[ allow( dead_code ) ]
struct GenericsTypes< T >( T );

impl< T > Deref for GenericsTypes< T >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl< T > DerefMut for GenericsTypes< T >
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

include!( "./only_test/generics_types.rs" );

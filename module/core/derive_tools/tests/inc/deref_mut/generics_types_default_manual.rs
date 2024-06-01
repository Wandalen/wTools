use core::ops::{ Deref, DerefMut };

#[ allow( dead_code ) ]
struct GenericsTypesDefault< T = i32 >( T );

impl< T > Deref for GenericsTypesDefault< T >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl< T > DerefMut for GenericsTypesDefault< T >
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

include!( "./only_tests/generics_types_default.rs" );

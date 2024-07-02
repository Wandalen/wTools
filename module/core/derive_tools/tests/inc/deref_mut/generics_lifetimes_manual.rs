use core::ops::{ Deref, DerefMut };

#[ allow( dead_code ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

impl< 'a > Deref for GenericsLifetimes< 'a >
{
  type Target = &'a i32;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl< 'a > DerefMut for GenericsLifetimes< 'a >
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

include!( "./only_test/generics_lifetimes.rs" );

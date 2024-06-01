use core::ops::{ Deref, DerefMut };

#[ allow( dead_code ) ]
struct GenericsConstantsDefault< const N : usize = 0 >( i32 );

impl< const N : usize > Deref for GenericsConstantsDefault< N >
{
  type Target = i32;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}
impl< const N : usize > DerefMut for GenericsConstantsDefault< N >
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

include!( "./only_tests/generics_constants_default.rs" );

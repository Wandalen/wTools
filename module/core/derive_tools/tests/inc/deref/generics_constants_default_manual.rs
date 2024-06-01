use core::ops::Deref;

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

include!( "./only_test/generics_constants_default.rs" );

use core::ops::Deref;
use derive_tools::DerefMut;

#[ allow( dead_code ) ]
#[ derive( DerefMut ) ]
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

use core::ops::Deref;
use derive_tools::{ DerefMut };

#[ allow( dead_code ) ]
#[ derive( DerefMut ) ]
struct GenericsConstants< const N : usize >( i32 );

impl< const N : usize > Deref for GenericsConstants< N >
{
  type Target = i32;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

include!( "./only_test/generics_constants.rs" );

use core::ops::Deref;
use derive_tools::DerefMut;

#[ allow( dead_code ) ]
#[ derive( DerefMut ) ]
struct GenericsTypes< T >( T );

impl< T > Deref for GenericsTypes< T >
{
  type Target = T;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

include!( "./only_test/generics_types.rs" );
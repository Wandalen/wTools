use core::ops::Deref;
use derive_tools::DerefMut;

#[ allow( dead_code ) ]
#[ derive( DerefMut ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

impl< 'a > Deref for GenericsLifetimes< 'a >
{
  type Target = &'a i32;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

include!( "./only_test/generics_lifetimes.rs" );

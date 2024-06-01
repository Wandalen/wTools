use core::ops::Deref;

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

include!( "./only_tests/generics_lifetimes.rs" );

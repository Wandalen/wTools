use core::ops::Deref;

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

include!( "./only_test/generics_types_default.rs" );

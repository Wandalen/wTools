#[ allow( dead_code ) ]
struct GenericsTypes< T >( T );

impl< T > AsMut< T > for GenericsTypes< T >
{
  fn as_mut( &mut self ) -> &mut T
  {
    &mut self.0
  }
}

include!( "./only_test/generics_types.rs" );

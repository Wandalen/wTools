#[ allow( dead_code ) ]
struct GenericsTypesDefault< T = i32 >( T );

impl< T > AsMut< T > for GenericsTypesDefault< T >
{
  fn as_mut( &mut self ) -> &mut T
  {
    &mut self.0
  }
}

include!( "./only_test/generics_types_default.rs" );

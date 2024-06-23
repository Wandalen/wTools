#[ allow( dead_code ) ]
struct GenericsTypes< T >( T );

impl< T > AsRef< T > for GenericsTypes< T >
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

include!( "./only_test/generics_types.rs" );

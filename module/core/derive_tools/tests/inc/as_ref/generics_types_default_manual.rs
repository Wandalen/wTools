#[ allow( dead_code ) ]
struct GenericsTypesDefault< T = i32 >( T );

impl< T > AsRef< T > for GenericsTypesDefault< T >
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

include!( "./only_test/generics_types_default.rs" );

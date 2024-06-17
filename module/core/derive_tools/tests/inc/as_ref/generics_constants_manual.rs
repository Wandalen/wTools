#[ allow( dead_code ) ]
struct GenericsConstants< const N : usize >( i32 );

impl< const N : usize > AsRef< i32 > for GenericsConstants< N >
{
  fn as_ref( &self ) -> &i32
  {
    &self.0
  }
}

include!( "./only_test/generics_constants.rs" );

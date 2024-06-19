#[ allow( dead_code ) ]
struct GenericsConstants< const N : usize >( i32 );

impl< const N : usize > From< GenericsConstants< N > > for i32
{
  fn from( other : GenericsConstants< N > ) -> Self
  {
    other.0
  }
}

include!( "./only_test/generics_constants.rs" );

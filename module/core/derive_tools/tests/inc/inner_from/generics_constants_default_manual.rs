#[ allow( dead_code ) ]
struct GenericsConstantsDefault< const N : usize = 0 >( i32 );

impl< const N : usize > From< GenericsConstantsDefault< N > > for i32
{
  fn from( other : GenericsConstantsDefault< N > ) -> Self
  {
    other.0
  }
}

include!( "./only_test/generics_constants_default.rs" );

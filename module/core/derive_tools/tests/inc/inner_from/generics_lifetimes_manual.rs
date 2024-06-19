#[ allow( dead_code ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

impl< 'a > From< GenericsLifetimes< 'a > > for &'a i32
{
  fn from( other : GenericsLifetimes< 'a > ) -> Self
  {
    other.0
  }
}

include!( "./only_test/generics_lifetimes.rs" );

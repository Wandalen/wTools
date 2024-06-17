#[ allow( dead_code ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

impl< 'a > AsRef< &'a i32 > for GenericsLifetimes< 'a >
{
  fn as_ref( &self ) -> &&'a i32
  {
    &self.0
  }
}

include!( "./only_test/generics_lifetimes.rs" );

#[ allow( dead_code ) ]
struct GenericsLifetimes< 'a >( &'a i32 );

impl< 'a > AsMut< &'a i32 > for GenericsLifetimes< 'a >
{
  fn as_mut( &mut self ) -> &mut &'a i32
  {
    &mut self.0
  }
}

include!( "./only_test/generics_lifetimes.rs" );

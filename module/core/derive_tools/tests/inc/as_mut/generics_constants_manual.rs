#[ allow( dead_code ) ]
struct GenericsConstants< const N : usize >( i32 );

impl< const N : usize > AsMut< i32 > for GenericsConstants< N >
{
  fn as_mut( &mut self ) -> &mut i32
  {
    &mut self.0
  }
}

include!( "./only_test/generics_constants.rs" );

#[ allow( dead_code ) ]
struct GenericsConstantsDefault< const N : usize = 0 >( i32 );

impl< const N : usize > AsMut< i32 > for GenericsConstantsDefault< N >
{
  fn as_mut( &mut self ) -> &mut i32
  {
    &mut self.0
  }
}

include!( "./only_test/generics_constants_default.rs" );

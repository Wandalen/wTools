#[ allow( dead_code ) ]
struct GenericsConstantsDefault< const N : usize = 0 >( i32 );

impl< const N : usize > AsRef< i32 > for GenericsConstantsDefault< N >
{
  fn as_ref( &self ) -> &i32
  {
    &self.0
  }
}

include!( "./only_test/generics_constants_default.rs" );

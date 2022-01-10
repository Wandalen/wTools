#![ allow( unused_imports ) ]
// #![ allow( unused_attributes ) ]
// #![ feature( type_name_of_val ) ]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

use wtest_basic::*;

#[cfg( feature = "with_wtools" )]
use wtools::typing as TheModule;
#[cfg( not( feature = "with_wtools" ) )]
use inspect_type as TheModule;

//

#[ cfg( feature = "nightly" ) ]
fn _inspect_to_str_type_of()
{

  let exp = "sizeof( &[1, 2, 3][..] : &[i32] ) = 16".to_string();
  let got = TheModule::inspect_to_str_type_of!( &[ 1, 2, 3 ][ .. ] );
  assert_eq!( got, exp );

  let exp = "sizeof( &[1, 2, 3] : &[i32; 3] ) = 8".to_string();
  let got = TheModule::inspect_to_str_type_of!( &[ 1, 2, 3 ] );
  assert_eq!( got, exp );

}

//

#[ cfg( feature = "nightly" ) ]
fn _inspect_type_of()
{

  let exp = "sizeof( &[1, 2, 3][..] : &[i32] ) = 16".to_string();
  let got = TheModule::inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  assert_eq!( got, exp );

  let exp = "sizeof( &[1, 2, 3] : &[i32; 3] ) = 8".to_string();
  let got = TheModule::inspect_type_of!( &[ 1, 2, 3 ] );
  assert_eq!( got, exp );

}

//

// #[ rustversion::nightly ]
#[ cfg( feature = "nightly" ) ]
test_suite!
{
  inspect_to_str_type_of,
  inspect_type_of,
}

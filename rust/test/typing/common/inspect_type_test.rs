#[ allow( unused_imports ) ]
use wtest_basic::*;

/* xxx : qqq : make sure CD run test on both stable and nightly channels */
/* xxx : qqq : make sure CD run debug tests and release tests */
/* xxx : qqq : introduce tag to run fewer tests */

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

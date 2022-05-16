#[ allow( unused_imports ) ]
use test_tools::*;

//

#[ cfg( feature = "nightly" ) ]
fn inspect_to_str_type_of_test()
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
fn inspect_type_of_test()
{

  let exp = "sizeof( &[1, 2, 3][..] : &[i32] ) = 16".to_string();
  let got = TheModule::inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  assert_eq!( got, exp );

  let exp = "sizeof( &[1, 2, 3] : &[i32; 3] ) = 8".to_string();
  let got = TheModule::inspect_type_of!( &[ 1, 2, 3 ] );
  assert_eq!( got, exp );

}

//

#[ cfg( feature = "nightly" ) ]
test_suite!
{
  inspect_to_str_type_of,
  inspect_type_of,
}
/* xxx : move cfg */
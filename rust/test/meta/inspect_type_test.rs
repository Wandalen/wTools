#![ feature( type_name_of_val ) ]

use inspect_type as TheModule;
use wtest_basic::*;

//

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

test_suite!
{
  inspect_to_str_type_of,
  inspect_type_of,
}

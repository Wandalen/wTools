
use super :: *;

//

#[ test ]
fn inspect_to_str_type_of_test()
{

  let exp = "sizeof( &[1, 2, 3][..] : &[i32] ) = 16".to_string();
  let got = the_module ::inspect_to_str_type_of!( &[ 1, 2, 3 ][ .. ] );
  assert_eq!( got, exp );

  let exp = "sizeof( &[1, 2, 3] : &[i32; 3] ) = 8".to_string();
  let got = the_module ::inspect_to_str_type_of!( &[ 1, 2, 3 ] );
  assert_eq!( got, exp );

}

//

#[ test ]
fn inspect_type_of_macro()
{

  let exp = "sizeof( &[1, 2, 3][..] : &[i32] ) = 16".to_string();
  let got = the_module ::inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  assert_eq!( got, exp );

  let exp = "sizeof( &[1, 2, 3] : &[i32; 3] ) = 8".to_string();
  let got = the_module ::inspect_type_of!( &[ 1, 2, 3 ] );
  assert_eq!( got, exp );

}

// #[ allow( unused_imports ) ]
// use test_tools::*;
#[ allow( unused_imports ) ]
use super::*;

//

#[ cfg( feature = "nightly" ) ]
tests_impls!
{

  #[ test ]
  fn inspect_to_str_type_of_test()
  {

    let exp = "sizeof( &[1, 2, 3][..] : &[i32] ) = 16".to_string();
    let got = TheModule::inspect_to_str_type_of!( &[ 1, 2, 3 ][ .. ] );
    a_id!( got, exp );

    let exp = "sizeof( &[1, 2, 3] : &[i32; 3] ) = 8".to_string();
    let got = TheModule::inspect_to_str_type_of!( &[ 1, 2, 3 ] );
    a_id!( got, exp );

  }

  //

  #[ test ]
  fn inspect_type_of()
  {

    let exp = "sizeof( &[1, 2, 3][..] : &[i32] ) = 16".to_string();
    let got = TheModule::inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
    a_id!( got, exp );

    let exp = "sizeof( &[1, 2, 3] : &[i32; 3] ) = 8".to_string();
    let got = TheModule::inspect_type_of!( &[ 1, 2, 3 ] );
    a_id!( got, exp );

  }

}

//

#[ cfg( feature = "nightly" ) ]
tests_index!
{
  inspect_to_str_type_of_test,
  inspect_type_of,
}

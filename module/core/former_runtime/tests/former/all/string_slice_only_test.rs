#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

//

tests_impls!
{
  fn test_complex()
  {
    // test.case( "default" );

    let command = Struct1::former()
    .form();
    let expected = Struct1
    {
      string_slice_1 : "",
    };
    a_id!( command, expected );

    // test.case( "set value" );

    let command = Struct1::former()
    .string_slice_1( "abc" )
    .form();
    let expected = Struct1
    {
      string_slice_1 : "abc",
    };
    a_id!( command, expected );
  }
}

//

tests_index!
{
  test_complex,
}

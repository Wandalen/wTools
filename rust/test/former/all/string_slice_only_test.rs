#[ allow( unused_imports ) ]
use super::*;
use test_tools::dependencies::*;

//

#[ test ]
fn test_complex() -> anyhow::Result< () >
{

  // test.case( "default" );

  let command = Struct1::former()
  .form();
  let expected = Struct1
  {
    string_slice_1 : "",
  };
  assert_eq!( command, expected );

  // test.case( "set value" );

  let command = Struct1::former()
  .string_slice_1( "abc" )
  .form();
  let expected = Struct1
  {
    string_slice_1 : "abc",
  };
  assert_eq!( command, expected );

  Ok( () )
}

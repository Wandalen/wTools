#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Struct1
{
  #[ former( default = 31 ) ]
  pub int_1 : i32,
  #[ former( default = "abc" ) ]
  string_optional_1 : Option< String >,
}

//

tests_impls!
{
  fn test_complex()
  {
    let command = Struct1::former().form();

    let expected = Struct1
    {
      int_1 : 31,
      string_optional_1 : Some( "abc".to_string() ),
    };
    a_id!( command, expected );
  }
}

//

tests_index!
{
  test_complex,
}
